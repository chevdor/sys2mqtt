
## Available metrics

All the following metrics will show up under the root topic: `sys2mqtt/<SOME_UID>`.
![MQTT Metrics](resources/screenshots/mqtt_metrics.png)

- `heart_beat`: a timestampt to ensure that `sys2mqtt` is "still there"
- `user_idle`: `active` or `idle`
- `system_load`:
    - `1m`: CPU load over the last 1m
    - `5m`: CPU load over the last 5m
    - `15m`: CPU load over the last 15m
