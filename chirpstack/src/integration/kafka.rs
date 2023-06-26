use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use handlebars::Handlebars;
use prost::Message;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use tracing::{error, info};

use super::Integration as IntegrationTrait;
use crate::config::KafkaIntegration as Config;
use chirpstack_api::integration;

pub struct Integration<'a> {
    templates: Handlebars<'a>,
    topic: String,
    json: bool,
    producer: FutureProducer,
}

#[derive(Serialize)]
struct EventKeyContext {
    pub application_id: String,
    pub dev_eui: String,
    pub event: String,
}

impl<'a> Integration<'a> {
    pub fn new(conf: &Config) -> Result<Integration<'a>> {
        info!("Initializing Kafka integration");

        // event-key template.
        let mut templates = Handlebars::new();
        templates.register_escape_fn(handlebars::no_escape);
        templates.register_template_string("event_key", &conf.event_key)?;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &conf.brokers.join(","))
            .set("message.timeout.ms", "5000")
            .set("allow.auto.create.topics", "true")
            .set(
                "sasl.mechanism",
                match conf.mechanism.as_ref() {
                    "PLAIN" => "PLAIN",
                    "SCRAM-SHA-256" => "SCRAM-SHA-256",
                    "SCRAM-SHA-512" => "SCRAM-SHA-512",
                    _ => {
                        return Err(anyhow!(
                            "mechanism must be PLAIN, SCRAM-SHA-256 or SCRAM-SHA-512"
                        ));
                    }
                },
            )
            .set("sasl.username", &conf.username)
            .set("sasl.password", &conf.password)
            .create()?;

        let i = Integration {
            templates,
            producer,
            json: conf.json,
            topic: conf.topic.clone(),
        };

        Ok(i)
    }

    async fn publish_event(&self, event: &str, event_key: String, b: &[u8]) -> Result<()> {
        info!(topic = %self.topic, event_key = %event_key, "Publishing event");

        let res = self
            .producer
            .send(
                FutureRecord::to(&self.topic)
                    .key(&event_key)
                    .headers(OwnedHeaders::new().insert(Header {
                        key: "event",
                        value: Some(event),
                    }))
                    .payload(b),
                Duration::from_secs(0),
            )
            .await;

        if let Err(e) = res {
            error!(error = ?e, "Publishing event error");
            return Err(anyhow!("{:?}", e));
        }

        Ok(())
    }

    fn get_event_key(&self, application_id: &str, dev_eui: &str, event: &str) -> Result<String> {
        Ok(self.templates.render(
            "event_key",
            &EventKeyContext {
                application_id: application_id.to_string(),
                dev_eui: dev_eui.to_string(),
                event: event.to_string(),
            },
        )?)
    }
}

#[async_trait]
impl<'a> IntegrationTrait for Integration<'a> {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "up")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("up", key, &b).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "join")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("join", key, &b).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "ack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("ack", key, &b).await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "txack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("txack", key, &b).await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "log")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("log", key, &b).await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "status")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("status", key, &b).await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "location")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("location", key, &b).await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_event_key(&di.application_id, &di.dev_eui, "integration")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event("integration", key, &b).await
    }
}

#[cfg(all(test, feature = "test-integration-kafka"))]
pub mod test {
    use std::env;

    use super::*;
    use crate::test;
    use rdkafka::consumer::stream_consumer::StreamConsumer;
    use rdkafka::consumer::Consumer;
    use rdkafka::message::Headers;
    use rdkafka::Message;
    use std::time::Duration;
    use tokio::time::sleep;
    use tracing::trace;

    use uuid::Uuid;

    #[tokio::test]
    async fn test_kafka() {
        let _guard = test::prepare().await;

        dotenv::dotenv().ok();
        dotenv::from_filename(".env.local").ok();

        let conf = Config {
            brokers: vec![env::var("TEST_KAFKA_BROKER").unwrap()],
            topic: "chirpstack".to_string(),
            json: true,
            ..Default::default()
        };

        let consumer: StreamConsumer = loop {
            match ClientConfig::new()
                .set("group.id", "testgroup")
                .set("bootstrap.servers", env::var("TEST_KAFKA_BROKER").unwrap())
                .set("allow.auto.create.topics", "true")
                .set("auto.offset.reset", "beginning")
                .create()
            {
                Ok(v) => {
                    break v;
                }
                Err(e) => {
                    error!("Kafka connect error: {:?}", e);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        };
        trace!("Consumer created");

        consumer.subscribe(&["chirpstack"]).unwrap();
        trace!("Subscription created");

        let i = Integration::new(&conf).unwrap();
        trace!("Integration created");

        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        trace!("Event published");

        let msg = consumer.recv().await.unwrap();
        trace!("Event received");

        assert_eq!(
            "application.00000000-0000-0000-0000-000000000000.device.0102030405060708.event.up"
                .as_bytes(),
            msg.key().unwrap()
        );
        assert_eq!(serde_json::to_vec(&pl).unwrap(), msg.payload().unwrap());
        assert_eq!(
            Header {
                key: "event",
                value: Some("up".as_bytes()),
            },
            msg.headers().unwrap().get(0)
        );
    }
}
