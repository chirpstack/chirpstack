use std::env::temp_dir;
use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::StreamExt;
use handlebars::Handlebars;
use paho_mqtt as mqtt;
use prost::Message;
use rand::Rng;
use serde::Serialize;
use tokio::sync::mpsc;
use tracing::{error, info, trace};

use lrwn::region::CommonName;

use crate::config::GatewayBackendMqtt;

use super::common::COMMAND_COUNTER;
use super::common::CommandLabels;
use super::common::gateway_is_json;
use super::common::message_callback;
use super::GatewayBackend;

struct MqttContext {
    region_name: String,
    region_common_name: CommonName,
}

pub struct MqttBackend<'a> {
    client: mqtt::AsyncClient,
    templates: handlebars::Handlebars<'a>,
    qos: usize,
}

#[derive(Serialize)]
struct CommandTopicContext {
    pub gateway_id: String,
    pub command: String,
}

impl<'a> MqttBackend<'a> {
    pub async fn new(
        region_name: &str,
        region_common_name: CommonName,
        conf: &GatewayBackendMqtt,
    ) -> Result<MqttBackend<'a>> {
        // topic templates
        let mut templates = Handlebars::new();
        templates.register_template_string("command_topic", &conf.command_topic)?;

        // get client id, this will generate a random client_id when no client_id has been
        // configured.
        let client_id = if conf.client_id.is_empty() {
            let mut rnd = rand::thread_rng();
            let client_id: u64 = rnd.gen();
            format!("{:x}", client_id)
        } else {
            conf.client_id.clone()
        };

        // Create subscribe channel
        // This is needed as we can't subscribe within the set_connected_callback as this would
        // block the callback (we want to wait for success or error), which would create a
        // deadlock. We need to re-subscribe on (re)connect to be sure we have a subscription. Even
        // in case of a persistent MQTT session, there is no guarantee that the MQTT persisted the
        // session and that a re-connect would recover the subscription.
        let (subscribe_tx, mut subscribe_rx) = mpsc::channel(10);

        // create client
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&conf.server)
            .client_id(&client_id)
            .user_data(Box::new(MqttContext {
                region_name: region_name.to_string(),
                region_common_name,
            }))
            .persistence(mqtt::create_options::PersistenceType::FilePath(temp_dir()))
            .finalize();
        let mut client = mqtt::AsyncClient::new(create_opts).context("Create MQTT client")?;
        client.set_connected_callback(move |client| {
            let ctx = client
                .user_data()
                .unwrap()
                .downcast_ref::<MqttContext>()
                .unwrap();

            info!(region_name = %ctx.region_name, "Connected to MQTT broker");

            if let Err(e) = subscribe_tx.try_send(()) {
                error!(region_name = %ctx.region_name, error = %e, "Send to subscribe channel error");
            }
        });
        client.set_connection_lost_callback(|client| {
            let ctx = client
                .user_data()
                .unwrap()
                .downcast_ref::<MqttContext>()
                .unwrap();

            info!(region_name = %ctx.region_name, "MQTT connection to broker lost");
        });

        // connection options
        let mut conn_opts_b = mqtt::ConnectOptionsBuilder::new();
        conn_opts_b.automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30));
        conn_opts_b.clean_session(conf.clean_session);
        conn_opts_b.user_name(&conf.username);
        conn_opts_b.password(&conf.password);
        if !conf.ca_cert.is_empty() || !conf.tls_cert.is_empty() || !conf.tls_key.is_empty() {
            info!(
                ca_cert = conf.ca_cert.as_str(),
                tls_cert = conf.tls_cert.as_str(),
                tls_key = conf.tls_key.as_str(),
                "Configuring connection with TLS certificate"
            );

            let mut ssl_opts_b = mqtt::SslOptionsBuilder::new();

            if !conf.ca_cert.is_empty() {
                ssl_opts_b
                    .ca_path(&conf.ca_cert)
                    .context("Failed to set gateway ca_cert")?;
            }

            if !conf.tls_cert.is_empty() {
                ssl_opts_b
                    .key_store(&conf.tls_cert)
                    .context("Failed to set gateway tls_cert")?;
            }

            if !conf.tls_key.is_empty() {
                ssl_opts_b
                    .private_key(&conf.tls_key)
                    .context("Failed to set gateway tls_key")?;
            }

            conn_opts_b.ssl_options(ssl_opts_b.finalize());
        }
        let conn_opts = conn_opts_b.finalize();

        // get message stream
        let mut stream = client.get_stream(25);

        let b = MqttBackend {
            client,
            templates,
            qos: conf.qos,
        };

        // connect
        info!(region_name = %region_name, server_uri = %conf.server, clean_session = conf.clean_session, client_id = %client_id, "Connecting to MQTT broker");
        b.client
            .connect(conn_opts)
            .await
            .context("Connect to MQTT broker")?;

        // Consumer loop.
        tokio::spawn({
            let region_name = region_name.to_string();

            async move {
                info!("Starting MQTT consumer loop");
                while let Some(msg_opt) = stream.next().await {
                    if let Some(msg) = msg_opt {
                        let mqtt_topic = msg.topic();
                        let topic: &str;
                        let qos = msg.qos();
                        if mqtt_topic.ends_with("/up") {
                            topic = "up";
                        } else if mqtt_topic.ends_with("/ack") {
                            topic = "ack";
                        } else if mqtt_topic.ends_with("/stats") {
                            topic = "stats";
                        } else {
                            topic = "";
                        }
                        trace!("mqtt_topic {}, region {}, qos {}", mqtt_topic,region_name,qos);
                        message_callback(region_common_name, msg.payload(), &region_name, &topic).await;
                    }
                }
            }
        });

        // (Re)subscribe loop.
        tokio::spawn({
            let region_name = region_name.to_string();
            let event_topic = conf.event_topic.clone();
            let client = b.client.clone();
            let qos = conf.qos as i32;

            async move {
                while subscribe_rx.recv().await.is_some() {
                    info!(region_name = %region_name, event_topic = %event_topic, "Subscribing to gateway event topic");
                    if let Err(e) = client.subscribe(&event_topic, qos).await {
                        error!(region_name = %region_name, event_topic = %event_topic, error = %e, "MQTT subscribe error");
                    }
                }
            }
        });

        // return backend
        Ok(b)
    }

    fn get_command_topic(&self, gateway_id: &str, command: &str) -> Result<String> {
        Ok(self.templates.render(
            "command_topic",
            &CommandTopicContext {
                gateway_id: gateway_id.to_string(),
                command: command.to_string(),
            },
        )?)
    }
}

#[async_trait]
impl GatewayBackend for MqttBackend<'_> {
    async fn send_downlink(&self, df: &chirpstack_api::gw::DownlinkFrame) -> Result<()> {
        COMMAND_COUNTER
            .get_or_create(&CommandLabels {
                command: "down".to_string(),
            })
            .inc();
        let topic = self.get_command_topic(&df.gateway_id, "down")?;
        let mut df = df.clone();
        df.v4_migrate();

        let json = gateway_is_json(&df.gateway_id);
        let b = match json {
            true => serde_json::to_vec(&df)?,
            false => df.encode_to_vec(),
        };

        info!(gateway_id = %df.gateway_id, topic = %topic, json = json, "Sending downlink frame");
        let msg = mqtt::Message::new(topic, b, self.qos as i32);
        self.client.publish(msg).await?;
        trace!("Message sent");

        Ok(())
    }

    async fn send_configuration(
        &self,
        gw_conf: &chirpstack_api::gw::GatewayConfiguration,
    ) -> Result<()> {
        COMMAND_COUNTER
            .get_or_create(&CommandLabels {
                command: "config".to_string(),
            })
            .inc();
        let topic = self.get_command_topic(&gw_conf.gateway_id, "config")?;
        let json = gateway_is_json(&gw_conf.gateway_id);
        let b = match json {
            true => serde_json::to_vec(&gw_conf)?,
            false => gw_conf.encode_to_vec(),
        };

        info!(gateway_id = %gw_conf.gateway_id, topic = %topic, json = json, "Sending gateway configuration");
        let msg = mqtt::Message::new(topic, b, self.qos as i32);
        self.client.publish(msg).await?;
        trace!("Message sent");

        Ok(())
    }
}