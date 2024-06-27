/* eslint-disable camelcase */
import * as vscode from 'vscode'
import { ExtensionContext } from 'vscode'
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { Dag, View } from '@shared/globalViewState'
import { useCallBackBackEnd } from '@shared/callBacks'
import {
  ListAssetsRequest_AssetsToSkip,
  ListAssetsResponse_Asset,
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { Table } from '@quary/proto/quary/service/v1/table'
import { rustWithoutDatabaseWasmServices } from './servicesRustWasm'
import { preInitSetup, Services } from './services'
import { renderingFunction } from './commandsScaffolding'
import { DEFAULT_LIMIT_FOR_SELECT } from './defaults'
import { cacheViewBuilder } from './cacheViewBuilder'

const getModelDetails = async ({
  services,
  projectRoot,
  modelName,
  rustServices,
}: {
  services: Services
  projectRoot: string
  modelName: string
  rustServices: ReturnType<typeof rustWithoutDatabaseWasmServices>
}): Promise<
  Result<{
    model: ListAssetsResponse_Asset
    limit: number
    limitedSQL: string
    table: Table | null
    dag: Dag
    isModelInSchema: boolean
  }>
> => {
  const modelsResponse = await services.rust.list_assets({
    projectRoot,
    assetsToSkip:
      ListAssetsRequest_AssetsToSkip.ASSETS_TO_SKIP_CHARTS_AND_DASHBOARDS,
  })
  if (isErr(modelsResponse)) {
    return modelsResponse
  }
  const assets = modelsResponse.value.assets
  const asset = assets.find((asset) => asset.name === modelName)
  if (asset === undefined) {
    return Err({
      code: ErrorCodes.NOT_FOUND,
      message: `Model ${modelName} not found`,
    })
  }

  const modelTableDetails = await services.rust.getModelTable({
    projectRoot,
    modelName: asset.name,
  })
  const table = !isErr(modelTableDetails)
    ? modelTableDetails.value.table ?? null
    : null
  const cacheViewInformation = await cacheViewBuilder(services.database)
  if (isErr(cacheViewInformation)) {
    return cacheViewInformation
  }
  const fullDetails = await services.rust.return_data_for_doc_view({
    projectRoot,
    assetName: asset.name,
    cacheViewInformation: cacheViewInformation.value,
  })
  if (isErr(fullDetails)) {
    return fullDetails
  }

  const { fullSql, dag } = fullDetails.value
  if (dag === undefined) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: `unexpected undefined dag for ${modelName}`,
    })
  }

  const limit = DEFAULT_LIMIT_FOR_SELECT
  const limitedSQL = rustServices.add_limit_to_select(fullSql, limit)

  return Ok({
    limitedSQL,
    limit,
    fullSql,
    dag: {
      dag,
      models: assets,
    },
    table,
    model: asset,
    isModelInSchema: fullDetails.value.isAssetInSchemaFiles,
  })
}

const extractBaseViewFromModelDetails = async ({
  services,
  projectRoot,
  modelName,
  rustServices,
}: {
  services: Services
  projectRoot: string
  modelName: string
  rustServices: ReturnType<typeof rustWithoutDatabaseWasmServices>
}) => {
  const modelDetails = await getModelDetails({
    services,
    projectRoot,
    modelName,
    rustServices,
  })
  if (isErr(modelDetails)) {
    return modelDetails
  }
  const { limit, dag, table, model, limitedSQL, isModelInSchema } =
    modelDetails.value
  const sqlDocumentation: View = {
    type: 'docsView',
    modelName: model.name,
    description: model.description || null,
    results: {
      type: 'loading',
    },
    limit,
    language: services.database.returnLanguage(),
    dag,
    tags: model.tags,
    table,
    isModelInSchema,
  }
  return Ok({ sqlDocumentation, limitedSQL }) satisfies Result<{
    sqlDocumentation: View
    limitedSQL: string
  }>
}

export const runDocumentationOnModel = async (
  modelName: string,
  services: Services,
  projectRoot: string,
  extensionContext: ExtensionContext,
  rustServices: ReturnType<typeof rustWithoutDatabaseWasmServices>,
) =>
  await renderingFunction({
    title: `Model: ${modelName}`,
    fn: async (setState, panel, extensionContext) => {
      const documentationViewLoad = async (): Promise<void> => {
        const extracted = await extractBaseViewFromModelDetails({
          services,
          projectRoot,
          modelName,
          rustServices,
        })
        if (isErr(extracted)) {
          await setState({
            type: 'error',
            error: extracted.error,
          })
          return
        }
        const { sqlDocumentation, limitedSQL } = extracted.value
        if (
          services.database.returnDatabaseConfiguration().runQueriesByDefault
        ) {
          const results = await services.database.runStatement(limitedSQL)
          if (isErr(results)) {
            await setState({
              ...sqlDocumentation,
              results: {
                type: 'error',
                error: results.error,
              },
            })
          } else {
            await setState({
              ...sqlDocumentation,
              results: {
                type: 'run',
                results: results.value,
                modelName,
              },
            })
          }
        } else {
          await setState({
            ...sqlDocumentation,
            results: {
              type: 'notYetRun',
            },
          })
        }
      }
      useCallBackBackEnd(
        [
          'documentationViewLoad',
          'documentationViewRunSqlQuery',
          'documentationViewOpenFile',
          'documentationViewAddToSchema',
          'documentationViewUpdateDescription',
          'documentationViewAddColumn',
          'documentationViewAddColumnTest',
          'documentationViewRemoveColumnTest',
          'documentationViewUpdateColumnDescription',
        ],
        {
          documentationViewLoad,
          documentationViewRunSqlQuery: async () => {
            const extracted = await extractBaseViewFromModelDetails({
              services,
              projectRoot,
              modelName,
              rustServices,
            })
            if (isErr(extracted)) {
              throw new Error(`error getting model details ${extracted.error}`)
            }
            const { sqlDocumentation, limitedSQL } = extracted.value

            await setState({
              ...sqlDocumentation,
              results: {
                type: 'loading',
              },
            })
            const results = await services.database.runStatement(limitedSQL)
            if (isErr(results)) {
              await setState({
                ...sqlDocumentation,
                results: {
                  type: 'error',
                  error: results.error,
                },
              })
              return
            }
            await setState({
              ...sqlDocumentation,
              results: {
                type: 'run',
                results: results.value,
                modelName,
              },
            })
          },
          documentationViewOpenFile: async ({ filePath }) => {
            const openFile = async (): Promise<Result<null>> => {
              try {
                const rootResult = services.fileSystem.getProjectRoot()
                if (isErr(rootResult)) {
                  return rootResult
                }
                const uriPath = vscode.Uri.joinPath(rootResult.value, filePath)
                const document =
                  await vscode.workspace.openTextDocument(uriPath)

                await vscode.window.showTextDocument(
                  document,
                  vscode.ViewColumn.One,
                )
                return Ok(null)
              } catch (error) {
                return Err({
                  code: ErrorCodes.INTERNAL,
                  message: `unexpected error in opening documentation view ${error}`,
                })
              }
            }
            const openFileResult = await openFile()
            if (isErr(openFileResult)) {
              vscode.window.showErrorMessage(openFileResult.error.message)
            }
          },
          documentationViewAddToSchema: async () => {
            const createmodelSchemaEntryResult =
              await services.rust.createModelSchemaEntry({
                projectRoot,
                modelName,
              })
            if (isErr(createmodelSchemaEntryResult)) {
              services.notifications.showErrorMessage(
                `Error creating schema entry: ${createmodelSchemaEntryResult.error}`,
              )
            } else {
              services.notifications.showMessage(`Added ${modelName} to schema`)
            }
            await documentationViewLoad()
          },
          documentationViewUpdateDescription: async ({ description }) => {
            const updateDescriptionResult =
              await services.rust.updateAssetDescription({
                projectRoot,
                assetName: modelName,
                description,
              })
            if (isErr(updateDescriptionResult)) {
              services.notifications.showErrorMessage(
                `Error updating description: ${updateDescriptionResult.error}`,
              )
            } else {
              services.notifications.showMessage(
                `Updated description for ${modelName}`,
              )
            }
            await documentationViewLoad()
          },
          documentationViewAddColumn: async ({ column }) => {
            const details = await preInitSetup(services)
            if (isErr(details)) {
              throw new Error('Error setting up documentation')
            }
            const { projectRoot } = details.value

            const updateDescriptionResult =
              await services.rust.addColumnToModelOrSource({
                projectRoot,
                modelOrSourceName: modelName,
                columnName: column,
              })
            if (isErr(updateDescriptionResult)) {
              services.notifications.showErrorMessage(
                `Error adding column: ${updateDescriptionResult.error}`,
              )
            }

            await documentationViewLoad()
          },
          documentationViewAddColumnTest: async ({ column, columnTest }) => {
            const details = await preInitSetup(services)
            if (isErr(details)) {
              throw new Error('Error setting up documentation')
            }
            const { projectRoot } = details.value

            const addTestResult =
              await services.rust.addColumnTestToModelOrSourceColumnRequest({
                projectRoot,
                modelOrSourceName: modelName,
                columnName: column,
                columnTest,
              })
            if (isErr(addTestResult)) {
              services.notifications.showErrorMessage(
                `Error adding column test: ${addTestResult.error}`,
              )
            }

            await documentationViewLoad()
          },
          documentationViewRemoveColumnTest: async ({ column, columnTest }) => {
            const details = await preInitSetup(services)
            if (isErr(details)) {
              throw new Error('Error setting up documentation')
            }
            const { projectRoot } = details.value

            const removeColumnTestResult =
              await services.rust.removeColumnTestFromModelOrSourceColumnRequest(
                {
                  projectRoot,
                  modelOrSourceName: modelName,
                  columnName: column,
                  columnTest,
                },
              )
            if (isErr(removeColumnTestResult)) {
              services.notifications.showErrorMessage(
                `Error removing column test: ${removeColumnTestResult.error}`,
              )
            }

            await documentationViewLoad()
          },
          documentationViewUpdateColumnDescription: async ({
            column,
            description,
          }) => {
            const details = await preInitSetup(services)
            if (isErr(details)) {
              throw new Error('Error setting up documentation')
            }
            const { projectRoot } = details.value

            const updateDescriptionResult =
              await services.rust.updateModelSourceColumnDescription({
                projectRoot,
                modelOrSourceName: modelName,
                columnName: column,
                description,
              })
            if (isErr(updateDescriptionResult)) {
              services.notifications.showErrorMessage(
                `Error adding column test: ${updateDescriptionResult.error}`,
              )
            }
            await documentationViewLoad()
          },
        },
        setState,
        panel,
        extensionContext,
      )
      const extracted = await extractBaseViewFromModelDetails({
        services,
        projectRoot,
        modelName,
        rustServices,
      })
      if (isErr(extracted)) {
        return extracted
      }
      const { sqlDocumentation, limitedSQL } = extracted.value
      if (services.database.returnDatabaseConfiguration().runQueriesByDefault) {
        const results = await services.database.runStatement(limitedSQL)
        if (isErr(results)) {
          return Ok({
            ...sqlDocumentation,
            results: {
              type: 'error',
              error: results.error,
            },
          })
        }
        return Ok({
          ...sqlDocumentation,
          results: {
            type: 'run',
            modelName: sqlDocumentation.modelName,
            results: results.value,
          },
        })
      }
      return Ok({
        ...sqlDocumentation,
        results: {
          type: 'notYetRun',
        },
      })
    },
    extensionContext,
  })
