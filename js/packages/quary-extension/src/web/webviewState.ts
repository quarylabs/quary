import type { ExtensionContext, WebviewPanel } from 'vscode'
import {
  USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET,
  USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
  View,
} from '@shared/globalViewState'

export const useGlobalState = (
  panel: WebviewPanel,
  extensionContext: ExtensionContext,
): [() => View, (view: View) => Promise<void>] => {
  let view: View = {
    type: 'loading',
  }

  const setState = async (newView: View) => {
    view = newView
    await panel.webview.postMessage({
      type: USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
      payload: view,
    })
  }

  panel.webview.onDidReceiveMessage(
    async (message) => {
      if (message.type === USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET) {
        await panel.webview.postMessage({
          type: USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
          payload: view,
        })
      }
    },
    undefined,
    extensionContext.subscriptions,
  )

  return [() => view, setState]
}
