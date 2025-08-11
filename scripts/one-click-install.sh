#!/bin/bash

# Claude Token Monitor - One-Click Installer
# Downloads, installs, and configures Claude Token Monitor automatically

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
REPO="kimdj2/claude-token-monitor"
APP_NAME="Claude Token Monitor"
APP_PATH="/Applications/${APP_NAME}.app"
TEMP_DIR="/tmp/claude-token-monitor-install"

# GitHub API to get latest release
GITHUB_API="https://api.github.com/repos/${REPO}/releases/latest"

echo -e "${PURPLE}🚀 Claude Token Monitor - One-Click Installer${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get latest release info
get_latest_release() {
    echo -e "${YELLOW}🔍 Fetching latest release information...${NC}"
    
    if command_exists curl; then
        RELEASE_INFO=$(curl -s "$GITHUB_API")
    elif command_exists wget; then
        RELEASE_INFO=$(wget -qO- "$GITHUB_API")
    else
        echo -e "${RED}❌ Neither curl nor wget found. Please install one of them.${NC}"
        exit 1
    fi
    
    # Extract version and download URL
    VERSION=$(echo "$RELEASE_INFO" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep '"browser_download_url".*\.dmg"' | head -1 | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/')
    
    if [ -z "$VERSION" ] || [ -z "$DOWNLOAD_URL" ]; then
        echo -e "${RED}❌ Failed to get release information${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Latest version: $VERSION${NC}"
}

# Function to download DMG
download_dmg() {
    echo -e "${YELLOW}⬇️ Downloading Claude Token Monitor $VERSION...${NC}"
    
    # Create temp directory
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    DMG_FILE="claude-token-monitor.dmg"
    
    if command_exists curl; then
        curl -L -o "$DMG_FILE" "$DOWNLOAD_URL"
    elif command_exists wget; then
        wget -O "$DMG_FILE" "$DOWNLOAD_URL"
    fi
    
    if [ ! -f "$DMG_FILE" ]; then
        echo -e "${RED}❌ Download failed${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Download completed${NC}"
}

# Function to install app
install_app() {
    echo -e "${YELLOW}📦 Installing application...${NC}"
    
    # Mount DMG
    MOUNT_POINT="/Volumes/Claude Token Monitor"
    hdiutil attach "$DMG_FILE" -quiet
    
    # Check if mount was successful
    if [ ! -d "$MOUNT_POINT" ]; then
        echo -e "${RED}❌ Failed to mount DMG${NC}"
        exit 1
    fi
    
    # Remove existing app if it exists
    if [ -d "$APP_PATH" ]; then
        echo -e "${YELLOW}🗑️ Removing existing installation...${NC}"
        rm -rf "$APP_PATH"
    fi
    
    # Copy app to Applications
    cp -R "$MOUNT_POINT/${APP_NAME}.app" "/Applications/"
    
    # Unmount DMG
    hdiutil detach "$MOUNT_POINT" -quiet
    
    if [ -d "$APP_PATH" ]; then
        echo -e "${GREEN}✅ Application installed successfully${NC}"
    else
        echo -e "${RED}❌ Installation failed${NC}"
        exit 1
    fi
}

# Function to remove quarantine
remove_quarantine() {
    echo -e "${YELLOW}🔓 Removing security restrictions...${NC}"
    
    # Remove quarantine attribute
    xattr -dr com.apple.quarantine "$APP_PATH" 2>/dev/null || true
    
    # Check if it was successful
    if xattr "$APP_PATH" 2>/dev/null | grep -q "com.apple.quarantine"; then
        echo -e "${YELLOW}⚠️ Quarantine attribute still exists (may require admin password)${NC}"
        echo -e "${YELLOW}🔑 Attempting with sudo...${NC}"
        sudo xattr -dr com.apple.quarantine "$APP_PATH"
    fi
    
    echo -e "${GREEN}✅ Security restrictions removed${NC}"
}

# Function to install dependencies
install_dependencies() {
    echo -e "${YELLOW}📦 Checking dependencies...${NC}"
    
    # Check Node.js
    if ! command_exists node; then
        echo -e "${RED}❌ Node.js is not installed${NC}"
        echo -e "${YELLOW}📥 Installing Node.js via Homebrew...${NC}"
        
        # Install Homebrew if not exists
        if ! command_exists brew; then
            echo -e "${YELLOW}🍺 Installing Homebrew...${NC}"
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        
        brew install node
    else
        echo -e "${GREEN}✅ Node.js is installed${NC}"
    fi
    
    # Check ccusage
    if ! command_exists ccusage; then
        echo -e "${YELLOW}📦 Installing ccusage...${NC}"
        npm install -g ccusage
    else
        echo -e "${GREEN}✅ ccusage is installed${NC}"
    fi
}

# Function to verify installation
verify_installation() {
    echo -e "${YELLOW}🔍 Verifying installation...${NC}"
    
    # Check if app exists
    if [ ! -d "$APP_PATH" ]; then
        echo -e "${RED}❌ App not found in Applications${NC}"
        return 1
    fi
    
    # Check if quarantine is removed
    if xattr "$APP_PATH" 2>/dev/null | grep -q "com.apple.quarantine"; then
        echo -e "${YELLOW}⚠️ Quarantine attribute still exists${NC}"
        return 1
    fi
    
    # Check dependencies
    if ! command_exists ccusage; then
        echo -e "${YELLOW}⚠️ ccusage is not installed${NC}"
        return 1
    fi
    
    echo -e "${GREEN}✅ Installation verification passed${NC}"
    return 0
}

# Function to launch app
launch_app() {
    echo -e "${YELLOW}🚀 Launching Claude Token Monitor...${NC}"
    
    if open "$APP_PATH"; then
        echo -e "${GREEN}✅ App launched successfully!${NC}"
    else
        echo -e "${RED}❌ Failed to launch app${NC}"
        return 1
    fi
}

# Function to cleanup
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up temporary files...${NC}"
    rm -rf "$TEMP_DIR"
}

# Main installation process
main() {
    echo -e "${BLUE}Starting automatic installation...${NC}"
    echo ""
    
    # Check if running on macOS
    if [[ "$OSTYPE" != "darwin"* ]]; then
        echo -e "${RED}❌ This installer is for macOS only${NC}"
        exit 1
    fi
    
    # Get latest release
    get_latest_release
    
    # Download DMG
    download_dmg
    
    # Install app
    install_app
    
    # Remove quarantine
    remove_quarantine
    
    # Install dependencies
    install_dependencies
    
    # Verify installation
    if verify_installation; then
        echo ""
        echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
        echo ""
        echo -e "${YELLOW}📋 What's next:${NC}"
        echo "• Look for the Claude Token Monitor icon in your menu bar"
        echo "• Click the tray icon to view your token usage"
        echo "• Use Claude from any source (web, API, or extensions) to see usage data"
        echo ""
        
        # Ask if user wants to launch the app
        echo -e "${YELLOW}🚀 Would you like to launch Claude Token Monitor now? (y/N)${NC}"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            launch_app
        fi
    else
        echo -e "${RED}❌ Installation verification failed${NC}"
        echo -e "${YELLOW}Please check the issues above and try again${NC}"
    fi
    
    # Cleanup
    cleanup
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Run main function
main

echo ""
echo -e "${BLUE}Thank you for using Claude Token Monitor! 🚀${NC}"
