import { Err, Ok, Result } from '@shared/result'
import { TestResult } from '@quary/proto/quary/service/v1/test_result'
import { Test } from '@shared/globalViewState'

/**
 * Maps a test result from the proto format to the internal bus format.
 *
 * TODO This is a bit of a mess. We should probably just use the proto format internally as well.
 */
export const testMapper = (test: TestResult): Result<Test> => {
  const { query, testName, testResult } = test
  if (testResult === undefined) {
    return Err(new Error(`testResult is undefined in ${JSON.stringify(test)}`))
  }
  switch (testResult.$case) {
    case 'failed': {
      const reason = testResult.failed.reason
      if (reason === undefined) {
        return Err(new Error(`reason is undefined in ${JSON.stringify(test)}`))
      }
      switch (reason.$case) {
        case 'ran': {
          return Ok({
            query,
            testName,
            status: { type: 'fail' },
          })
        }
        case 'inferredFromTests': {
          return Ok({
            query,
            testName,
            status: {
              type: 'fail_inferred',
              sourceTest: reason.inferredFromTests.inferredChain,
            },
          })
        }
        case 'inferredThroughTestsOperation': {
          return Ok({
            query,
            testName,
            status: {
              type: 'fail_inferred',
              sourceTest: [
                `'${
                  reason.inferredThroughTestsOperation.operation
                }' ➡️${reason.inferredThroughTestsOperation.inferredChain.join(
                  '➡️',
                )}`,
              ],
            },
          })
        }
        default: {
          return Err(
            new Error(
              `unknown reason '${reason}' in '${JSON.stringify(test)}'`,
            ),
          )
        }
      }
    }
    case 'passed': {
      const reason = testResult.passed.reason
      if (reason === undefined) {
        return Err(new Error(`reason is undefined in ${JSON.stringify(test)}`))
      }
      switch (reason.$case) {
        case 'ran': {
          return Ok({
            query,
            testName,
            status: { type: 'pass' },
          })
        }
        case 'inferredFromTests': {
          return Ok({
            query,
            testName,
            status: {
              type: 'pass_inferred',
              sourceTest: reason.inferredFromTests.inferredChain,
            },
          })
        }
        case 'inferredFromLogic': {
          return Ok({
            query,
            testName,
            status: {
              type: 'pass_inferred_from_logic',
              explanation: reason.inferredFromLogic,
            },
          })
        }
        case 'inferredThroughTestsOperation': {
          return Ok({
            query,
            testName,
            status: {
              type: 'pass_inferred',
              sourceTest: [
                `'${
                  reason.inferredThroughTestsOperation.operation
                }' ➡️ ${reason.inferredThroughTestsOperation.inferredChain.join(
                  '➡️',
                )}`,
              ],
            },
          })
        }
        default: {
          return Err(
            new Error(
              `unknown reason '${reason}' in '${JSON.stringify(test)}'`,
            ),
          )
        }
      }
    }
    default: {
      return Err(new Error(`unknown testResult ${JSON.stringify(testResult)}`))
    }
  }
}
