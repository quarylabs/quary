import { Err, Ok, Result, isErr, ErrorCodes } from '@shared/result'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { CLIRPCServiceClientImpl } from '@quary/proto/quary/service/v1/cli_rpc_calls'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import { ModifiedConnectionConfig, ServicesDatabase } from '@shared/database'
import {
  CLIDatabaseService,
  CLIDatabaseServiceWrapper,
} from './servicesDatabase'
import { TerminalExecutor } from './terminalExecutor'

const DefaultDatabaseDependentSettings = {
  runQueriesByDefault: false,
  lookForCacheViews: false,
}

export class servicesDatabaseBigQueryNode implements ServicesDatabase {
  protected readonly client: CLIRPCServiceClientImpl
  private readonly projectId: string
  private readonly datasetId: string

  constructor(
    terminalExecutor: TerminalExecutor,
    projectId: string,
    datasetId: string,
  ) {
    this.client = CLIDatabaseService(terminalExecutor)
    this.projectId = projectId
    this.datasetId = datasetId
  }

  returnDatabaseConfiguration: () => DatabaseDependentSettings = () => ({
    ...DefaultDatabaseDependentSettings,
    importSourcesAfterOnboarding: false,
  })

  async runStatement(query: string): Promise<Result<QueryResult>> {
    const response = await CLIDatabaseServiceWrapper(this.client.Query, {
      query,
    })
    if (isErr(response)) {
      return response
    }
    if (!response.value.result) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: 'unexpected empty query result',
      })
    }
    return Ok(response.value.result)
  }

  async listTables() {
    const response = await CLIDatabaseServiceWrapper(this.client.ListTables, {})
    if (isErr(response)) {
      return response
    }
    return Ok(response.value.tables)
  }

  async listViews() {
    const response = await CLIDatabaseServiceWrapper(this.client.ListViews, {})
    if (isErr(response)) {
      return response
    }
    return Ok(response.value.views)
  }

  async listColumns(tableName: string) {
    const response = await CLIDatabaseServiceWrapper(this.client.ListColumns, {
      tableName,
    })
    if (isErr(response)) {
      return response
    }
    return Ok(response.value.columns)
  }

  async listSources(): Promise<Result<ProjectFileSource[]>> {
    const response = await CLIDatabaseServiceWrapper(
      this.client.ListSources,
      {},
    )
    if (isErr(response)) {
      return response
    }
    return Ok(response.value.sources)
  }

  returnPreTableQualifier(): string {
    return `${this.projectId}.${this.datasetId}`
  }

  returnLanguage(): SqlLanguage {
    return 'bigquery'
  }

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'bigQuery',
      bigQuery: {
        projectId: this.projectId,
        datasetId: this.datasetId,
      },
    }
  }
}
