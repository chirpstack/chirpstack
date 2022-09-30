use std::{str, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver as ChanRceiver, SyncSender as ChanSender};
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
use fe2o3_amqp::session::SessionHandle;
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
use tokio_util::sync::CancellationToken;

#[cfg(not(test))]
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
    hub_name: String,
    connection_string: String,
    topic_name: String,
}

struct AzureKafkaClient {
    consumer: Option<StreamConsumer>,
}

struct MessageProperties {
    payload: Vec<u8>,
    region_name: String,
    topic: String,
}

pub struct AzureBackend {
    tx: Option<ChanSender<AmqpMessage<serde_amqp::value::Value>>>,
}

struct AmqpConnectivity {
    connection: Option<ConnectionHandle<()>>,
    session: Option<SessionHandle<()>>,
}

#[async_trait]
trait SendConnectivity<T>: Send {
    async fn connect(&mut self, parameters: &BusConnectionInformation, duration: Duration) -> Result<()>;
    async fn reconnect(&mut self, parameters: &BusConnectionInformation, duration: Duration) -> Result<()>;
    async fn send(&mut self, message: T) -> Result<()>;
}

impl AmqpConnectivity {
    fn new() -> AmqpConnectivity {
        return AmqpConnectivity {
            session: None,
            connection: None,
        };
    }
}

#[async_trait]
impl SendConnectivity<AmqpMessage<serde_amqp::value::Value>> for AmqpConnectivity {
    async fn connect(&mut self, parameters: &BusConnectionInformation, duration: Duration) -> Result<()> {
        let port = 5671;
        let sa_key_value = create_sas_token(&parameters.hostname, &parameters.shared_access_key_name, &parameters.shared_access_key, &(SystemTime::now() + duration)).unwrap();
        let url = format!("amqps://{}:{}", &parameters.hostname, port);
        let mut connection = match Connection::builder()
            .container_id("rust-receiver-connection-1")
            .alt_tls_establishment(true) // ServiceBus uses alternative TLS establishement
            .hostname(&parameters.hostname[..])
            .sasl_profile(SaslProfile::Plain {
                username: parameters.shared_access_key_name.as_str().to_string(),
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

        let session = match Session::begin(&mut connection).await {
            Ok(s) => s,
            Err(e) => {
                error!("can not build session{:?}",e);
                return Err(Error::from(e));
            }
        };

        self.connection = Some(connection);
        self.session = Some(session);

        Ok(())
    }

    async fn reconnect(&mut self, parameters: &BusConnectionInformation, duration: Duration) -> Result<()> {
        match self.session.as_mut().unwrap().close().await {
            Ok(r) => r,
            Err(err) => {
                trace!("can't close amqp connection: {:?}", err);
                ()
            }
        };
        match self.connection.as_mut().unwrap().close().await {
            Ok(r) => r,
            Err(err) => {
                trace!("can't close amqp connection: {:?}", err);
                ()
            }
        };
        match self.connect(parameters, duration).await {
            Ok(_) => (),
            Err(err) => {
                error!("can't connect: {:?}", err);
                return Err(err);
            }
        };
        Ok(())
    }

    async fn send(&mut self, message: AmqpMessage<serde_amqp::value::Value>) -> Result<()> {
        let mut sender = match Sender::attach(&mut self.session.as_mut().unwrap(), "chirpstack-sender", "/messages/devicebound")
            .await {
            Ok(s) => s,
            Err(e) => {
                error!("can not create sender {:?}",e);
                return Err(Error::new(e));
            }
        };

        let outcome = match sender.send(message).await {
            Ok(m) => m,
            Err(e) => {
                error!("error while sending amqp message {:?}",e);
                return Err(Error::new(e));
            }
        };

        let _ = match outcome.accepted_or_else(|outcome| outcome) {
            Ok(_) => (),
            Err(e) => {
                error!("amqp message not accepted {:?}",e);
                ()
            }
        };

        match sender.close().await {
            Ok(r) => r,
            Err(e) => {
                error!("can't close amqp sender: {:?}", e);
                ()
            }
        };

        Ok(())
    }
}

impl AzureBackend {
    pub async fn new(region_name: &str,
                     region_common_name: CommonName, conf: &GatewayBackendAzure) -> Result<AzureBackend> {
        let bus_information = build_bus_connection_string(conf.commands_connection_string.to_string()).unwrap();
        let (tx, rx): (ChanSender<AmqpMessage<serde_amqp::value::Value>>, ChanRceiver<AmqpMessage<serde_amqp::value::Value>>) = mpsc::sync_channel(1000);
        let azure_backend = AzureBackend {
            tx: Some(tx),
        };
        let connectivity = AmqpConnectivity::new();
        let token = CancellationToken::new();
        tokio::spawn(async move {
            AzureBackend::run_downlink(Box::new(connectivity), bus_information, rx, token).await;
        });


        let event_information = build_event_connection_string(conf.events_connection_string.to_string()).unwrap();
        let region = region_name.to_string();
        tokio::spawn(async move {
            AzureBackend::run_uplink(region, region_common_name, event_information).await;
        });

        Ok(azure_backend)
    }

    async fn run_downlink(mut connectivity: Box<dyn SendConnectivity<AmqpMessage<serde_amqp::value::Value>>>, parameters: BusConnectionInformation, rx: ChanRceiver<AmqpMessage<serde_amqp::value::Value>>, cancel: CancellationToken) {
        loop {
            if cancel.is_cancelled() {
                return;
            }
            trace!("Initiate amqp connection");
            //Let's use the token for a day
            //let sas_token_duration = Duration::from_secs(60 * 60 * 24);
            let sas_token_duration = Duration::from_secs(245);
            // after 4 min of inactivity, the connection is broken
            let event_pubsub_connection_duration = Duration::from_millis(240000);
            let mut start = Instant::now();
            let mut last_receive_message = Instant::now();

            match connectivity.connect(&parameters, sas_token_duration.clone()).await {
                Ok(_) => {}
                Err(e) => {
                    error!("cant connect, let's try later {:?}",e);
                    thread::sleep(Duration::from_secs(10));
                    continue;
                }
            };

            let mut reconnect = false;
            loop {
                if cancel.is_cancelled() {
                    return;
                }

                let check_sas_duration = start.elapsed();
                if check_sas_duration >= (sas_token_duration - event_pubsub_connection_duration) {
                    trace!("sas about to expire, needs reconnection");
                    reconnect = true;
                }

                let check_last_message_duration = last_receive_message.elapsed();
                if check_last_message_duration >= event_pubsub_connection_duration - Duration::from_secs(10) {
                    trace!("no message for the connection duration, needs reconnection");
                    reconnect = true;
                }

                if reconnect {
                    trace!("Reconnect amqp client");
                    match connectivity.reconnect(&parameters, sas_token_duration.clone()).await {
                        Ok(_) => {
                            if cancel.is_cancelled() {
                                return;
                            }
                        }
                        Err(e) => {
                            if cancel.is_cancelled() {
                                return;
                            }
                            error!("cant reconnect, let's try to wait {:?}",e);
                            thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                    };
                    reconnect = false;
                    start = Instant::now();
                    last_receive_message = Instant::now();
                }

                let message = match rx.recv_timeout(Duration::from_secs(10)) {
                    Ok(message) => {
                        if cancel.is_cancelled() {
                            return;
                        }
                        message
                    }
                    Err(_) => {
                        if cancel.is_cancelled() {
                            return;
                        }
                        reconnect = true;
                        trace!("Inactivity occured, set reconnection");
                        continue;
                    }
                };

                match connectivity.send(message).await {
                    Ok(_) => {
                        if cancel.is_cancelled() {
                            return;
                        }
                    }
                    Err(e) => {
                        if cancel.is_cancelled() {
                            return;
                        }
                        reconnect = true;
                        error!("cant send message, reconnect {:?}",e);
                        continue;
                    }
                };
            }
        }
    }


    async fn run_uplink(region: String, region_common_name: CommonName, event_information: EventConnectionInformation) {
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

fn build_event_connection_string(azure_connection_string: String) -> Result<EventConnectionInformation> {
    let mut hostname = "".to_string();
    let mut topic_name = "".to_string();
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
        hub_name: hub_name.unwrap().to_string(),
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
        #[cfg(not(test))]
        info!(gateway_id = %df.gateway_id, topic = %topic, json = json, "Sending downlink frame");
        #[cfg(test)]
        info!("Sending downlink frame");

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
        #[cfg(test)]
        info!("Sending gateway configuration");
        #[cfg(not(test))]
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

impl AzureKafkaClient {
    pub async fn new(region: &str, parameters: &EventConnectionInformation) -> Result<AzureKafkaClient> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", format!("{}.servicebus.windows.net:9093", &parameters.hub_name))
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
use std::{println as trace, println as info, println as error};
#[cfg(test)]
pub mod test {
    use tokio_util::sync::CancellationToken;

    use super::*;

    struct MockAmqpConnectivity {
        mock_connect_fn: fn(parameters: &BusConnectionInformation, duration: Duration) -> Result<()>,
        mock_reconnect_fn: fn(parameters: &BusConnectionInformation, duration: Duration) -> Result<()>,
        mock_send_fn: fn(message: AmqpMessage<serde_amqp::Value>) -> Result<()>,
        call_connect_tx: Option<ChanSender<bool>>,
        call_reconnect_tx: Option<ChanSender<bool>>,
        call_send_tx: Option<ChanSender<bool>>,
    }

    #[async_trait]
    impl SendConnectivity<AmqpMessage<serde_amqp::value::Value>> for MockAmqpConnectivity {
        async fn connect(&mut self, parameters: &BusConnectionInformation, duration: Duration) -> Result<()> {
            if self.call_connect_tx.is_some() {
                self.call_connect_tx.as_ref().unwrap().send(true).expect("TODO: panic message");
            }
            return (self.mock_connect_fn)(parameters, duration);
        }

        async fn reconnect(&mut self, parameters: &BusConnectionInformation, duration: Duration) -> Result<()> {
            if self.call_reconnect_tx.is_some() {
                self.call_reconnect_tx.as_ref().unwrap().send(true).expect("TODO: panic message");
            }
            return (self.mock_reconnect_fn)(parameters, duration);
        }

        async fn send(&mut self, message: AmqpMessage<serde_amqp::Value>) -> Result<()> {
            if self.call_send_tx.is_some() {
                self.call_send_tx.as_ref().unwrap().send(true).expect("TODO: panic message");
            }
            return (self.mock_send_fn)(message);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_downlink_connect() {
        let info = BusConnectionInformation {
            shared_access_key: "a share key".to_string(),
            hostname: "a hostname".to_string(),
            shared_access_key_name: "a share access key name".to_string(),
        };

        let (_tx, rx): (ChanSender<AmqpMessage<serde_amqp::value::Value>>, ChanRceiver<AmqpMessage<serde_amqp::value::Value>>) = mpsc::sync_channel(1);
        let (call_connect_tx, call_connect_rx): (ChanSender<bool>, ChanRceiver<bool>) = mpsc::sync_channel(1);
        let mock_box = Box::new(MockAmqpConnectivity {
            mock_connect_fn: |_info: &BusConnectionInformation, _d: Duration| {
                Ok(())
            },
            mock_reconnect_fn: |_info, _d: Duration| {
                Ok(())
            },
            mock_send_fn: |_message| {
                Ok(())
            },
            call_connect_tx: Some(call_connect_tx),
            call_reconnect_tx: None,
            call_send_tx: None,

        });

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        tokio::spawn(async move {
            AzureBackend::run_downlink(mock_box, info, rx, cloned_token).await
        });
        let result = call_connect_rx.recv_timeout(Duration::from_secs(2));
        token.cancel();

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_downlink_process_message() {
        let (tx, rx): (ChanSender<AmqpMessage<serde_amqp::value::Value>>, ChanRceiver<AmqpMessage<serde_amqp::value::Value>>) = mpsc::sync_channel(1);
        let (call_connect_tx, call_connect_rx): (ChanSender<bool>, ChanRceiver<bool>) = mpsc::sync_channel(1);
        let (call_send_tx, call_send_rx): (ChanSender<bool>, ChanRceiver<bool>) = mpsc::sync_channel(1);
        let mock_box = Box::new(MockAmqpConnectivity {
            mock_connect_fn: |_info: &BusConnectionInformation, _d: Duration| {
                Ok(())
            },
            mock_reconnect_fn: |_info, _d: Duration| {
                Ok(())
            },
            mock_send_fn: |_message| {
                Ok(())
            },
            call_connect_tx: Some(call_connect_tx),
            call_reconnect_tx: None,
            call_send_tx: Some(call_send_tx),

        });
        let info = BusConnectionInformation {
            shared_access_key: "a share key".to_string(),
            hostname: "a hostname".to_string(),
            shared_access_key_name: "a share access key name".to_string(),
        };
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        tokio::spawn(async move {
            AzureBackend::run_downlink(mock_box, info, rx, cloned_token).await
        });
        let result = call_connect_rx.recv_timeout(Duration::from_secs(2));

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), true);


        let handle = tokio::spawn(async move {
            let result = call_send_rx.recv_timeout(Duration::from_secs(20));
            if result.is_err() {
                info!("{:?}",result.err())
            }
            assert_eq!(result.is_err(), false);
            assert_eq!(result.unwrap(), true);
        });

        let message = AmqpMessage::builder().data(Binary::from("test".to_bytes())).build();
        tx.send(message).expect("problem");

        thread::sleep(Duration::from_secs(2));

        handle.abort();
        token.cancel();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_downlink_needs_reconnect() {
        let (tx, rx): (ChanSender<AmqpMessage<serde_amqp::value::Value>>, ChanRceiver<AmqpMessage<serde_amqp::value::Value>>) = mpsc::sync_channel(1);
        let (call_connect_tx, call_connect_rx): (ChanSender<bool>, ChanRceiver<bool>) = mpsc::sync_channel(1);
        let (call_reconnect_tx, call_reconnect_rx): (ChanSender<bool>, ChanRceiver<bool>) = mpsc::sync_channel(1);
        let (call_send_tx, call_send_rx): (ChanSender<bool>, ChanRceiver<bool>) = mpsc::sync_channel(1);
        let mock_box = Box::new(MockAmqpConnectivity {
            mock_connect_fn: |_info: &BusConnectionInformation, _d: Duration| {
                Ok(())
            },
            mock_reconnect_fn: |_info, _d: Duration| {
                Ok(())
            },
            mock_send_fn: |_message| {
                return Err(Error::msg("problem"));
            },
            call_connect_tx: Some(call_connect_tx),
            call_reconnect_tx: Some(call_reconnect_tx),
            call_send_tx: Some(call_send_tx),

        });
        let info = BusConnectionInformation {
            shared_access_key: "a share key".to_string(),
            hostname: "a hostname".to_string(),
            shared_access_key_name: "a share access key name".to_string(),
        };
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        tokio::spawn(async move {
            AzureBackend::run_downlink(mock_box, info, rx, cloned_token).await
        });
        let result = call_connect_rx.recv_timeout(Duration::from_secs(2));

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), true);


        let handle_send = tokio::spawn(async move {
            let result = call_send_rx.recv_timeout(Duration::from_secs(20));
            if result.is_err() {
                info!("{:?}",result.err())
            }
            assert_eq!(result.is_err(), false);
            assert_eq!(result.unwrap(), true);
        });

        let handle_reconnect = tokio::spawn(async move {
            let result = call_reconnect_rx.recv_timeout(Duration::from_secs(20));
            if result.is_err() {
                info!("{:?}",result.err())
            }
            assert_eq!(result.is_err(), false);
            assert_eq!(result.unwrap(), true);
        });

        let message = AmqpMessage::builder().data(Binary::from("test".to_bytes())).build();
        tx.send(message).expect("problem");

        thread::sleep(Duration::from_secs(2));

        handle_send.abort();
        handle_reconnect.abort();
        token.cancel();
    }

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

    #[test]
    fn test_build_bus_connection_string() {
        let connection_information = build_bus_connection_string("HostName=an-iot-hub.azure-devices.net;SharedAccessKeyName=service;SharedAccessKey=yuui58399mfCvhRvmowieujbsdsd".to_string()).unwrap();
        assert_eq!(connection_information.hostname, "an-iot-hub.azure-devices.net");
        assert_eq!(connection_information.shared_access_key_name, "service@sas.root.an-iot-hub");
        assert_eq!(connection_information.shared_access_key, "yuui58399mfCvhRvmowieujbsdsd");
    }

    #[test]
    fn test_build_event_connection_string() {
        let connection_information = build_event_connection_string("Endpoint=sb://an-event-hub.servicebus.windows.net/;SharedAccessKeyName=a_key;SharedAccessKey=uuuuuuuuuuuuuuu;EntityPath=an_entity".to_string()).unwrap();
        assert_eq!(connection_information.hub_name, "an-event-hub");
        assert_eq!(connection_information.connection_string, "Endpoint=sb://an-event-hub.servicebus.windows.net/;SharedAccessKeyName=a_key;SharedAccessKey=uuuuuuuuuuuuuuu;EntityPath=an_entity");
        assert_eq!(connection_information.topic_name, "an_entity");
    }
}