# Claude Token Monitor 📊

A sleek, lightweight desktop application for monitoring your Claude token usage in real-time. Built with Tauri, Svelte, and TypeScript.

![Claude Token Monitor](https://img.shields.io/badge/Platform-macOS-blue)
![License](https://img.shields.io/badge/License-MIT-green)
![Tauri](https://img.shields.io/badge/Tauri-v2.0-orange)
![Svelte](https://img.shields.io/badge/Svelte-5.0-red)

<img width="401" height="581" alt="スクリーンショット 2025-08-11 21 43 33" src="https://github.com/user-attachments/assets/d7164aba-cb48-46bf-b171-89f3a97a0b42" />


## ✨ Features

- **Real-time Monitoring**: Track your Claude token usage as you work
- **System Tray Integration**: Runs quietly in the background with easy access
- **Usage Analytics**: View daily, weekly, and monthly usage statistics
- **Cost Tracking**: Monitor your API costs with detailed breakdowns
- **Smart Notifications**: Get warned when approaching usage limits
- **Privacy First**: All data stays local on your device
- **macOS Optimized**: Designed specifically for macOS with native system tray integration

## 🖥️ Screenshots

### System Tray Interface
The app runs as a background utility with a clean tray icon interface:

- Click the tray icon to view current usage
- Right-click for quick actions like refresh
- Auto-hide when clicking outside

### Usage Dashboard
- **Current Session**: Active token count and cost
- **Daily Usage**: Today's total consumption
- **Period Analytics**: Weekly and monthly trends
- **Model Information**: Which Claude model you're using

## 📋 Prerequisites

Before installing Claude Token Monitor, make sure you have:

1. **ccusage CLI tool** installed globally
   ```bash
   npm install -g ccusage
   ```

2. **Claude Code extension** for VS Code/Cursor
   - Install from VS Code marketplace
   - Configure your Claude API key

## 🚀 Installation

### 🚀 Easy Install Methods

#### **Method 1: One-Click Installer (Recommended)**
```bash
curl -fsSL https://raw.githubusercontent.com/kimdj2/claude-token-monitor/main/scripts/one-click-install.sh | bash
```
This script automatically:
- Downloads the latest version
- Installs the app to Applications
- Removes security restrictions
- Installs ccusage dependency
- Launches the app

#### **Method 2: Manual Download**

1. Go to the [Releases page](../../releases)
2. Download the `.dmg` file for macOS
3. Open the DMG file and drag the app to Applications folder
4. **Important**: When you first launch the app, you may see a security warning from macOS Gatekeeper
5. Follow the steps below to allow the app to run

##### 🔐 macOS Security Warning Fix

If you see the message *"Apple cannot check it for malicious software"*, follow these steps:

**Option A: Automated Fix**
```bash
curl -fsSL https://raw.githubusercontent.com/kimdj2/claude-token-monitor/main/scripts/install-claude-token-monitor.sh | bash
```

**Option B: System Preferences (Manual)**
1. Open **System Preferences** → **Privacy & Security**
2. Look for the message about "Claude Token Monitor" being blocked
3. Click **"Open Anyway"**
4. Confirm when prompted

**Option C: Right-click Method**
1. Right-click on the app in Applications folder
2. Select **"Open"** from the context menu
3. Click **"Open"** when the security dialog appears

**Option D: Terminal Method**
```bash
sudo xattr -rd com.apple.quarantine "/Applications/Claude Token Monitor.app"
```

After completing any of these methods, the app will launch normally in the future.

6. Launch Claude Token Monitor from Applications

### System Requirements

- **macOS 10.15** or later
- **Node.js 18+** (for ccusage dependency)
- **Claude Code extension** for VS Code/Cursor

## 🔧 Usage

### First Launch
1. Make sure `ccusage` is installed and accessible in your PATH
2. Launch Claude Token Monitor from your Applications folder
3. The app will appear in your system tray
4. Click the tray icon to view your usage statistics

### Daily Workflow
- The app runs silently in the background
- Click the tray icon whenever you want to check your usage
- Data refreshes automatically every 5 seconds
- Use period selectors (Day/Week/Month) to view different timeframes

### Features Overview

#### Real-time Session Tracking
- **Active Session**: Shows current session token count and cost
- **Burn Rate**: Tokens consumed per minute
- **Session Cost**: Real-time cost calculation

#### Usage Analytics
- **Daily Usage**: Current day's total consumption
- **Weekly Trends**: Last 7 days of usage patterns
- **Monthly Summary**: Current month's statistics

#### Smart Notifications
- **Usage Warnings**: Alerts when approaching daily limits
- **Cost Alerts**: Notifications for spending thresholds
- **Adaptive Thresholds**: Learns from your usage patterns

## ⚙️ Configuration

The app works out of the box with `ccusage`. Additional configuration options:

### System Permissions (macOS)
You may need to grant permissions:
1. System Settings → Privacy & Security → Accessibility
2. Find 'Claude Token Monitor' and enable it

### Tray Icon Behavior
- **Left Click**: Toggle usage window
- **Right Click**: Access menu (Refresh, Quit)
- **Auto-hide**: Window closes when clicking outside

## 🛠️ Development

### Prerequisites
- Node.js 18+
- Rust 1.70+
- pnpm (recommended)

### Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/claude-token-monitor.git
cd claude-token-monitor

# Install dependencies
pnpm install

# Install Rust dependencies
cd src-tauri
cargo fetch

# Run in development mode
cd ..
pnpm tauri dev
```

### Building
```bash
# Build for production
pnpm tauri build

# The built app will be in src-tauri/target/release/bundle/
```

### Tech Stack
- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **Styling**: Custom CSS with smooth animations
- **Data Source**: ccusage CLI integration
- **Platform Support**: macOS native

## 🐛 Troubleshooting

### Common Issues

#### "ccusage not found"
- Install ccusage globally: `npm install -g ccusage`
- Make sure ccusage is in your PATH
- Restart the application after installing ccusage

#### Window not appearing on macOS
- Check System Settings → Privacy & Security → Accessibility
- Grant permissions to Claude Token Monitor
- Try the emergency show window option from tray menu

#### Data not updating
- Verify ccusage is working: `ccusage blocks --json`
- Check if Claude API credentials are configured
- Use the Refresh option from the tray menu

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines
- Follow the existing code style (2-space indentation)
- Add tests for new features
- Update documentation as needed
- Ensure macOS compatibility and performance

## 🙏 Acknowledgments

- [Tauri](https://tauri.app) for the amazing desktop app framework
- [Svelte](https://svelte.dev) for the reactive UI framework
- [ccusage](https://github.com/ccusage) for Claude usage tracking
- The Claude API team for providing the usage data

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](../../issues) page for existing solutions
2. Create a new issue with detailed information
3. Include your OS version and error messages
