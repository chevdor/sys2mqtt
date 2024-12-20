use std::env;

use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MqttConfig {
	host: String,
	port: u16,
	// client_id: String,
	username: Option<String>,
}

impl Default for MqttConfig {
	fn default() -> Self {
		Self { host: "localhost".to_string(), port: 1883, username: None }
	}
}

pub(crate) fn get_mqtt_password() -> Option<String> {
	env::var("MQTT_PASSWORD").ok()
}

pub(crate) fn create_mqtt_client(config: &MqttConfig) -> (AsyncClient, EventLoop) {
	let mut mqttoptions = MqttOptions::new("rust-cli", &config.host, config.port);

	if let Some(username) = &config.username {
		if let Some(password) = get_mqtt_password() {
			mqttoptions.set_credentials(username, &password);
		} else {
			mqttoptions.set_credentials(username, "");
		}
	}

	let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
	(client, eventloop)
}
