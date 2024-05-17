import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import * as vscode from 'vscode'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import {
  BigQueryDataset,
  BigQueryProject,
  BigQueryTable,
  BigQueryTableSchema,
} from '@quary/proto/quary/service/v1/connection_response'
import {
  makeBigQueryRequest,
  runBigQueryStatement,
} from '@shared/databaseBigQuery'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import {
  ModifiedConnectionConfig,
  ServicesDatabase,
  SourcesLister,
} from './servicesDatabase'

abstract class BigQueryBase {
  protected async makeBigQueryRequest<T>(
    url: string,
    method: 'GET' | 'POST' | 'PUT' | 'DELETE' = 'GET',
    body?: object,
  ): Promise<Result<T>> {
    const accessToken = await this.getAccessToken()
    if (isErr(accessToken)) {
      return accessToken
    }
    return makeBigQueryRequest(accessToken.value, url, method, body)
  }

  protected async getAccessToken(): Promise<Result<string>> {
    const session = await vscode.authentication.getSession(
      'quaryBigQuery',
      [],
      {
        createIfNone: true,
      },
    )
    if (!session) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: 'Failed to get BigQuery session',
      })
    }
    return Ok(session.accessToken)
  }

  async listProjects(): Promise<Result<BigQueryProject[]>> {
    const response = await this.makeBigQueryRequest<{
      projects: BigQueryProject[]
    }>(`https://bigquery.googleapis.com/bigquery/v2/projects`)

    if (isErr(response)) {
      return response
    }
    return Ok(response.value.projects)
  }

  async listDatasetsRoot(
    projectId: string,
  ): Promise<Result<BigQueryDataset[]>> {
    const response = await this.makeBigQueryRequest<{
      datasets: BigQueryDataset[]
    }>(
      `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/datasets`,
    )
    if (isErr(response)) {
      return response
    }
    return Ok(response.value.datasets || [])
  }

  async listTablesRoot(
    projectId: string,
    datasetId: string,
  ): Promise<Result<TableAddress[]>> {
    const response = await this.makeBigQueryRequest<{
      tables: BigQueryTable[]
    }>(
      `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/datasets/${datasetId}/tables`,
    )
    if (isErr(response)) {
      return response
    }
    try {
      if (!Array.isArray(response.value.tables)) {
        return Ok([])
      }
      return Ok(
        response.value.tables.reduce((acc: TableAddress[], table) => {
          if (table.type === 'TABLE' && table.tableReference?.tableId) {
            acc.push({
              name: table.tableReference.tableId,
              fullPath: `${projectId}.${datasetId}.${table.tableReference.tableId}`,
            })
          }
          return acc
        }, []),
      )
    } catch (error) {
      if (error instanceof Error) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: `Failed to parse BigQuery tables: ${error.message}`,
        })
      }
      return Err({
        code: ErrorCodes.INTERNAL,
        message: 'Failed to parse BigQuery tables ${error}',
      })
    }
  }

  async listViewsRoot(
    projectId: string,
    datasetId: string,
  ): Promise<Result<TableAddress[]>> {
    const response = await this.makeBigQueryRequest<{
      tables: BigQueryTable[]
    }>(
      `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/datasets/${datasetId}/tables`,
    )
    if (isErr(response)) {
      return response
    }
    try {
      if (!Array.isArray(response.value.tables)) {
        return Ok([])
      }
      return Ok(
        response.value.tables.reduce((acc: TableAddress[], table) => {
          if (table.type === 'VIEW' && table.tableReference?.tableId) {
            acc.push({
              name: table.tableReference.tableId,
              fullPath: `${projectId}.${datasetId}.${table.tableReference.tableId}`,
            })
          }
          return acc
        }, []),
      )
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : 'Unknown error occurred'
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Failed to parse BigQuery views: ${errorMessage}`,
      })
    }
  }

  async listColumnsRoot(
    tableName: string,
    projectId: string,
    datasetId: string,
  ) {
    const response = await this.makeBigQueryRequest<{
      schema: BigQueryTableSchema
    }>(
      `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/datasets/${datasetId}/tables/${tableName}`,
    )

    if (isErr(response)) {
      return response
    }

    const columnNames = response.value.schema.fields.map((field) => field.name)

    return Ok(columnNames.sort())
  }

  // This function only lists Tables in a BigQuery account - Datasets and Projects without tables will not be returned.
  async listSources(): Promise<Result<ProjectFileSource[]>> {
    const resolveExternalProjects = await this.listProjects()
    if (isErr(resolveExternalProjects)) {
      return Err(resolveExternalProjects.error)
    }

    const externalProjects = resolveExternalProjects.value
    const results: ProjectFileSource[] = []

    const projectPromises = externalProjects.map(async (project) => {
      const resolveExternalDatasets = await this.listDatasetsRoot(project.id)
      if (isErr(resolveExternalDatasets)) {
        throw resolveExternalDatasets.error
      }

      const datasetPromises = resolveExternalDatasets.value.map(
        async (dataset) => {
          if (dataset.datasetReference === undefined) {
            throw new Error(`unexpected datasets for undefined ${project.id}`)
          }

          const resolveExternalTables = await this.listTablesRoot(
            dataset.datasetReference?.projectId,
            dataset.datasetReference?.datasetId,
          )
          if (isErr(resolveExternalTables)) {
            throw resolveExternalTables.error
          }

          const tablePromises = resolveExternalTables.value.map(
            async (table) => {
              if (dataset.datasetReference === undefined) {
                throw new Error(`unexpected tables for undefined ${project.id}`)
              }
              const resolveTableColumns = await this.listColumnsRoot(
                table.name,
                dataset.datasetReference?.projectId,
                dataset.datasetReference?.datasetId,
              )
              if (isErr(resolveTableColumns)) {
                throw resolveTableColumns.error
              }

              const source: ProjectFileSource = {
                name: table.name,
                tests: [],
                tags: [],
                path: `${dataset.datasetReference?.projectId}.${dataset.datasetReference?.datasetId}.${table.name}`,
                columns: resolveTableColumns.value.map((column) => ({
                  name: column,
                  tests: [],
                })),
              }
              results.push(source)
            },
          )

          await Promise.all(tablePromises)
        },
      )

      await Promise.all(datasetPromises)
    })

    try {
      await Promise.all(projectPromises)
      return Ok(results)
    } catch (error) {
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `Failed to list BigQuery sources: ${error}`,
      })
    }
  }
}

interface BigQueryOptions {
  projectId: string
  datasetId: string
}

export class BigQueryOAuth extends BigQueryBase implements ServicesDatabase {
  private readonly projectId: string
  private readonly datasetId: string

  constructor(options: BigQueryOptions) {
    super()
    this.projectId = options.projectId
    this.datasetId = options.datasetId
  }

  returnDatabaseConfiguration: () => DatabaseDependentSettings = () => ({
    runQueriesByDefault: false,
    lookForCacheViews: true,
    importSourcesAfterOnboarding: true,
  })

  returnLanguage(): SqlLanguage {
    return 'bigquery'
  }

  returnPreTableQualifier(): string {
    return `${this.projectId}.${this.datasetId}`
  }

  async listTables() {
    return this.listTablesRoot(this.projectId, this.datasetId)
  }

  async listViews() {
    return this.listViewsRoot(this.projectId, this.datasetId)
  }

  async listColumns(tableName: string) {
    return this.listColumnsRoot(tableName, this.projectId, this.datasetId)
  }

  async runStatement(query: string) {
    const accessToken = await this.getAccessToken()
    if (isErr(accessToken)) {
      return accessToken
    }
    return runBigQueryStatement(accessToken.value, query, this.projectId)
  }

  returnDatabaseConfig(): ModifiedConnectionConfig {
    return {
      $case: 'bigQuery',
      bigQuery: {
        projectId: this.projectId,
        datasetId: this.datasetId,
      },
    }
  }
}

export class BigQueryOauthHeadless
  extends BigQueryBase
  implements SourcesLister {}
