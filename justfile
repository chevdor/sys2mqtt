VERSION := `toml get Cargo.toml package.version | jq -r`

# Install all for you
install_sys2mqtt_cargo:
    cargo install sys2mqtt

build:
    cargo build --release

install_sys2mqtt_local: build sign
    cp ./target/release/sys2mqtt $HOME/.cargo/bin/sys2mqtt
    codesign --display --entitlements - $HOME/.cargo/bin/sys2mqtt

install_service: gen_plist
    cp com.chevdor.sys2mqtt.plist ~/Library/LaunchAgents/
    ls -al ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist

install: install_sys2mqtt_local install_service

# Unload service
unload:
    launchctl unload -w com.chevdor.sys2mqtt.plist

# Load service
load:
    launchctl load -w com.chevdor.sys2mqtt.plist

reload: unload load

restart:
    launchctl kickstart -k gui/`id -u`/com.chevdor.sys2mqtt

logs:
    tail -f /private/tmp/sys2mqtt*.log

uninstall:
    cargo uninstall sys2mqtt
    rm ~/Library/LaunchAgents/com.chevdor.sys2mqtt.plist

sign:
    codesign --sign "FEAEB73A06F1C176951BCD9B2CB54B93C08976C2" \
         --entitlements entitlements.plist \
         --timestamp \
         --force \
         ./target/release/sys2mqtt

check:
    codesign --display --entitlements - ./target/release/sys2mqtt

lint_plist:
    plutil -lint com.chevdor.sys2mqtt.plist

reinstall: install_sys2mqtt_local lint_plist install_service reload

gen_plist:
    #!/usr/bin/env bash
    BIN_TARGET=$HOME/.cargo/bin
    envsubst < com.chevdor.sys2mqtt.plist.template > com.chevdor.sys2mqtt.plist

# Set a tag for the release
tag:
    @git tag -a v{{VERSION}} -m "Release v{{VERSION}}" -f
    @git tag

# Push the tag
tag_push:
    @git push github v{{VERSION}}
