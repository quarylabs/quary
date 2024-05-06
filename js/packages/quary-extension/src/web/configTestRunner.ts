import { QuickPickItem, window } from 'vscode'
import { TestRunner } from '@quary/proto/quary/service/v1/test_runner'
import { ServicesStorage } from './servicesStorage'

export const getTestRunnerType = async (
  storage: ServicesStorage,
): Promise<TestRunner> => {
  const testRunnerType = storage.getTestRunner()
  if (testRunnerType !== undefined) {
    return testRunnerType
  }

  const quickPicks: QuickPickItem[] = [
    {
      label: 'all',
      description: 'This runner will run all tests in the project.',
    },
    {
      label: 'skipper',
      description: 'This runner will skip tests it can infer from other tests.',
    },
  ]
  const input = await window.showQuickPick(quickPicks, {
    title: 'Select a test runner',
    canPickMany: false,
  })
  if (input === undefined) {
    throw Error('No test runner selected')
  }

  switch (input.label) {
    case 'all':
      storage.setTestRunner(TestRunner.TEST_RUNNER_ALL)
      return TestRunner.TEST_RUNNER_ALL
    case 'skipper':
      storage.setTestRunner(TestRunner.TEST_RUNNER_SKIP)
      return TestRunner.TEST_RUNNER_SKIP
    default:
      throw Error('Unknown test runner')
  }
}
