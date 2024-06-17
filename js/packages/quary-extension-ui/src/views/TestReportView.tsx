import type { Test, TestStatus } from '@shared/globalViewState'
import { TestRunner } from '@quary/proto/quary/service/v1/test_runner'
import { toSingleLine } from '@ui/utils/sql'
import { PageTitle } from '@ui/components/PageTitle'
import { Table } from '@ui/components/Table'

interface Props {
  testRunner: TestRunner
  tests: Test[]
}

export const TestReportView: React.FC<Props> = ({ tests, testRunner }) => {
  const table =
    testRunner === TestRunner.TEST_RUNNER_ALL ? (
      <Table
        headers={['Pass/Fail', 'Test Name', 'Reason', 'Query']}
        rows={tests.sort(failureFirst).map(({ testName, status, query }) => {
          const [pass, reason] = testStatusToVisual(status)
          return [
            pass,
            testName,
            <p key={testName} className="whitespace-nowrap">
              {reason}
            </p>,
            <TestVisual key={testName} code={query} />,
          ]
        })}
      />
    ) : (
      <Table
        headers={['Pass/Fail', 'Test Name', 'Reason', 'Query']}
        rows={tests.sort(failureFirst).map(({ testName, status, query }) => {
          const [pass, reason] = testStatusToVisual(status)
          return [
            pass,
            testName,
            <p key={testName} className="whitespace-nowrap">
              {reason}
            </p>,
            <TestVisual key={testName} code={query} />,
          ]
        })}
      />
    )

  const numberOfFailedTests = tests.filter(
    ({ status }) =>
      status.type === 'fail_with_result' ||
      status.type === 'fail_inferred' ||
      status.type === 'fail_with_message',
  ).length

  return (
    <div>
      <div className="pt-5">
        <PageTitle>Test Report</PageTitle>
      </div>
      <div>
        {numberOfFailedTests === 0 ? (
          <div className="pt-2 text-green-500">All tests passed</div>
        ) : (
          <div className="pt-2 text-red-500">
            {numberOfFailedTests} tests failed
          </div>
        )}
      </div>
      <div>
        {tests.filter(
          ({ status }) =>
            status.type === 'pass_inferred' ||
            status.type === 'fail_inferred' ||
            status.type === 'pass_inferred_from_logic',
        ).length === 0 ? (
          <div className="pt-2">All tests ran</div>
        ) : (
          <div className="pt-2">
            {
              tests.filter(
                ({ status }) =>
                  status.type === 'pass_inferred' ||
                  status.type === 'fail_inferred' ||
                  status.type === 'pass_inferred_from_logic',
              ).length
            }{' '}
            tests were inferred while{' '}
            {tests.length -
              tests.filter(
                ({ status }) =>
                  status.type === 'pass_inferred' ||
                  status.type === 'fail_inferred' ||
                  status.type === 'pass_inferred_from_logic',
              ).length}{' '}
            tests were run.
          </div>
        )}
      </div>
      <div className="pt-5">{table}</div>
    </div>
  )
}

const TestVisual: React.FC<{ code: string }> = ({ code }) => {
  const singleLine = toSingleLine(code)
  return <p className="whitespace-nowrap">{singleLine}</p>
}

const testStatusToVisual = (status: TestStatus): [string, string] => {
  const part1 =
    status.type === 'pass' ||
    status.type === 'pass_inferred' ||
    status.type === 'pass_inferred_from_logic'
      ? '✅'
      : '❌'
  const part2 = getPart2(status)
  return [part1, part2]
}

const getPart2 = (status: TestStatus): string => {
  switch (status.type) {
    case 'pass':
      return 'Ran Test'
    case 'fail_with_result':
      return 'Ran Test'
    case 'fail_with_message':
      return status.message
    case 'pass_inferred':
      return `inferred from ${status.sourceTest.join(' ➡️ ')}`
    case 'fail_inferred':
      return `inferred from ${status.sourceTest.join(' ➡️ ')}`
    case 'pass_inferred_from_logic':
      return status.explanation
    default:
      throw new Error('Unknown test status')
  }
}

const failureFirst = (a: Test, b: Test): number => {
  if (
    (a.status.type === 'pass' ||
      a.status.type === 'pass_inferred' ||
      a.status.type === 'pass_inferred_from_logic') &&
    (b.status.type === 'fail_with_result' ||
      b.status.type === 'fail_with_message' ||
      b.status.type === 'fail_inferred')
  ) {
    return -1
  }
  if (
    (a.status.type === 'pass' ||
      a.status.type === 'pass_inferred' ||
      a.status.type === 'pass_inferred_from_logic') &&
    (b.status.type === 'pass' || b.status.type === 'pass_inferred')
  ) {
    return 1
  }

  return a.testName.localeCompare(b.testName)
}
