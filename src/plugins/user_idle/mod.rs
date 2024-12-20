use std::path::PathBuf;

use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use tokio::time::Duration;
use user_idle::UserIdle;

#[allow(dead_code)]
pub(crate) struct UserIdlePlugin {
	enabled: bool,
	delay: u64,
	timeout: u64,
	config_path: PathBuf,
	root_topic: String,
}

impl UserIdlePlugin {
	pub fn new(config_path: PathBuf, root_topic: String) -> Self {
		let enabled =
			std::env::var("SYS2MQTT_USER_IDLE_ENABLED").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
		let delay = std::env::var("SYS2MQTT_USER_IDLE_DELAY").unwrap_or_else(|_| "30".to_string()).parse().unwrap_or(5);
		let timeout =
			std::env::var("SYS2MQTT_USER_IDLE_TIMEOUT").unwrap_or_else(|_| "60".to_string()).parse().unwrap_or(60);
		// todo: load config from file
		UserIdlePlugin { enabled, delay, timeout, config_path, root_topic }
	}
}

#[async_trait]
impl Plugin for UserIdlePlugin {
	fn name(&self) -> &str {
		"user_idle"
	}

	fn is_enabled(&self) -> bool {
		self.enabled
	}

	async fn start(&self, client: &AsyncClient) {
		if cfg!(debug_assertions) {
			log::debug!("Starting user idle plugin...");
		}

		if !self.is_enabled() {
			log::warn!("Plugin {} is disabled.", self.name());
			return;
		}

		let topic = format!("{}/{}", self.root_topic, self.name());
		if cfg!(debug_assertions) {
			log::debug!("UserIdle topic: {}", topic);
		}

		let mut previous_idle_state: Option<bool> = None;
		loop {
			let idle = UserIdle::get_time().unwrap();
			let idle_time_secs = idle.as_seconds();
			let current_idle_state = idle_time_secs >= self.timeout;

			if cfg!(debug_assertions) {
				log::debug!("Idle state: {}", current_idle_state);
			}

			match previous_idle_state {
				Some(_) => {
					let state = if current_idle_state { "idle" } else { "active" };

					if current_idle_state != previous_idle_state.unwrap() {
						if let Err(e) = client.publish(&topic, QoS::AtLeastOnce, false, state).await {
							log::error!("Failed to send idle state: {:?}", e);
						}
					}

					previous_idle_state = Some(current_idle_state);
					// nothing to do
				}
				None => {
					let state = if current_idle_state { "idle" } else { "active" };
					if let Err(e) = client.publish(&topic, QoS::AtLeastOnce, false, state).await {
						log::error!("Failed to send idle state: {:?}", e);
					}
					previous_idle_state = Some(current_idle_state);
				}
			}

			tokio::time::sleep(Duration::from_secs(self.delay)).await;
		}
	}
}
