use std::io::Cursor;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::StreamExt;
use handlebars::Handlebars;
use paho_mqtt as mqtt;
use prost::Message;
use serde::Serialize;
use tracing::{error, info, trace};

use crate::config::GatewayBackendMqtt;
use crate::{downlink, uplink};
use lrwn::region::CommonName;
use lrwn::EUI64;

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

        // create client
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&conf.server)
            .client_id(&conf.client_id)
            .user_data(Box::new(MqttContext {
                region_name: region_name.to_string(),
                region_common_name,
            }))
            .finalize();
        let mut client = mqtt::AsyncClient::new(create_opts).context("Create MQTT client")?;
        client.set_connected_callback(connected_callback);
        client.set_connection_lost_callback(connection_lost_callback);

        // connection options
        let mut conn_opts_b = mqtt::ConnectOptionsBuilder::new();
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
        info!(
            server_uri = conf.server.as_str(),
            "Connecting to MQTT broker"
        );
        b.client
            .connect(conn_opts)
            .await
            .context("Connect to MQTT broker")?;

        info!(
            event_topic = conf.event_topic.as_str(),
            "Subscribing to gateway event topic"
        );
        b.client
            .subscribe(&conf.event_topic, conf.qos as i32)
            .await
            .context("MQTT subscribe error")?;

        tokio::spawn({
            let region_name = region_name.to_string();

            async move {
                info!("Starting MQTT consumer loop");
                while let Some(msg_opt) = stream.next().await {
                    if let Some(msg) = msg_opt {
                        message_callback(&region_name, region_common_name, msg).await;
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
        let gateway_id = EUI64::from_slice(&df.gateway_id)?;
        let topic = self.get_command_topic(&gateway_id.to_string(), "down")?;
        let b = df.encode_to_vec();

        info!(gateway_id = %gateway_id, topic = %topic, "Sending downlink frame");
        let msg = mqtt::Message::new(topic, b, self.qos as i32);
        self.client.publish(msg).await?;
        trace!("Message sent");

        Ok(())
    }

    async fn send_configuration(
        &self,
        gw_conf: &chirpstack_api::gw::GatewayConfiguration,
    ) -> Result<()> {
        let gateway_id = EUI64::from_slice(&gw_conf.gateway_id)?;
        let topic = self.get_command_topic(&gateway_id.to_string(), "config")?;
        let b = gw_conf.encode_to_vec();

        info!(gateway_id = %gateway_id, topic = %topic, "Sending gateway configuration");
        let msg = mqtt::Message::new(topic, b, self.qos as i32);
        self.client.publish(msg).await?;
        trace!("Message sent");

        Ok(())
    }
}

async fn message_callback(region_name: &str, region_common_name: CommonName, msg: mqtt::Message) {
    let topic = msg.topic();
    let qos = msg.qos();
    let b = msg.payload();

    info!(
        region_name = region_name,
        topic = topic,
        qos = qos,
        "Message received from gateway"
    );

    let err = || -> Result<()> {
        if topic.ends_with("/up") {
            let mut event = chirpstack_api::gw::UplinkFrame::decode(&mut Cursor::new(b))?;
            if let Some(rx_info) = &mut event.rx_info {
                rx_info
                    .metadata
                    .insert("region_name".to_string(), region_name.to_string());
                rx_info.metadata.insert(
                    "region_common_name".to_string(),
                    region_common_name.to_string(),
                );
            }

            tokio::spawn(uplink::deduplicate_uplink(event));
        } else if topic.ends_with("/stats") {
            let mut event = chirpstack_api::gw::GatewayStats::decode(&mut Cursor::new(b))?;
            event
                .meta_data
                .insert("region_name".to_string(), region_name.to_string());
            event.meta_data.insert(
                "region_common_name".to_string(),
                region_common_name.to_string(),
            );
            tokio::spawn(uplink::stats::Stats::handle(event));
        } else if topic.ends_with("/ack") {
            let event = chirpstack_api::gw::DownlinkTxAck::decode(&mut Cursor::new(b))?;
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
            qos = qos,
            "Processing gateway event error: {}",
            err.as_ref().unwrap()
        );
    }
}

fn connected_callback(client: &mqtt::AsyncClient) {
    let ctx = client
        .user_data()
        .unwrap()
        .downcast_ref::<MqttContext>()
        .unwrap();

    info!(
        region_name = ctx.region_name.as_str(),
        "Connected to MQTT broker"
    );
}

fn connection_lost_callback(client: &mqtt::AsyncClient) {
    let ctx = client
        .user_data()
        .unwrap()
        .downcast_ref::<MqttContext>()
        .unwrap();

    info!(
        region_name = ctx.region_name.as_str(),
        "MQTT connection to broker lost"
    );
}
