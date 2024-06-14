import type { ExtensionContext, WebviewPanel } from 'vscode'
import type { WebviewApi } from 'vscode-webview'
import {
  OnboardingViewDatabaseType,
  USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
  USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET,
  View,
} from './globalViewState'
import {
  ColumnTest,
  ProjectFileSource,
} from '@quary/proto/quary/service/v1/project_file'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { JSONStruct } from './jsonValue'

export type Callbacks = {
  useGlobalStateNotSet: null
  documentationViewRunSqlQuery: null
  documentationViewLoad: null
  documentationViewOpenFile: { filePath: string }
  documentationViewAddToSchema: null
  documentationViewUpdateColumnDescription: {
    column: string
    description: string
  }
  documentationViewUpdateDescription: { description: string }
  documentationViewAddColumn: { column: string }
  documentationViewAddColumnTest: {
    column: string
    columnTest: ColumnTest
  }
  documentationViewRemoveColumnTest: {
    column: string
    columnTest: ColumnTest
  }
  onboardingViewGenerateProject: ConnectionConfig
  onboardingViewRestartFlow: null
  onboardingViewSelectDatabase: OnboardingViewDatabaseType
  executeSQLViewRunQuery: { limit: number | undefined }
  executeSQLViewExportCSV: { data: QueryResult }
  executeSQLViewCopyToClipboard: { data: QueryResult }
  executeSQLViewCreateChart: {
    model: string
    chartSettings: JSONStruct
  }
  importSources: {
    sources: ProjectFileSource[]
    folderPath: string
  }
  chartViewRunQuery: null
  chartViewChangeHandler: ChartFile
  chartViewMakeSourceEdit: ChartFile['source']
  chartViewMakeChartEdit: ChartFile['config']
  chartViewOpenTextEditor: null
  chartViewCreateModel: string
}

export const useCallBackFrontEnd = <T extends keyof Callbacks>(
  methodArray: T[],
  messagePoster: WebviewApi<any>['postMessage'],
): { [K in T]: (message: Callbacks[K]) => void } => {
  const handlers: Partial<Record<T, (message: Callbacks[T]) => void>> = {}
  methodArray.forEach((method) => {
    handlers[method] = async (message: Callbacks[T]) => {
      // Implementation of each method, for example, you could post a message back to the parent window
      messagePoster({ type: method, payload: message })
    }
  })
  return handlers as Record<T, (message: Callbacks[T]) => void>
}

export const useCallBackBackEnd = <T extends keyof Callbacks>(
  methodArray: T[],
  callbacks: Partial<{
    [K in T]: (
      message: Callbacks[K],
      setState: (state: View) => void,
      panel: WebviewPanel,
    ) => Promise<void>
  }>,
  setState: (state: View) => void,
  panel: WebviewPanel,
  context: ExtensionContext,
) => {
  const methodSet = new Set(methodArray)
  panel.webview.onDidReceiveMessage(
    async (event: { type: string; payload: Object }) => {
      if (
        event.type === USE_GLOBAL_STATE_MESSAGE_TYPE_SET ||
        event.type === USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET
      ) {
        return
      } else {
        const { type, payload } = event
        if (!methodSet.has(type as T)) {
          throw new Error(`Unknown command ${type}`)
        }
        const callback = callbacks[type as T]
        if (callback) {
          await callback(payload as Callbacks[T], setState, panel)
        } else {
          throw new Error(`Unknown command ${type}`)
        }
      }
    },
    undefined,
    context.subscriptions,
  )
}
