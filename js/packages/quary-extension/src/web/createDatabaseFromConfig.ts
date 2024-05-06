import * as vscode from 'vscode'
import { Err, isErr, Ok, Result } from '@shared/result'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { ServicesDatabase } from './servicesDatabase'
import { InMemorySqlite, PathBasedSqlite } from './servicesDatabaseSqlite'
import { BigQueryOAuth } from './servicesDatabaseBigQuery'
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
    return Err(new Error('No config provided.'))
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
          return Err(new Error(`Unknown UIKind: ${vscode.env.uiKind}`))
        }
      }
    }
    case 'bigQuery': {
      return Ok(new BigQueryOAuth(config.config.bigQuery))
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
          return Err(new Error(`Unknown UIKind: ${vscode.env.uiKind}`))
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
          return Err(new Error(`Unknown UIKind: ${vscode.env.uiKind}`))
        }
      }
    }
    case 'duckdb': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Err(new Error('DuckDB is not supported in the web app'))
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
          return Err(new Error(`Unknown UIKind: ${vscode.env.uiKind}`))
      }
    }
    case 'postgres': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Err(new Error('Postgres is not supported in the web app'))
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
          return Err(new Error(`Unknown UIKind: ${vscode.env.uiKind}`))
      }
    }
    case 'redshift': {
      switch (vscode.env.uiKind) {
        case vscode.UIKind.Web: {
          return Err(new Error('Redshift is not supported in the web app'))
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
          return Err(new Error(`Unknown UIKind: ${vscode.env.uiKind}`))
      }
    }
    default:
      return Err(new Error(`Unknown config: ${JSON.stringify(config)}`))
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
