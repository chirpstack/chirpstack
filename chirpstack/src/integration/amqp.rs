use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use handlebars::Handlebars;
use lapin::{
    options::BasicPublishOptions, BasicProperties, Channel, Connection, ConnectionProperties,
};
use prost::Message;
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::{error, info};

use super::Integration as IntegrationTrait;
use crate::config::AmqpIntegration as Config;
use chirpstack_api::integration;

// We define the connection and channel outside the Integration struct as the AMQP client does not
// implement re-connect on error. To reconnect within the Integration struct would require
// mutability of the Integration struct, which is not possible without changing the
// IntegrationTrait as we would need to change the (&self, ...) signatures to (&mut self, ...).
lazy_static! {
    static ref CONNECTION: RwLock<Option<Connection>> = RwLock::new(None);
    static ref CHANNEL: RwLock<Option<Channel>> = RwLock::new(None);
}

pub struct Integration<'a> {
    templates: Handlebars<'a>,
    json: bool,
    url: String,
}

#[derive(Serialize)]
struct EventRoutingKeyContext {
    pub application_id: String,
    pub dev_eui: String,
    pub event: String,
}

impl<'a> Integration<'a> {
    pub async fn new(conf: &Config) -> Result<Integration<'a>> {
        info!("Initializing AMQP integration");

        // routing-key template
        let mut templates = Handlebars::new();
        templates.register_escape_fn(handlebars::no_escape);
        templates.register_template_string("event_routing_key", &conf.event_routing_key)?;

        let i = Integration {
            templates,
            url: conf.url.clone(),
            json: conf.json,
        };
        i.connect().await?;

        Ok(i)
    }

    async fn connect(&self) -> Result<()> {
        info!("(Re)connecting to AMQP broker");

        let mut conn_w = CONNECTION.write().await;
        let mut chan_w = CHANNEL.write().await;

        let options = ConnectionProperties::default()
            // Use tokio executor and reactor.
            // At the moment the reactor is only available for unix.
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        let conn = Connection::connect(&self.url, options).await?;
        let chan = conn.create_channel().await?;

        *conn_w = Some(conn);
        *chan_w = Some(chan);

        Ok(())
    }

    async fn publish_event(&self, routing_key: String, b: &[u8]) -> Result<()> {
        info!(routing_key = %routing_key, "Publishing event");

        // The publishing code is scoped, to make sure that when the scope returns, the channel
        // mutex has been released. This is important since in case of an error, we will attempt to
        // reconnect. Trying to aquire a write lock on the mutex while still having a read lock
        // would cause a deadlock.
        let res = {
            let chan_r = CHANNEL.read().await;

            match chan_r
                .as_ref()
                .unwrap()
                .basic_publish(
                    "amq.topic",
                    &routing_key,
                    BasicPublishOptions::default(),
                    b,
                    BasicProperties::default().with_content_type(match self.json {
                        true => "application/json".into(),
                        false => "application/octet-stream".into(),
                    }),
                )
                .await
            {
                Ok(v) => v.await,
                Err(e) => Err(e),
            }
        };
        if let Err(e) = res {
            error!(error = %e, "Publishing event error");
            self.connect().await?;
            return Err(anyhow::Error::new(e));
        }

        Ok(())
    }

    fn get_routing_key(&self, application_id: &str, dev_eui: &str, event: &str) -> Result<String> {
        Ok(self.templates.render(
            "event_routing_key",
            &EventRoutingKeyContext {
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
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "up")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "join")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "ack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "txack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "log")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "status")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "location")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let key = self.get_routing_key(&di.application_id, &di.dev_eui, "integration")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(key, &b).await
    }
}

#[cfg(all(test, feature = "test-integration-amqp"))]
pub mod test {
    use std::env;

    use super::*;
    use futures::stream::StreamExt;
    use lapin::options::{
        BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions,
    };
    use lapin::types::FieldTable;
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_amqp() {
        dotenv::dotenv().ok();
        dotenv::from_filename(".env.local").ok();

        let conf = Config {
            url: env::var("TEST_AMQP_URL").unwrap(),
            json: true,
            event_routing_key: "application.{{application_id}}.device.{{dev_eui}}.event.{{event}}"
                .to_string(),
        };

        let conn = loop {
            match Connection::connect(
                &conf.url,
                ConnectionProperties::default()
                    .with_executor(tokio_executor_trait::Tokio::current())
                    .with_reactor(tokio_reactor_trait::Tokio),
            )
            .await
            {
                Ok(v) => {
                    break v;
                }
                Err(e) => {
                    println!("AMQP connect error: {:?}", e);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        };

        let chan = conn.create_channel().await.unwrap();
        let _queue = chan
            .queue_declare(
                "test-queue",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        chan.queue_bind(
            "test-queue",
            "amq.topic",
            "*.*.*.*.*.*",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

        let mut consumer = chan
            .basic_consume(
                "test-queue",
                "test-consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        let i = Integration::new(&conf).await.unwrap();

        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.uplink_event(&HashMap::new(), &pl).await.unwrap();

        let delivery = consumer.next().await.unwrap().unwrap();
        delivery.ack(BasicAckOptions::default()).await.unwrap();

        assert_eq!(
            "application.00000000-0000-0000-0000-000000000000.device.0102030405060708.event.up",
            delivery.routing_key.to_string()
        );
        assert_eq!(serde_json::to_vec(&pl).unwrap(), delivery.data);
    }
}
