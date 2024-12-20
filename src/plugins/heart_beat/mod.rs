use std::path::PathBuf;

use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
pub(crate) struct HeartBeatPlugin {
	enabled: bool,
	delay: u64,
	config_path: PathBuf,
	root_topic: String,
}

impl HeartBeatPlugin {
	pub fn new(config_path: PathBuf, root_topic: String) -> Self {
		let enabled =
			std::env::var("SYS2MQTT_HEART_BEAT_ENABLED").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
		let delay =
			std::env::var("SYS2MQTT_HEART_BEAT_DELAY").unwrap_or_else(|_| "30".to_string()).parse().unwrap_or(5);
		// todo: load config from file
		HeartBeatPlugin { enabled, delay, config_path, root_topic }
	}
}

#[async_trait]
impl Plugin for HeartBeatPlugin {
	fn name(&self) -> &str {
		"heart_beat"
	}

	fn is_enabled(&self) -> bool {
		self.enabled
	}

	async fn start(&self, client: &AsyncClient) {
		if cfg!(debug_assertions) {
			log::debug!("Starting heart beat plugin...");
		}

		if !self.is_enabled() {
			log::warn!("Plugin {} is disabled.", self.name());
			return;
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
			sleep(Duration::from_secs(self.delay)).await;
		}
	}
}
