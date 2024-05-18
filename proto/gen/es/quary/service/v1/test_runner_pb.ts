// @generated by protoc-gen-es v1.9.0 with parameter "target=ts"
// @generated from file quary/service/v1/test_runner.proto (package quary.service.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { proto3 } from "@bufbuild/protobuf";

/**
 * @generated from enum quary.service.v1.TestRunner
 */
export enum TestRunner {
  /**
   * @generated from enum value: TEST_RUNNER_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * TEST_RUNNER_ALL is a runner that will run all the tests in the test suite.
   *
   * @generated from enum value: TEST_RUNNER_ALL = 1;
   */
  ALL = 1,

  /**
   * TEST_RUNNER_NONE is a runner that will run skip tests in the test suite that can be inferred from other tests in the test suite.
   *
   * @generated from enum value: TEST_RUNNER_SKIP = 2;
   */
  SKIP = 2,
}
// Retrieve enum metadata with: proto3.getEnumType(TestRunner)
proto3.util.setEnumType(TestRunner, "quary.service.v1.TestRunner", [
  { no: 0, name: "TEST_RUNNER_UNSPECIFIED" },
  { no: 1, name: "TEST_RUNNER_ALL" },
  { no: 2, name: "TEST_RUNNER_SKIP" },
]);

