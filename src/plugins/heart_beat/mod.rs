use std::path::PathBuf;

use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
pub(crate) struct HeartBeatPlugin {
	config: HeartBeatConfig,
	config_path: PathBuf,
	root_topic: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct HeartBeatConfig {
	enabled: bool,
	delay: u64,
}

impl Default for HeartBeatConfig {
	fn default() -> Self {
		Self { enabled: true, delay: 5 }
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct WrappedConfig {
	heart_beat: Option<HeartBeatConfig>,
}

impl HeartBeatConfig {
	/// Load the configuration from the specified path and the environment variables.
	/// The environment variables take precedence over the configuration file.
	pub fn load(config_path: &PathBuf) -> Self {
		let config_str = std::fs::read_to_string(config_path).unwrap();
		let config: WrappedConfig = serde_yaml::from_str(&config_str).unwrap();
		let config = config.heart_beat;

		let enabled_env = std::env::var("SYS2MQTT_HEART_BEAT_ENABLED");
		let delay_env = std::env::var("SYS2MQTT_HEART_BEAT_DELAY");
		let enabled_config = if let Some(ref config) = config { config.enabled } else { true };
		let delay_config = if let Some(ref config) = config { config.delay } else { 5 };

		let enabled =
			if let Ok(enabled_env) = enabled_env { enabled_env.parse().unwrap_or(false) } else { enabled_config };
		let delay = if let Ok(delay_env) = delay_env { delay_env.parse().unwrap_or(5) } else { delay_config };

		Self { enabled, delay }
	}
}

impl HeartBeatPlugin {
	pub fn new(config_path: PathBuf, root_topic: String) -> Self {
		let config = HeartBeatConfig::load(&config_path);
		HeartBeatPlugin { config, config_path, root_topic }
	}
}

#[async_trait]
impl Plugin for HeartBeatPlugin {
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

		let topic = format!("{}/{}", self.root_topic, self.name());

		if cfg!(debug_assertions) {
			log::debug!("Heartbeat topic: {}", topic);
		}

		loop {
			let tmsp = chrono::Utc::now().timestamp().to_string();
			// Get the current load average
			client.publish(&topic, QoS::AtLeastOnce, false, tmsp).await.unwrap();
			// Sleep for 5 seconds before checking the load again
			sleep(Duration::from_secs(self.config.delay)).await;
		}
	}
}
