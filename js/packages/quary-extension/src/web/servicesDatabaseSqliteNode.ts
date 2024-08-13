import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { CLIRPCServiceClientImpl } from '@quary/proto/quary/service/v1/cli_rpc_calls'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import {
  CLIDatabaseService,
  CLIDatabaseServiceWrapper,
} from './servicesDatabase'
import { TerminalExecutor } from './terminalExecutor'
import {
  ModifiedConnectionConfig,
  ServicesDatabase,
  SourcesLister,
} from '@shared/database'

const DefaultSqliteDatabaseDependentSetting = {
  runQueriesByDefault: true,
  lookForCacheViews: false,
}

abstract class ServicesDatabaseSqliteBase
  implements SourcesLister, ServicesDatabase
{
  protected readonly client: CLIRPCServiceClientImpl

  constructor(terminalExecutor: TerminalExecutor) {
    this.client = CLIDatabaseService(terminalExecutor)
  }

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
        message: 'Unexpected empty query result',
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
    return ''
  }

  returnLanguage(): SqlLanguage {
    return 'sqlite'
  }

  abstract returnDatabaseConfig(): ModifiedConnectionConfig

  abstract returnDatabaseConfiguration(): DatabaseDependentSettings
}

export class ServicesDatabasePathBasedSqliteNode extends ServicesDatabaseSqliteBase {
  private readonly path: string

  constructor(terminalExecutor: TerminalExecutor, path: string) {
    super(terminalExecutor)
    this.path = path
  }

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'sqlite',
      sqlite: {
        path: this.path,
      },
    }
  }

  returnDatabaseConfiguration(): DatabaseDependentSettings {
    return {
      ...DefaultSqliteDatabaseDependentSetting,
      importSourcesAfterOnboarding: true,
    }
  }
}

export class ServicesDatabaseInMemorySqliteNode extends ServicesDatabaseSqliteBase {
  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'sqliteInMemory',
      sqliteInMemory: {},
    }
  }

  returnDatabaseConfiguration(): DatabaseDependentSettings {
    return {
      ...DefaultSqliteDatabaseDependentSetting,
      importSourcesAfterOnboarding: false,
    }
  }
}
