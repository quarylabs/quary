import { SqlLanguage } from './config'
import { ProjectDag } from '@quary/proto/quary/service/v1/project_dag'
import {
    ListAssetsResponse_Asset,
    ReturnDashboardWithSqlResponse
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { Project } from '@quary/proto/quary/service/v1/project'
import { TestRunner } from '@quary/proto/quary/service/v1/test_runner'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { Table } from '@quary/proto/quary/service/v1/table'
import { QuaryError } from './result'

/**
 * The message type that is sent to the webview when the global state is set.
 */
export const USE_GLOBAL_STATE_MESSAGE_TYPE_SET = 'useGlobalStateSet'
/**
 * The message type that is sent to the webview when the global state is not set aimed at retrieving the current state.
 */
export const USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET = 'useGlobalStateNotSet'

export type Dag = {
  dag: ProjectDag
  models: ListAssetsResponse_Asset[]
}

export type View =
  | {
      type: 'loading'
    }
  | {
      type: 'error'
      error: QuaryError
    }
  | {
      type: 'onboarding'
      states: OnboardingViewStates
    }
  | {
      type: 'tables'
      tables: string[]
      views: string[]
    }
  | {
      type: 'tests'
      runner: TestRunner
      tests: Test[]
    }
  | {
      type: 'project'
      project: Project
      seedQueries: string[]
    }
  | {
      type: 'schema'
      fullSchema: string
      language: SqlLanguage
    }
  | {
      type: 'queryResults'
      originalSql: string
      language: SqlLanguage
      results: QueryResult
      limit?: number
    }
  | {
      type: 'databaseConfigShow'
      config?: ConnectionConfig
    }
  | {
      type: 'databaseShowProjectFile'
      projectFile: string
    }
  | {
      type: 'docsView'
      modelName: string
      description: string | null
      tags: string[]
      language: SqlLanguage
      results: SqlDocumentationResultsView
      limit?: number
      dag: Dag
      table: Table | null
      isModelInSchema: boolean
    }
  | {
      type: 'importSources'
      state: ImportSourcesViewState
    }
  | {
      type: 'executeSQL'
      results: SqlDocumentationResultsView
      limit?: number
    }
  | {
      // Used for editing charts
      type: 'chartEditor'
      data: ChartEditorData
    }
  | {
      type: 'dashboardEditor'
      data: DashboardEditorData
    }

export type ChartEditorData = {
  title: string
  chartFile?: ChartFile
  allAssets: string[]
  results:
    | {
        type: 'not loaded'
      }
    | {
        type: 'loading'
      }
    | {
        type: 'error'
        error: QuaryError
      }
    | {
        type: 'success'
        queryResult: QueryResult
      }
}

export type DashboardEditorData = {
  dashboardFile: ReturnDashboardWithSqlResponse
}

export type SqlDocumentationResultsView =
  | {
      type: 'notYetRun'
    }
  | {
      type: 'run'
      modelName: string
      results: QueryResult
    }
  | {
      type: 'error'
      error: QuaryError
    }
  | {
      type: 'loading'
    }

// TODO: Move to Proto or use existing Proto
export enum DatabaseOnboardingOptions {
  BigQuery = 'bigQuery',
  SQLite = 'sqlite',
  SQLiteInMemory = 'sqliteInMemory',
  Snowflake = 'snowflake',
}

export type OnboardingViewStates =
  | { type: 'init' }
  | {
      type: 'listSourcesLoading'
      database: OnboardingViewDatabaseType
    }
  | {
      type: 'listSourcesError'
      database: OnboardingViewDatabaseType
      error: string
    }
  | {
      type: 'listSourcesSuccess'
      sourceDetails:
        | {
            type: DatabaseOnboardingOptions.BigQuery
            projectsAndDatasets: Record<string, string[]>
          }
        | {
            type: DatabaseOnboardingOptions.Snowflake
            databasesAndSchemas: Record<string, string[]>
            config: {
              accountUrl: string
              clientId: string
              clientSecret: string
              role: string
              warehouse: string
            }
          }
        | {
            type: DatabaseOnboardingOptions.SQLite
            path: string
          }
        | {
            type: DatabaseOnboardingOptions.SQLiteInMemory
          }
    }
  | {
      type: 'generateProjectError'
      error: string
    }

export type OnboardingViewDatabaseType =
  | {
      type: DatabaseOnboardingOptions.BigQuery
    }
  | {
      type: DatabaseOnboardingOptions.SQLite
      path: string
    }
  | {
      type: DatabaseOnboardingOptions.SQLiteInMemory
    }
  | {
      type: DatabaseOnboardingOptions.Snowflake
      accountUrl: string
      clientId: string
      clientSecret: string
      warehouse: string
      role: string
    }

export type TestStatus =
  | {
      type: 'pass'
    }
  | {
      type: 'fail_with_result'
      // TODO Add lines that failed
    }
  | {
      type: 'fail_with_message'
      message: string
    }
  | {
      type: 'pass_inferred'
      sourceTest: string[]
    }
  | {
      type: 'fail_inferred'
      sourceTest: string[]
    }
  | {
      type: 'pass_inferred_from_logic'
      explanation: string
    }

export type Test = {
  testName: string
  status: TestStatus
  query: string
}

export type ImportSourcesViewState =
  | {
      type: 'loading'
    }
  | {
      type: 'error'
      error: string
    }
  | {
      type: 'success'
      sources: ProjectFileSource[]
    }
