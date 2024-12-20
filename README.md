# sys2mqtt

Sends system information to an MQTT broker.

Primarily developed and tested on MacOS but changes are it should work or be closed to working on Linnux and maybe
Windows. PR are welcome for Linux/Windows whether regarding the code or simply the documentation.

Most of the configuration can be done via the configuration file called `config.yaml`.
If running in debug mode, `config.yaml` is expected to be local. Otherwise, it is expected to be under
`$HOME/.config/sys2mqtt/config.yaml`.

Some of the information cannot be configured via the configuration file: credentials. If needed those, must come  from
the environment variables `MQTT_USERNAME` and `MQTT_PASSWORD`.

You can find more doc here:
- [Installation Guide](doc/install.md)
- [Usage Guide](doc/usage.md)
- [OpenHAB Integration](doc/openhab.md)
- [Metrics](doc/metrics.md)
