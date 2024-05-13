#!/bin/bash

set -e

file1="js/packages/quary-extension/README.md"
file2="README.md"

if diff -q --suppress-common-lines "$file1" "$file2" > /dev/null; then
  echo "File $file1 is a subsection of $file2"
  exit 0
else
  echo "Error: File $file1 is not a subsection of $file2" >&2
  exit 1
fi