use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::time::Duration;
use user_idle::UserIdle;

#[allow(dead_code)]
pub(crate) struct UserIdlePlugin {
	config: UserIdleConfig,
	config_path: PathBuf,
	root_topic: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct UserIdleConfig {
	enabled: bool,
	delay: u64,
	timeout: u64,
}

impl Default for UserIdleConfig {
	fn default() -> Self {
		Self { enabled: true, delay: 5, timeout: 60 }
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct WrappedConfig {
	user_idle: Option<UserIdleConfig>,
}

impl UserIdleConfig {
	/// Load the configuration from the specified path and the environment variables.
	/// The environment variables take precedence over the configuration file.
	/// If no valid configuration is found, the plugin is disabled.
	pub fn load(config_path: &PathBuf) -> Self {
		let config_str = std::fs::read_to_string(config_path).unwrap();
		let config: WrappedConfig = serde_yaml::from_str(&config_str).unwrap();
		let config = config.user_idle;

		let enabled_env = std::env::var("SYS2MQTT_USER_IDLE_ENABLED");
		let delay_env = std::env::var("SYS2MQTT_USER_IDLE_DELAY");
		let timeout_env = std::env::var("SYS2MQTT_USER_IDLE_TIMEOUT");
		let enabled_config = if let Some(ref config) = config { config.enabled } else { false };
		let delay_config = if let Some(ref config) = config { config.delay } else { 5 };
		let timeout_config = if let Some(ref config) = config { config.delay } else { 5 };

		let enabled =
			if let Ok(enabled_env) = enabled_env { enabled_env.parse().unwrap_or(false) } else { enabled_config };
		let delay = if let Ok(delay_env) = delay_env { delay_env.parse().unwrap_or(5) } else { delay_config };
		let timeout = if let Ok(timeout_env) = timeout_env { timeout_env.parse().unwrap_or(5) } else { timeout_config };

		Self { enabled, delay, timeout }
	}
}

impl UserIdlePlugin {
	pub fn new(config_path: PathBuf, root_topic: String) -> Self {
		let config = UserIdleConfig::load(&config_path);
		UserIdlePlugin { config, config_path, root_topic }
	}
}

#[async_trait]
impl Plugin for UserIdlePlugin {
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
			log::debug!("UserIdle topic: {}", topic);
		}

		let mut previous_idle_state: Option<bool> = None;
		loop {
			let idle = UserIdle::get_time().unwrap();
			let idle_time_secs = idle.as_seconds();
			let current_idle_state = idle_time_secs >= self.config.timeout;

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

			tokio::time::sleep(Duration::from_secs(self.config.delay)).await;
		}
	}
}
