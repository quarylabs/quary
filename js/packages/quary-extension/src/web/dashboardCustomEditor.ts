import * as vscode from 'vscode'
import {
  DashboardEditorData,
  USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET,
  USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
  View,
} from '@shared/globalViewState'
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import {
  ListAssetsRequest_AssetsToSkip,
  ListAssetsResponse_Asset_AssetType,
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { DashboardFile } from '@quary/proto/quary/service/v1/dashboard_file'
import { disposeAll } from './dispose'
import { HTML_STRING } from './panels'
import { getServices, PreInitServices, preInitSetup } from './services'
import { WebviewCollection } from './chartCustomEditorWebviewCollection'
import { DashboardDocument } from './dashboardCustomEditorDashboardDocument'

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
export class DashboardEditorProvider
  implements vscode.CustomEditorProvider<DashboardDocument>
{
  private static newDocumentField = 1

  public static register(
    context: vscode.ExtensionContext,
    services: PreInitServices,
  ): vscode.Disposable {
    // TODO Implement new dashboard file creation
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
      DashboardEditorProvider.viewType,
      new DashboardEditorProvider(context, services),
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

  private static readonly viewType = 'quary.dashboardFile'

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
  ): Promise<DashboardDocument> {
    const document: DashboardDocument = await DashboardDocument.create(
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
          let dashboardFile = DashboardFile.create({})
          if (e.content) {
            const parsed = this._services.rust.parse_dashboard_file(e.content)
            if (isErr(parsed)) {
              return
            }
            dashboardFile = parsed.value
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
            dashboardFile,
          })
        }
      }),
    )
    document.onDidDispose(() => disposeAll(listeners))
    return document
  }

  async resolveCustomEditor(
    document: DashboardDocument,
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
    vscode.CustomDocumentEditEvent<DashboardDocument>
  >()
  public readonly onDidChangeCustomDocument =
    this._onDidChangeCustomDocument.event

  public saveCustomDocument(
    document: DashboardDocument,
    cancellation: vscode.CancellationToken,
  ): Thenable<void> {
    return document.save(cancellation)
  }

  public saveCustomDocumentAs(
    document: DashboardDocument,
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken,
  ): Thenable<void> {
    return document.saveAs(destination, cancellation)
  }

  public revertCustomDocument(document: DashboardDocument): Thenable<void> {
    return document.revert()
  }

  public backupCustomDocument(
    document: DashboardDocument,
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

  private getDashboardmame(): string {

  }

  private postSetData(
    panel: vscode.WebviewPanel,
    payload: DashboardEditorData,
  ): void {
    panel.webview.postMessage({
      type: USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
      payload: {
        type: 'dashboardEditor',
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
      assetsToSkip: ListAssetsRequest_AssetsToSkip.ASSETS_TO_SKIP_DASHBOARDS,
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
    document: DashboardDocument,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    e: any,
  ) {
    switch (e.type) {
      case USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET: {
        const services = await getServices(this._context)

        const dashboardData = await services.rust.returnDashboardWithSql({
        })
        if (isErr(dashboardData)) {
          return this.postMessage(webviewPanel, USE_GLOBAL_STATE_MESSAGE_TYPE_SET, {
            type: 'error',
            error: dashboardData.error
          })
        }
        this.postMessage(webviewPanel, USE_GLOBAL_STATE_MESSAGE_TYPE_SET, {
          type: 'dashboardEditor',
          data: {
            dashboardFile: dashboardData.value
          },
        })
        break
      }
      default: {
        throw new Error(`Unknown message type: ${e.type}`)
      }
    }
  }
}
