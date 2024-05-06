#!/bin/bash

set -euxo pipefail

# Check if a version number is provided as an argument
if [ -z "$1" ]; then
  echo "Please provide a version number as an argument."
  exit 1
fi

# Set variables
version="$1"
project_name="quary"
timestamp=$(date +%Y%m%d-%H%M%S)
build_dir="build/$project_name-$timestamp"

# Create the build directory
mkdir -p "$build_dir"

# Iterate over the architectures and operating systems
archs=("amd64" "arm64")
oses=("linux" "darwin")

for arch in "${archs[@]}"; do
  for os in "${oses[@]}"; do
    # Set the output binary name
    output_binary="$build_dir/${project_name}_${os}_${arch}_${version}"

    # Build the Go binary for the specific architecture and operating system
    echo "Building $project_name for $os/$arch..."
    env CGO=0 GOOS="$os" GOARCH="$arch" go build -o "$output_binary" ./main.go

    echo "Build successful: $output_binary"

    # Compress the binary using gzip
    echo "Compressing binary..."
    gzip "$output_binary"
    echo "Compression successful: ${output_binary}.gz"
  done
done
