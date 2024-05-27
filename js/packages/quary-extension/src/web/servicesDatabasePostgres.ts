import { Ok } from '@shared/result'
import postgres from './postgres/index'
import { ServicesDatabase } from './servicesDatabase'

const DefaultDatabaseDependentSettings = {
  runQueriesByDefault: false,
  lookForCacheViews: false,
}

// @ts-ignore
export class ServicesDatabasePostgres implements ServicesDatabase {
  readonly db: any

  // @ts-ignore
  async runStatement(statement: string): Promise<Result<QueryResult>> {
    const results = this.db`
      ${statement}
    `
    console.log(results)
    throw new Error('Not implemented')
  }

  constructor() {
    this.db = postgres({
      port: 5432,
      host: 'localhost',
      database: 'postgres',
      username: 'postgres',
      password: 'mysecretpassword',
    }, undefined)
  }

  async listTables() {
    return Ok([])
  }
}
