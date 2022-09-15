use std::str;

use anyhow::{Error, Result};
use async_trait::async_trait;
use base64::decode;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message as KafkaMessage};
use serde_json::Value;
use tracing::{error, info, trace};

use lrwn::region::CommonName;

use crate::config::GatewayBackendAzure;

use super::common::message_callback;
use super::GatewayBackend;

struct AzureKafkaClient {
    consumer: StreamConsumer,
}

struct MessageProperties {
    payload: Vec<u8>,
    region_name: String,
    topic: String,
}

pub struct AzureKafkaBackend {}

impl AzureKafkaBackend {
    pub async fn new(region_name: &str,
                     region_common_name: CommonName, conf: &GatewayBackendAzure) -> Result<AzureKafkaBackend> {
        let azure_backend = AzureKafkaBackend {};
        let client = AzureKafkaClient::new(region_name, conf).await.expect("can't init azure client");
        azure_backend.run_consumer(region_name, region_common_name, client);
        Ok(azure_backend)
    }

    fn run_consumer(&self, region: &str, region_common_name: CommonName, client: AzureKafkaClient) {
        tokio::spawn({
            let region = region.to_string();
            async move {
                loop {
                    match client.consumer.recv().await {
                        Err(e) => {
                            error!("Kafka error: {}", e);
                        }
                        Ok(m) => {
                            match client.process(&m).await {
                                Err(e) => {
                                    error!("error when processing message and retrieving payload: {}", e);
                                }
                                Ok(payload) => {
                                    let region = region.clone();
                                    match Self::check_payload_and_callback(payload) {
                                        Ok(m) => {
                                            if region == m.region_name {
                                                let payload_decode_array: &[u8] = &m.payload;
                                                message_callback(region_common_name, payload_decode_array, m.region_name.as_str(), m.topic.as_str()).await;
                                                trace!("message has been processed");
                                            }
                                            continue;
                                        }
                                        Err(err) => {
                                            error!("Problem processing msg {:?}",err);
                                            continue;
                                        }
                                    }
                                }
                            };
                        }
                    };
                }
            }
        });
    }

    fn check_payload_and_callback(payload: &str) -> Result<MessageProperties, Error> {
        type Err = Error;

        info!("payload: '{}'",payload);
        let msg: Value = serde_json::from_str(payload).unwrap_or_default();
        let gateway_id = match msg[0]["data"]["systemProperties"]["iothub-connection-device-id"].as_str() {
            Some(s) => s,
            None => ""
        };
        let region_name = match msg[0]["data"]["properties"]["region"].as_str() {
            Some(s) => s,
            None => return Err(anyhow!("Region does not exist"))
        };
        let payload = match msg[0]["data"]["body"].as_str() {
            Some(s) => s,
            None => return Err(anyhow!("Payload does not exist"))
        };
        let payload = payload.as_bytes();
        trace!("gateway_id {}, region {}", gateway_id,region_name );
        let payload_decode = match decode(payload) {
            Ok(vec) => vec,
            Err(err) => {
                return Err(anyhow!("Problem decoding payload in base64: {:?}", err));
            }
        };
        let topic = match msg[0]["data"]["properties"]["event_type"].as_str() {
            Some(s) => s,
            None => return Err(anyhow!("Problem retrieving topic"))
        };
        Ok(MessageProperties {
            payload: payload_decode,
            topic: topic.to_string(),
            region_name: region_name.to_string(),
        })
    }
}

#[async_trait]
impl GatewayBackend for AzureKafkaBackend {
    async fn send_downlink(&self, _df: &chirpstack_api::gw::DownlinkFrame) -> Result<()> {
        Ok(())
    }

    async fn send_configuration(&self, _gw_conf: &chirpstack_api::gw::GatewayConfiguration) -> Result<()> {
        Ok(())
    }
}

impl AzureKafkaClient {
    pub async fn new(region: &str, parameters: &GatewayBackendAzure) -> Result<AzureKafkaClient> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", format!("{}.servicebus.windows.net:9093", &parameters.event_hub_namespace))
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", "$ConnectionString")
            .set("sasl.password", &parameters.share_access_key)
            .set("group.id", region)
            .create()
            .expect("Consumer creation failed");
        consumer.subscribe(&[&parameters.topic_name])
            .expect("Can't subscribe to topic");

        let azure_client = AzureKafkaClient {
            consumer,
        };
        Ok(azure_client)
    }

    pub async fn process<'a>(&'a self, m: &'a BorrowedMessage<'_>) -> Result<&str> {
        let payload = match m.payload_view::<str>() {
            None => "",
            Some(Ok(s)) => s,
            Some(Err(e)) => {
                error!("Error while deserializing message payload: {:?}", e);
                ""
            }
        };
        trace!("key: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                 m.key(), m.topic(), m.partition(), m.offset(), m.timestamp());
        self.consumer.commit_message(&m, CommitMode::Async).unwrap();
        Ok(payload)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_check_payload_and_callback_ok() {
        let callback = AzureKafkaBackend::check_payload_and_callback(r#"[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"event_type":"up","region":"eu868"},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).unwrap();
        assert_eq!(callback.region_name, "eu868");
        assert_eq!(callback.topic, "up");
        assert_eq!(callback.payload, decode(b"aGVsbG8=").unwrap());
    }

    #[test]
    fn test_check_payload_and_callback_no_region() {
        let callback_err = AzureKafkaBackend::check_payload_and_callback(r#"[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"event_type":"up"},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).err();
        assert_eq!(callback_err.is_some(), true);
    }

    #[test]
    fn test_check_payload_and_callback_no_event_type() {
        let callback_err = AzureKafkaBackend::check_payload_and_callback(r#"[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"region":"eu868"},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).err();
        assert_eq!(callback_err.is_some(), true);
    }
}