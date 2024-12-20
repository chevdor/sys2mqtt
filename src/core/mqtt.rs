use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MqttConfig {
	pub host: String,
	pub port: u16,
	// client_id: String,
	username: Option<String>,
	password: Option<String>,
}

impl Default for MqttConfig {
	fn default() -> Self {
		Self { host: "localhost".to_string(), port: 1883, username: None, password: None }
	}
}



pub(crate) fn create_mqtt_client(config: &MqttConfig) -> (AsyncClient, EventLoop) {
	let mut mqttoptions = MqttOptions::new("rust-cli", &config.host, config.port);

	if let Some(username) = &config.username {
		if let Some(password) = &config.password {
			mqttoptions.set_credentials(username, password);
		} else {
			mqttoptions.set_credentials(username, "");
		}
	}
	log::debug!("Connecting to MQTT Broker...");
	log::debug!("  - host/port: {}:{}", config.host, config.port);
	log::debug!("  - username/pass: {:?}:{:?}", config.username, if config.password.is_none() {"none"} else {"***"});
	let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
	(client, eventloop)
}
