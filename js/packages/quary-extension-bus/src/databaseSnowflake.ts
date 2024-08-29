import { collectResults, Err, ErrorCodes, isErr, Ok, Result } from './result'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { columnsValuesToQueryResult } from './shared'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import { ModifiedConnectionConfig, ServicesDatabase } from './database'
import { DatabaseDependentSettings, SqlLanguage } from './config'
import * as vscode from 'vscode'

export async function makeSnowflakeRequest<T>(
  accessToken: string,
  accountUrl: string,
  body?: object,
): Promise<Result<T>> {
  try {
    const headers = {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${accessToken}`,
    }
    const fetchOptions: RequestInit = { method: 'POST', headers }

    if (body) {
      fetchOptions.body = JSON.stringify(body)
    }
    const response = await fetch(
      `${accountUrl}/api/v2/statements`,
      fetchOptions,
    )

    if (!response.ok) {
      const error = await response.json()
      switch (response.status) {
        case 400:
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: `Invalid argument in call to Snowflake API, ${JSON.stringify(error)}`,
          })
        default: {
          return Err({
            code: ErrorCodes.UNKNOWN,
            message: `HTTP error: ${response.status}\nDetails: ${error.message}`,
          })
        }
      }
    }
    const jsonResponse = await response.json()
    return Ok(jsonResponse as T)
  } catch (error) {
    const errorMessage =
      error instanceof Error ? error.message : 'Unknown error occurred'
    return Err({
      code: ErrorCodes.UNKNOWN,
      message: `Failed to make Snowflake request: ${errorMessage}`,
    })
  }
}

export async function snowflakeRunStatement(
  accessToken: string,
  accountUrl: string,
  database: string,
  schema: string,
  warehouse: string,
  statement: string,
): Promise<Result<QueryResult>> {
  const body = {
    statement: statement.replace(/`/g, ''),
    database,
    schema,
    warehouse,
  }
  const runStatementResponse = await makeSnowflakeRequest(
    accessToken,
    accountUrl,
    body,
  )
  if (isErr(runStatementResponse)) {
    return runStatementResponse
  }
  const { resultSetMetaData, data: values } = runStatementResponse.value as {
    resultSetMetaData: {
      rowType: {
        name: string
        type: string
      }[]
    }
    data: string[][]
  }

  const columns = resultSetMetaData.rowType.map((row) => ({
    column: row.name,
    type: row.type,
  }))
  const out = columnsValuesToQueryResult({ columns, values })
  return Ok(out)
}

interface SnowflakeBaseOptions {
  accountUrl: string
  clientId: string
  clientSecret: string
  role: string
}

export abstract class SnowflakeBase {
  protected accountUrl: string
  protected clientId: string
  protected clientSecret: string
  protected role: string

  constructor(options: SnowflakeBaseOptions) {
    this.accountUrl = options.accountUrl
    this.clientId = options.clientId
    this.clientSecret = options.clientSecret
    this.role = options.role
  }

  protected async makeSnowflakeRequest<T>(body?: object): Promise<Result<T>> {
    const accessToken = await this.getAccessToken()
    if (isErr(accessToken)) {
      return accessToken
    }
    return makeSnowflakeRequest(accessToken.value, this.accountUrl, body)
  }

  protected async getAccessToken(): Promise<Result<string>> {
    const session = await vscode.authentication.getSession(
      'quarySnowflake',
      [this.accountUrl, this.clientId, this.clientSecret, this.role],
      {
        createIfNone: true,
      },
    )
    if (!session) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: 'Unable to authenticate with Snowflake.',
      })
    }
    return Ok(session.accessToken)
  }

  async listDatabases(): Promise<Result<string[]>> {
    const listDatabasesResponse = await this.makeSnowflakeRequest({
      statement: 'SHOW DATABASES',
    })
    if (isErr(listDatabasesResponse)) {
      return listDatabasesResponse
    }
    const { data } = listDatabasesResponse.value as { data: string[][] }
    const databaseNames = data
      .map((db: string[]) => db[1])
      .map((db: string) => db.toLowerCase())
    return Ok(databaseNames)
  }

  async listSchemasRoot(database: string): Promise<Result<string[]>> {
    const listSchemasResponse = await this.makeSnowflakeRequest({
      statement: `SHOW SCHEMAS IN DATABASE ${database}`,
    })
    if (isErr(listSchemasResponse)) {
      return listSchemasResponse
    }
    const { data } = listSchemasResponse.value as { data: string[][] }
    const schemaNames = data
      .map((db: string[]) => db[1])
      .map((db) => db.toLowerCase())
    return Ok(schemaNames)
  }

  async listTablesRoot(
    database: string,
    schema: string,
  ): Promise<Result<TableAddress[]>> {
    const listTablesResponse = await this.makeSnowflakeRequest({
      statement: `SHOW TABLES IN SCHEMA ${database}.${schema}`,
    })
    if (isErr(listTablesResponse)) {
      return listTablesResponse
    }
    const { data } = listTablesResponse.value as { data: string[][] }
    return Ok(
      data.map((db: string[]) => ({
        name: db[1].toLowerCase(),
        fullPath: `${db[2]}.${db[3]}.${db[1]}`,
      })),
    )
  }

  async listViewsRoot(
    database: string,
    schema: string,
  ): Promise<Result<TableAddress[]>> {
    return Ok([{ name: database, fullPath: schema }]) //TODO: implement properly (returns incorrect values)
  }

  async listColumnsRoot(database: string, schema: string, table: string) {
    const listColumnsResponse = await this.makeSnowflakeRequest({
      statement: `DESCRIBE TABLE ${database}.${schema}.${table}`,
    })
    if (isErr(listColumnsResponse)) {
      return listColumnsResponse
    }
    const { data } = listColumnsResponse.value as { data: string[][] } // TODO: type safe snowflake reqs
    return Ok(data.map((db: string[]) => db[0]))
  }

  async listSources(): Promise<Result<ProjectFileSource[]>> {
    const listDatabasesResult = await this.listDatabases()
    if (isErr(listDatabasesResult)) {
      return listDatabasesResult
    }
    const databases = listDatabasesResult.value

    const promises = databases.map(async (database) => {
      const listSchemasResponse = await this.listSchemasRoot(database)
      if (isErr(listSchemasResponse)) {
        return [listSchemasResponse]
      }
      const schemas = listSchemasResponse.value

      return Promise.all(
        schemas.map(async (schema) => {
          const listTablesResponse = await this.listTablesRoot(database, schema)
          if (isErr(listTablesResponse)) {
            return [listTablesResponse]
          }
          const tables = listTablesResponse.value

          return Promise.all(
            tables.map(async (table): Promise<Result<ProjectFileSource>> => {
              const listColumnsResponse = await this.listColumnsRoot(
                database,
                schema,
                table.name,
              )
              if (isErr(listColumnsResponse)) {
                return listColumnsResponse
              }
              const columns = listColumnsResponse.value

              return Ok({
                name: table.name,
                path: `${database}.${schema}.${table.name}`,
                tests: [],
                tags: [],
                columns: columns.map((column) => ({
                  name: column,
                  tests: [],
                })),
              })
            }),
          )
        }),
      )
    })

    const results = await Promise.all(promises)
    const flattened = results.flat(3)
    const out = collectResults(flattened)
    if (isErr(out)) {
      return Err(out.error[0])
    }
    return out
  }

  async listDatabasesAndSchemas() {
    const listDatabasesResult = await this.listDatabases()
    if (isErr(listDatabasesResult)) {
      return listDatabasesResult
    }
    const databases = listDatabasesResult.value

    const schemaPromises = databases.map(async (database) => {
      const listSchemasResult = await this.listSchemasRoot(database)
      if (isErr(listSchemasResult)) {
        return { database, schemas: [] }
      }
      return {
        database,
        schemas: listSchemasResult.value.map((schema) => schema),
      }
    })

    const schemaResults = await Promise.all(schemaPromises)

    return Ok(
      schemaResults.reduce(
        (acc, { database, schemas }) => {
          acc[database] = schemas
          return acc
        },
        {} as {
          [database: string]: string[]
        },
      ),
    )
  }
}

interface SnowflakeOptions extends SnowflakeBaseOptions {
  database: string
  schema: string
  warehouse: string
}

export class Snowflake extends SnowflakeBase implements ServicesDatabase {
  private readonly database: string
  private readonly schema: string
  private readonly warehouse: string

  constructor(options: SnowflakeOptions) {
    super(options)
    this.database = options.database
    this.schema = options.schema
    this.warehouse = options.warehouse
  }

  returnDatabaseConfiguration: () => DatabaseDependentSettings = () => ({
    runQueriesByDefault: false,
    lookForCacheViews: true,
    importSourcesAfterOnboarding: true,
  })

  returnLanguage(): SqlLanguage {
    return 'snowflake'
  }

  returnPreTableQualifier(): string {
    return `${this.database}.${this.schema}`
  }

  async listTables() {
    return this.listTablesRoot(this.database, this.schema)
  }

  async listViews() {
    return this.listViewsRoot(this.database, this.schema)
  }

  async listColumns(tableName: string) {
    return this.listColumnsRoot(tableName, this.database, this.schema)
  }

  async runStatement(statement: string) {
    const accessToken = await this.getAccessToken()
    if (isErr(accessToken)) {
      return accessToken
    }
    return snowflakeRunStatement(
      accessToken.value,
      this.accountUrl,
      this.database,
      this.schema,
      this.warehouse,
      statement,
    )
  }

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'snowflake',
      snowflake: {
        accountUrl: this.accountUrl,
        clientId: this.clientId,
        clientSecret: this.clientSecret,
        role: this.role,
        database: this.database,
        schema: this.schema,
        warehouse: this.warehouse,
      },
    }
  }
}
