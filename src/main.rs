mod config;
mod modbus;
mod mqtt;
mod sensors;

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("General type conversion error")]
    ConversionError,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to configuration file
    config_file: PathBuf,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    log::debug!("{:?}", cli);

    let config = config::Config::load(&cli.config_file);
    log::debug!("{:#?}", config);

    let mut mqtt_client = mqtt::connect(config.mqtt).await;
    let modbus_client = modbus::ModbusClient::new(config.modbus_address).await;

    let mut poll_interval = tokio::time::interval(config.poll_interval);
    loop {
        tokio::select! {
            _ = poll_interval.tick() => {
                update_sensors(&modbus_client, &mqtt_client, &config.sensors).await;
            }
            _ = tokio::signal::ctrl_c() => {
                log::info!("Exiting");
                break;
            }
        }
    }

    let _ = mqtt_client.stop().await;
}

async fn update_sensors(
    modbus: &modbus::ModbusClient,
    mqtt: &mqtt_channel_client::Client,
    sensors: &config::SensorMappings,
) {
    log::info!("Updating sensors");

    for sensor in sensors {
        if let Err(e) = sensor.update(modbus, mqtt).await {
            log::warn!("Failed to update sensor, error is {}", e);
        }
    }
}
