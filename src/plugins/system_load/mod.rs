use std::path::PathBuf;

use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use sysinfo::System;
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
pub(crate) struct SystemLoadPlugin {
	enabled: bool,
	delay: u64,
	config_path: PathBuf,
	root_topic: String,
}

impl SystemLoadPlugin {
	pub fn new(config_path: PathBuf, root_topic: String) -> Self {
		let enabled = std::env::var("SYS2MQTT_SYSTEM_LOAD_ENABLED")
			.unwrap_or_else(|_| "true".to_string())
			.parse()
			.unwrap_or(true);
		let delay =
			std::env::var("SYS2MQTT_SYSTEM_LOAD_DELAY").unwrap_or_else(|_| "5".to_string()).parse().unwrap_or(5);

		// todo: load config from file
		SystemLoadPlugin { enabled, delay, config_path, root_topic }
	}
}

#[async_trait]
impl Plugin for SystemLoadPlugin {
	fn name(&self) -> &str {
		"system_load"
	}

	fn is_enabled(&self) -> bool {
		self.enabled
	}

	async fn start(&self, client: &AsyncClient) {
		if cfg!(debug_assertions) {
			log::debug!("Starting system load plugin...");
		}

		if !self.is_enabled() {
			log::warn!("Plugin {} is disabled.", self.name());
			return;
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
			sleep(Duration::from_secs(self.delay)).await;
		}
	}
}
