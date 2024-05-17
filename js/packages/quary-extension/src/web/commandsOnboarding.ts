import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { useCallBackBackEnd } from '@shared/callBacks'
import {
  DatabaseOnboardingOptions,
  OnboardingViewStates,
} from '@shared/globalViewState'
import { ExtensionContext, Uri, ViewColumn } from 'vscode'
import * as vscode from 'vscode'
import { getPreInitServices, getServices, preInitSetup } from './services'
import { renderingFunction } from './commandsScaffolding'
import { BigQueryOauthHeadless } from './servicesDatabaseBigQuery'
import { PathBasedSqlite } from './servicesDatabaseSqlite'
// eslint-disable-next-line import/no-cycle
import { returnFullCommandName } from './commands'
import { SnowflakeHeadless } from './servicesDatabaseSnowflake'

export const onboarding = (extensionContext: ExtensionContext) => async () => {
  const preInitServices = await getPreInitServices(extensionContext)
  const preInitSetupResult = await preInitSetup(preInitServices)
  if (isErr(preInitSetupResult)) {
    return preInitSetupResult
  }
  const { projectRoot } = preInitSetupResult.value

  const isPathEmptyResult = await preInitServices.rust.is_path_empty({
    projectRoot,
  })
  if (isErr(isPathEmptyResult)) {
    preInitServices.notifications.showErrorMessage(
      `Error checking if folder is empty: ${isPathEmptyResult.error}`,
    )
    return Ok(undefined)
  }
  if (!isPathEmptyResult.value.isEmpty) {
    preInitServices.notifications.showErrorMessage(
      'Directory must be empty to initialise a project.',
    )
    return Ok(undefined)
  }
  return await renderingFunction({
    title: 'Onboarding',
    viewColumn: ViewColumn.Active,
    fn: async (setState, panel, extensionContext) => {
      useCallBackBackEnd(
        [
          'onboardingViewRestartFlow',
          'onboardingViewSelectDatabase',
          'onboardingViewGenerateProject',
        ],
        {
          onboardingViewRestartFlow: async (_, setState) => {
            setState({
              type: 'onboarding',
              states: {
                type: 'init',
              },
            })
          },
          onboardingViewSelectDatabase: async (payload, setState) => {
            setState({
              type: 'onboarding',
              states: {
                type: 'listSourcesLoading',
                database: payload,
              },
            })

            const handleDatabaseSelection = async (): Promise<
              Result<
                Extract<OnboardingViewStates, { type: 'listSourcesSuccess' }>
              >
            > => {
              switch (payload.type) {
                case DatabaseOnboardingOptions.BigQuery: {
                  const bq = new BigQueryOauthHeadless()

                  const listProjectsResult = await bq.listProjects()
                  if (isErr(listProjectsResult)) {
                    return listProjectsResult
                  }

                  // TODO: decouple vendor-specific logic elsewhere
                  const projects = listProjectsResult.value
                  const projectsWithDatasetsResults = await Promise.all(
                    projects.map(async (project) => {
                      const projectId = project.projectReference?.projectId
                      if (!projectId) {
                        return Ok(null)
                      }

                      const listDatasetsResult =
                        await bq.listDatasetsRoot(projectId)
                      if (isErr(listDatasetsResult)) {
                        return Ok(null)
                      }
                      return Ok({
                        projectId,
                        datasets:
                          listDatasetsResult.value?.map(
                            (dataset) => dataset.datasetReference?.datasetId,
                          ) ?? [],
                      })
                    }),
                  )

                  const projectsAndDatasets =
                    projectsWithDatasetsResults.reduce(
                      (acc: Record<string, string[]>, curr) => {
                        if (curr && curr.ok && curr.value) {
                          acc[curr.value.projectId] =
                            curr.value.datasets.filter(
                              (dataset) => dataset !== undefined,
                            ) as string[]
                        }
                        return acc
                      },
                      {},
                    )
                  return Ok({
                    type: 'listSourcesSuccess',
                    sourceDetails: {
                      type: DatabaseOnboardingOptions.BigQuery,
                      projectsAndDatasets,
                    },
                  })
                }
                case DatabaseOnboardingOptions.SQLite: {
                  const { path } = payload
                  const projectRoot =
                    preInitServices.fileSystem.getProjectRoot()
                  if (isErr(projectRoot)) {
                    return projectRoot
                  }
                  const dbPath = Uri.joinPath(projectRoot.value, path)
                  const relativePath = Uri.parse(path)
                  const db = await PathBasedSqlite.create(
                    dbPath,
                    relativePath,
                    preInitServices.fileSystem.writeFile,
                    preInitServices.fileSystem.readFile,
                  )
                  const sources = await db.listSources()
                  if (isErr(sources)) {
                    return sources
                  }
                  return Ok({
                    type: 'listSourcesSuccess',
                    sourceDetails: {
                      type: DatabaseOnboardingOptions.SQLite,
                      path,
                    },
                  })
                }
                case DatabaseOnboardingOptions.SQLiteInMemory: {
                  return Ok({
                    type: 'listSourcesSuccess',
                    sourceDetails: {
                      type: DatabaseOnboardingOptions.SQLiteInMemory,
                    },
                  })
                }
                case DatabaseOnboardingOptions.Snowflake: {
                  const snowflake = new SnowflakeHeadless(payload)
                  const listSourcesResult = await snowflake.listSources()
                  if (isErr(listSourcesResult)) {
                    return listSourcesResult
                  }
                  const listDatabasesAndSchemasResult =
                    await snowflake.listDatabasesAndSchemas()
                  if (isErr(listDatabasesAndSchemasResult)) {
                    return listDatabasesAndSchemasResult
                  }
                  return Ok({
                    type: 'listSourcesSuccess',
                    sourceDetails: {
                      type: DatabaseOnboardingOptions.Snowflake,
                      databasesAndSchemas: listDatabasesAndSchemasResult.value,
                      config: payload,
                    },
                  })
                }
                default: {
                  return Err({
                    code: ErrorCodes.INVALID_ARGUMENT,
                    message: 'Invalid database selected',
                  })
                }
              }
            }

            const result = await handleDatabaseSelection()
            if (isErr(result)) {
              setState({
                type: 'onboarding',
                states: {
                  type: 'listSourcesError',
                  error: result.error.message,
                  database: payload,
                },
              })
            } else {
              setState({
                type: 'onboarding',
                states: result.value,
              })
            }
          },
          onboardingViewGenerateProject: async (payload, setState, panel) => {
            const services = await getPreInitServices(extensionContext)
            const handleGenerateProjectFiles = async (): Promise<
              Result<undefined>
            > => {
              const root = services.fileSystem.getProjectRoot()
              if (isErr(root)) {
                return root
              }
              const projectFiles = await services.rust.generate_project_files({
                connectionConfig: payload,
              })
              if (isErr(projectFiles)) {
                return projectFiles
              }
              return Ok(undefined)
            }

            const filesToWriteResult = await handleGenerateProjectFiles()
            if (isErr(filesToWriteResult)) {
              setState({
                type: 'onboarding',
                states: {
                  type: 'generateProjectError',
                  error: filesToWriteResult.error.message,
                },
              })
            } else {
              panel.dispose()
              const rustServicesWithDB = await getServices(extensionContext)
              if (
                rustServicesWithDB.database.returnDatabaseConfiguration()
                  .importSourcesAfterOnboarding
              ) {
                vscode.commands.executeCommand(
                  returnFullCommandName('importSources'),
                )
              }
            }
          },
        },
        setState,
        panel,
        extensionContext,
      )
      return Ok({
        type: 'onboarding',
        states: {
          type: 'init',
        },
      })
    },
    extensionContext,
  })
}
