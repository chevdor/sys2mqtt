use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use tokio::time::{sleep, Duration};

pub(crate) struct HeartBeatPlugin {
	enabled: bool,
	// config: Option<HashMap<String, String>>,
	delay: u64,
}

impl HeartBeatPlugin {
	pub fn new() -> Self {
		let enabled =
			std::env::var("SYS2MQTT_HEART_BEAT_ENABLED").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
		let delay =
			std::env::var("SYS2MQTT_HEART_BEAT_DELAY").unwrap_or_else(|_| "30".to_string()).parse().unwrap_or(5);
		// todo: load config from file
		HeartBeatPlugin { enabled, delay }
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

	// fn config(&self) -> Option<HashMap<String, String>> {
	// 	None
	// }

	async fn start(&self, client: &AsyncClient, _config_path: String, root_topic: String) {
		log::debug!("Starting heart beat plugin...");
		if !self.is_enabled() {
			log::warn!("Plugin {} is disabled.", self.name());
			return;
		}

		let topic = format!("{}/{}", root_topic, self.name());
		log::debug!("Heartbeat topic: {}", topic);

		loop {
			let tmsp = chrono::Utc::now().timestamp().to_string();
			// Get the current load average
			client.publish(&topic, QoS::AtLeastOnce, false, tmsp).await.unwrap();
			// Sleep for 5 seconds before checking the load again
			sleep(Duration::from_secs(self.delay)).await;
		}
	}
}
