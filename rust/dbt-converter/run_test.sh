#!/usr/bin/env bash

set -euo pipefail

runfiles_root="${RUNFILES_DIR:?RUNFILES_DIR is required}"
workspace_root="${runfiles_root}/${TEST_WORKSPACE:?TEST_WORKSPACE is required}"
materialized_root="${TEST_TMPDIR:?TEST_TMPDIR is required}/workspace"

mkdir -p "$materialized_root/rust/dbt-converter"
cp -LR \
    "$workspace_root/rust/dbt-converter/test_fixtures" \
    "$materialized_root/rust/dbt-converter/test_fixtures"

cd "$materialized_root"
exec "${runfiles_root}/${RUST_TEST:?RUST_TEST is required}"
