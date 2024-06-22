import * as vscode from 'vscode'
import { isErr, Ok } from '@shared/result'
import { DefinitionProvider, Uri } from 'vscode'
import { ListAssetsResponse_Asset_AssetType } from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { getServices, preInitSetup } from './services'

// SQL Autocomplete Provider
export const sqlAutocompleteProvider = (
  context: vscode.ExtensionContext,
): vscode.CompletionItemProvider => ({
  async provideCompletionItems(
    document: vscode.TextDocument,
    position: vscode.Position,
  ): Promise<vscode.CompletionItem[]> {
    const lineText = document.lineAt(position).text
    const linePrefix = lineText.substring(0, position.character)

    if (!linePrefix.endsWith('q.')) {
      return []
    }

    const getAssets = async () => {
      const services = await getServices(context)
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const assets = await services.rust.list_assets({
        projectRoot: details.value.projectRoot,
        assetsToSkip: {
          charts: true,
        },
      })
      if (isErr(assets)) {
        return assets
      }
      return Ok(assets.value.assets)
    }

    const entries = await getAssets()
    if (isErr(entries)) {
      return []
    }

    const filteredWithoutActiveAsset = entries.value.filter(
      (asset) => !document.fileName.includes(asset.name),
    )

    return filteredWithoutActiveAsset.map((asset) => {
      const item = new vscode.CompletionItem(
        asset.name,
        mapCompletionKind(asset.assetType),
      )
      item.documentation = new vscode.MarkdownString(asset.description)
      return item
    })
  },
})

const mapCompletionKind = (assetType: number): vscode.CompletionItemKind => {
  switch (assetType) {
    case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_MODEL:
      return vscode.CompletionItemKind.Field
    case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SEED:
      return vscode.CompletionItemKind.Reference
    case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SOURCE:
      return vscode.CompletionItemKind.Variable
    case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SNAPSHOT:
      return vscode.CompletionItemKind.Snippet
    default:
      return vscode.CompletionItemKind.Field
  }
}

export const sqlDefinitionProvider = (
  context: vscode.ExtensionContext,
): DefinitionProvider => ({
  async provideDefinition(
    document: vscode.TextDocument,
  ): Promise<vscode.LocationLink[]> {
    const getLocations = async () => {
      const services = await getServices(context)
      const projectRoot = services.fileSystem.getProjectRoot()
      if (isErr(projectRoot)) {
        return projectRoot
      }
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const locations = await services.rust.returnDefinitionLocationsForSQL({
        projectRoot: details.value.projectRoot,
        sql: document.getText(),
      })
      if (isErr(locations)) {
        return locations
      }
      return Ok({
        definitions: locations.value.definitions,
        projectRoot: projectRoot.value,
      })
    }

    const entries = await getLocations()
    if (isErr(entries)) {
      return []
    }

    const locations = entries.value.definitions.map((location) => {
      const start = new vscode.Position(
        location.range!.start!.line,
        location.range!.start!.character,
      )
      const end = new vscode.Position(
        location.range!.end!.line,
        location.range!.end!.character,
      )
      const range = new vscode.Range(start, end)

      const targetUri = Uri.joinPath(
        entries.value.projectRoot,
        location.targetFile,
      )

      return {
        originSelectionRange: range,
        targetUri,
        targetRange: new vscode.Range(
          new vscode.Position(0, 0),
          new vscode.Position(0, 0),
        ),
      }
    })

    return locations
  },
})
