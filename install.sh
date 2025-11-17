#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="enheit/jumper"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="jumper"

echo -e "${GREEN}Jumper Installer${NC}"
echo "=================="

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

echo "Detected: $OS-$ARCH"

# Check if curl is installed
if ! command -v curl &> /dev/null; then
    echo -e "${RED}Error: curl is required but not installed${NC}"
    exit 1
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Get latest release info from GitHub API
echo "Fetching latest release..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")

# Extract download URL for the appropriate binary
DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep "browser_download_url.*${OS}.*${ARCH}" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${YELLOW}No pre-built binary found for $OS-$ARCH${NC}"
    echo "Installing from source using cargo..."

    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo is required to build from source${NC}"
        echo "Install Rust from https://rustup.rs/"
        exit 1
    fi

    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"

    echo "Cloning repository..."
    git clone "https://github.com/$REPO.git"
    cd jumper

    echo "Building from source..."
    cargo build --release

    echo "Installing binary..."
    cp target/release/jumper "$INSTALL_DIR/"

    cd ~
    rm -rf "$TEMP_DIR"
else
    echo "Downloading from: $DOWNLOAD_URL"

    TEMP_FILE=$(mktemp)
    curl -L "$DOWNLOAD_URL" -o "$TEMP_FILE"

    # Extract and install
    if [[ $DOWNLOAD_URL == *.tar.gz ]]; then
        tar -xzf "$TEMP_FILE" -C "$INSTALL_DIR" "$BINARY_NAME"
    elif [[ $DOWNLOAD_URL == *.zip ]]; then
        unzip -o "$TEMP_FILE" "$BINARY_NAME" -d "$INSTALL_DIR"
    else
        # Assume it's a raw binary
        mv "$TEMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
    fi

    rm -f "$TEMP_FILE"
fi

# Make executable
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo -e "${GREEN}✓ Jumper installed successfully!${NC}"
echo ""
echo "Binary location: $INSTALL_DIR/$BINARY_NAME"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: $INSTALL_DIR is not in your PATH${NC}"
    echo ""
    echo "Add the following line to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo -e "${GREEN}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    echo ""
else
    echo -e "${GREEN}$INSTALL_DIR is already in your PATH${NC}"
fi

# Install shell integration scripts
SHELL_DIR="$HOME/.local/share/jumper/shell"
mkdir -p "$SHELL_DIR"

# Determine script location
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if [ -d "$SCRIPT_DIR/shell" ]; then
    # Installing from source
    cp "$SCRIPT_DIR/shell/jumper.sh" "$SHELL_DIR/jumper.sh"
    cp "$SCRIPT_DIR/shell/jumper.fish" "$SHELL_DIR/jumper.fish"
else
    # Download shell integration scripts
    curl -sL "https://raw.githubusercontent.com/$REPO/main/shell/jumper.sh" -o "$SHELL_DIR/jumper.sh"
    curl -sL "https://raw.githubusercontent.com/$REPO/main/shell/jumper.fish" -o "$SHELL_DIR/jumper.fish"
fi

echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}  Shell Integration (Required)${NC}"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Shell integration scripts installed to: $SHELL_DIR"
echo ""
echo "Add ONE of these lines to your shell config:"
echo ""
echo -e "${GREEN}Bash (~/.bashrc):${NC}"
echo "  source $SHELL_DIR/jumper.sh"
echo ""
echo -e "${GREEN}Zsh (~/.zshrc):${NC}"
echo "  source $SHELL_DIR/jumper.sh"
echo ""
echo -e "${GREEN}Fish (~/.config/fish/config.fish):${NC}"
echo "  source $SHELL_DIR/jumper.fish"
echo ""
echo "Then reload your shell or run: source ~/.bashrc (or ~/.zshrc)"
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "After setup, run 'jumper' - it will automatically cd on exit!"
