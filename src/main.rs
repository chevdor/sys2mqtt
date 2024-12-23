mod core;
mod plugins;

use core::{config_path, mqtt::create_mqtt_client, Config};
use env_logger::Env;
use plugins::{core::Plugin, heart_beat::HeartBeatPlugin, system_load::SystemLoadPlugin, user_idle::UserIdlePlugin};
use std::{fs::File, sync::Arc, thread::sleep};
use tokio::{task, time::Duration};

#[tokio::main]
async fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::debug!("Starting sys2mqtt...");

	let config_path = config_path();
	log::info!("Loading configuration from: {:?}", config_path);

	let config: Config = {
		let file = File::open(config_path.clone()).expect("Failed to open config.yaml");
		serde_yaml::from_reader(file).expect("Failed to parse config.yaml")
	};

	let (client, mut eventloop) = create_mqtt_client(&config.mqtt);
	let mqtt_client = Arc::new(client);
	log::info!("Connection to MQTT Broker OK");

	let hardware_uuid = machine_uid::get().unwrap_or_else(|_| "unknown".to_string());
	let root_topic = format!("sys2mqtt/{}", hardware_uuid);
	log::info!("Root topic: {}", root_topic);

	let heart_beat_plugin = HeartBeatPlugin::new(config_path.clone(), root_topic.clone());
	let user_idle_plugin = UserIdlePlugin::new(config_path.clone(), root_topic.clone());
	let system_load_plugin = SystemLoadPlugin::new(config_path.clone(), root_topic.clone());

	let client_clone = mqtt_client.clone();
	task::spawn(async move {
		if heart_beat_plugin.is_enabled() {
			heart_beat_plugin.start(&client_clone).await;
		} else {
			log::warn!("{} plugin is disabled.", heart_beat_plugin.name());
		}
	});

	let client_clone = mqtt_client.clone();
	task::spawn(async move {
		if system_load_plugin.is_enabled() {
			system_load_plugin.start(&client_clone).await;
		} else {
			log::warn!("{} plugin is disabled.", system_load_plugin.name());
		}
	});

	let client_clone = mqtt_client.clone();
	task::spawn(async move {
		if user_idle_plugin.is_enabled() {
			user_idle_plugin.start(&client_clone).await;
		} else {
			log::warn!("{} plugin is disabled.", user_idle_plugin.name());
		}
	});

	// Keep the event loop running to process MQTT events
	loop {
		match eventloop.poll().await {
			Ok(_) => {}
			Err(e) => {
				log::error!("MQTT connection error: {:?}", e);
				sleep(Duration::from_secs(5));
				continue;
			}
		}
	}
}
