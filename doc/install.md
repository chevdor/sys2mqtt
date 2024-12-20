## Installation

Installing the binary using `cargo` or the pre-built binary is trivial.
However, getting the binary to run as an Agent is more challenging.

While eveything required to hack around is availabe in the repo (see `justfile`), I suggest using the `install.sh`
script.

The rest of the documentation assumes you installed using `cargo` and the binary is installed under `$HOME/.cargo/bin/sys2mqtt`.

### `install.sh` script:

First let's run the installer:

```sh
curl -sSL https://raw.githubusercontent.com/chevdor/sys2mqtt/refs/heads/master/install.sh | sh
```

You will be prompted to allow local network access, without that, `sys2mqtt` will not be able to reach your MQTT broker *when running as an Agent* (it would work if you call the binary manually, but you then need to start the binary yourself every time you boot up your machine)

![Local Network Access](resources/screenshots/local_network.png)

### Using cargo
```
cargo install sys2mqtt
```

### Precompiled binary

A pre-compiled binary can be found on github. See https://github.com/chevdor/sys2mqtt/releases/latest

### From source

If that's your option, you likely know how it goes:
```
git clone https://github.com/chevdor/sys2mqtt
cd sys2mqtt
cargo run
```
