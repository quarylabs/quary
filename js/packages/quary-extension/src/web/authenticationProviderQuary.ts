import {
  authentication,
  AuthenticationProvider,
  AuthenticationProviderAuthenticationSessionsChangeEvent,
  AuthenticationSession,
  Disposable,
  env,
  EventEmitter,
  CancellationToken,
  ExtensionContext,
  ProgressLocation,
  Uri,
  UriHandler,
  window,
} from 'vscode'
import type { Analytics } from '@june-so/analytics-node'
import { Err, isErr, Ok, ResultE } from '@shared/result'
import { ServicesLogger } from './servicesLogger'

const AUTH_TYPE = `quaryDevice`
const AUTH_NAME = `Quary`
const CLIENT_ID = `0xOrIg1kFRxbE3rwvzBpjOCK9ZA4SbAT`
const AUTH0_DOMAIN = `idp.quary.dev`
const SESSIONS_SECRET_STORE_KEY = `${AUTH_TYPE}.sessions`
const SCOPES = ['openid', 'profile', 'email']

const POLLING_INTERVAL = 1000 * 5 // 5 seconds
const MAX_ATTEMPTS = 12 // 5*12 = 1 minute

interface DeviceAuthDetails {
  user_code: string
  verification_uri: string
  device_code: string
}

interface TokenResponse {
  access_token: string
  refresh_token?: string
  expires_in: number
}

interface UserInfo {
  sub: string
  name?: string
  email?: string
  picture?: string
}

/**
 * -----------------------------------------------------
 * AuthenticationProvider - Quary (Auth0 w/ Device Flow)
 * ------------------------------------------------------
 */

// Helper
class UriEventHandler extends EventEmitter<Uri> implements UriHandler {
  public handleUri(uri: Uri) {
    this.fire(uri)
  }
}

export class AuthenticationProviderQuary
  implements AuthenticationProvider, Disposable
{
  private readonly logger: ServicesLogger
  private readonly analytics: Analytics
  private _sessionChangeEmitter =
    new EventEmitter<AuthenticationProviderAuthenticationSessionsChangeEvent>()
  private _disposable: Disposable
  private _uriHandler = new UriEventHandler()

  constructor(
    private readonly context: ExtensionContext,
    logger: ServicesLogger,
    analytics: Analytics,
  ) {
    this._disposable = Disposable.from(
      authentication.registerAuthenticationProvider(
        AUTH_TYPE,
        AUTH_NAME,
        this,
        { supportsMultipleAccounts: false },
      ),
      window.registerUriHandler(this._uriHandler),
    )
    this.logger = logger
    this.analytics = analytics
  }

  get onDidChangeSessions() {
    return this._sessionChangeEmitter.event
  }

  public async getSessions(): Promise<readonly AuthenticationSession[]> {
    const allSessions = await this.context.secrets.get(
      SESSIONS_SECRET_STORE_KEY,
    )
    return allSessions ? JSON.parse(allSessions) : []
  }

  public async createSession(): Promise<AuthenticationSession> {
    return window.withProgress(
      {
        location: ProgressLocation.Notification,
        title: 'Signing in to Quary...',
        cancellable: true,
      },
      async (_, token) => {
        try {
          const loginResult = await this.login()
          if (isErr(loginResult)) {
            const errorMessage = loginResult.error
            window.showErrorMessage(`Authentication failed: ${errorMessage}`)
            throw new Error(errorMessage)
          }

          /* eslint-disable camelcase */
          const { user_code, verification_uri, device_code } = loginResult.value
          env.openExternal(
            Uri.parse(`${verification_uri}?user_code=${user_code}`),
          )
          /* eslint-enable camelcase */

          const pollResult = await this.pollForToken(device_code, token)
          if (isErr(pollResult)) {
            const errorMessage = pollResult.error
            window.showErrorMessage(`Authentication failed: ${errorMessage}`)
            throw new Error(errorMessage)
          }
          window.showInformationMessage('Authentication succesful ✅')
          return pollResult.value
        } catch (error) {
          const errorMessage =
            error instanceof Error ? error.message : 'Unknown error'
          window.showErrorMessage(`Authentication failed: ${errorMessage}`)
          throw new Error(errorMessage)
        }
      },
    )
  }

  private async pollForToken(
    deviceCode: string,
    cancellationToken: CancellationToken,
  ): Promise<ResultE<AuthenticationSession, string>> {
    let attempts = 0
    while (attempts < MAX_ATTEMPTS) {
      try {
        const tokenResult = await this.requestToken(deviceCode)
        if (isErr(tokenResult)) {
          const errorMessage = tokenResult.error
          if (errorMessage !== 'authorization_pending') {
            return Err(errorMessage)
          }
        } else {
          const tokenResponse = tokenResult.value
          if (tokenResponse.access_token) {
            const session = await this.createSessionFromToken(
              tokenResponse.access_token,
            )
            if (isErr(session)) {
              return session
            }
            return Ok(session.value)
          }
        }
      } catch (error) {
        const errorMessage =
          error instanceof Error ? error.message : 'Unknown error'
        return Err(errorMessage)
      }

      if (cancellationToken.isCancellationRequested) {
        return Err('Authentication cancelled by user')
      }

      await new Promise((resolve) => setTimeout(resolve, POLLING_INTERVAL))
      attempts++
    }
    return Err('Authentication timeout')
  }

  private async login(): Promise<ResultE<DeviceAuthDetails, string>> {
    try {
      const response = await fetch(
        `https://${AUTH0_DOMAIN}/oauth/device/code`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
          body: `client_id=${CLIENT_ID}&scope=${SCOPES.join(' ')}`,
        },
      )

      if (!response.ok) {
        return Err('Failed to initiate login')
      }

      return Ok(await response.json())
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (error) {
      return Err('Network error during login')
    }
  }

  private async requestToken(
    deviceCode: string,
  ): Promise<ResultE<TokenResponse, string>> {
    try {
      const response = await fetch(`https://${AUTH0_DOMAIN}/oauth/token`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: `grant_type=urn:ietf:params:oauth:grant-type:device_code&device_code=${deviceCode}&client_id=${CLIENT_ID}`,
      })

      if (!response.ok) {
        const errorResponse = await response.json()
        throw new Error(errorResponse.error)
      }
      return Ok(await response.json())
    } catch (error) {
      const errorMessage =
        error instanceof Error
          ? error.message
          : 'Network error during token request'
      return Err(errorMessage)
    }
  }

  private async createSessionFromToken(
    token: string,
  ): Promise<ResultE<AuthenticationSession, string>> {
    try {
      const userInfoResponse = await this.getUserInfo(token)
      if (isErr(userInfoResponse)) {
        return Err(userInfoResponse.error)
      }

      const userInfo = userInfoResponse.value
      const session: AuthenticationSession = {
        id: userInfo.sub,
        accessToken: token,
        account: {
          id: userInfo.sub,
          label: userInfo.name || userInfo.email || 'Unknown User',
        },
        scopes: SCOPES,
      }

      // Store the new session
      const allSessions = await this.context.secrets.get(
        SESSIONS_SECRET_STORE_KEY,
      )
      const sessions: AuthenticationSession[] = allSessions
        ? JSON.parse(allSessions)
        : []
      sessions.push(session)
      await this.context.secrets.store(
        SESSIONS_SECRET_STORE_KEY,
        JSON.stringify(sessions),
      )

      this._sessionChangeEmitter.fire({
        added: [session],
        removed: [],
        changed: [],
      })

      this.logger.setUser({
        id: userInfo.sub,
        email: userInfo.email || '',
      })
      this.analytics.identify({
        userId: userInfo.sub,
        traits: {
          email: userInfo.email,
          name: userInfo.name,
          avatar: userInfo.picture,
        },
      })
      this.analytics.track({
        userId: userInfo.sub,
        event: 'Signed In',
        properties: {
          environment: env.appHost,
        },
      })

      return Ok(session)
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : 'Unknown error'
      return Err(errorMessage)
    }
  }

  private async getUserInfo(token: string): Promise<ResultE<UserInfo, string>> {
    try {
      const response = await fetch(`https://${AUTH0_DOMAIN}/userinfo`, {
        headers: { Authorization: `Bearer ${token}` },
      })

      if (!response.ok) {
        return Err('Failed to fetch user info')
      }

      return Ok(await response.json())
    } catch (error) {
      const errorMessage =
        error instanceof Error
          ? error.message
          : 'Network error during user info retrieval'
      return Err(errorMessage)
    }
  }

  public async removeSession(sessionId: string): Promise<void> {
    const allSessions = await this.context.secrets.get(
      SESSIONS_SECRET_STORE_KEY,
    )
    if (allSessions) {
      const sessions = JSON.parse(allSessions) as AuthenticationSession[]
      const sessionIdx = sessions.findIndex((s) => s.id === sessionId)

      if (sessionIdx !== -1) {
        const removedSession = sessions[sessionIdx]
        sessions.splice(sessionIdx, 1)

        await this.context.secrets.store(
          SESSIONS_SECRET_STORE_KEY,
          JSON.stringify(sessions),
        )

        this.logger.setUser(null)

        this._sessionChangeEmitter.fire({
          added: [],
          removed: [removedSession],
          changed: [],
        })
      } else {
        window.showErrorMessage(`Unable to sign out, session not found`)
      }
    }
  }

  public dispose(): void {
    this._disposable.dispose()
  }
}
