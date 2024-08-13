import { Err, Ok, Result, isErr, ErrorCodes } from '@shared/result'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { CLIRPCServiceClientImpl } from '@quary/proto/quary/service/v1/cli_rpc_calls'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import {
  CLIDatabaseService,
  CLIDatabaseServiceWrapper,
} from './servicesDatabase'
import { TerminalExecutor } from './terminalExecutor'
import { ModifiedConnectionConfig, ServicesDatabase } from '@shared/database'

const DefaultDatabaseDependentSettings = {
  runQueriesByDefault: false,
  lookForCacheViews: true,
  importSourcesAfterOnboarding: true,
}

export class ServicesDatabaseClickhouseNode implements ServicesDatabase {
  protected readonly client: CLIRPCServiceClientImpl
  protected readonly database: string

  constructor(terminalExecutor: TerminalExecutor, database: string) {
    this.client = CLIDatabaseService(terminalExecutor)
    this.database = database
  }

  returnDatabaseConfiguration: () => DatabaseDependentSettings = () =>
    DefaultDatabaseDependentSettings

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
        message: 'unexpected empty result from query',
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

  returnPreTableQualifier(): string {
    return this.database ?? ''
  }

  returnLanguage(): SqlLanguage {
    return 'clickhouse'
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

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'clickhouse',
      clickhouse: {
        database: this.database,
      },
    }
  }
}
