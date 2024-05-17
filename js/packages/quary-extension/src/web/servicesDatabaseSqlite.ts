import * as sql from 'sql.js'
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import { Uri } from 'vscode'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { columnsValuesToQueryResult } from '@shared/shared'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import {
  ModifiedConnectionConfig,
  Reader,
  ServicesDatabase,
  SourcesLister,
  Writer,
} from './servicesDatabase'

abstract class Sqlite implements SourcesLister {
  protected db: sql.Database

  /**
   * postHook is called after the database is changed. This is used to
   * write the database to the file system for the path based version
   */
  protected abstract postHook: () => Promise<void>

  protected constructor(db: sql.Database) {
    this.db = db
  }

  async runStatement(command: string): Promise<Result<QueryResult>> {
    try {
      const db = this.db
      const output = db.exec(command)
      if (output.length === 0) {
        return Ok({ columns: [], values: [] })
      }
      const result = columnsValuesToQueryResult({
        values:
          output[0]?.values.map((row) => row.map((v) => v?.toString() ?? '')) ??
          [],
        columns: output[0].columns.map((column) => ({ column })),
      })

      await this.postHook()
      return Ok(result)
    } catch (e) {
      if (e instanceof Error) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: e.message,
        })
      }
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Unrecognised error ${e}`,
      })
    }
  }

  async listTables() {
    const statement = await this.runStatement(
      "SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name",
    )
    if (isErr(statement)) {
      return statement
    }

    // for case where nothing is returned
    return Ok(
      statement.value.columns[0].values.map((value): TableAddress => {
        if (!(typeof value === 'string')) {
          throw new Error('expect type string')
        }
        return {
          name: value,
          fullPath: value,
        }
      }),
    )
  }

  async listViews() {
    const statement = await this.runStatement(
      "SELECT name FROM sqlite_schema WHERE type='view' ORDER BY name",
    )
    if (isErr(statement)) {
      return statement
    }
    // for case where nothing is returned
    return Ok(
      statement.value.columns[0].values.map((v): TableAddress => {
        if (!(typeof v === 'string')) {
          throw new Error('expect type string')
        }
        return {
          name: v,
          fullPath: v,
        }
      }),
    )
  }

  async listSources() {
    const tables = await this.listTables()
    if (isErr(tables)) {
      return tables
    }
    const views = await this.listViews()
    if (isErr(views)) {
      return views
    }
    const allSources = tables.value.concat(views.value)
    const sources: ProjectFileSource[] = []
    for (const source of allSources) {
      const columns = await this.listColumns(source.name)
      if (isErr(columns)) {
        return columns
      }
      sources.push({
        name: source.name,
        path: source.fullPath,
        tags: [],
        tests: [],
        columns: columns.value.map((column) => ({
          name: column,
          tests: [],
        })),
      })
    }
    return Ok(sources)
  }

  // TODO Need to think about escaping table names properly to avoid SQL injection
  async listColumns(table: string) {
    const statement = await this.runStatement(
      `SELECT p.name as columnName,
                    m.name as tableName

             FROM sqlite_master m
                      left outer join pragma_table_info((m.name)) p
                                      on m.name <> p.name
             where m.name = '${table}'
             order by tableName, columnName
            `,
    )
    if (isErr(statement)) {
      return statement
    }

    return Ok(statement.value.columns[0].values)
  }

  returnPreTableQualifier(): string {
    return ''
  }

  returnLanguage(): SqlLanguage {
    return 'sqlite'
  }

  abstract returnDatabaseConfig(): ModifiedConnectionConfig
}

const DefaultSqliteDatabaseDependentSetting = {
  runQueriesByDefault: true,
  lookForCacheViews: false,
}

export class InMemorySqlite extends Sqlite implements ServicesDatabase {
  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'sqliteInMemory',
      sqliteInMemory: {},
    }
  }

  private constructor(db: sql.Database) {
    super(db)
  }

  static async create() {
    const initializer = await sql({ locateFile })
    const db = new initializer.Database()
    return new InMemorySqlite(db)
  }

  protected postHook = async () => {}

  returnDatabaseConfiguration(): DatabaseDependentSettings {
    return {
      ...DefaultSqliteDatabaseDependentSetting,
      importSourcesAfterOnboarding: false,
    }
  }
}

// TODO - this is a hack to get around the fact that the sql.js library is currently broken
const locateFile = (file: string) =>
  `https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.10.3/${file}`

export class PathBasedSqlite extends Sqlite implements ServicesDatabase {
  private readonly fullPath: Uri
  private readonly relativePath: Uri
  private readonly writer: Writer

  static async create(
    fullPath: Uri,
    relativePath: Uri,
    writer: Writer,
    fileReader: Reader,
  ) {
    const initializer = await sql({ locateFile })
    let buffer: Buffer
    try {
      const read = await fileReader(fullPath)
      buffer = Buffer.from(read)
    } catch (error) {
      buffer = Buffer.from('')
      await writer(fullPath, buffer)
    }
    const db = new initializer.Database(buffer)
    return new PathBasedSqlite(db, relativePath, fullPath, writer)
  }

  private constructor(
    db: sql.Database,
    relativePath: Uri,
    path: Uri,
    writer: Writer,
  ) {
    super(db)
    this.relativePath = relativePath
    this.fullPath = path
    this.writer = writer
  }

  returnDatabaseConfiguration(): DatabaseDependentSettings {
    return {
      ...DefaultSqliteDatabaseDependentSetting,
      importSourcesAfterOnboarding: true,
    }
  }

  protected postHook = async () => {
    const data = this.db.export()
    await this.writer(this.fullPath, data)
  }

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'sqlite',
      sqlite: {
        path: this.relativePath.fsPath,
      },
    }
  }
}
