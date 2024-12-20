use mqtt::MqttConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod mqtt;

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Config {
	pub mqtt: MqttConfig,
}

pub(crate) fn config_path() -> PathBuf {
	// Load configuration from a YAML file
	let config_path = if cfg!(debug_assertions) {
		PathBuf::from("config.yaml")
	} else {
		let path = dirs::home_dir()
			.map(|path| path.join(".config/sys2mqtt/config.yaml"))
			.expect("Failed to determine home directory");

		if !path.exists() {
			let config = Config::default();
			let default_config = serde_yaml::to_string(&config).unwrap();
			if let Some(parent) = path.parent() {
				std::fs::create_dir_all(parent).expect(format!("Failed to create config directory as {}", parent.display()).as_str());
			}

			std::fs::write(&path, default_config).expect("Failed to write default config");
		}
		path
	};
	config_path
}
