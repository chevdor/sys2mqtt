## Usage

If you installed using the installer and everything went fine, `sys2mqtt` should be running already.

The first version is rather simple and the command does not support any argument.
If you have no configuration file, the first run will generate a default one for you.
The configuration file is expected to be under `$HOME/.config/sys2mqtt/config.yaml` and should be self explanatory.

Before starting `sys2mqtt`, make sure you have a running MQTT broker. Running `mosquitto` will start one locally and you
may confirm using `mosquitto_sub -v -t "#`.

You may want to monitor the output of the command to get started. It looks like:
![stdout](resources/screenshots/stdout.png)


An important piece of information is the `Root topic` which is used to publish the information. The root topic looks
like: `sys2mqtt/112376B0-BB5C-78DF-BFF8-42C6E4AABA4`

You may now subscribe to the root topic to get the information:
```
mosquitto_sub -v -t "sys2mqtt/112376B0-BB5C-78DF-BFF8-42C6E4AABA4/#"
```

`sys2mqtt` by default should not report data very often so you may need to wait a minute or two to start seeing changes.
You may tweak the configuration to increase/decrese the frequency of the updates.

`sys2mqtt` requires very little cpu and memory as can be seen below:
![CPU Usage](resources/screenshots/cpu.png)
![Memory Usage](resources/screenshots/mem.png)

### Launch as Agent (MacOS)

You probably do not want to start `sys2mqtt` manually every time.
To run a binary as Agent, accessing the local network, on MacOS (Sequoia), the binary needs to be signed.
You can find all you need in the repo to do it yourself.

This is much easier if you use the installer as you will pull from Github a build that is signed "from an unknown
developer" and contains the right entitlements, which means MacOS will ask you for permissions:

![Local Network Access](../resources/screenshots/local_network.png)
