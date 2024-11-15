use serde::Deserialize;

use crate::globals::SETTINGS;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub distributor_address: Option<String>,
    pub kafka_address: Option<String>,
    pub kafka_topic: Option<String>,
    pub kafka_partition: Vec<usize>,
    pub tcp_address: String,
    pub ws_address: String,
    pub mode: Mode,
    pub interface_ip: String,
    pub udp_multicast_address: String,
}

#[derive(Debug, Deserialize, Clone, Default, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Tcp,
    Udp,
    Kafka,
}

pub fn init(path: &String) {
    let data = std::fs::read_to_string(path).unwrap();
    let settings: Settings = serde_json::from_str(&data).unwrap();

    let _ = SETTINGS.set(settings);
}

pub fn get() -> &'static Settings {
    SETTINGS.get().unwrap()
}
