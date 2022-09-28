use std::{str, thread};
use std::sync::mpsc::{Receiver as ChanRceiver, SyncSender as ChanSender};
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Error, Result};
use async_trait::async_trait;
use base64::decode;
use fe2o3_amqp::{
    Connection, sasl_profile::SaslProfile, Sender, Session,
};
use fe2o3_amqp::{
    types::{
        messaging::{Message as AmqpMessage, Properties},
        primitives::Binary,
    },
};
use fe2o3_amqp::connection::ConnectionHandle;
use fe2o3_amqp::types::messaging::ApplicationProperties;
use hmac::{Hmac, Mac};
use prost::Message;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message as KafkaMessage};
use rdkafka::message::ToBytes;
use serde_amqp;
use serde_json::Value;
use sha2::Sha256;
use tracing::{error, info, trace};
use uuid::Uuid;

use lrwn::region::CommonName;

use crate::config::GatewayBackendAzure;

use super::common::COMMAND_COUNTER;
use super::common::CommandLabels;
use super::common::gateway_is_json;
use super::common::message_callback;
use super::GatewayBackend;

struct BusConnectionInformation {
    hostname: String,
    shared_access_key_name: String,
    shared_access_key: String,
}

struct EventConnectionInformation {
    hostname: String,
    connection_string: String,
    topic_name: String,
}

struct AzureKafkaClient {
    consumer: Option<StreamConsumer>,
}

struct AzureAmqpClient {
    parameters: BusConnectionInformation,
}

struct MessageProperties {
    payload: Vec<u8>,
    region_name: String,
    topic: String,
}

pub struct AzureBackend {
    tx: Option<ChanSender<AmqpMessage<serde_amqp::value::Value>>>,
}

impl AzureBackend {
    pub async fn new(region_name: &str,
                     region_common_name: CommonName, conf: &GatewayBackendAzure) -> Result<AzureBackend> {
        let bus_information = build_bus_connection_string(conf.commands_connection_string.to_string()).unwrap();
        let downlink_client = AzureAmqpClient::new(bus_information).await.unwrap();
        let (tx, rx): (ChanSender<AmqpMessage<serde_amqp::value::Value>>, ChanRceiver<AmqpMessage<serde_amqp::value::Value>>) = mpsc::sync_channel(1000);
        let azure_backend = AzureBackend {
            tx: Some(tx),
        };
        azure_backend.run_downlink(downlink_client, rx);

        let event_information = build_event_connection_string(conf.events_connection_string.to_string()).unwrap();

        azure_backend.run_uplink(region_name, region_common_name, event_information);
        Ok(azure_backend)
    }

    fn run_downlink(&self, downlink_client: AzureAmqpClient, rx: ChanRceiver<AmqpMessage<serde_amqp::value::Value>>) {
        tokio::spawn(async move {
            loop {
                trace!("Initiate amqp connection");
                let sas_token_duration = Duration::from_secs(60 * 60 * 24);
                let event_pubsub_connection_duration = Duration::from_millis(240000);
                let mut start = Instant::now();
                let mut connection = match downlink_client.build_sender(sas_token_duration.clone()).await {
                    Ok(s) => s,
                    Err(e) => {
                        error!("can not build connection{:?}",e);
                        thread::sleep(Duration::from_secs(10));
                        continue;
                    }
                };

                let mut session = match Session::begin(&mut connection).await {
                    Ok(s) => s,
                    Err(e) => {
                        error!("can not build session{:?}",e);
                        continue;
                    }
                };

                let mut reconnect = false;
                loop {
                    let check_duration = start.elapsed();
                    if check_duration >= (sas_token_duration - event_pubsub_connection_duration) {
                        trace!("sas about to expire, needs reconnection");
                        reconnect = true;
                    }

                    if reconnect {
                        trace!("Reconnect amqp client");
                        match session.close().await {
                            Ok(r) => r,
                            Err(err) => {
                                trace!("can't close amqp connection: {:?}", err);
                                ()
                            }
                        };
                        match connection.close().await {
                            Ok(r) => r,
                            Err(err) => {
                                trace!("can't close amqp connection: {:?}", err);
                                ()
                            }
                        };
                        start = Instant::now();
                        let clone_duration = sas_token_duration.clone();
                        connection = match downlink_client.build_sender(clone_duration).await {
                            Ok(s) => s,
                            Err(e) => {
                                error!("can not build connection{:?}",e);
                                break;
                            }
                        };
                        session = match Session::begin(&mut connection).await {
                            Ok(s) => s,
                            Err(e) => {
                                error!("can not build session{:?}",e);
                                break;
                            }
                        };
                        reconnect = false;
                    }
                    let timeout_duration = event_pubsub_connection_duration.clone();
                    let message = match rx.recv_timeout(timeout_duration) {
                        Ok(message) => message,
                        Err(_) => {
                            reconnect = true;
                            trace!("Inactivity occured, set reconnection");
                            continue;
                        }
                    };

                    let mut sender = match Sender::attach(&mut session, "chirpstack-sender", "/messages/devicebound")
                        .await {
                        Ok(s) => s,
                        Err(e) => {
                            error!("can not create sender {:?}",e);
                            reconnect = true;
                            continue;
                        }
                    };

                    let outcome = match sender.send(message).await {
                        Ok(m) => m,
                        Err(err) => {
                            error!("error while sending amqp message {:?}",err);
                            reconnect = true;
                            continue;
                        }
                    };

                    let _state = match outcome.accepted_or_else(|outcome| outcome) {
                        Ok(m) => m,
                        Err(err) => {
                            error!("amqp message not accepted {:?}",err);
                            continue;
                        }
                    };

                    match sender.close().await {
                        Ok(r) => r,
                        Err(err) => {
                            error!("can't close amqp sender: {:?}", err);
                            ()
                        }
                    };
                }
            }
        }
        );
    }


    fn run_uplink(&self, region: &str, region_common_name: CommonName, event_information: EventConnectionInformation) {
        tokio::spawn({
            let region = region.to_string();
            async move {
                let mut uplink_client = AzureKafkaClient::new(region.as_str(), &event_information).await.expect("can't init azure client");
                loop {
                    match uplink_client.consumer.as_ref().unwrap().recv().await {
                        Err(e) => {
                            error!("Kafka error, rebuilding the client: {}", e);
                            uplink_client = AzureKafkaClient::new(region.as_str(), &event_information).await.expect("can't init azure client");
                            continue;
                        }
                        Ok(m) => {
                            match uplink_client.process(&m).await {
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

fn build_bus_connection_string(azure_connection_string: String) -> Result<BusConnectionInformation> {
    let mut hostname = "";
    let mut shared_access_key_name = "";
    let mut shared_access_key = "";
    for key_value in azure_connection_string.split(";") {
        let mut spl = key_value.split("=");
        let key_option = spl.next();
        let value_option = spl.next();
        if key_option.is_none() || value_option.is_none() {
            continue;
        }
        let key = key_option.unwrap();
        let value = value_option.unwrap();
        if key == "HostName" {
            hostname = value;
        } else if key == "SharedAccessKeyName" {
            shared_access_key_name = value;
        } else if key == "SharedAccessKey" {
            shared_access_key = value;
        }
    }
    let mut hostname_split = hostname.split(".");
    let hub_name = hostname_split.next();
    if hub_name.is_none() {
        error!("no hubname");
    }
    Ok(BusConnectionInformation {
        shared_access_key_name: format!("{}@sas.root.{}", shared_access_key_name, hub_name.unwrap()),
        hostname: hostname.to_string(),
        shared_access_key: shared_access_key.to_string(),
    })
}

//Endpoint=sb://testrust2.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=uuuuuuuuuuuuuuu
/// //Endpoint=sb://testrust2.servicebus.windows.net/;SharedAccessKeyName=iothubroutes_hiber-iot-hub2;SharedAccessKey=uuuuuuuuuuuuuuu=;EntityPath=eventdirect
//TODO test
fn build_event_connection_string(azure_connection_string: String) -> Result<EventConnectionInformation> {
    let mut hostname = "".to_string();
    //todo
    let mut topic_name = "eventdirect".to_string();
    for key_value in azure_connection_string.split(";") {
        let mut spl = key_value.split("=");
        let key_option = spl.next();
        let value_option = spl.next();
        if key_option.is_none() || value_option.is_none() {
            continue;
        }
        let key = key_option.unwrap();
        let value = value_option.unwrap().to_string();
        if key == "Endpoint" {
            let first_replace = value.replace("sb://", "").replace("/", "");
            hostname = first_replace.as_str().to_string();
        }
        if key == "EntityPath" {
            topic_name = value.as_str().to_string();
        }
    }
    let mut hostname_split = hostname.split(".");
    let hub_name = hostname_split.next();
    if hub_name.is_none() {
        error!("no hubname");
    }
    Ok(EventConnectionInformation {
        connection_string: azure_connection_string,
        hostname: hub_name.unwrap().to_string(),
        topic_name,
    })
}

#[async_trait]
impl GatewayBackend for AzureBackend {
    async fn send_downlink(&self, df: &chirpstack_api::gw::DownlinkFrame) -> Result<()> {
        COMMAND_COUNTER
            .get_or_create(&CommandLabels {
                command: "down".to_string(),
            })
            .inc();
        info!("message: {:?}",df);


        let mut df = df.clone();
        df.v4_migrate();

        let json = gateway_is_json(&df.gateway_id);
        let b = match json {
            true => serde_json::to_vec(&df).unwrap(),
            false => df.encode_to_vec(),
        };

        let topic = format!("/devices/{}/messages/devicebound", &df.gateway_id);
        info!(gateway_id = %df.gateway_id, topic = %topic, json = json, "Sending downlink frame");

        let message_id = Uuid::new_v4();
        // All of the Microsoft AMQP clients represent the event body as an uninterpreted bag of bytes.
        let data = b.to_bytes();

        let message = AmqpMessage::builder()
            .properties(Properties::builder()
                .message_id(message_id.to_string())
                .to(topic)
                .build())
            .application_properties(ApplicationProperties::builder()
                .insert("iothub-ack", "none")
                .insert("command", "down")
                .build()
            )
            .data(Binary::from(data))
            .build();

        match self.tx.as_ref().unwrap().send(message) {
            Err(err) => {
                error!("{:?}",err)
            }
            _ => {}
        };


        Ok(())
    }

    async fn send_configuration(&self, gw_conf: &chirpstack_api::gw::GatewayConfiguration) -> Result<()> {
        let json = gateway_is_json(&gw_conf.gateway_id);
        let b = match json {
            true => serde_json::to_vec(&gw_conf)?,
            false => gw_conf.encode_to_vec(),
        };

        let topic = format!("/devices/{}/messages/devicebound", &gw_conf.gateway_id);
        info!(gateway_id = %gw_conf.gateway_id, topic = %topic, json = json, "Sending gateway configuration");

        let message_id = Uuid::new_v4();
        // All of the Microsoft AMQP clients represent the event body as an uninterpreted bag of bytes.
        let data = b.to_bytes();

        let message = AmqpMessage::builder()
            .properties(Properties::builder()
                .message_id(message_id.to_string())
                .to(topic)
                .build())
            .application_properties(ApplicationProperties::builder()
                .insert("iothub-ack", "none")
                .insert("command", "config")
                .build()
            )
            .data(Binary::from(data))
            .build();

        match self.tx.as_ref().unwrap().send(message) {
            Err(err) => {
                error!("{:?}",err)
            }
            _ => {}
        };

        Ok(())
    }
}

impl AzureAmqpClient {
    pub async fn new(parameters: BusConnectionInformation) -> Result<AzureAmqpClient> {
        Ok(AzureAmqpClient { parameters })
    }

    async fn build_sender(&self, duration: Duration) -> Result<ConnectionHandle<()>> {
        let port = 5671;
        let sa_key_value = create_sas_token(&self.parameters.hostname, &self.parameters.shared_access_key_name, &self.parameters.shared_access_key, &(SystemTime::now() + duration)).unwrap();
        let url = format!("amqps://{}:{}", &self.parameters.hostname, port);
        let connection = match Connection::builder()
            .container_id("rust-receiver-connection-1")
            .alt_tls_establishment(true) // ServiceBus uses alternative TLS establishement
            .hostname(&self.parameters.hostname[..])
            .sasl_profile(SaslProfile::Plain {
                username: self.parameters.shared_access_key_name.as_str().to_string(),
                password: sa_key_value,
            })
            .open(&url[..])
            .await {
            Ok(c) => c,
            Err(e) => {
                error!("can not build connection{:?}",e);
                return Err(Error::new(e));
            }
        };
        Ok(connection)
    }
}

impl AzureKafkaClient {
    pub async fn new(region: &str, parameters: &EventConnectionInformation) -> Result<AzureKafkaClient> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", format!("{}.servicebus.windows.net:9093", &parameters.hostname))
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", "$ConnectionString")
            .set("sasl.password", &parameters.connection_string)
            .set("group.id", region)
            .create()
            .expect("Consumer creation failed");
        consumer.subscribe(&[&parameters.topic_name])
            .expect("Can't subscribe to topic");

        let azure_client = AzureKafkaClient {
            consumer: Some(consumer),
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
        self.consumer.as_ref().unwrap().commit_message(&m, CommitMode::Async).unwrap();
        Ok(payload)
    }
}

fn create_sas_token(
    hostname: &str,
    key_name: &str,
    key: &str,
    expiration: &SystemTime,
) -> Result<String> {
    type HmacSha256 = Hmac<Sha256>;
    let encoded_url = urlencoding::encode(&hostname);
    let exp = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let sig = format!("{}\n{}", encoded_url, exp);
    let x = sig.as_bytes();
    println!("{:?}", x);
    let vec = base64::decode(key.as_bytes()).unwrap();
    let key_decode = vec.to_bytes();
    let mut m = HmacSha256::new_from_slice(key_decode)?;

    m.update(x);
    let result = base64::encode(m.finalize().into_bytes());

    let hash = urlencoding::encode(&result);

    Ok(format!(
        "SharedAccessSignature sr={}&sig={}&se={}&skn={}",
        encoded_url, hash, exp, key_name
    ))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_check_payload_and_callback_ok() {
        let callback = AzureBackend::check_payload_and_callback(r#"[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"event_type":"up","region":"eu868"},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).unwrap();
        assert_eq!(callback.region_name, "eu868");
        assert_eq!(callback.topic, "up");
        assert_eq!(callback.payload, decode(b"aGVsbG8=").unwrap());
    }

    #[test]
    fn test_check_payload_and_callback_no_region() {
        let callback_err = AzureBackend::check_payload_and_callback(r#"[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"event_type":"up"},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).err();
        assert_eq!(callback_err.is_some(), true);
    }

    #[test]
    fn test_check_payload_and_callback_no_event_type() {
        let callback_err = AzureBackend::check_payload_and_callback(r#"[{"id":"47b0b740-8c32-1d88-b84b-343c765f11d8","topic":"/SUBSCRIPTIONS/D2037D29-C0F0-4E03-8E1E-87A0BFEAF16F/RESOURCEGROUPS/TEST/PROVIDERS/MICROSOFT.DEVICES/IOTHUBS/HIBER-TEST-HUB","subject":"devices/test-cecile","eventType":"Microsoft.Devices.DeviceTelemetry","data":{"properties":{"region":"eu868"},"systemProperties":{"iothub-connection-device-id":"test-cecile","iothub-connection-auth-method":"{\"scope\":\"device\",\"type\":\"sas\",\"issuer\":\"iothub\",\"acceptingIpFilterRule\":null}","iothub-connection-auth-generation-id":"637968607726449711","iothub-enqueuedtime":"2022-09-13T13:41:36.7740000Z","iothub-message-source":"Telemetry"},"body":"aGVsbG8="},"dataVersion":"","metadataVersion":"1","eventTime":"2022-09-13T13:41:36.774Z"}]
"#).err();
        assert_eq!(callback_err.is_some(), true);
    }
}