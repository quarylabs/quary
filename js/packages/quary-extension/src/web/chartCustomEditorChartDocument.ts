import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import * as vscode from 'vscode'
import { cloneDeep, isEqual } from 'lodash'
import { isErr } from '@shared/result'
import { Chart } from '@quary/proto/quary/service/v1/chart'
import { Disposable } from './dispose'
import { PreInitServices } from './services'

/**
 * Define the types of edits that can be made to the document.
 */
type ChartEdit = Pick<ChartFile, 'config'>
type SourceEdit = Pick<ChartFile, 'source'>

interface ChartDocumentDelegate {
  getFileData(): Promise<Uint8Array>
}

/**
 * Define the document (the data model) used for chart files.
 */
export class ChartDocument extends Disposable implements vscode.CustomDocument {
  static async create(
    uri: vscode.Uri,
    backupId: string | undefined,
    delegate: ChartDocumentDelegate,
    services: PreInitServices,
  ): Promise<ChartDocument | PromiseLike<ChartDocument>> {
    // If we have a backup, read that. Otherwise read the resource from the workspace
    const dataFile =
      typeof backupId === 'string' ? vscode.Uri.parse(backupId) : uri
    const [chartFile, fileData] = await ChartDocument.readFile(
      dataFile,
      services,
    )
    return new ChartDocument(uri, fileData, chartFile, delegate, services)
  }

  private static async readFile(
    uri: vscode.Uri,
    services: PreInitServices,
  ): Promise<[ChartFile, Uint8Array]> {
    if (uri.scheme === 'untitled') {
      return [Chart.create({}), new Uint8Array()]
    }
    const fileData = await vscode.workspace.fs.readFile(uri)
    const file = services.rust.parse_chart_file(fileData)
    if (isErr(file)) {
      throw file.error
    }
    return [file.value, fileData]
  }

  private readonly _uri: vscode.Uri
  private readonly initialContent: Uint8Array
  private readonly initialFile: ChartFile

  private readonly _delegate: ChartDocumentDelegate
  private readonly _services: PreInitServices

  private _chartEdits: Array<ChartEdit> = []
  private _sourceEdits: Array<SourceEdit> = []

  private _savedChartEdits: Array<ChartEdit> = []
  private _savedSourceEdits: Array<SourceEdit> = []

  private constructor(
    uri: vscode.Uri,
    initialContent: Uint8Array,
    initialDocument: ChartFile,
    delegate: ChartDocumentDelegate,
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

  private set documentData(data: ChartFile) {
    this._chartEdits = [{ config: data.config }]
    this._sourceEdits = [{ source: data.source }]
  }

  public get documentData(): ChartFile {
    return {
      ...this.initialFile,
      config:
        this._chartEdits.length > 0
          ? this._chartEdits[this._chartEdits.length - 1].config
          : this.initialFile.config,
      source:
        this._sourceEdits.length > 0
          ? this._sourceEdits[this._sourceEdits.length - 1].source
          : this.initialFile.source,
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
      readonly chartEdits: readonly ChartEdit[]
      readonly sourceEdits: readonly SourceEdit[]
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

  makeChartEdit(edit: ChartEdit) {
    // if there are no edits yet, compare to the initial state
    const currentChartConfigState =
      this._chartEdits.length === 0
        ? this.documentData.config
        : this._chartEdits[this._chartEdits.length - 1].config

    // create clones to avoid mutation of the original objects
    const editCopy = cloneDeep(edit)
    const currentStateCopy = cloneDeep({ config: currentChartConfigState })

    // ignore the settings attribute from Perspective's config
    delete editCopy?.config?.settings
    delete currentStateCopy?.config?.settings

    if (!isEqual(editCopy, currentStateCopy)) {
      this._chartEdits.push(edit)
      this._onDidChange.fire({
        undo: async () => {
          this._chartEdits.pop()
          this._onDidChangeDocument.fire({
            chartEdits: this._chartEdits,
            sourceEdits: this._sourceEdits,
          })
        },
        redo: async () => {
          this._chartEdits.push(edit)
          this._onDidChangeDocument.fire({
            chartEdits: this._chartEdits,
            sourceEdits: this._sourceEdits,
          })
        },
      })
    }
  }

  /**
   * Called when the user edits the Source configuration in a webview.
   *
   * This fires an event to notify VS Code that the document has been edited.
   */

  makeSourceEdit(edit: SourceEdit) {
    // if there are no edits yet, compare to the initial state
    const currentSourceConfigState =
      this._sourceEdits.length === 0
        ? this.documentData.source
        : this._sourceEdits[this._sourceEdits.length - 1].source

    // create clones to avoid mutation of the original objects
    const editCopy = cloneDeep(edit)
    const currentStateCopy = cloneDeep(currentSourceConfigState)

    if (!isEqual(editCopy, currentStateCopy)) {
      this._sourceEdits.push(edit)
      this._onDidChange.fire({
        undo: async () => {
          this._chartEdits.pop()
          this._onDidChangeDocument.fire({
            chartEdits: this._chartEdits,
            sourceEdits: this._sourceEdits,
          })
        },
        redo: async () => {
          this._sourceEdits.push(edit)
          this._onDidChangeDocument.fire({
            chartEdits: this._chartEdits,
            sourceEdits: this._sourceEdits,
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
    this._savedChartEdits = Array.from(this._chartEdits)
    this._savedSourceEdits = Array.from(this._sourceEdits)
  }

  /**
   * Called by VS Code when the user saves the document to a new location.
   */
  async saveAs(
    targetResource: vscode.Uri,
    cancellation: vscode.CancellationToken,
  ): Promise<void> {
    const constructedChartFile: ChartFile = {
      ...this.initialFile,
      config:
        this._chartEdits.length > 0
          ? this._chartEdits[this._chartEdits.length - 1].config
          : this.initialFile.config,
      source:
        this._sourceEdits.length > 0
          ? this._sourceEdits[this._sourceEdits.length - 1].source
          : this.initialFile.source,
    }
    const yaml =
      this._services.rust.write_chart_file_to_yaml(constructedChartFile)
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
    const [diskContent, data] = await ChartDocument.readFile(
      this.uri,
      this._services,
    )
    this.documentData = diskContent
    this._chartEdits = this._savedChartEdits
    this._sourceEdits = this._savedSourceEdits
    this._onDidChangeDocument.fire({
      content: data,
      chartEdits: this._chartEdits,
      sourceEdits: this._sourceEdits,
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
