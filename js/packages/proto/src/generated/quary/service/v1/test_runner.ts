/* eslint-disable */

export const protobufPackage = "quary.service.v1";

export enum TestRunner {
  TEST_RUNNER_UNSPECIFIED = 0,
  /** TEST_RUNNER_ALL - TEST_RUNNER_ALL is a runner that will run all the tests in the test suite. */
  TEST_RUNNER_ALL = 1,
  /** TEST_RUNNER_SKIP - TEST_RUNNER_NONE is a runner that will run skip tests in the test suite that can be inferred from other tests in the test suite. */
  TEST_RUNNER_SKIP = 2,
  UNRECOGNIZED = -1,
}

export function testRunnerFromJSON(object: any): TestRunner {
  switch (object) {
    case 0:
    case "TEST_RUNNER_UNSPECIFIED":
      return TestRunner.TEST_RUNNER_UNSPECIFIED;
    case 1:
    case "TEST_RUNNER_ALL":
      return TestRunner.TEST_RUNNER_ALL;
    case 2:
    case "TEST_RUNNER_SKIP":
      return TestRunner.TEST_RUNNER_SKIP;
    case -1:
    case "UNRECOGNIZED":
    default:
      return TestRunner.UNRECOGNIZED;
  }
}

export function testRunnerToJSON(object: TestRunner): string {
  switch (object) {
    case TestRunner.TEST_RUNNER_UNSPECIFIED:
      return "TEST_RUNNER_UNSPECIFIED";
    case TestRunner.TEST_RUNNER_ALL:
      return "TEST_RUNNER_ALL";
    case TestRunner.TEST_RUNNER_SKIP:
      return "TEST_RUNNER_SKIP";
    case TestRunner.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}
