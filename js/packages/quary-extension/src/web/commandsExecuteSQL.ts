/* eslint-disable camelcase */
import * as vscode from 'vscode'
import {ExtensionContext} from 'vscode'
import {Err, ErrorCodes, isErr, Ok, Result} from '@shared/result'
import * as Papa from 'papaparse'
import {useCallBackBackEnd} from '@shared/callBacks'
import {
  ListAssetsRequest_AssetsToSkip,
  ListAssetsResponse_Asset
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import {queryResultToColumnsValues} from '@shared/shared'
import {rustWithoutDatabaseWasmServices} from './servicesRustWasm'
import {getPreInitServices, Services} from './services'
import {renderingFunction} from './commandsScaffolding'
import {DEFAULT_LIMIT_FOR_SELECT} from './defaults'
import {cacheViewBuilder} from './cacheViewBuilder'

const getModelDetails = async ({
  services,
  projectRoot,
  modelName,
  rustServices,
  limit,
}: {
  services: Services
  projectRoot: string
  modelName: string
  rustServices: ReturnType<typeof rustWithoutDatabaseWasmServices>
  limit: number | undefined
}): Promise<
  Result<{
    model: ListAssetsResponse_Asset
    limit: number | undefined
    sql: string
  }>
> => {
  const assetsResponse = await services.rust.list_assets({
    projectRoot,
    assetsToSkip: ListAssetsRequest_AssetsToSkip.ASSETS_TO_SKIP_CHARTS,
  })
  if (isErr(assetsResponse)) {
    return assetsResponse
  }
  const assets = assetsResponse.value.assets

  const asset = assets.find((asset) => asset.name === modelName)
  if (asset === undefined) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: `Model ${modelName} not found`,
    })
  }

  const cacheViewInformation = await cacheViewBuilder(services.database)
  if (isErr(cacheViewInformation)) {
    return cacheViewInformation
  }
  const fullDetails = await services.rust.return_full_sql_for_asset({
    projectRoot,
    assetName: asset.name,
    cacheViewInformation: cacheViewInformation.value,
  })

  if (isErr(fullDetails)) {
    return fullDetails
  }

  const { fullSql } = fullDetails.value

  const sql = limit ? rustServices.add_limit_to_select(fullSql, limit) : fullSql

  return Ok({
    limit,
    sql,
    model: asset,
  })
}

export const executeSQLOnModel = async (
  modelName: string,
  services: Services,
  projectRoot: string,
  extensionContext: ExtensionContext,
  rustServices: ReturnType<typeof rustWithoutDatabaseWasmServices>,
) =>
  await renderingFunction({
    title: `Run: ${modelName}`,
    fn: async (setState, panel, extensionContext) => {
      const executeSQLViewRunQuery = async ({
        limit,
      }: {
        limit: number | undefined
      }) => {
        setState({
          type: 'executeSQL',
          limit,
          results: {
            type: 'loading',
          },
        })
        const modelDetails = await getModelDetails({
          services,
          projectRoot,
          modelName,
          rustServices,
          limit,
        })
        if (isErr(modelDetails)) {
          return await setState({
            type: 'executeSQL',
            results: {
              type: 'error',
              error: modelDetails.error,
            },
          })
        }
        const { sql } = modelDetails.value
        const results = await services.database.runStatement(sql)
        if (isErr(results)) {
          return await setState({
            type: 'executeSQL',
            results: {
              type: 'error',
              error: results.error,
            },
          })
        }
        return await setState({
          type: 'executeSQL',
          limit,
          results: {
            type: 'run',
            modelName,
            results: results.value,
          },
        })
      }

      useCallBackBackEnd(
        [
          'executeSQLViewRunQuery',
          'executeSQLViewExportCSV',
          'executeSQLViewCopyToClipboard',
          'executeSQLViewCreateChart',
        ],
        {
          executeSQLViewRunQuery,
          executeSQLViewExportCSV: async ({ data }) => {
            try {
              const { values, columns } = queryResultToColumnsValues(data)
              const csv = Papa.unparse({
                fields: columns.map((column) => column.column),
                data: values,
              })
              const uri = await vscode.window.showSaveDialog({
                filters: {
                  'CSV files': ['csv'],
                },
              })
              if (uri) {
                const data = new TextEncoder().encode(csv)
                await services.fileSystem.writeFile(uri, data)
                vscode.window.showInformationMessage(
                  `File saved successfully to ${uri.fsPath}`,
                )
              } else {
                vscode.window.showInformationMessage(
                  'Save operation cancelled.',
                )
              }
            } catch (error) {
              vscode.window.showErrorMessage(
                `Error saving file ${error instanceof Error ? error.message : 'unkown error'}`,
              )
            }
          },
          executeSQLViewCopyToClipboard: async ({ data }) => {
            try {
              const { values, columns } = queryResultToColumnsValues(data)
              const csv = Papa.unparse({
                fields: columns.map((column) => column.column),
                data: values,
              })
              await vscode.env.clipboard.writeText(csv)
              vscode.window.showInformationMessage('CSV copied to clipboard.')
            } catch (error) {
              vscode.window.showErrorMessage(
                `Error copying to clipboard ${error instanceof Error ? error.message : 'unknown error'}`,
              )
            }
          },
          executeSQLViewCreateChart: async ({ model, chartSettings }) => {
            const services = getPreInitServices(extensionContext)
            const content = await services.rust.create_model_chart_file({
              modelName: model,
              config: chartSettings,
            })
            if (isErr(content)) {
              vscode.window.showErrorMessage(
                `Error creating chart ${content.error.message}`,
              )
              return
            }

            const doc = await vscode.workspace.openTextDocument({
              language: 'yaml',
              content: content.value.chartFile,
            })
            vscode.window.showTextDocument(doc, {
              preview: true,
            })
          },
        },
        setState,
        panel,
        extensionContext,
      )

      await executeSQLViewRunQuery({ limit: DEFAULT_LIMIT_FOR_SELECT })

      return Ok(undefined)
    },
    extensionContext,
  })
