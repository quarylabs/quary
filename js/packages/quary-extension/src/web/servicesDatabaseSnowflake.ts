import {
  SnowflakeOptions,
  SnowflakeBaseWithLocation,
  SnowflakeBase,
} from '@shared/databaseSnowflake'
import { SourcesLister } from '@shared/database'
import { Err, ErrorCodes, Ok, Result } from '@shared/result'
import * as vscode from 'vscode'

const getAccessToken = async (
  accountUrl: string,
  clientId: string,
  clientSecret: string,
  role: string,
): Promise<Result<string>> => {
  const session = await vscode.authentication.getSession(
    'quarySnowflake',
    [accountUrl, clientId, clientSecret, role],
    {
      createIfNone: true,
    },
  )
  if (!session) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: 'Unable to authenticate with Snowflake.',
    })
  }
  return Ok(session.accessToken)
}

export class Snowflake extends SnowflakeBaseWithLocation {
  readonly database: string
  readonly schema: string
  readonly warehouse: string

  constructor(options: SnowflakeOptions) {
    super(options)
    this.database = options.database
    this.schema = options.schema
    this.warehouse = options.warehouse
  }

  getAccessToken = async (): Promise<Result<string>> =>
    getAccessToken(this.accountUrl, this.clientId, this.clientSecret, this.role)
}

export class SnowflakeHeadless extends SnowflakeBase implements SourcesLister {
  getAccessToken = () =>
    getAccessToken(this.accountUrl, this.clientId, this.clientSecret, this.role)
}
