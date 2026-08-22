#!/bin/bash

# Extract version from Cargo.toml
cargo_version=$(grep "^version" Cargo.toml | awk -F '"' '{print $2}')

# Extract version from package.json without relying on a host-installed jq.
ts_version=$(sed -n 's/^[[:space:]]*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' js/packages/quary-extension/package.json | head -n 1)

# Optional GitHub release version passed as an argument
github_release_version=$1

# Function to compare two versions
compare_versions() {
    if [ "$1" != "$2" ]; then
        echo "Versions do not match: $1 vs $2"
        exit 1
    fi
}

compare_versions "$cargo_version" "$ts_version"

# If GitHub release version is provided, compare it as well
if [ -n "$github_release_version" ]; then
    compare_versions "$ts_version" "$github_release_version"
fi

echo "Versions match."
