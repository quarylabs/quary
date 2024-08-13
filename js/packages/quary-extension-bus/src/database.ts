import { Result } from './result'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import { DatabaseDependentSettings, SqlLanguage } from './config'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'

export interface ServicesDatabase extends SourcesLister {
  runStatement: (command: string) => Promise<Result<QueryResult>>
  /**
   * If the relevant schema/data set is empty, listTables does not return an error but rather an empty array.
   * If the schema/dataset does not exist, listTables should throw an error.
   */
  listTables: () => Promise<Result<Array<TableAddress>>>
  /**
   * If the relevant schema/data set is empty, listViews does not return an error but rather an empty array.
   * If the schema/dataset does not exist, listViews should throw an error.
   */
  listViews: () => Promise<Result<Array<TableAddress>>>
  listColumns: (table: string) => Promise<Result<Array<string>>>
  returnPreTableQualifier: () => string
  returnLanguage: () => SqlLanguage
  /**
   * Returns the database configuration specific for each database implementation. Without including the global project configuration settings i.e. variables.
   * This is necessary as the databaes configuration needs to be modified in the context of the extension as vscode file-system path needs to be infused for sqlite & duckdb.
   */
  returnDatabaseConfig: () => ModifiedConnectionConfig
  /**
   * Return database dependent settings.
   */
  returnDatabaseConfiguration: () => DatabaseDependentSettings
}

/**
 * ModifiedConnectionConfig is a type that represents only the database configuration
 * extracted from the Project Configuration (ConnectionConfig) type.
 */
export type ModifiedConnectionConfig = ConnectionConfig['config']

export interface SourcesLister {
  /**
   * listSources returns a list of all sources in the database. If the database does not support sources, it should
   * return an error.
   *
   * TODO: This is a good opportunity to use the ResultE type and have strict error types.
   */
  listSources: () => Promise<Result<Array<ProjectFileSource>>>
}
