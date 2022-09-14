use std::str;

use anyhow::Result;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message as KafkaMessage};
use serde_json::Value;
use tracing::{error, trace};

use lrwn::region::CommonName;

use super::common::message_callback;

pub struct AzureEventHubParameters {
    event_hub_namespace: String,
    topic_name: String,
    share_access_key: String,
    group_id: String,
}

impl AzureEventHubParameters {
    pub fn new(event_hub_namespace: String,
               topic_name: String,
               share_access_key: String,
               group_id: String, ) -> AzureEventHubParameters {
        return AzureEventHubParameters {
            event_hub_namespace,
            topic_name,
            share_access_key,
            group_id,
        };
    }
}

struct AzureKafkaClient {
    consumer: StreamConsumer,
}

pub struct AzureKafkaBackend {}

impl AzureKafkaBackend {
    pub fn new() -> AzureKafkaBackend {
        let azure_backend = AzureKafkaBackend {};
        azure_backend
    }

    pub async fn run(&self, parameters: AzureEventHubParameters) {
        let fetch_thread = tokio::spawn(AzureKafkaBackend::_start_fetch(parameters));
        fetch_thread.await.unwrap();
    }

    async fn _start_fetch(parameters: AzureEventHubParameters) {
        let client = AzureKafkaClient::new(parameters).await.expect("can't init azure client");
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
                            trace!("payload: '{}'",
                                     payload);
                            let msg: Value = serde_json::from_str(payload).expect("");
                            //TODO
                            let gateway_id = msg[0]["data"]["systemProperties"]["iothub-connection-device-id"].as_str().expect("can't retrieve gateway id");
                            let region_name = msg[0]["data"]["properties"]["region"].as_str().expect("can't retrieve region id");
                            let payload = msg[0]["data"]["body"].as_str().expect("can't retrieve body").as_bytes();
                            trace!("gateway_id {}, region {}", gateway_id,region_name);

                            let topic = msg[0]["data"]["properties"]["event_type"].as_str().expect("can't retrieve topic");
                            message_callback(CommonName::EU868, payload, region_name, topic).await;
                        }
                    };
                }
            };
        }
    }
}

impl AzureKafkaClient {
    pub async fn new(parameters: AzureEventHubParameters) -> Result<AzureKafkaClient> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", format!("{}.servicebus.windows.net:9093", &parameters.event_hub_namespace))
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", "$ConnectionString")
            .set("sasl.password", &parameters.share_access_key)
            .set("group.id", &parameters.group_id)
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


//payload: '[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"up":""},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]'

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let parsed: Value = serde_json::from_str(r#"

[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"up":""},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).unwrap();
        let gateway_id = parsed[0]["data"]["systemProperties"]["iothub-connection-device-id"].as_str().unwrap();
        println!("gateway_id {}", gateway_id);
        assert_eq!(!parsed[0]["data"]["properties"]["up"].is_null(), true);
        assert_eq!(gateway_id, "test-cecile");
    }
}