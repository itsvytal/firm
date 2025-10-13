#!/usr/bin/env bash
set -euo pipefail

# Configuration
NAME="firm"
GITHUB_REPO="42futures/firm"

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

# Normalize architecture names
case "$ARCH" in
  x86_64)
    ARCH="amd64"
    ;;
  aarch64)
    ARCH="arm64"
    ;;
  arm64)
    ARCH="arm64"
    ;;
esac

# Map to archive names
case "${OS}_${ARCH}" in
  Linux_amd64)
    ARCHIVE="firm-linux-amd64"
    ;;
  Linux_arm64)
    ARCHIVE="firm-linux-arm64"
    ;;
  Darwin_amd64)
    ARCHIVE="firm-darwin-amd64"
    ;;
  Darwin_arm64)
    ARCHIVE="firm-darwin-arm64"
    ;;
  *)
    echo "Error: Unsupported system: ${OS}_${ARCH}"
    exit 1
    ;;
esac

URL="https://github.com/$GITHUB_REPO/releases/latest/download/$ARCHIVE.tar.gz"

# Download archive
echo "Downloading $NAME..."
curl -fsSL "$URL" -o "/tmp/$ARCHIVE.tar.gz"

# Extract archive to a temporary folder
mkdir -p "/tmp/$ARCHIVE"
tar -xzf "/tmp/$ARCHIVE.tar.gz" -C "/tmp/$ARCHIVE"

# Fix permissions and move archive to bin
INSTALL_DIR="/usr/local/bin"
chmod +x "/tmp/$ARCHIVE/$NAME"
mv "/tmp/$ARCHIVE/$NAME" "$INSTALL_DIR/$NAME"

# Clean up temporary files
rm -rf "/tmp/$ARCHIVE.tar.gz" "/tmp/$ARCHIVE"

echo "✓ Firm was installed to $INSTALL_DIR/$NAME"
