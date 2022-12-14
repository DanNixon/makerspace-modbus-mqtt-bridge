use crate::Error;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use tokio_modbus::client::{self, Reader};

pub(crate) struct ModbusClient {
    context: Mutex<client::Context>,
}

impl ModbusClient {
    pub(crate) async fn new(address: SocketAddr) -> Self {
        log::info!("Connecting to Modbus TCP at {}", address);
        let context = tokio_modbus::client::tcp::connect(address)
            .await
            .expect("Modbus TCP client should be created");
        log::info!("Modbus TCP connected");

        Self {
            context: context.into(),
        }
    }

    /// Parses two 16 bit integer registers as a single 32 bit float value.
    pub(crate) async fn get_f32(&self, register: u16) -> Result<f32, Error> {
        let data = self
            .context
            .lock()
            .await
            .read_holding_registers(register, 2)
            .await?;

        let data: Vec<u8> = data.into_iter().flat_map(|i| i.to_be_bytes()).collect();
        let data: [u8; 4] = data.try_into().map_err(|_| Error::ConversionError)?;

        Ok(f32::from_be_bytes(data))
    }
}
