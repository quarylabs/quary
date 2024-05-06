import { ExtensionContext } from 'vscode'
import { TestRunner } from '@quary/proto/quary/service/v1/test_runner'
import { AIConfig } from '@shared/config'

export interface ServicesStorage {
  setAiConfig: (config: AIConfig) => void
  getAiConfig: () => AIConfig | undefined
  setTestRunner: (testRunner: TestRunner) => void
  getTestRunner: () => TestRunner | undefined
}

export const createExtensionStorageService = (
  extension: ExtensionContext,
): ServicesStorage => ({
  setAiConfig: (config: AIConfig) =>
    extension.workspaceState.update('ai_config', config),
  getAiConfig: () => extension.workspaceState.get<AIConfig>('ai_config'),
  setTestRunner: (testRunner: TestRunner) =>
    extension.workspaceState.update('test_runner', testRunner),
  getTestRunner: () => extension.workspaceState.get<TestRunner>('test_runner'),
})
