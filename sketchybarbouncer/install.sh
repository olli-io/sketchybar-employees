#!/bin/bash

set -e

# Check if running on macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo "Error: This script is only for macOS"
    exit 1
fi

# Check if Swift compiler is available
if ! command -v swiftc &> /dev/null; then
    echo "Error: Swift compiler not found. Please install Xcode Command Line Tools:"
    echo "  xcode-select --install"
    exit 1
fi

# Determine installation directory
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="sketchybarbouncer"
LAUNCH_AGENTS_DIR="${HOME}/Library/LaunchAgents"
PLIST_NAME="com.sketchybar.bouncer.plist"

# Create installation directory if it doesn't exist
mkdir -p "${INSTALL_DIR}"

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if main.swift exists
if [[ ! -f "${SCRIPT_DIR}/main.swift" ]]; then
    echo "Error: main.swift not found in ${SCRIPT_DIR}"
    exit 1
fi

# Compile the Swift source
if ! swiftc -O "${SCRIPT_DIR}/main.swift" -o "${INSTALL_DIR}/${BINARY_NAME}"; then
    echo "Error: Compilation failed"
    exit 1
fi

# Make the binary executable
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

# Create LaunchAgent plist
mkdir -p "${LAUNCH_AGENTS_DIR}"

cat > "${LAUNCH_AGENTS_DIR}/${PLIST_NAME}" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.sketchybar.bouncer</string>
    <key>ProgramArguments</key>
    <array>
        <string>${INSTALL_DIR}/${BINARY_NAME}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/sketchybarbouncer.out.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/sketchybarbouncer.err.log</string>
    <key>ProcessType</key>
    <string>Interactive</string>
</dict>
</plist>
EOF

# Stop existing service if running
if launchctl list | grep -q "com.sketchybar.bouncer"; then
    launchctl unload "${LAUNCH_AGENTS_DIR}/${PLIST_NAME}" 2>/dev/null || true
fi

# Load the LaunchAgent
if ! launchctl load "${LAUNCH_AGENTS_DIR}/${PLIST_NAME}"; then
    echo "Warning: Failed to load LaunchAgent. You may need to load it manually:"
    echo "  launchctl load ${LAUNCH_AGENTS_DIR}/${PLIST_NAME}"
fi
