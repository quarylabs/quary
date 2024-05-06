#!/bin/bash

set -euxo pipefail

for f in ./.hacking/scripts/*.sh
do
  shellcheck "$f"
done
