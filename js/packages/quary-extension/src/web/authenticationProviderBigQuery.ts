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
  BigQueryOauthToken,
  BigQueryOauthTokenRefresh,
} from '@quary/proto/quary/service/v1/connection_response'

const AUTH_TYPE = 'quaryBigQuery'
const AUTH_NAME = 'Quary: BigQuery'

const BIGQUERY_PROXY_ROUTE = 'https://www.quary.dev/api/quary-auth/bigquery'
const BIGQUERY_TOKEN_URL = `${BIGQUERY_PROXY_ROUTE}/start`
const BIGQUERY_REFRESH = `${BIGQUERY_PROXY_ROUTE}/refresh`
const BIGQUERY_REFRESH_BUFFER = 1000 * 60 * 5 // 5 minutes in milliseconds
const BIGQUERY_USER_INFO_URL = 'https://www.googleapis.com/oauth2/v2/userinfo'
const SESSIONS_SECRET_STORE_KEY = `${AUTH_TYPE}.sessions`

interface BigQueryAuthenticationSession extends AuthenticationSession {
  refreshToken: string
  expiryTime: number
}

/**
 * ---------------------------------------------------------
 * AuthenticationProvider - BigQuery database authentication
 * ---------------------------------------------------------
 */

export class AuthenticationProviderBigQuery
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
  ): Promise<BigQueryAuthenticationSession> {
    return window.withProgress(
      {
        location: ProgressLocation.Notification,
        title: 'Signing in to BigQuery...',
        cancellable: true,
      },
      async (_, cancellationToken) => {
        env.openExternal(Uri.parse(BIGQUERY_TOKEN_URL))
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
        const userInfo = await this.getUserInfo(tokens.value.accessToken)
        if (isErr(userInfo)) {
          throw new Error(
            `Failed to fetch user info: ${userInfo.error.message}`,
          )
        }
        const { sub, name } = userInfo.value
        const session: BigQueryAuthenticationSession = {
          id: sub,
          accessToken,
          refreshToken,
          expiryTime: parseInt(expiryTime),
          account: {
            id: sub,
            label: name || 'BigQuery User',
          },
          scopes,
        }

        this.storeSession(session)
        return session
      },
    )
  }

  private async storeSession(
    session: BigQueryAuthenticationSession,
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

  public async getSessions(): Promise<BigQueryAuthenticationSession[]> {
    const allSessions = await this.context.secrets.get(
      SESSIONS_SECRET_STORE_KEY,
    )
    if (!allSessions || JSON.parse(allSessions).length < 1) {
      return []
    }

    const session = JSON.parse(allSessions)[0]

    // check token expiry
    if (Date.now() > session.expiryTime - BIGQUERY_REFRESH_BUFFER) {
      const refreshSessionResult = await this.refreshSession(session)
      if (isErr(refreshSessionResult)) {
        return []
      }
      return [refreshSessionResult.value]
    }
    return [session]
  }

  private async refreshSession(
    session: BigQueryAuthenticationSession,
  ): Promise<Result<BigQueryAuthenticationSession>> {
    const response = await fetch(BIGQUERY_REFRESH, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ refreshToken: session.refreshToken }),
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
    const { accessToken, expiryTime } = BigQueryOauthTokenRefresh.decode(buffer)

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
  ): Promise<Result<{ sub: string; name: string }>> {
    const response = await fetch(BIGQUERY_USER_INFO_URL, {
      headers: { Authorization: `Bearer ${authToken}` },
    })
    if (!response.ok) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Failed to fetch user info: ${response.statusText}`,
      })
    }
    return Ok(await response.json())
  }

  private async promptForToken(title: string): Promise<Result<string>> {
    const input = await window.showInputBox({ title })
    if (!input) {
      return Err({
        code: ErrorCodes.INVALID_ARGUMENT,
        message: 'No token provided',
      })
    }
    return Ok(input)
  }

  private async extractTokens(
    serializedOauthToken: string,
  ): Promise<Result<BigQueryOauthToken>> {
    const buffer = Buffer.from(serializedOauthToken, 'base64')

    const deserializedTokens = BigQueryOauthToken.decode(buffer)

    return Ok(deserializedTokens)
  }

  public dispose(): void {
    this._disposable.dispose()
  }
}
