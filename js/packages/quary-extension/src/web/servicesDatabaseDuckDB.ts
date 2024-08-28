import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import * as duckdb from '@duckdb/duckdb-wasm'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { columnsValuesToQueryResult } from '@shared/shared'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import { ModifiedConnectionConfig, ServicesDatabase } from '@shared/database'
import { ServicesFiles } from './servicesFiles'

const JSDELIVR_BUNDLES = duckdb.getJsDelivrBundles()

const DefaultDatabaseDependentSettings = {
  runQueriesByDefault: true,
  lookForCacheViews: true,
}

abstract class Base {
  protected readonly db: duckdb.AsyncDuckDB
  protected readonly schema: string | undefined
  protected readonly fileSystem: ServicesFiles

  abstract postStatementCallback(): Promise<void>

  protected constructor(
    db: duckdb.AsyncDuckDB,
    fileSystem: ServicesFiles,
    schema?: string | undefined,
  ) {
    this.db = db
    this.schema = schema
    this.fileSystem = fileSystem
  }

  async runStatement(command: string): Promise<Result<QueryResult>> {
    // TODO Optimize this by registering a file buffer handler that is lazy and doesn't require all files to be read up
    //   front. This will be important when we start supporting large files.

    // Register all the file in the file system for the database to use
    // TODO Make this by only reading the files once.
    const pb = await this.fileSystem.getProtoFileSystem()
    if (isErr(pb)) {
      return pb
    }
    const pb1 = await this.fileSystem.getProtoFileSystem()
    if (isErr(pb1)) {
      return pb1
    }
    const pb2 = await this.fileSystem.getProtoFileSystem()
    if (isErr(pb2)) {
      return pb2
    }
    const projectRoot = this.fileSystem.getProjectRoot()
    if (isErr(projectRoot)) {
      return projectRoot
    }

    try {
      // Register all the files in the file system for the database to use
      // so that a file can be referenced by its path 'data/iris.csv'
      for (const [path, file] of Object.entries(pb.value.files)) {
        let filePath = path.replace(projectRoot.value.path, '')
        if (filePath.startsWith('/')) {
          filePath = filePath.slice(1)
        }
        await this.db.registerFileBuffer(filePath, file.contents)
      }
      // so that a file can be referenced by its internal path './data/iris.csv'
      for (const [path, file] of Object.entries(pb1.value.files)) {
        let filePath = path.replace(projectRoot.value.path, '')
        if (filePath.startsWith('/')) {
          filePath = filePath.slice(1)
        }
        const filePathWithLeadingDotSlash = './' + filePath
        await this.db.registerFileBuffer(
          filePathWithLeadingDotSlash,
          file.contents,
        )
      }
      // so that a file can be referenced by its full path '/Users/.../data/iris.csv'
      for (const [path, file] of Object.entries(pb2.value.files)) {
        await this.db.registerFileBuffer(path, file.contents)
      }

      // Do call
      const conn = await this.db.connect()
      const result = await conn.query(command)
      const data = result.toArray()
      await this.postStatementCallback()

      const columns = result.schema.fields.map((f) => ({
        column: f.name,
      }))

      // DuckDB returns BigInt and so want to convert them to strings
      // Transform each row object into an array of strings based on the columns
      const values = data.map((row) =>
        columns.map((column) =>
          // Assuming the values are already in string format or can be converted to string
          // You may need to handle different data types accordingly
          String(row[column.column]),
        ),
      )
      return Ok(columnsValuesToQueryResult({ columns, values }))
    } catch (e: unknown) {
      if (e instanceof Error) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: e.message,
        })
      }
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Unknown error: ${e}`,
      })
    }
  }

  async listTables() {
    const result = await this.runStatement(
      `
                SELECT table_schema, table_name
                FROM information_schema.tables
                WHERE table_type = 'BASE TABLE'
                ORDER BY table_schema, table_name
            `,
    )

    if (isErr(result)) {
      return result
    }

    const len = result.value.columns[0].values.length
    const tables: TableAddress[] = []
    for (let i = 0; i < len; i++) {
      const schema = result.value.columns[0].values[i]
      const name = result.value.columns[1].values[i]
      tables.push({ name, fullPath: `${schema}.${name}` })
    }
    return Ok(tables)
  }

  async listViews() {
    const result = await this.runStatement(
      `
                SELECT table_schema, table_name
                FROM information_schema.tables
                WHERE table_type = 'VIEW'
                ORDER BY table_schema, table_name
            `,
    )
    if (isErr(result)) {
      return result
    }
    const len = result.value.columns[0].values.length
    const tables: TableAddress[] = []
    for (let i = 0; i < len; i++) {
      const schema = result.value.columns[0].values[i]
      const name = result.value.columns[1].values[i]
      tables.push({ name, fullPath: `${schema}.${name}` })
    }
    return Ok(tables)
  }

  async listColumns(table: string) {
    const result = await this.runStatement(`PRAGMA table_info(${table})`)
    if (isErr(result)) {
      return result
    }
    return Ok(result.value.columns.map(({ values }) => values[1] as string))
  }

  returnPreTableQualifier(): string {
    return this.schema ?? ''
  }

  returnLanguage(): SqlLanguage {
    return 'duckdb'
  }

  async listSources(): Promise<Result<ProjectFileSource[]>> {
    const tables = await this.listTables()
    if (isErr(tables)) {
      return tables
    }
    const views = await this.listViews()
    if (isErr(views)) {
      return views
    }

    const all = tables.value.concat(views.value)
    // const all = tables.value
    const out: ProjectFileSource[] = []
    for (const table of all) {
      const columns = await this.listColumns(table.name)
      if (isErr(columns)) {
        return columns
      }
      out.push({
        name: table.name,
        tags: [],
        tests: [],
        path: table.fullPath,
        columns: columns.value.map((name) => ({
          name,
          tests: [],
        })),
      })
    }
    return Ok(out)
  }
}

const commmonCreateDuckDB = async (): Promise<duckdb.AsyncDuckDB> => {
  const bundle = await duckdb.selectBundle(JSDELIVR_BUNDLES)
  const workUrl = URL.createObjectURL(
    new Blob([`importScripts("${bundle.mainWorker!}");`], {
      type: 'text/javascript',
    }),
  )

  // Instantiate the asynchronus version of DuckDB-Wasm
  const worker = new Worker(workUrl)
  const logger = new duckdb.ConsoleLogger()
  const db = new duckdb.AsyncDuckDB(logger, worker)
  await db.instantiate(bundle.mainModule, bundle.pthreadWorker)
  URL.revokeObjectURL(workUrl)
  return db
}

export class ServicesDatabaseDuckDBInMemory
  extends Base
  implements ServicesDatabase
{
  returnDatabaseConfiguration: () => DatabaseDependentSettings = () => ({
    ...DefaultDatabaseDependentSettings,
    importSourcesAfterOnboarding: false,
  })

  static async create(
    fileSystem: ServicesFiles,
    schema?: string | undefined,
  ): Promise<Result<ServicesDatabaseDuckDBInMemory>> {
    // Select a bundle based on browser checks
    const db = await commmonCreateDuckDB()
    return Ok(new ServicesDatabaseDuckDBInMemory(db, fileSystem, schema))
  }

  // returnLanguage: () => SqlLanguage
  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'duckdbInMemory',
      duckdbInMemory: {
        schema: this.schema,
      },
    }
  }

  async postStatementCallback(): Promise<void> {}
}
