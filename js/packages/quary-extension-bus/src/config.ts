/**
 * Database dependent settings are settings in the extension that are
 * dependent on the database type. For example, any behavior that should
 * be different for sqlite vs bigquery should be in this type.
 */
export type DatabaseDependentSettings = {
  /**
   * If true, queries will be run by default when certain windows are
   * opened, for example, when the documentation of a model is opened.
   *
   * For local development like in Sqlite, this should be set to true because
   * it is fast to run queries. For BigQuery, this should be set to false
   * because it is slower to run queries, and it costs money.
   */
  runQueriesByDefault: boolean
  /**
   * If true, the extension will look for cached views when attempting to run a query on a model against the data warehouse.
   *
   * For example, when running against BigQuery, the extension will look for a cached view in BigQuery that matches
   * the model name and hash of the query. If it finds one, it will run the query against the cached view instead of the
   * raw model.
   */
  lookForCacheViews: boolean
  /**
   * If true, the extension will open the import sources view after the onboarding step has been completed. This should
   * be made true for databases that exist outside Quary and where users may want to pull tables in. This is likely
   * to be false for a in-process/in-memory database that spins up with Quary.
   */
  importSourcesAfterOnboarding: boolean
}

export type AIConfig =
  | undefined
  | {
      type: 'openai'
      apiKey: string
    }

export type SqlLanguage =
  | 'sqlite'
  | 'bigquery'
  | 'snowflake'
  | 'duckdb'
  | 'postgresql'
