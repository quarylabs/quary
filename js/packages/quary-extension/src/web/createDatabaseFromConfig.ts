import * as vscode from 'vscode'
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { ServicesDatabase } from './servicesDatabase'
import { InMemorySqlite, PathBasedSqlite } from './servicesDatabaseSqlite'
import { BigQueryOAuth } from './servicesDatabaseBigQuery'
import { servicesDatabaseBigQueryNode } from './servicesDatabaseBigQueryNode'
import { Snowflake } from './servicesDatabaseSnowflake'
import { ServicesDatabaseDuckDBInMemory } from './servicesDatabaseDuckDB'
import { ServicesFiles } from './servicesFiles'
import { TerminalExecutor } from './terminalExecutor'
import {
  ServicesDatabaseInMemorySqliteNode,
  ServicesDatabasePathBasedSqliteNode,
} from './servicesDatabaseSqliteNode'
import {
  ServicesDatabaseDuckDBInMemoryNode,
  ServicesDatabaseDuckDBNode,
} from './servicesDatabaseDuckDBNode'
import { ServicesDatabaseRedshiftNode } from './servicesDatabaseRedshiftNode'
import { ServicesDatabasePostgresNode } from './servicesDatabasePostgresNode'

/**
 * Creates a database instance from a given configuration.
 *
 * @returns An instance of ServicesDatabase.
 */
export const databaseFromConfig = async (
  rootUri: vscode.Uri,
  fileSystem: ServicesFiles,
  writer: (uri: vscode.Uri, content: Uint8Array) => Promise<void>,
  reader: (uri: vscode.Uri) => Promise<Uint8Array>,
  config: ConnectionConfig,
): Promise<Result<ServicesDatabase>> => {
  if (config.config === undefined) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: `No config provided`,
    })
  }
  switch (config.config.$case) {
    case 'sqliteInMemory': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          if (!inBrowserSqlite) {
            inBrowserSqlite = await InMemorySqlite.create()
          }
          return Ok(inBrowserSqlite)
        }
        case vscode.UIKind.Desktop: {
          const sqliteInMemory = new ServicesDatabaseInMemorySqliteNode(
            getTerminal(),
          )
          return Ok(sqliteInMemory)
        }
        default: {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'Invalid UI Kind',
          })
        }
      }
    }
    case 'bigQuery': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Ok(new BigQueryOAuth(config.config.bigQuery))
        }
        case vscode.UIKind.Desktop: {
          return Ok(
            new servicesDatabaseBigQueryNode(
              getTerminal(),
              config.config.bigQuery.projectId,
              config.config.bigQuery.datasetId,
            ),
          )
        }
        default: {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'Invalid UI Kind',
          })
        }
      }
    }
    case 'snowflake': {
      return Ok(new Snowflake(config.config.snowflake))
    }
    case 'sqlite': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          if (!pathBasedSqlite) {
            const { path } = config.config.sqlite
            const pathUri = vscode.Uri.joinPath(rootUri, path)
            const database = await PathBasedSqlite.create(
              pathUri,
              rootUri,
              writer,
              reader,
            )
            pathBasedSqlite = database
            return Ok(pathBasedSqlite)
          }
          return Ok(pathBasedSqlite)
        }
        case vscode.UIKind.Desktop: {
          const { path } = config.config.sqlite
          const duckDB = new ServicesDatabasePathBasedSqliteNode(
            getTerminal(),
            path,
          )
          return Ok(duckDB)
        }
        default: {
          return Err({
            code: ErrorCodes.INTERNAL,
            message: `Unknown UIKind: ${vscode.env.uiKind}`,
          })
        }
      }
    }
    case 'duckdbInMemory': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          if (!inMemoryDuckDb) {
            const db = await ServicesDatabaseDuckDBInMemory.create(
              fileSystem,
              config.config.duckdbInMemory.schema,
            )
            if (isErr(db)) {
              return Err(db.error)
            }
            inMemoryDuckDb = db.value
          }
          return Ok(inMemoryDuckDb)
        }
        case vscode.UIKind.Desktop: {
          const duckDB = new ServicesDatabaseDuckDBInMemoryNode(
            getTerminal(),
            config.config.duckdbInMemory.schema,
          )
          return Ok(duckDB)
        }
        default: {
          return Err({
            code: ErrorCodes.INTERNAL,
            message: `Unknown UIKind: ${vscode.env.uiKind}`,
          })
        }
      }
    }
    case 'duckdb': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'DuckDb is not supported in the web extension',
          })
        }
        case vscode.UIKind.Desktop: {
          const { path, schema } = config.config.duckdb
          const duckDB = new ServicesDatabaseDuckDBNode(
            getTerminal(),
            path,
            schema,
          )
          return Ok(duckDB)
        }
        default:
          return Err({
            code: ErrorCodes.INTERNAL,
            message: `Unknown UIKind: ${vscode.env.uiKind}`,
          })
      }
    }
    case 'postgres': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'Postgres is not supported in the web extension',
          })
        }
        case vscode.UIKind.Desktop: {
          const { schema } = config.config.postgres
          const postgres = new ServicesDatabasePostgresNode(
            getTerminal(),
            schema,
          )
          return Ok(postgres)
        }
        default:
          return Err({
            code: ErrorCodes.INTERNAL,
            message: `Unknown UIKind: ${vscode.env.uiKind}`,
          })
      }
    }
    case 'redshift': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'Redshift is not supported in the web extension',
          })
        }
        case vscode.UIKind.Desktop: {
          const { schema } = config.config.redshift
          const redshift = new ServicesDatabaseRedshiftNode(
            getTerminal(),
            schema,
          )
          return Ok(redshift)
        }
        default:
          return Err({
            code: ErrorCodes.INTERNAL,
            message: `Unknown UIKind: ${vscode.env.uiKind}`,
          })
      }
    }
    default:
      return Err({
        code: ErrorCodes.INVALID_ARGUMENT,
        message: `Invalid configured database: ${config.config}`,
      })
  }
}

const getTerminal = () => {
  if (!terminal) {
    terminal = new TerminalExecutor('Quary')
  }
  return terminal
}

let terminal: TerminalExecutor | undefined

let inBrowserSqlite: InMemorySqlite | undefined

let pathBasedSqlite: PathBasedSqlite | undefined

let inMemoryDuckDb: ServicesDatabaseDuckDBInMemory | undefined
