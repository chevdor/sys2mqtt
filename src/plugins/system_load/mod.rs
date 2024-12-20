use std::path::PathBuf;

use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
pub(crate) struct SystemLoadPlugin {
	config: SystemLoadConfig,
	config_path: PathBuf,
	root_topic: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SystemLoadConfig {
	enabled: bool,
	delay: u64,
}

impl Default for SystemLoadConfig {
	fn default() -> Self {
		Self { enabled: true, delay: 5 }
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct WrappedConfig {
	system_load: Option<SystemLoadConfig>,
}

impl SystemLoadConfig {
	/// Load the configuration from the specified path and the environment variables.
	/// The environment variables take precedence over the configuration file.
	/// If no valid configuration is found, the plugin is disabled.
	pub fn load(config_path: &PathBuf) -> Self {
		let config_str = std::fs::read_to_string(config_path).unwrap();
		let config: WrappedConfig = serde_yaml::from_str(&config_str).unwrap();
		let config = config.system_load;

		let enabled_env = std::env::var("SYS2MQTT_SYSTEM_LOAD_ENABLED");
		let delay_env = std::env::var("SYS2MQTT_SYSTEM_LOAD_DELAY");
		let enabled_config = if let Some(ref config) = config { config.enabled } else { false };
		let delay_config = if let Some(ref config) = config { config.delay } else { 5 };

		let enabled =
			if let Ok(enabled_env) = enabled_env { enabled_env.parse().unwrap_or(false) } else { enabled_config };
		let delay = if let Ok(delay_env) = delay_env { delay_env.parse().unwrap_or(5) } else { delay_config };

		Self { enabled, delay }
	}
}

impl SystemLoadPlugin {
	pub fn new(config_path: PathBuf, root_topic: String) -> Self {
		let config = SystemLoadConfig::load(&config_path);
		SystemLoadPlugin { config, config_path, root_topic }
	}
}

#[async_trait]
impl Plugin for SystemLoadPlugin {
	fn is_enabled(&self) -> bool {
		self.config.enabled
	}

	fn name(&self) -> &str {
		module_path!().split("::").last().unwrap()
	}

	async fn start(&self, client: &AsyncClient) {
		if !self.is_enabled() {
			log::warn!("Plugin {} is disabled.", self.name());
			return;
		} else {
			log::info!("Plugin {} is enabled.", self.name());
		}

		loop {
			// Get the current load average
			let load_avg = System::load_average();
			let topic_1m = format!("{}/{}/1m", self.root_topic, self.name());
			let topic_5m = format!("{}/{}/5m", self.root_topic, self.name());
			let topic_15m = format!("{}/{}/15m", self.root_topic, self.name());

			// log::debug!("topics: {:?}/{:?}/{:?}", topic_1m, topic_5m, topic_15m);
			// log::debug!("{:?}", load_avg);

			client.publish(&topic_1m, QoS::AtLeastOnce, false, load_avg.one.to_string()).await.unwrap();
			client.publish(&topic_5m, QoS::AtLeastOnce, false, load_avg.five.to_string()).await.unwrap();
			client.publish(&topic_15m, QoS::AtLeastOnce, false, load_avg.fifteen.to_string()).await.unwrap();

			// Sleep for 5 seconds before checking the load again
			sleep(Duration::from_secs(self.config.delay)).await;
		}
	}
}
