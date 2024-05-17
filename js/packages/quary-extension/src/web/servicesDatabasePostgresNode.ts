import { Err, Ok, Result, isErr, ErrorCodes } from '@shared/result'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { CLIRPCServiceClientImpl } from '@quary/proto/quary/service/v1/cli_rpc_calls'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import {
  ServicesDatabase,
  CLIDatabaseService,
  CLIDatabaseServiceWrapper,
  ModifiedConnectionConfig,
} from './servicesDatabase'
import { TerminalExecutor } from './terminalExecutor'

const DefaultDatabaseDependentSettings = {
  runQueriesByDefault: false,
  lookForCacheViews: false,
}

export class ServicesDatabasePostgresNode implements ServicesDatabase {
  protected readonly client: CLIRPCServiceClientImpl
  protected readonly schema: string

  constructor(terminalExecutor: TerminalExecutor, schema: string) {
    this.client = CLIDatabaseService(terminalExecutor)
    this.schema = schema
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
    return this.schema
  }

  returnLanguage(): SqlLanguage {
    return 'postgresql'
  }

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'postgres',
      postgres: {
        schema: this.schema,
      },
    }
  }
}
