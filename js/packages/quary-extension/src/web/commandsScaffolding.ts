import { View } from '@shared/globalViewState'
import { ExtensionContext, WebviewPanel, ViewColumn } from 'vscode'
import { getErrorDetails, isErr, Ok, Result } from '@shared/result'
import { createWebViewPanel } from './panels'
import { useGlobalState } from './webviewState'

export type RenderingFunctionOptions = {
  title: string
  fn: (
    setState: (view: View) => Promise<void>,
    panel: WebviewPanel,
    extensionContext: ExtensionContext,
  ) => Promise<Result<View | undefined>>
  extensionContext: ExtensionContext
  viewColumn?: ViewColumn
}

export const renderingFunction = async ({
  title,
  fn,
  extensionContext,
  viewColumn,
}: RenderingFunctionOptions): Promise<Result<undefined>> => {
  const panel = createWebViewPanel({ title, viewColumn })
  const [, setState] = useGlobalState(panel, extensionContext)

  const result = await fn(setState, panel, extensionContext)
  if (isErr(result)) {
    await setState({
      type: 'error',
      error: getErrorDetails(result),
    })
    return result
  }
  if (result.value) {
    await setState(result.value)
  }
  return Ok(undefined)
}

export type CommandFunctionType = () => Promise<Result<undefined>>
