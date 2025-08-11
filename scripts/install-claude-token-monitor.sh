#!/bin/bash

# Claude Token Monitor - Easy Install Script
# This script automatically handles macOS Gatekeeper issues

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

APP_NAME="Claude Token Monitor"
APP_PATH="/Applications/${APP_NAME}.app"
DMG_NAME="Claude Token Monitor"

echo -e "${BLUE}🚀 Claude Token Monitor - Easy Install${NC}"
echo ""

# Function to check if app is installed
check_app_installed() {
    if [ -d "$APP_PATH" ]; then
        return 0
    else
        return 1
    fi
}

# Function to remove quarantine attribute
remove_quarantine() {
    echo -e "${YELLOW}🔓 Removing macOS quarantine attribute...${NC}"
    if sudo xattr -rd com.apple.quarantine "$APP_PATH" 2>/dev/null; then
        echo -e "${GREEN}✅ Quarantine attribute removed successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ Failed to remove quarantine attribute${NC}"
        return 1
    fi
}

# Function to check if ccusage is installed
check_ccusage() {
    if command -v ccusage &> /dev/null; then
        echo -e "${GREEN}✅ ccusage is installed${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️ ccusage is not installed${NC}"
        return 1
    fi
}

# Function to install ccusage
install_ccusage() {
    echo -e "${YELLOW}📦 Installing ccusage...${NC}"
    
    # Check if npm is available
    if ! command -v npm &> /dev/null; then
        echo -e "${RED}❌ npm is not installed${NC}"
        echo "Please install Node.js from: https://nodejs.org"
        return 1
    fi
    
    # Install ccusage globally
    if npm install -g ccusage; then
        echo -e "${GREEN}✅ ccusage installed successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ Failed to install ccusage${NC}"
        echo "You may need to run with sudo or use a Node version manager"
        return 1
    fi
}

# Main installation process
echo -e "${YELLOW}1. Checking current installation...${NC}"
if check_app_installed; then
    echo -e "${GREEN}✅ ${APP_NAME} is already installed${NC}"
    
    # Remove quarantine if it exists
    if xattr "$APP_PATH" 2>/dev/null | grep -q "com.apple.quarantine"; then
        echo -e "${YELLOW}🔍 Found quarantine attribute${NC}"
        remove_quarantine
    else
        echo -e "${GREEN}✅ No quarantine attribute found${NC}"
    fi
else
    echo -e "${RED}❌ ${APP_NAME} is not installed${NC}"
    echo ""
    echo -e "${YELLOW}📋 To install ${APP_NAME}:${NC}"
    echo "1. Download the DMG file from GitHub releases"
    echo "2. Open the DMG and drag the app to Applications"
    echo "3. Run this script again to remove security restrictions"
    echo ""
    echo -e "${BLUE}🔗 Download from: https://github.com/kimdj2/claude-token-monitor/releases${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}2. Checking ccusage dependency...${NC}"
if ! check_ccusage; then
    echo -e "${YELLOW}📥 Do you want to install ccusage now? (y/N)${NC}"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        install_ccusage
    else
        echo -e "${YELLOW}⚠️ You'll need to install ccusage manually later:${NC}"
        echo "npm install -g ccusage"
    fi
else
    # Check ccusage version
    CCUSAGE_VERSION=$(ccusage --version 2>/dev/null || echo "unknown")
    echo -e "${GREEN}✅ ccusage version: $CCUSAGE_VERSION${NC}"
fi

echo ""
echo -e "${YELLOW}3. Testing app launch...${NC}"
if open "$APP_PATH"; then
    echo -e "${GREEN}✅ ${APP_NAME} launched successfully!${NC}"
    echo ""
    echo -e "${GREEN}🎉 Installation complete!${NC}"
    echo ""
    echo -e "${YELLOW}📋 Quick tips:${NC}"
    echo "• Look for the app icon in your menu bar"
    echo "• Click the tray icon to view token usage"
    echo "• Right-click for refresh and quit options"
    echo "• Use Claude from any source (web, API, or extensions) to see usage data"
else
    echo -e "${RED}❌ Failed to launch ${APP_NAME}${NC}"
    echo ""
    echo -e "${YELLOW}🔧 Manual troubleshooting:${NC}"
    echo "1. Try opening the app from Applications folder"
    echo "2. If you see a security warning, go to:"
    echo "   System Settings → Privacy & Security → Open Anyway"
    echo "3. Or run: sudo xattr -rd com.apple.quarantine \"$APP_PATH\""
fi

echo ""
