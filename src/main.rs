use core::{
	mqtt::{create_mqtt_client, MqttConfig},
	Config,
};
use machine_uid;
use plugins::{core::Plugin, system_load::SystemLoadPlugin};
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde::Deserialize;
use std::{fs::File, time::Duration};
use sysinfo::System;
use tokio::task;
use user_idle::UserIdle;

mod core;
mod plugins;

// async fn send_keep_alive(client: AsyncClient, topic: String) {
// 	let mut interval = time::interval(Duration::from_secs(1));
// 	loop {
// 		interval.tick().await;
// 		let payload = "keep alive";
// 		if let Err(e) = client.publish(&topic, QoS::AtLeastOnce, false, payload).await {
// 			eprintln!("Failed to send keep alive: {:?}", e);
// 		}
// 	}
// }

// async fn monitor_user_idle(client: AsyncClient, topic: String) {
// 	let mut previous_idle_state = false;

// 	loop {
// 		let idle = UserIdle::get_time().unwrap();
// 		let idle_time_secs = idle.as_seconds();
// 		let current_idle_state = idle_time_secs >= 60; // User idle after 60 seconds
// 		println!("Idle time: {} seconds", idle_time_secs);

// 		if current_idle_state != previous_idle_state {
// 			let state = if current_idle_state { "idle" } else { "active" };
// 			if let Err(e) = client.publish(&topic, QoS::AtLeastOnce, false, state).await {
// 				eprintln!("Failed to send idle state: {:?}", e);
// 			}
// 			previous_idle_state = current_idle_state;
// 		}

// 		tokio::time::sleep(Duration::from_secs(5)).await;
// 	}
// }

#[tokio::main]
async fn main() {
	// Load configuration from a YAML file
	let config_path = "config.yaml";
	let config: Config = {
		let file = File::open(config_path).expect("Failed to open config.yaml");
		serde_yaml::from_reader(file).expect("Failed to parse config.yaml")
	};

	println!("connecting to MQTT...");
	let (client, mut eventloop) = create_mqtt_client(&config.mqtt);

	let system_load_plugin = SystemLoadPlugin::new();
	let system = System::new_all();
	let hardware_uuid = machine_uid::get().unwrap_or_else(|_| "unknown".to_string());

	let root_topic = format!("sys2mqtt/{}", hardware_uuid);
	println!("Root topic: {}", root_topic);

	if system_load_plugin.is_enabled() {
		task::spawn(async move {
			system_load_plugin.start(client, config_path.to_string(), root_topic).await;
		});
	}

	// Keep the event loop running to process MQTT events
	loop {
		match eventloop.poll().await {
			Ok(_) => {}
			Err(e) => {
				eprintln!("MQTT connection error: {:?}", e);
				break;
			}
		}
	}
}
