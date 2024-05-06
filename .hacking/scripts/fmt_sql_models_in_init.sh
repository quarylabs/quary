#!/bin/bash

set -euxo pipefail

for f in ./go/lib/init/models/**/*.sql
do
  pnpx sql-formatter --fix "$f"
done
