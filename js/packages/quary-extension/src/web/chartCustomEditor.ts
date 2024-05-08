import * as vscode from 'vscode'
import {
  ChartEditorData,
  USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET,
  USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
  View,
} from '@shared/globalViewState'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { isErr } from '@shared/result'
import { ListAssetsResponse_Asset_AssetType } from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { disposeAll } from './dispose'
import { HTML_STRING } from './panels'
import { getServices, PreInitServices, setup } from './services'
import { WebviewCollection } from './chartCustomEditorWebviewCollection'
import { ChartDocument } from './chartCustomEditorChartDocument'

/**
 * Provider for chart editors.
 *
 * Chart editors are used for `.chart.yaml` files.
 *
 * This implements:
 * - A custom web view for a `*.chart.yaml` file.
 * - Implementing save, undo, redo, and revert.
 * - Backup.
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
          this.postSetData(webviewPanel, {
            title: document.uri.fsPath.split('/').pop() || 'Untitled',
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

  public revertCustomDocument(
    document: ChartDocument,
    cancellation: vscode.CancellationToken,
  ): Thenable<void> {
    return document.revert(cancellation)
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

  private async getAssets(): Promise<string[] | undefined> {
    let allAssets: string[] = []
    try {
      const services = await getServices(this._context)
      const setupValues = await setup(services)
      if (isErr(setupValues)) {
        return undefined
      }
      const returned = await services.rust.list_assets({
        projectRoot: setupValues.value.projectRoot,
      })
      if (isErr(returned)) {
        return undefined
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
    } catch (e) {
      return []
    }
    allAssets = allAssets.sort()
    return allAssets
  }

  private async onMessage(
    webviewPanel: vscode.WebviewPanel,
    document: ChartDocument,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    e: any,
  ) {
    const title =
      document.uri.fsPath.split('/').pop()?.split('.').at(0) || 'No title'

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

        let allAssets = await this.getAssets()
        const chartFile = document.documentData
        allAssets =
          (allAssets?.length === 0 || allAssets === undefined) &&
          chartFile.source?.$case === 'reference'
            ? [chartFile.source.reference.name]
            : allAssets
        const view: View = {
          type: 'chartEditor',
          data: {
            title,
            allAssets: allAssets || [],
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
      case 'chartViewRunQuery': {
        const chartFile = e.payload as ChartFile
        document.makeEdit(chartFile)
        const services = await getServices(this._context)
        switch (chartFile.source?.$case) {
          case 'rawSql': {
            const returned = await services.database.runStatement(
              chartFile.source.rawSql,
            )
            const allAssets = (await this.getAssets()) || []
            if (isErr(returned)) {
              return this.postSetData(webviewPanel, {
                title,
                allAssets,
                chartFile,
                results: {
                  type: 'error',
                  errorMessage: JSON.stringify(returned.error),
                },
              })
            }
            return this.postSetData(webviewPanel, {
              title,
              allAssets,
              chartFile,
              results: {
                type: 'success',
                queryResult: returned.value,
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
            const setupValues = await setup(services)
            if (isErr(setupValues)) {
              return
            }
            const allAssets = (await this.getAssets()) || [
              chartFile.source.reference.name,
            ]
            const sql = await services.rust.return_full_sql_for_asset({
              projectRoot: setupValues.value.projectRoot,
              assetName: chartFile.source.reference.name,
              cacheView: {
                $case: 'doNotUse',
                doNotUse: {},
              },
            })
            if (isErr(sql)) {
              return
            }
            const returned = await services.database.runStatement(
              sql.value.fullSql,
            )
            if (isErr(returned)) {
              return this.postSetData(webviewPanel, {
                title,
                allAssets,
                chartFile,
                results: {
                  type: 'error',
                  errorMessage: JSON.stringify(returned),
                },
              })
            }
            return this.postSetData(webviewPanel, {
              title: document.uri.fsPath.split('/').pop() || 'Untitled',
              allAssets,
              chartFile,
              results: {
                type: 'success',
                queryResult: returned.value,
              },
            })
          }
          case 'preTemplatedSql': {
            const setupValues = await setup(services)
            if (isErr(setupValues)) {
              return
            }
            const allAssets = (await this.getAssets()) || []
            const sql = await services.rust.returnSQLForInjectedModel({
              projectRoot: setupValues.value.projectRoot,
              sql: chartFile.source.preTemplatedSql,
            })
            if (isErr(sql)) {
              return this.postSetData(webviewPanel, {
                title,
                allAssets,
                chartFile,
                results: {
                  type: 'error',
                  errorMessage: JSON.stringify(sql.error),
                },
              })
            }
            const returned = await services.database.runStatement(sql.value.sql)
            if (isErr(returned)) {
              return this.postSetData(webviewPanel, {
                title,
                allAssets,
                chartFile,
                results: {
                  type: 'error',
                  errorMessage: JSON.stringify(returned.error),
                },
              })
            }
            return this.postSetData(webviewPanel, {
              title,
              allAssets,
              chartFile,
              results: {
                type: 'success',
                queryResult: returned.value,
              },
            })
          }
          default:
            return this.postSetData(webviewPanel, {
              title,
              allAssets: [],
              chartFile,
              results: {
                type: 'error',
                errorMessage: 'Unknown source type',
              },
            })
        }
      }
      case 'chartViewChangeHandler': {
        const config = e.payload as ChartFile
        return document.makeEdit(config)
      }
      case 'chartViewOpenTextEditor': {
        return vscode.commands.executeCommand(
          'vscode.openWith',
          document.uri,
          'default',
        )
      }
      default: {
        throw new Error(`Error message, received message of type ${e.type}`)
      }
    }
  }
}
