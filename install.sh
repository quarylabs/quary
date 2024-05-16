#!/bin/bash

set -e -o pipefail

readonly GREEN="$(tput setaf 2 2>/dev/null || echo '')"
readonly CYAN="$(tput setaf 6 2>/dev/null || echo '')"
readonly NO_COLOR="$(tput sgr0 2>/dev/null || echo '')"

if ! command -v unzip >/dev/null 2>&1; then
    echo "Error: unzip is required to install quary."
    exit 1
fi

# Define the release information
RELEASE_URL="https://api.github.com/repos/quarylabs/quary/releases/latest"

# Determine the operating system
OS=$(uname -s)
if [ "$OS" = "Darwin" ]; then
    # Determine the CPU architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ]; then
        ASSET_NAME="-macos-aarch64-gnu.zip"
    else
        ASSET_NAME="-macos-x86_64-gnu.zip"
    fi
elif [ "$OS" = "Linux" ]; then
    # Determine the CPU architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "aarch64" ]; then
        ASSET_NAME="-linux-aarch64-gnu.zip"
    elif [ "$ARCH" = "x86_64" ]; then
        ASSET_NAME="-linux-x86_64-gnu.zip"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
else
    echo "Unsupported operating system: $OS"
    exit 1
fi

# Retrieve the download URL for the desired asset
DOWNLOAD_URL=$(curl -sSL $RELEASE_URL | grep -o "browser_download_url.*$ASSET_NAME\"" | cut -d ' ' -f 2)

ASSET_NAME=$(basename $DOWNLOAD_URL)

# Define the installation directory
INSTALL_DIR="/usr/local/bin"

DOWNLOAD_URL=`echo $DOWNLOAD_URL | tr -d '\"'`

# Download the asset
curl -SL $DOWNLOAD_URL -o /tmp/$ASSET_NAME

# Extract the asset
unzip -xzf /tmp/$ASSET_NAME -C /tmp

# Set the correct permissions for the binary
chmod +x /tmp/quary

# Move the extracted binary to the installation directory
# use sudo if available
if command -v sudo >/dev/null 2>&1; then
    sudo mv /tmp/quary $INSTALL_DIR
else
    mv /tmp/quary $INSTALL_DIR
fi

# Clean up temporary files
rm /tmp/$ASSET_NAME

cat << EOF
${CYAN}

  __  _  _  __   ___   _  _
 /  \( )( )(  ) (  ,) ( \/ )
( () ))()( /__\  )  \  \  /
 \___\\__/(_)(_)(_)\_)(__/

${NO_COLOR}
BI for engineers!

${GREEN}https://github.com/quarylabs/quary${NO_COLOR}

Please file an issue if you encounter any problems!

===============================================================================

Installation completed! 🎉
EOF
