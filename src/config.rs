use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::{net::SocketAddr, path::Path, time::Duration};
use url::Url;

#[serde_as]
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) modbus_address: SocketAddr,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub(crate) poll_interval: Duration,

    pub(crate) mqtt: MqttConfig,

    #[serde(default)]
    pub(crate) sensors: SensorMappings,
}

impl Config {
    pub(crate) fn load(filename: &Path) -> Self {
        toml::from_str(
            &std::fs::read_to_string(filename).expect("Configuration file should be readable"),
        )
        .expect("Configuration file should be valid")
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct MqttConfig {
    pub(crate) broker: Url,
    pub(crate) client_id: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

pub(crate) type SensorMappings = Vec<crate::sensors::SensorMapping>;
