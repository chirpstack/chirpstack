use std::collections::HashMap;
use std::io::Cursor;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::StreamExt;
use handlebars::Handlebars;
use paho_mqtt as mqtt;
use prost::Message;
use regex::Regex;
use serde::Serialize;
use tracing::{error, info};

use super::Integration as IntegrationTrait;
use crate::config::MqttIntegration as Config;
use chirpstack_api::integration;

pub struct Integration<'a> {
    client: mqtt::AsyncClient,
    templates: Handlebars<'a>,
    json: bool,
    qos: usize,
    command_regex: Regex,
}

#[derive(Serialize)]
struct EventTopicContext {
    pub application_id: String,
    pub dev_eui: String,
    pub event: String,
}

#[derive(Serialize)]
struct StateTopicContext {
    pub application_id: String,
    pub dev_eui: String,
    pub state: String,
}

#[derive(Serialize)]
struct CommandTopicContext {
    pub application_id: String,
    pub dev_eui: String,
    pub command: String,
}

impl<'a> Integration<'a> {
    pub async fn new(conf: &Config) -> Result<Integration<'a>> {
        info!("Initializing MQTT integration");

        // topic templates
        let mut templates = Handlebars::new();
        templates.register_escape_fn(handlebars::no_escape);
        templates.register_template_string("event_topic", &conf.event_topic)?;
        templates.register_template_string("state_topic", &conf.state_topic)?;
        templates.register_template_string("command_topic", &conf.command_topic)?;

        // create client
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&conf.server)
            .client_id(&conf.client_id)
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
                ca_cert = %conf.ca_cert,
                tls_cert = %conf.tls_cert,
                tls_key = %conf.tls_key,
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

        let i = Integration {
            command_regex: Regex::new(&templates.render(
                "command_topic",
                &CommandTopicContext {
                    application_id: r#"(?P<application_id>[\w-]+)"#.to_string(),
                    dev_eui: r#"(?P<dev_eui>[\w]+)"#.to_string(),
                    command: r#"(?P<command>[\w]+)"#.to_string(),
                },
            )?)?,
            qos: conf.qos,
            json: conf.json,
            client,
            templates,
        };

        // connect
        info!(server_uri = %conf.server, "Connecting to MQTT broker");
        i.client
            .connect(conn_opts)
            .await
            .context("Connect to MQTT broker")?;

        let command_topic = i.templates.render(
            "command_topic",
            &CommandTopicContext {
                application_id: "+".into(),
                dev_eui: "+".into(),
                command: "+".into(),
            },
        )?;
        info!(
            command_topic = %command_topic,
            "Subscribing to command topic"
        );
        i.client
            .subscribe(&command_topic, conf.qos as i32)
            .await
            .context("MQTT subscribe")?;

        tokio::spawn({
            let command_regex = i.command_regex.clone();

            async move {
                info!("Starting MQTT consumer loop");
                while let Some(msg_opt) = stream.next().await {
                    if let Some(msg) = msg_opt {
                        let caps = match command_regex.captures(&msg.topic()) {
                            Some(v) => v,
                            None => {
                                error!(topic = %msg.topic(), "Error parsing command topic (regex captures returned None)");
                                continue;
                            }
                        };
                        if caps.len() != 4 {
                            error!(topic = %msg.topic(), "Parsing command topic returned invalid match count");
                            continue;
                        }

                        message_callback(
                            caps.get(1).map_or("", |m| m.as_str()).to_string(),
                            caps.get(2).map_or("", |m| m.as_str()).to_string(),
                            caps.get(3).map_or("", |m| m.as_str()).to_string(),
                            i.json,
                            msg,
                        )
                        .await;
                    }
                }
            }
        });

        // Return integration.
        Ok(i)
    }

    fn get_event_topic(&self, application_id: &str, dev_eui: &str, event: &str) -> Result<String> {
        Ok(self.templates.render(
            "event_topic",
            &EventTopicContext {
                application_id: application_id.to_string(),
                dev_eui: dev_eui.to_string(),
                event: event.to_string(),
            },
        )?)
    }

    fn get_state_topic(&self, application_id: &str, dev_eui: &str, state: &str) -> Result<String> {
        Ok(self.templates.render(
            "state_topic",
            &StateTopicContext {
                application_id: application_id.to_string(),
                dev_eui: dev_eui.to_string(),
                state: state.to_string(),
            },
        )?)
    }

    async fn publish_event(&self, topic: &str, b: &[u8]) -> Result<()> {
        info!(topic = %topic, "Publishing event");
        let msg = mqtt::Message::new(topic, b, self.qos as i32);
        self.client.publish(msg).await?;
        Ok(())
    }

    async fn publish_state(&self, topic: &str, b: &[u8]) -> Result<()> {
        info!(topic = %topic, "Publishing state");
        let msg = mqtt::Message::new_retained(topic, b, self.qos as i32);
        self.client.publish(msg).await?;
        Ok(())
    }
}

#[async_trait]
impl IntegrationTrait for Integration<'_> {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "up")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, &b).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic = self.get_state_topic(&dev_info.application_id, &dev_info.dev_eui, "join")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_state(&topic, &b).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "ack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, &b).await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "txack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, &b).await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "log")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, &b).await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic = self.get_state_topic(&dev_info.application_id, &dev_info.dev_eui, "status")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_state(&topic, &b).await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic =
            self.get_state_topic(&dev_info.application_id, &dev_info.dev_eui, "location")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_state(&topic, &b).await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or(anyhow!("device_info is None"))?;

        let topic =
            self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "integration")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, &b).await
    }
}

fn connected_callback(_: &mqtt::AsyncClient) {
    info!("Connected to MQTT broker");
}

fn connection_lost_callback(_: &mqtt::AsyncClient) {
    info!("Connection to MQTT broker lost");
}

async fn message_callback(
    application_id: String,
    dev_eui: String,
    command: String,
    json: bool,
    msg: mqtt::Message,
) {
    let topic = msg.topic();
    let qos = msg.qos();
    let b = msg.payload();

    info!(topic = topic, qos = qos, "Command received for device");

    let err = || -> Result<()> {
        match command.as_ref() {
            "down" => {
                let cmd: integration::DownlinkCommand = match json {
                    true => serde_json::from_slice(&b)?,
                    false => integration::DownlinkCommand::decode(&mut Cursor::new(b))?,
                };
                if dev_eui != cmd.dev_eui {
                    return Err(anyhow!(
                        "Payload dev_eui {} does not match topic dev_eui {}",
                        cmd.dev_eui,
                        dev_eui
                    ));
                }
                tokio::spawn(super::handle_down_command(application_id, cmd));
            }
            _ => {
                return Err(anyhow!("Unknown command type"));
            }
        }

        Ok(())
    }()
    .err();

    if err.is_some() {
        error!(
            topic = topic,
            qos = qos,
            "Processing command error: {}",
            err.as_ref().unwrap()
        );
    }
}

#[cfg(test)]
pub mod test {

    use super::*;
    use crate::config::MqttIntegration;
    use crate::storage::{application, device, device_profile, device_queue, tenant};
    use crate::test;
    use futures::stream::StreamExt;
    use lrwn::EUI64;
    use paho_mqtt as mqtt;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mqtt() {
        // to avoid race-conditions with other tests using MQTT
        let _guard = test::prepare().await;

        // setup base objects
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();
        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();
        let dev = device::create(device::Device {
            name: "test-device".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // setup of integration and MQTT client
        let conf = MqttIntegration {
            event_topic: "application/{{application_id}}/device/{{dev_eui}}/event/{{event}}".into(),
            state_topic: "application/{{application_id}}/device/{{dev_eui}}/state/{{state}}".into(),
            json: true,
            server: "tcp://mosquitto:1883/".into(),
            clean_session: true,
            ..Default::default()
        };
        let i = Integration::new(&conf).await.unwrap();

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&conf.server)
            .finalize();
        let mut client = mqtt::AsyncClient::new(create_opts).unwrap();
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .clean_session(true)
            .finalize();
        let mut stream = client.get_stream(10);
        client.connect(conn_opts).await.unwrap();

        // remove retained messages by sending empty payloads
        client.publish(mqtt::Message::new_retained("application/00000000-0000-0000-0000-000000000000/device/0102030405060708/state/join", vec![], mqtt::QOS_0)).await.unwrap();
        client.publish(mqtt::Message::new_retained("application/00000000-0000-0000-0000-000000000000/device/0102030405060708/state/status", vec![], mqtt::QOS_0)).await.unwrap();
        client.publish(mqtt::Message::new_retained("application/00000000-0000-0000-0000-000000000000/device/0102030405060708/state/location", vec![], mqtt::QOS_0)).await.unwrap();

        client
            .subscribe(
                "application/00000000-0000-0000-0000-000000000000/device/+/event/+",
                mqtt::QOS_0,
            )
            .await
            .unwrap();
        client
            .subscribe(
                "application/00000000-0000-0000-0000-000000000000/device/+/state/+",
                mqtt::QOS_0,
            )
            .await
            .unwrap();

        // uplink event
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/up",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // join event
        let pl = integration::JoinEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.join_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/state/join",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // ack event
        let pl = integration::AckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.ack_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/ack",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // txack event
        let pl = integration::TxAckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.txack_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/txack",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // log event
        let pl = integration::LogEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.log_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/log",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // status event
        let pl = integration::StatusEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.status_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/state/status",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // location event
        let pl = integration::LocationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.location_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/state/location",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // integration event
        let pl = integration::IntegrationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.integration_event(&HashMap::new(), &pl).await.unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/integration",
            msg.topic()
        );
        assert_eq!(serde_json::to_string(&pl).unwrap(), msg.payload_str());

        // downlink command
        let down_cmd = integration::DownlinkCommand {
            id: Uuid::new_v4().to_string(),
            dev_eui: dev.dev_eui.to_string(),
            confirmed: false,
            f_port: 10,
            data: vec![1, 2, 3],
            object: None,
        };
        let down_cmd_json = serde_json::to_string(&down_cmd).unwrap();
        client
            .publish(mqtt::Message::new(
                format!("application/{}/device/{}/command/down", app.id, dev.dev_eui),
                down_cmd_json,
                mqtt::QOS_0,
            ))
            .await
            .unwrap();

        // give the async consumer some time to process
        sleep(Duration::from_millis(200)).await;

        let queue_items = device_queue::get_for_dev_eui(&dev.dev_eui).await.unwrap();
        assert_eq!(1, queue_items.len());
        assert_eq!(down_cmd.id, queue_items[0].id.to_string());
        assert_eq!(dev.dev_eui, queue_items[0].dev_eui);
        assert_eq!(10, queue_items[0].f_port);
        assert_eq!(vec![1, 2, 3], queue_items[0].data);
    }
}
