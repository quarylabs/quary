import {
  BigQueryBase,
  BigQueryOptions,
  BigQueryBaseWithLocation,
} from '@shared/databaseBigQuery'
import { SourcesLister } from '@shared/database'
import { Err, ErrorCodes, Ok, Result } from '@shared/result'
import * as vscode from 'vscode'

const getAccessToken = async () => {
  const session = await vscode.authentication.getSession('quaryBigQuery', [], {
    createIfNone: true,
  })
  if (!session) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: 'Failed to get BigQuery session',
    })
  }
  return Ok(session.accessToken)
}

export class BigQueryOAuth extends BigQueryBaseWithLocation {
  readonly projectId: string
  readonly datasetId: string

  constructor(options: BigQueryOptions) {
    super(options)
    this.projectId = options.projectId
    this.datasetId = options.datasetId
  }

  getAccessToken = async (): Promise<Result<string>> => getAccessToken()
}

export class BigQueryOauthHeadless
  extends BigQueryBase
  implements SourcesLister
{
  getAccessToken = () => getAccessToken()
}
