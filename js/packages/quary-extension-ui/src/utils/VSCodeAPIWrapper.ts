// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import { WebviewApi } from 'vscode-webview'
import { action } from '@storybook/addon-actions'

/**
 * A utility wrapper around the acquireVsCodeApi() function, which enables
 * message passing and state management between the webview and extension
 * contexts.
 *
 * This utility also enables webview code to be run in a web browser-based
 * dev server by using native web browser features that mock the functionality
 * enabled by acquireVsCodeApi.
 */
class VSCodeAPIWrapper {
  private readonly vsCodeApi: WebviewApi<unknown> | undefined

  constructor() {
    // Check if the acquireVsCodeApi function exists in the current development
    // context (i.e. VS Code development window or web browser)
    if (typeof acquireVsCodeApi === 'function') {
      this.vsCodeApi = acquireVsCodeApi()
    }
    this.postMessage = this.postMessage.bind(this)
  }

  /**
   * Post a message (i.e. send arbitrary data) to the owner of the webview.
   *
   * @remarks When running webview code inside a web browser, postMessage will instead
   * log the given message to the console.
   *
   * @param message Abitrary data (must be JSON serializable) to send to the extension context.
   */
  public postMessage({ type, payload }: { type: string; payload: object }) {
    if (this.vsCodeApi) {
      this.vsCodeApi.postMessage({ type, payload })
    } else {
      // eslint-disable-next-line no-console
      console.log(`Error sending message: ${type}: ${payload}`)
      action('postMessage')(type, payload)
    }
  }

  /**
   * Get the persistent state stored for this webview.
   *
   * @remarks When running webview source code inside a web browser, getState will retrieve state
   * from local storage (https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage).
   *
   * @return The current state or `undefined` if no state has been set.
   */
  public getState(): unknown | undefined {
    if (this.vsCodeApi) {
      return this.vsCodeApi.getState()
    }
    const state = localStorage.getItem('vscodeState')
    return state !== null ? JSON.parse(state) : undefined
  }

  /**
   * Set the persistent state stored for this webview.
   *
   * @remarks When running webview source code inside a web browser, setState will set the given
   * state using local storage (https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage).
   *
   * @param newState New persisted state. This must be a JSON serializable object. Can be retrieved
   * using {@link getState}.
   *
   * @return The new state.
   */
  public setState<T extends unknown | undefined>(newState: T): T {
    if (this.vsCodeApi) {
      return this.vsCodeApi.setState(newState)
    }
    localStorage.setItem('vscodeState', JSON.stringify(newState))
    return newState
  }

  public addEventListener(listener: (event: MessageEvent) => void): () => void {
    if (this.vsCodeApi) {
      window.addEventListener('message', listener)
      return () => window.removeEventListener('message', listener)
    }
    // eslint-disable-next-line no-console
    console.log('addEventListener', listener)
    // eslint-disable-next-line no-console
    return () => console.log('removeEventListener', listener)
  }
}

// Exports class singleton to prevent multiple invocations of acquireVsCodeApi.
export const vscode = new VSCodeAPIWrapper()
