import {
  authentication,
  AuthenticationProvider,
  AuthenticationProviderAuthenticationSessionsChangeEvent,
  AuthenticationSession,
  Disposable,
  EventEmitter,
  ExtensionContext,
  Uri,
  window,
  env,
  ProgressLocation,
} from 'vscode'
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import {
  SnowflakeOauthProxyRequest,
  SnowflakeOauthRefreshToken,
  SnowflakeOauthToken,
} from '@quary/proto/quary/service/v1/connection_response'

const AUTH_TYPE = 'quarySnowflake'
const AUTH_NAME = 'Quary: Snowflake'

const WEBSITE_URL = 'https://www.quary.dev'
const SNOWFLAKE_START = `${WEBSITE_URL}/api/quary-auth/snowflake/start`
const SNOWFLAKE_REFRESH = `${WEBSITE_URL}/api/quary-auth/snowflake/refresh`
const SNOWFLAKE_REFRESH_BUFFER = 1000 * 60 * 2 // 2 minutes in milliseconds
const SESSIONS_SECRET_STORE_KEY = `${AUTH_TYPE}.sessions`

interface SnowflakeAuthenticationSession extends AuthenticationSession {
  refreshToken: string
  expiryTime: number
  accountUrl: string
  clientId: string
  clientSecret: string
}

/**
 * --------------------------------------
 * AuthenticationProvider - Snowflake SSO
 * --------------------------------------
 */

export class AuthenticationProviderSnowflake
  implements AuthenticationProvider, Disposable
{
  private _sessionChangeEmitter =
    new EventEmitter<AuthenticationProviderAuthenticationSessionsChangeEvent>()
  private _disposable: Disposable

  constructor(private readonly context: ExtensionContext) {
    this._disposable = Disposable.from(
      authentication.registerAuthenticationProvider(
        AUTH_TYPE,
        AUTH_NAME,
        this,
        {
          supportsMultipleAccounts: false,
        },
      ),
    )
  }

  get onDidChangeSessions() {
    return this._sessionChangeEmitter.event
  }

  public async createSession(
    scopes: string[],
  ): Promise<SnowflakeAuthenticationSession> {
    const [accountUrl, clientId, clientSecret, role] = scopes // TODO: investigate better way to parse these to the auth provider

    if (!accountUrl || !clientId || !clientSecret || !role) {
      throw Err(new Error('Missing required snowflake auth paramaters'))
    }

    return window.withProgress(
      {
        location: ProgressLocation.Notification,
        title: 'Signing in to Snowflake...',
        cancellable: true,
      },
      async (_, cancellationToken) => {
        const snowflakeParamsEncoded = SnowflakeOauthProxyRequest.encode({
          clientId,
          clientSecret,
          accountUrl,
          role,
        }).finish()

        const snowflakeParamsEncodedBase64 = Buffer.from(
          snowflakeParamsEncoded,
        ).toString('base64')

        const params = new URLSearchParams()
        params.append('params', snowflakeParamsEncodedBase64)

        // Construct the full URL
        const snowflakeStartUrl = new URL(SNOWFLAKE_START)
        snowflakeStartUrl.search = params.toString()

        env.openExternal(Uri.parse(snowflakeStartUrl.toString()))

        const proceed = await window.showInformationMessage(
          'You will be redirected to get a token. Please click proceed to enter it.',
          'Proceed',
        )
        if (!proceed) {
          throw Err(new Error('Authentication cancelled by user'))
        }
        const tokenStringResult = await this.promptForToken(
          'Paste the authentication token here',
        )

        if (isErr(tokenStringResult)) {
          window.showErrorMessage(tokenStringResult.error.message)
          throw tokenStringResult.error
        }

        const tokens = await this.extractTokens(tokenStringResult.value)
        if (isErr(tokens)) {
          window.showErrorMessage(tokens.error.message)
          throw tokens.error
        }
        const { accessToken, refreshToken, expiryTime } = tokens.value

        if (cancellationToken.isCancellationRequested) {
          throw new Error('Authentication cancelled by user')
        }
        const userInfo = await this.getUserInfo(
          tokens.value.accessToken,
          accountUrl,
        )
        if (isErr(userInfo)) {
          throw new Error(
            `Failed to fetch user info: ${userInfo.error.message}`,
          )
        }
        const { user, role: retrievedRole } = userInfo.value
        const session: SnowflakeAuthenticationSession = {
          id: user,
          accessToken,
          refreshToken,
          expiryTime: parseInt(expiryTime),
          accountUrl,
          clientId,
          clientSecret,
          account: {
            id: user,
            label: `${user} | ${retrievedRole}` || 'Snowflake User',
          },
          scopes,
        }

        this.storeSession(session)
        return session
      },
    )
  }

  private async storeSession(
    session: SnowflakeAuthenticationSession,
  ): Promise<void> {
    const allSessions = await this.getSessions()
    const updatedSessions = allSessions.filter((s) => s.id !== session.id)
    updatedSessions.push(session)
    await this.context.secrets.store(
      SESSIONS_SECRET_STORE_KEY,
      JSON.stringify(updatedSessions),
    )
    this._sessionChangeEmitter.fire({
      added: [session],
      removed: [],
      changed: [],
    })
    window.showInformationMessage('Authentication succesful ✅')
  }

  public async removeSession(sessionId: string): Promise<void> {
    const currentSession = await this.getSessions()
    if (currentSession.length === 0 || currentSession[0].id !== sessionId) {
      throw new Error('No matching session found to remove.')
    }

    await this.context.secrets.store(
      SESSIONS_SECRET_STORE_KEY,
      JSON.stringify([]),
    )

    this._sessionChangeEmitter.fire({
      added: [],
      removed: [currentSession[0]],
      changed: [],
    })
  }

  public async getSessions(): Promise<SnowflakeAuthenticationSession[]> {
    const allSessions = await this.context.secrets.get(
      SESSIONS_SECRET_STORE_KEY,
    )
    if (!allSessions || JSON.parse(allSessions).length < 1) {
      return []
    }

    const session = JSON.parse(allSessions)[0]

    // check token expiry
    if (Date.now() > session.expiryTime - SNOWFLAKE_REFRESH_BUFFER) {
      const refreshSessionResult = await this.refreshSession(session)
      if (isErr(refreshSessionResult)) {
        return []
      }
      return [refreshSessionResult.value]
    }
    return [session]
  }

  private async refreshSession(
    session: SnowflakeAuthenticationSession,
  ): Promise<Result<SnowflakeAuthenticationSession>> {
    const response = await fetch(SNOWFLAKE_REFRESH, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        accountUrl: session.accountUrl,
        refreshToken: session.refreshToken,
        clientId: session.clientId,
        clientSecret: session.clientSecret,
      }),
    })

    if (!response.ok) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Failed to refresh token: ${response.statusText}`,
      })
    }

    const { serializedTokens } = await response.json()
    if (!serializedTokens) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: 'Invalid response received from the token refresh endpoint',
      })
    }

    const buffer = Buffer.from(serializedTokens, 'base64')
    const { accessToken, expiryTime } =
      SnowflakeOauthRefreshToken.decode(buffer)

    const refreshedSession = {
      ...session,
      accessToken,
      expiryTime: parseInt(expiryTime),
    }
    this.storeSession(refreshedSession)
    return Ok(refreshedSession)
  }

  private async getUserInfo(
    authToken: string,
    accountUrl: string,
  ): Promise<Result<{ user: string; role: string }>> {
    const body = JSON.stringify({
      statement: 'SELECT CURRENT_USER(), CURRENT_ROLE()',
    })
    const response = await fetch(`${accountUrl}/api/v2/statements`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${authToken}`,
        'Content-Type': 'application/json',
      },
      body,
    })
    if (!response.ok) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Failed to fetch user info: ${response.statusText}`,
      })
    }
    const responseData = await response.json()
    const [user, role] = responseData.data[0]
    return Ok({ user, role })
  }

  private async promptForToken(title: string): Promise<Result<string>> {
    const input = await window.showInputBox({ title })
    if (!input) {
      return Err({
        code: ErrorCodes.INVALID_ARGUMENT,
        message: 'Token was not provided',
      })
    }
    return Ok(input)
  }

  private async extractTokens(
    serializedOauthToken: string,
  ): Promise<Result<SnowflakeOauthToken>> {
    const buffer = Buffer.from(serializedOauthToken, 'base64')

    const deserializedTokens = SnowflakeOauthToken.decode(buffer)

    return Ok(deserializedTokens)
  }

  public dispose(): void {
    this._disposable.dispose()
  }
}
