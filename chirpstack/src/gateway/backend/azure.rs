use std::str;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message as KafkaMessage};
use serde_json::Value;
use tracing::{error, info, trace};


use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::io::Cursor;
use std::sync::RwLock;
use anyhow::{Result};
use prometheus_client::encoding::text::Encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use tokio::task;

use crate::monitoring::prometheus;
use crate::storage::{get_redis_conn, redis_key};
use crate::{downlink, uplink};
use lrwn::region::CommonName;
use prost::Message;
//TODO move
lazy_static! {
    static ref EVENT_COUNTER: Family<EventLabels, Counter> = {
        let counter = Family::<EventLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_events",
            "Number of events received",
            Box::new(counter.clone()),
        );
        counter
    };
    static ref COMMAND_COUNTER: Family<CommandLabels, Counter> = {
        let counter = Family::<CommandLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_commands",
            "Number of commands sent",
            Box::new(counter.clone()),
        );
        counter
    };
    static ref GATEWAY_JSON: RwLock<HashMap<String, bool>> = RwLock::new(HashMap::new());
}
//TODO move
#[derive(Clone, Hash, PartialEq, Eq, Encode)]
struct EventLabels {
    event: String,
}
//TODO move
#[derive(Clone, Hash, PartialEq, Eq, Encode)]
struct CommandLabels {
    command: String,
}
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
                            let parsed: Value = serde_json::from_str(payload).expect("");
                            //TODO
                            message_callback(CommonName::EU868,parsed).await;
                        }
                    };
                }
            };
        }
    }
}

async fn message_callback(region_common_name: CommonName,msg: Value) {
    let gateway_id = msg[0]["data"]["systemProperties"]["iothub-connection-device-id"].as_str().expect("can't retrieve gateway id");
    let region_name = msg[0]["data"]["properties"]["region"].as_str().expect("can't retrieve region id");
    let payload = msg[0]["data"]["body"].as_str().expect("can't retrieve body").as_bytes();
    trace!("gateway_id {}, region {}", gateway_id,region_name);

    let topic = msg[0]["data"]["properties"]["event_type"].as_str().expect("can't retrieve topic");

    let mut hasher = DefaultHasher::new();
    hasher.write(payload);
    let key = redis_key(format!("gw:mqtt:lock:{:x}", hasher.finish()));
    let locked = is_locked(key).await;


    let err = || -> Result<()> {
        if locked? {
            trace!(
                region_name = region_name,
                topic = topic,
                "Message is already handled by different instance"
            );
            return Ok(());
        }

        let json = payload_is_json(payload);

        info!(
            region_name = region_name,
            topic = topic,
            json = json,
            "Message received from gateway"
        );

        if topic.eq("up") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "up".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(payload)?,
                false => chirpstack_api::gw::UplinkFrame::decode(&mut Cursor::new(payload))?,
            };
            event.v4_migrate();

            if let Some(rx_info) = &mut event.rx_info {
                set_gateway_json(&rx_info.gateway_id, json);
                rx_info.set_metadata_string("region_name", region_name);
                rx_info.set_metadata_string("region_common_name", &region_common_name.to_string());
            }

            tokio::spawn(uplink::deduplicate_uplink(event));
        } else if topic.eq("stats") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "stats".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(payload)?,
                false => chirpstack_api::gw::GatewayStats::decode(&mut Cursor::new(payload))?,
            };
            event.v4_migrate();
            event
                .meta_data
                .insert("region_name".to_string(), region_name.to_string());
            event.meta_data.insert(
                "region_common_name".to_string(),
                region_common_name.to_string(),
            );
            set_gateway_json(&event.gateway_id, json);
            tokio::spawn(uplink::stats::Stats::handle(event));
        } else if topic.eq("ack") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "ack".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(payload)?,
                false => chirpstack_api::gw::DownlinkTxAck::decode(&mut Cursor::new(payload))?,
            };
            event.v4_migrate();
            set_gateway_json(&event.gateway_id, json);
            tokio::spawn(downlink::tx_ack::TxAck::handle(event));
        } else {
            return Err(anyhow!("Unknown event type"));
        }

        Ok(())
    }()
        .err();

    if err.is_some() {
        error!(
            topic = topic,
            "Processing gateway event error: {}",
            err.as_ref().unwrap()
        );
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

//TODO common
async fn is_locked(key: String) -> Result<bool> {
    task::spawn_blocking({
        move || -> Result<bool> {
            let mut c = get_redis_conn()?;

            let set: bool = redis::cmd("SET")
                .arg(key)
                .arg("lock")
                .arg("PX")
                .arg(5000)
                .arg("NX")
                .query(&mut *c)?;

            Ok(!set)
        }
    })
        .await?
}
//TODO common
fn payload_is_json(b: &[u8]) -> bool {
    String::from_utf8_lossy(b).contains("gatewayId")
}
//TODO common
fn set_gateway_json(gateway_id: &str, is_json: bool) {
    let mut gw_json_w = GATEWAY_JSON.write().unwrap();
    gw_json_w.insert(gateway_id.to_string(), is_json);
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