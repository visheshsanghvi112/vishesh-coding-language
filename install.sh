#!/bin/bash
# Vish Language Installer
# Created by Vishesh Sanghvi
# https://github.com/visheshsanghvi112/vishesh-coding-language

set -e

REPO="visheshsanghvi112/vishesh-coding-language"
BINARY_NAME="vish"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}"
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              🕉️  VISH LANGUAGE INSTALLER  🕉️                   ║"
echo "║           Vedic on Steroids - by Vishesh Sanghvi              ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Detect OS and Architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    linux*)
        PLATFORM="linux"
        case "$ARCH" in
            x86_64) ARCH_NAME="gnu-x86_64" ;;
            aarch64) ARCH_NAME="gnu-aarch64" ;;
            armv7l) ARCH_NAME="gnueabihf-armv7" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/vish-linux-$ARCH_NAME.tar.xz"
        ;;
    darwin*)
        PLATFORM="darwin"
        case "$ARCH" in
            x86_64) ARCH_NAME="x86_64" ;;
            arm64) ARCH_NAME="aarch64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/vish-darwin-$ARCH_NAME.tar.gz"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        echo "Please download manually from: https://github.com/$REPO/releases"
        exit 1
        ;;
esac

echo -e "${YELLOW}Detected: $OS ($ARCH)${NC}"
echo -e "${YELLOW}Downloading from: $DOWNLOAD_URL${NC}"

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download
echo -e "\n${GREEN}Downloading Vish Language...${NC}"
if command -v curl &> /dev/null; then
    curl -fsSL "$DOWNLOAD_URL" -o vish.tar.gz
elif command -v wget &> /dev/null; then
    wget -q "$DOWNLOAD_URL" -O vish.tar.gz
else
    echo -e "${RED}Error: curl or wget required${NC}"
    exit 1
fi

# Extract
echo -e "${GREEN}Extracting...${NC}"
if [[ "$PLATFORM" == "darwin" ]]; then
    tar -xzf vish.tar.gz
else
    tar -xJf vish.tar.gz 2>/dev/null || tar -xzf vish.tar.gz
fi

# Install
INSTALL_DIR="/usr/local/bin"
echo -e "${GREEN}Installing to $INSTALL_DIR...${NC}"

if [ -w "$INSTALL_DIR" ]; then
    mv vish "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/vish"
else
    sudo mv vish "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/vish"
fi

# Cleanup
cd ~
rm -rf "$TMP_DIR"

# Verify
echo -e "\n${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              ✅ VISH LANGUAGE INSTALLED!                       ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Run ${YELLOW}vish --help${NC} to get started!"
echo -e "Create a file with ${YELLOW}.vish${NC} extension and run: ${YELLOW}vish your_script.vish${NC}"
echo ""
echo -e "Example:"
echo -e "  ${YELLOW}echo 'वद(\"नमस्ते विश्व!\");' > hello.vish${NC}"
echo -e "  ${YELLOW}vish hello.vish${NC}"
echo ""

# Show version
vish --version 2>/dev/null || echo -e "${YELLOW}Note: You may need to restart your terminal.${NC}"
