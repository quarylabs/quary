import * as vscode from 'vscode'
import {
  ChartEditorData,
  USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET,
  USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
  View,
} from '@shared/globalViewState'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { Err, ErrorCodes, isErr, Ok, QuaryError, Result } from '@shared/result'
import { ListAssetsResponse_Asset_AssetType } from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { disposeAll } from './dispose'
import { HTML_STRING } from './panels'
import { getServices, PreInitServices, preInitSetup } from './services'
import { WebviewCollection } from './chartCustomEditorWebviewCollection'
import { ChartDocument } from './chartCustomEditorChartDocument'
import { cacheViewBuilder } from './cacheViewBuilder'

/**
 * Provider for chart editors.
 *
 * Chart editors are used for `.chart.yaml` files.
 *
 * This implements:
 * - A custom web view for a `*.chart.yaml` file.
 * - Implementing save, undo, redo, and revert.
 * - Backup.
 *
 * State in the ChartEditorProvider is quite complicated because it has to manage three layers of state:
 * 1. The state of saved file data
 * 2. The state of the document in the editor
 * 3. The state of the webview
 *
 * Equally, to the dimension of state, there are different types of state that interact differently with the different layers:
 * 1. The state of the config part in the file that defines the sort of data
 * 2. The state of the config part in the file that defines the config of the chart
 * 3. The state of the loaded data
 * 4. The state of the assets that have been loaded
 */
export class ChartEditorProvider
  implements vscode.CustomEditorProvider<ChartDocument>
{
  private static newChartFileId = 1

  public static register(
    context: vscode.ExtensionContext,
    services: PreInitServices,
  ): vscode.Disposable {
    // TODO Implement new chart file creation
    // vscode.commands.registerCommand('catCustoms.pawDraw.new', () => {
    //   const workspaceFolders = vscode.workspace.workspaceFolders
    //   if (!workspaceFolders) {
    //     vscode.window.showErrorMessage(
    //       'Creating new Paw Draw files currently requires opening a workspace',
    //     )
    //     return
    //   }
    //
    //   const uri = vscode.Uri.joinPath(
    //     workspaceFolders[0].uri,
    //     `new-${PawDrawEditorProvider.newPawDrawFileId++}.pawdraw`,
    //   ).with({ scheme: 'untitled' })
    //
    //   vscode.commands.executeCommand(
    //     'vscode.openWith',
    //     uri,
    //     PawDrawEditorProvider.viewType,
    //   )
    // })

    return vscode.window.registerCustomEditorProvider(
      ChartEditorProvider.viewType,
      new ChartEditorProvider(context, services),
      {
        // For this demo extension, we enable `retainContextWhenHidden` which keeps the
        // webview alive even when it is not visible. You should avoid using this setting
        // unless is absolutely required as it does have memory overhead.
        webviewOptions: {
          retainContextWhenHidden: true,
        },
        supportsMultipleEditorsPerDocument: false,
      },
    )
  }

  private static readonly viewType = 'quary.chartFile'

  /**
   * Tracks all known webviews
   */
  private readonly webviews = new WebviewCollection()

  constructor(
    private readonly _context: vscode.ExtensionContext,
    private readonly _services: PreInitServices,
  ) {}

  //#region CustomEditorProvider

  async openCustomDocument(
    uri: vscode.Uri,
    openContext: { backupId?: string },
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _: vscode.CancellationToken,
  ): Promise<ChartDocument> {
    const document: ChartDocument = await ChartDocument.create(
      uri,
      openContext.backupId,
      {
        getFileData: async () => {
          const webviewsForDocument = Array.from(
            this.webviews.get(document.uri),
          )
          if (!webviewsForDocument.length) {
            throw new Error('Could not find webview to save for')
          }
          const panel = webviewsForDocument[0]
          const response = await this.postMessageWithResponse<number[]>(
            panel,
            'getFileData',
            {},
          )
          return new Uint8Array(response)
        },
      },
      this._services,
    )
    const listeners: vscode.Disposable[] = []
    listeners.push(
      document.onDidChange((e) => {
        // Tell VS Code that the document has been edited by the use.
        this._onDidChangeCustomDocument.fire({
          document,
          ...e,
        })
      }),
    )
    listeners.push(
      document.onDidChangeContent((e) => {
        // Update all webviews when the document changes
        for (const webviewPanel of this.webviews.get(document.uri)) {
          let chartFile = ChartFile.create({})
          if (e.content) {
            const parsed = this._services.rust.parse_chart_file(e.content)
            if (isErr(parsed)) {
              return
            }
            chartFile = parsed.value
          }
          const fileName = document.uri.fsPath.split('/').pop()

          // return an error if the file name is undefined
          if (!fileName) {
            throw Err({
              code: ErrorCodes.INTERNAL,
              message: 'Unable to extract chart/file name from file system.',
            })
          }
          this.postSetData(webviewPanel, {
            title: fileName,
            allAssets: [],
            chartFile,
            results: {
              type: 'not loaded',
            },
          })
        }
      }),
    )
    document.onDidDispose(() => disposeAll(listeners))
    return document
  }

  async resolveCustomEditor(
    document: ChartDocument,
    webviewPanel: vscode.WebviewPanel,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _: vscode.CancellationToken,
  ): Promise<void> {
    // Add the webview to our internal set of active webviews
    this.webviews.add(document.uri, webviewPanel)

    // Setup initial content for the webview
    webviewPanel.webview.options = { enableScripts: true }
    webviewPanel.webview.html = this.getHtmlForWebview()
    webviewPanel.webview.onDidReceiveMessage((e) =>
      this.onMessage(webviewPanel, document, e),
    )
  }

  private readonly _onDidChangeCustomDocument = new vscode.EventEmitter<
    vscode.CustomDocumentEditEvent<ChartDocument>
  >()
  public readonly onDidChangeCustomDocument =
    this._onDidChangeCustomDocument.event

  public saveCustomDocument(
    document: ChartDocument,
    cancellation: vscode.CancellationToken,
  ): Thenable<void> {
    return document.save(cancellation)
  }

  public saveCustomDocumentAs(
    document: ChartDocument,
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken,
  ): Thenable<void> {
    return document.saveAs(destination, cancellation)
  }

  public revertCustomDocument(document: ChartDocument): Thenable<void> {
    return document.revert()
  }

  public backupCustomDocument(
    document: ChartDocument,
    context: vscode.CustomDocumentBackupContext,
    cancellation: vscode.CancellationToken,
  ): Thenable<vscode.CustomDocumentBackup> {
    return document.backup(context.destination, cancellation)
  }

  //#endregion

  /**
   * Get the static HTML used for in our editor's webviews.
   */
  private getHtmlForWebview(): string {
    return HTML_STRING
  }

  private _requestId = 1
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private readonly _callbacks = new Map<number, (response: any) => void>()

  private postMessageWithResponse<R = unknown>(
    panel: vscode.WebviewPanel,
    type: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    body: any,
  ): Promise<R> {
    const requestId = this._requestId++
    const p = new Promise<R>((resolve) =>
      this._callbacks.set(requestId, resolve),
    )
    panel.webview.postMessage({ type, requestId, body })
    return p
  }

  private postMessage(
    panel: vscode.WebviewPanel,
    type: string,
    payload: View,
  ): void {
    panel.webview.postMessage({ type, payload })
  }

  private postSetData(
    panel: vscode.WebviewPanel,
    payload: ChartEditorData,
  ): void {
    panel.webview.postMessage({
      type: USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
      payload: {
        type: 'chartEditor',
        data: payload,
      },
    })
  }

  private async getAssets(): Promise<Result<string[]>> {
    let allAssets: string[] = []
    const services = await getServices(this._context)
    const setupValues = await preInitSetup(services)
    if (isErr(setupValues)) {
      return setupValues
    }
    const returned = await services.rust.list_assets({
      projectRoot: setupValues.value.projectRoot,
    })
    if (isErr(returned)) {
      return returned
    }
    allAssets = returned.value.assets
      .filter(
        (asset) =>
          asset.assetType ===
            ListAssetsResponse_Asset_AssetType.ASSET_TYPE_MODEL ||
          asset.assetType ===
            ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SOURCE,
      )
      .map((asset) => asset.name)
    allAssets = allAssets.sort()
    return Ok(allAssets)
  }

  private async onMessage(
    webviewPanel: vscode.WebviewPanel,
    document: ChartDocument,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    e: any,
  ) {
    // the file name of the chart file i.e. "chart_name.chart.yaml"
    const fileName = document.uri.fsPath.split('/').pop()
    // the name of the chart i.e. "chart_name"
    const chartName = fileName?.split('.').at(0)

    // return an error if the file name or chart name are undefined
    if (!chartName || !fileName) {
      throw Err({
        code: ErrorCodes.INTERNAL,
        message: 'Unable to extract chart/file name from file system.',
      })
    }

    const title = document.documentData?.config?.title || chartName

    switch (e.type) {
      case USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET: {
        // TODO Implement new chart file creation
        // if (document.uri.scheme === 'untitled') {
        //   this.postMessage(webviewPanel, 'init', {
        //     untitled: true,
        //     editable: true,
        //   })
        // } else {
        // const editable = vscode.workspace.fs.isWritableFileSystem(
        //   document.uri.scheme,
        // )
        const chartFile = document.documentData
        const allAssetsAttempt = await this.getAssets()
        const allAssets = isErr(allAssetsAttempt) ? [] : allAssetsAttempt.value
        const withReferenceName =
          chartFile.source?.$case === 'reference' && allAssets.length === 0
            ? [chartFile.source.reference.name]
            : allAssets
        const view: View = {
          type: 'chartEditor',
          data: {
            title,
            allAssets: withReferenceName,
            chartFile,
            results: {
              type: 'not loaded',
            },
          },
        }
        return this.postMessage(
          webviewPanel,
          USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
          view,
        )
      }
      case 'chartViewMakeSourceEdit': {
        const source = e.payload as ChartFile['source']
        if (source?.$case === 'reference') {
          const allAssetsAttempt = await this.getAssets()
          const allAssets = isErr(allAssetsAttempt)
            ? []
            : allAssetsAttempt.value
          const payload = {
            title,
            allAssets,
            chartFile: document.documentData,
            results: {
              type: 'not loaded' as const,
            },
          }
          this.postSetData(webviewPanel, payload)
        } else {
          const payload = {
            title,
            allAssets: [],
            chartFile: document.documentData,
            results: {
              type: 'not loaded' as const,
            },
          }
          this.postSetData(webviewPanel, payload)
        }
        return document.makeSourceEdit({ source })
      }
      case 'chartViewMakeChartEdit': {
        const config = e.payload as ChartFile['config']
        return document.makeChartEdit({ config })
      }
      case 'chartViewRunQuery': {
        const chartFile = document.documentData
        const services = await getServices(this._context)

        const handleError = (error: QuaryError, assets: string[] = []) => {
          this.postSetData(webviewPanel, {
            title,
            allAssets: assets,
            chartFile,
            results: {
              type: 'error',
              error,
            },
          })
        }

        switch (chartFile.source?.$case) {
          case 'rawSql': {
            this.postSetData(webviewPanel, {
              title,
              allAssets: [],
              chartFile,
              results: {
                type: 'loading',
              },
            })
            const queryResult = await services.database.runStatement(
              chartFile.source.rawSql,
            )
            if (isErr(queryResult)) {
              return handleError(queryResult.error)
            }
            return this.postSetData(webviewPanel, {
              title,
              allAssets: [],
              chartFile,
              results: {
                type: 'success',
                queryResult: queryResult.value,
              },
            })
          }
          case 'reference': {
            this.postSetData(webviewPanel, {
              title,
              allAssets: [chartFile.source.reference.name],
              chartFile,
              results: {
                type: 'loading',
              },
            })
            const allAssetsAttempt = await this.getAssets()
            const allAssets = isErr(allAssetsAttempt)
              ? [chartFile.source.reference.name]
              : allAssetsAttempt.value
            const preInitSetupResult = await preInitSetup(services)
            if (isErr(preInitSetupResult)) {
              return handleError(preInitSetupResult.error, allAssets)
            }
            const cacheView = await cacheViewBuilder(services.database)
            if (isErr(cacheView)) {
              return handleError(cacheView.error, allAssets)
            }
            const sqlForAssetResult =
              await services.rust.return_full_sql_for_asset({
                projectRoot: preInitSetupResult.value.projectRoot,
                assetName: chartFile.source.reference.name,
                cacheViewInformation: cacheView.value,
              })
            if (isErr(sqlForAssetResult)) {
              return handleError(sqlForAssetResult.error, allAssets)
            }

            const queryResult = await services.database.runStatement(
              sqlForAssetResult.value.fullSql,
            )
            if (isErr(queryResult)) {
              return handleError(queryResult.error, allAssets)
            }
            return this.postSetData(webviewPanel, {
              title,
              allAssets,
              chartFile,
              results: {
                type: 'success',
                queryResult: queryResult.value,
              },
            })
          }
          case 'preTemplatedSql': {
            this.postSetData(webviewPanel, {
              title,
              allAssets: [],
              chartFile,
              results: {
                type: 'loading',
              },
            })
            const setupValues = await preInitSetup(services)
            if (isErr(setupValues)) {
              return handleError(setupValues.error)
            }
            const returnSqlResult =
              await services.rust.returnSQLForInjectedModel({
                projectRoot: setupValues.value.projectRoot,
                sql: chartFile.source.preTemplatedSql,
                // treat the chart file name i.e. "chart_name.chart.yaml" as the temporary id
                temporaryId: fileName,
              })
            if (isErr(returnSqlResult)) {
              return handleError(returnSqlResult.error)
            }
            const queryResult = await services.database.runStatement(
              returnSqlResult.value.sql,
            )
            if (isErr(queryResult)) {
              return handleError(queryResult.error)
            }
            return this.postSetData(webviewPanel, {
              title,
              allAssets: [],
              chartFile,
              results: {
                type: 'success',
                queryResult: queryResult.value,
              },
            })
          }
          default:
            return handleError({
              code: ErrorCodes.INTERNAL,
              message: 'Unexpected source type',
            })
        }
      }
      case 'chartViewOpenTextEditor': {
        return vscode.commands.executeCommand(
          'vscode.openWith',
          document.uri,
          'default',
        )
      }
      case 'chartViewCreateModel': {
        const content = e.payload as string
        const doc = await vscode.workspace.openTextDocument({
          language: 'sql',
          content,
        })
        return vscode.window.showTextDocument(doc, {
          preview: true,
        })
      }
      default: {
        throw new Error(`Error message, received message of type ${e.type}`)
      }
    }
  }
}
