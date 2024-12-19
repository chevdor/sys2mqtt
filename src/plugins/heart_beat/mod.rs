use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

pub(crate) struct HeartBeatPlugin {
	enabled: bool,
	config: Option<HashMap<String, String>>,
}

impl HeartBeatPlugin {
	pub fn new() -> Self {
		// todo: load config from file
		HeartBeatPlugin { enabled: true, config: None }
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

	fn config(&self) -> Option<HashMap<String, String>> {
		self.config.clone()
	}

	async fn start(&self, client: &AsyncClient, config_path: String, root_topic: String) {
		if !self.is_enabled() {
			eprintln!("Plugin {} is disabled.", self.name());
			return;
		}

		let topic = format!("{}/{}", root_topic, self.name());
		loop {
			let tmsp = chrono::Utc::now().timestamp().to_string();
			// Get the current load average
			client.publish(&topic, QoS::AtLeastOnce, false, tmsp).await.unwrap();
			// Sleep for 5 seconds before checking the load again
			sleep(Duration::from_secs(5)).await;
		}
	}
}
