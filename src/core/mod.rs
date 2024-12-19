use mqtt::MqttConfig;
use serde::Deserialize;

pub mod mqtt;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
	pub mqtt: MqttConfig,
}
