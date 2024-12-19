use crate::plugins::core::Plugin;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use sysinfo::{LoadAvg, System};
use tokio::time::{sleep, Duration};

pub(crate) struct SystemLoadPlugin {
	enabled: bool,
	config: Option<HashMap<String, String>>,
}

impl SystemLoadPlugin {
	pub fn new() -> Self {
		// todo: load config from file
		SystemLoadPlugin { enabled: true, config: None }
	}
}

#[derive(Serialize)]
struct LoadPayload {
	one_minute: f64,
	five_minutes: f64,
	fifteen_minutes: f64,
}

#[async_trait]
impl Plugin for SystemLoadPlugin {
	fn name(&self) -> &str {
		"system_load"
	}

	fn is_enabled(&self) -> bool {
		self.enabled
	}

	fn config(&self) -> Option<HashMap<String, String>> {
		self.config.clone()
	}

	async fn start(&self, client: AsyncClient, config_path: String, root_topic: String) {
		if !self.is_enabled() {
			eprintln!("Plugin {} is disabled.", self.name());
			return;
		}

		// let mut previous_load: Option<LoadAvg> = None;

		loop {
			// Get the current load average
			let load_avg = System::load_average();
			let topic_1m = format!("{}/{}/1m", root_topic, self.name());
			let topic_5m = format!("{}/{}/5m", root_topic, self.name());
			let topic_15m = format!("{}/{}/15m", root_topic, self.name());

			println!("topics: {:?}/{:?}/{:?}", topic_1m, topic_5m, topic_15m);
			println!("{:?}", load_avg);

			// Compare the load averages manually
			// if previous_load.is_none()
			// 	|| previous_load.as_ref().unwrap().one != load_avg.one
			// 	|| previous_load.as_ref().unwrap().five != load_avg.five
			// 	|| previous_load.as_ref().unwrap().fifteen != load_avg.fifteen
			// {
			// let payload = format!("1m: {:.2}, 5m: {:.2}, 15m: {:.2}", load_avg.one, load_avg.five, load_avg.fifteen);

			// let payload = json!({
            //     "1m": load_avg.one,
            //     "5m": load_avg.five,
            //     "15m": load_avg.fifteen
            // });

			client.publish(&topic_1m, QoS::AtLeastOnce, false, load_avg.one.to_string()).await.unwrap();
			client.publish(&topic_5m, QoS::AtLeastOnce, false, load_avg.five.to_string()).await.unwrap();
			client.publish(&topic_15m, QoS::AtLeastOnce, false, load_avg.fifteen.to_string()).await.unwrap();

			// previous_load = Some(load_avg);
			// }

			// Sleep for 5 seconds before checking the load again
			sleep(Duration::from_secs(5)).await;
		}
	}
}
