#!/bin/bash

# Extract version from Cargo.toml
cargo_version_cli=$(grep "^version" rust/cli/Cargo.toml | awk -F '"' '{print $2}')
cargo_version_core=$(grep "^version" rust/core/Cargo.toml | awk -F '"' '{print $2}')
cargo_version_databases=$(grep "^version" rust/quary-databases/Cargo.toml | awk -F '"' '{print $2}')
cargo_version_dbt=$(grep "^version" rust/dbt-converter/Cargo.toml | awk -F '"' '{print $2}')
cargo_version_sqlinference=$(grep "^version" rust/sqlinference/Cargo.toml | awk -F '"' '{print $2}')
cargo_version_wasm=$(grep "^version" rust/wasm-binding/Cargo.toml | awk -F '"' '{print $2}')

# Extract version from package.json
ts_version=$(jq -r '.version' js/packages/quary-extension/package.json)

# Optional GitHub release version passed as an argument
github_release_version=$1

# Function to compare two versions
compare_versions() {
    if [ "$1" != "$2" ]; then
        echo "Versions do not match: $1 vs $2"
        exit 1
    fi
}

compare_versions "$cargo_version_cli" "$ts_version"
compare_versions "$cargo_version_core" "$ts_version"
compare_versions "$cargo_version_databases" "$ts_version"
compare_versions "$cargo_version_dbt" "$ts_version"
compare_versions "$cargo_version_sqlinference" "$ts_version"
compare_versions "$cargo_version_wasm" "$ts_version"

# If GitHub release version is provided, compare it as well
if [ -n "$github_release_version" ]; then
    compare_versions "$ts_version" "$github_release_version"
fi

echo "Versions match."
