import * as vscode from 'vscode'
import { cloneDeep, isEqual } from 'lodash'
import { isErr } from '@shared/result'
import { DashboardFile } from '@quary/proto/quary/service/v1/dashboard_file'
import { Disposable } from './dispose'
import { PreInitServices } from './services'

/**
 * Define the types of edits that can be made to the document.
 */
interface Edit extends DashboardFile {}

interface DashboardDocumentDelegate {
  getFileData(): Promise<Uint8Array>
}

/**
 * Define the document (the data model) used for chart files.
 */
export class DashboardDocument
  extends Disposable
  implements vscode.CustomDocument
{
  static async create(
    uri: vscode.Uri,
    backupId: string | undefined,
    delegate: DashboardDocumentDelegate,
    services: PreInitServices,
  ): Promise<DashboardDocument | PromiseLike<DashboardDocument>> {
    // If we have a backup, read that. Otherwise read the resource from the workspace
    const dataFile =
      typeof backupId === 'string' ? vscode.Uri.parse(backupId) : uri
    const [chartFile, fileData] = await DashboardDocument.readFile(
      dataFile,
      services,
    )
    return new DashboardDocument(uri, fileData, chartFile, delegate, services)
  }

  private static async readFile(
    uri: vscode.Uri,
    services: PreInitServices,
  ): Promise<[DashboardFile, Uint8Array]> {
    if (uri.scheme === 'untitled') {
      return [DashboardFile.create({}), new Uint8Array()]
    }
    const fileData = await vscode.workspace.fs.readFile(uri)
    const file = services.rust.parse_dashboard_file(fileData)
    if (isErr(file)) {
      throw file.error
    }
    return [file.value, fileData]
  }

  private readonly _uri: vscode.Uri
  private readonly initialContent: Uint8Array
  private readonly initialFile: DashboardFile

  private readonly _delegate: DashboardDocumentDelegate
  private readonly _services: PreInitServices

  private _edits: Array<Edit> = []

  private _savedEdits: Array<Edit> = []

  private constructor(
    uri: vscode.Uri,
    initialContent: Uint8Array,
    initialDocument: DashboardFile,
    delegate: DashboardDocumentDelegate,
    services: PreInitServices,
  ) {
    super()
    this._uri = uri
    this.initialContent = initialContent
    this.initialFile = initialDocument
    this._delegate = delegate
    this._services = services
  }

  public get uri() {
    return this._uri
  }

  private set documentData(data: DashboardFile) {
    this._edits = [data]
  }

  public get documentData(): DashboardFile {
    return {
      ...this.initialFile,
      ...(this._edits.length > 0
        ? this._edits[this._edits.length - 1]
        : this.initialFile),
    }
  }

  private readonly _onDidDispose = this._register(
    new vscode.EventEmitter<void>(),
  )

  /**
   * Fired when the document is disposed of.
   */
  public readonly onDidDispose = this._onDidDispose.event

  private readonly _onDidChangeDocument = this._register(
    new vscode.EventEmitter<{
      readonly content?: Uint8Array
      readonly edits: readonly DashboardFile[]
    }>(),
  )

  /**
   * Fired to notify webviews that the document has changed.
   */
  public readonly onDidChangeContent = this._onDidChangeDocument.event

  private readonly _onDidChange = this._register(
    new vscode.EventEmitter<{
      undo(): void
      redo(): void
    }>(),
  )

  /**
   * Fired to tell VS Code that an edit has occurred in the document.
   *
   * This updates the document's dirty indicator.
   */
  public readonly onDidChange = this._onDidChange.event

  /**
   * Called by VS Code when there are no more references to the document.
   *
   * This happens when all editors for it have been closed.
   */
  dispose(): void {
    this._onDidDispose.fire()
    super.dispose()
  }

  /**
   * Called when the user edits the Chart configuration in a webview.
   *
   * This fires an event to notify VS Code that the document has been edited.
   */

  makeEdit(edit: DashboardFile) {
    // if there are no edits yet, compare to the initial state
    const currentChartConfigState =
      this._edits.length === 0
        ? this.documentData
        : this._edits[this._edits.length - 1]

    // create clones to avoid mutation of the original objects
    const editCopy = cloneDeep(edit)
    const currentStateCopy = cloneDeep({ config: currentChartConfigState })

    if (!isEqual(editCopy, currentStateCopy)) {
      this._edits.push(edit)
      this._onDidChange.fire({
        undo: async () => {
          this._edits.pop()
          this._onDidChangeDocument.fire({
            edits: this._edits,
          })
        },
        redo: async () => {
          this._edits.push(edit)
          this._onDidChangeDocument.fire({
            edits: this._edits,
          })
        },
      })
    }
  }

  /**
   * Called by VS Code when the user saves the document.
   */
  async save(cancellation: vscode.CancellationToken): Promise<void> {
    await this.saveAs(this.uri, cancellation)
    this._edits = Array.from(this._edits)
  }

  /**
   * Called by VS Code when the user saves the document to a new location.
   */
  async saveAs(
    targetResource: vscode.Uri,
    cancellation: vscode.CancellationToken,
  ): Promise<void> {
    const constructedChartFile: DashboardFile = {
      ...this.initialFile,
      ...(this._edits.length > 0
        ? this._edits[this._edits.length - 1]
        : this.initialFile),
    }
    const yaml =
      this._services.rust.write_dashboard_file_to_yaml(constructedChartFile)
    if (cancellation.isCancellationRequested) {
      return
    }
    if (isErr(yaml)) {
      throw yaml.error
    }
    await vscode.workspace.fs.writeFile(targetResource, yaml.value)
  }

  /**
   * Called by VS Code when the user calls `revert` on a document.
   */
  async revert(): Promise<void> {
    const [diskContent, data] = await DashboardDocument.readFile(
      this.uri,
      this._services,
    )
    this.documentData = diskContent
    this._edits = this._savedEdits
    this._onDidChangeDocument.fire({
      content: data,
      edits: this._edits,
    })
  }

  /**
   * Called by VS Code to backup the edited document.
   *
   * These backups are used to implement hot exit.
   */
  async backup(
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken,
  ): Promise<vscode.CustomDocumentBackup> {
    await this.saveAs(destination, cancellation)

    return {
      id: destination.toString(),
      delete: async () => {
        try {
          await vscode.workspace.fs.delete(destination)
        } catch {
          // noop
        }
      },
    }
  }
}
