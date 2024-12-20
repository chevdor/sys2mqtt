#!/usr/bin/env bash

set -e

org=chevdor
repo=sys2mqtt

TMP_DIR=$(mktemp -d)
pushd $TMP_DIR > /dev/null

# Fetch the latest build
LATEST_ZIP=$(curl -s https://api.github.com/repos/$org/$repo/releases/latest | jq -r .assets[0].browser_download_url)
curl -s -L -o latest.zip $LATEST_ZIP
unzip latest.zip

export BIN_TARGET=${BIN_TARGET:-/usr/local/bin}

cp -f sys2mqtt $BIN_TARGET
chmod +x $BIN_TARGET/sys2mqtt
codesign --display --entitlements - $BIN_TARGET/sys2mqtt
echo "Installed sys2mqtt to $BIN_TARGET"

# Install the LaunchAgent
echo "Installing LaunchAgent com.chevdor.sys2mqtt.plist"
curl -s -L -o com.chevdor.sys2mqtt.plist.template https://raw.githubusercontent.com/$org/$repo/master/com.chevdor.sys2mqtt.plist.template
envsubst < com.chevdor.sys2mqtt.plist.template > ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist
chmod 644 ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist
plutil -lint ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist

# In case this is a reinstall, we reload the service to ensure we run the latest version
echo "Unloading the agent in case it was already running"
launchctl unload -w ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist 2>/dev/null || true
killall sys2mqtt 2>/dev/null || true

echo "Loading LaunchAgent anew"
launchctl load -w ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist
echo "Installed LaunchAgent com.chevdor.sys2mqtt.plist"

echo "sys2mqtt is now installed and running"
echo "You should check the logs with:"
echo "  tail -f ~/Library/Logs/sys2mqtt/sys2mqtt*.log"

popd > /dev/null
