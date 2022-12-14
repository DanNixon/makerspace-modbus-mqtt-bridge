use crate::config::MqttConfig;
use mqtt_channel_client::{Client, ClientConfig};
use paho_mqtt::{
    connect_options::ConnectOptionsBuilder, create_options::CreateOptionsBuilder, PersistenceType,
};
use std::time::Duration;

pub(crate) async fn connect(config: MqttConfig) -> Client {
    let mut client = Client::new(
        CreateOptionsBuilder::new()
            .server_uri(config.broker)
            .client_id(config.client_id)
            .persistence(PersistenceType::None)
            .finalize(),
        ClientConfig::default(),
    )
    .expect("MQTT client should be created");

    log::info!("Starting MQTT client");
    client
        .start(
            ConnectOptionsBuilder::new()
                .clean_session(true)
                .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(5))
                .keep_alive_interval(Duration::from_secs(5))
                .user_name(config.username)
                .password(config.password)
                .finalize(),
        )
        .await
        .expect("MQTT client should be started");

    client
}
