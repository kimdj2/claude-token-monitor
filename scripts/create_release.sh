#!/bin/bash

# Claude Token Monitor release creation script

set -e

VERSION=$(node -pe "require('./package.json').version")
TAG="v$VERSION"

echo "🚀 Creating release for Claude Token Monitor $VERSION"

# Check current status
echo "📋 Current status:"
echo "   Version: $VERSION"
echo "   Tag: $TAG"
echo "   Branch: $(git branch --show-current)"

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
  echo "⚠️  Warning: You have uncommitted changes"
  read -p "Continue anyway? (y/N): " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Aborted"
    exit 1
  fi
fi

# Build application
echo "🔨 Building application..."
pnpm tauri build

BUILD_DIR="src-tauri/target/release/bundle"

# Verify build files
if [ ! -f "$BUILD_DIR/dmg/Claude Token Monitor_${VERSION}_universal.dmg" ]; then
  echo "❌ DMG file not found. Build may have failed."
  echo "   Expected: $BUILD_DIR/dmg/Claude Token Monitor_${VERSION}_universal.dmg"
  exit 1
fi

echo "✅ Build completed successfully"

# Create git tag
echo "🏷️  Creating git tag..."
if git tag -l "$TAG" | grep -q "$TAG"; then
  echo "⚠️  Tag $TAG already exists. Deleting it..."
  git tag -d "$TAG" 2>/dev/null || true
  git push origin --delete "$TAG" 2>/dev/null || true
fi

git tag -a "$TAG" -m "Release $VERSION"
git push origin "$TAG"

echo "✅ Git tag created and pushed"

# Create GitHub release with CLI
echo "📦 Creating GitHub release..."

if command -v gh &> /dev/null; then
  gh release create "$TAG" \
    "$BUILD_DIR/dmg/Claude Token Monitor_${VERSION}_universal.dmg" \
    --title "Claude Token Monitor $VERSION" \
    --notes "🎉 **Claude Token Monitor $VERSION**

### ✨ Features
- Real-time Claude token usage monitoring
- System tray integration for macOS
- Daily, weekly, and monthly usage analytics
- Cost tracking with detailed breakdowns
- Smart notifications for usage limits

### 📥 Installation
1. Download the \`.dmg\` file below
2. Open the DMG and drag the app to Applications
3. Install \`ccusage\`: \`npm install -g ccusage\`
4. Launch from Applications folder

### 📋 Requirements
- macOS 10.15 or later
- Node.js 18+ (for ccusage)
- Claude Code extension for VS Code/Cursor

### 🐛 Issues?
If you encounter any problems, please [report them here](../../issues)."

  echo "✅ GitHub release created successfully!"
  echo "🌐 View at: https://github.com/$(git config --get remote.origin.url | sed 's/.*github\.com[:/]\([^.]*\)\.git/\1/')/releases/tag/$TAG"
else
  echo "⚠️  GitHub CLI not found. Please create the release manually:"
  echo "   1. Go to: https://github.com/$(git config --get remote.origin.url | sed 's/.*github\.com[:/]\([^.]*\)\.git/\1/')/releases/new"
  echo "   2. Tag: $TAG"
  echo "   3. Upload: $BUILD_DIR/dmg/Claude Token Monitor_${VERSION}_universal.dmg"
fi

echo ""
echo "🎉 Release process completed!"
echo "📁 Build artifacts in: $BUILD_DIR"
