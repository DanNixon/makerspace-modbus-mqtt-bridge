use crate::{modbus::ModbusClient, Error};
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub(crate) trait SensorPoll {
    async fn poll_string(&self, modbus: &ModbusClient) -> Result<String, Error>;
}

#[derive(Debug, Deserialize)]
pub(crate) struct SensorMapping {
    topic: String,
    source: SensorMappingSource,
}

impl SensorMapping {
    pub(crate) async fn update(
        &self,
        modbus: &ModbusClient,
        mqtt: &mqtt_channel_client::Client,
    ) -> Result<(), Error> {
        log::info!("Updating: {:?}", self);

        let value_str = self.source.poll_string(modbus).await?;
        log::info!("Got value: {}", value_str);

        if let Err(e) = mqtt.send(paho_mqtt::Message::new(&self.topic, value_str, 2)) {
            log::warn!(
                "Failed to send message for topic {} with error {}",
                self.topic,
                e
            );
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum SensorMappingSource {
    Float32(Float32SensorMapping),
}

#[async_trait]
impl SensorPoll for SensorMappingSource {
    async fn poll_string(&self, modbus: &ModbusClient) -> Result<String, Error> {
        match &self {
            SensorMappingSource::Float32(sensor) => sensor.poll_string(modbus).await,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Float32SensorMapping {
    register: u16,
}

#[async_trait]
impl SensorPoll for Float32SensorMapping {
    async fn poll_string(&self, modbus: &ModbusClient) -> Result<String, Error> {
        Ok(modbus.get_f32(self.register).await?.to_string())
    }
}
