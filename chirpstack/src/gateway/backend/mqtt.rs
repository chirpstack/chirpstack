use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::env::temp_dir;
use std::hash::Hasher;
use std::io::Cursor;
use std::sync::RwLock;
use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::StreamExt;
use handlebars::Handlebars;
use paho_mqtt as mqtt;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prost::Message;
use rand::Rng;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{error, info, trace};

use super::GatewayBackend;
use crate::config::GatewayBackendMqtt;
use crate::monitoring::prometheus;
use crate::storage::{get_redis_conn, redis_key};
use crate::{downlink, uplink};
use lrwn::region::CommonName;

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
struct EventLabels {
    event: String,
}

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
struct CommandLabels {
    command: String,
}

lazy_static! {
    static ref EVENT_COUNTER: Family<EventLabels, Counter> = {
        let counter = Family::<EventLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_events",
            "Number of events received",
            counter.clone(),
        );
        counter
    };
    static ref COMMAND_COUNTER: Family<CommandLabels, Counter> = {
        let counter = Family::<CommandLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_commands",
            "Number of commands sent",
            counter.clone(),
        );
        counter
    };
    static ref GATEWAY_JSON: RwLock<HashMap<String, bool>> = RwLock::new(HashMap::new());
}

struct MqttContext {
    region_config_id: String,
}

pub struct MqttBackend<'a> {
    client: mqtt::AsyncClient,
    templates: handlebars::Handlebars<'a>,
    qos: usize,
    v4_migrate: bool,
}

#[derive(Serialize)]
struct CommandTopicContext {
    pub gateway_id: String,
    pub command: String,
}

impl<'a> MqttBackend<'a> {
    pub async fn new(
        region_config_id: &str,
        region_common_name: CommonName,
        conf: &GatewayBackendMqtt,
    ) -> Result<MqttBackend<'a>> {
        // topic templates
        let mut templates = Handlebars::new();
        templates.register_template_string(
            "command_topic",
            if conf.command_topic.is_empty() {
                let command_topic = "gateway/{{ gateway_id }}/command/{{ command }}".to_string();
                if conf.topic_prefix.is_empty() {
                    command_topic
                } else {
                    format!("{}/{}", conf.topic_prefix, command_topic)
                }
            } else {
                conf.command_topic.clone()
            },
        )?;

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
                region_config_id: region_config_id.to_string(),
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

            info!(region_config_id = %ctx.region_config_id, "Connected to MQTT broker");

            if let Err(e) = subscribe_tx.try_send(()) {
                error!(region_config_id = %ctx.region_config_id, error = %e, "Send to subscribe channel error");
            }
        });
        client.set_connection_lost_callback(|client| {
            let ctx = client
                .user_data()
                .unwrap()
                .downcast_ref::<MqttContext>()
                .unwrap();

            info!(region_config_id = %ctx.region_config_id, "MQTT connection to broker lost");
        });

        // connection options
        let mut conn_opts_b = mqtt::ConnectOptionsBuilder::new();
        conn_opts_b.automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30));
        conn_opts_b.clean_session(conf.clean_session);
        conn_opts_b.keep_alive_interval(conf.keep_alive_interval);
        if !conf.username.is_empty() {
            conn_opts_b.user_name(&conf.username);
        }
        if !conf.password.is_empty() {
            conn_opts_b.password(&conf.password);
        }
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
                    .trust_store(&conf.ca_cert)
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
            v4_migrate: conf.v4_migrate,
        };

        // connect
        info!(region_config_id = %region_config_id, server_uri = %conf.server, clean_session = conf.clean_session, client_id = %client_id, "Connecting to MQTT broker");
        b.client
            .connect(conn_opts)
            .await
            .context("Connect to MQTT broker")?;

        // Consumer loop.
        tokio::spawn({
            let region_config_id = region_config_id.to_string();
            let v4_migrate = conf.v4_migrate;

            async move {
                info!("Starting MQTT consumer loop");
                while let Some(msg_opt) = stream.next().await {
                    if let Some(msg) = msg_opt {
                        message_callback(v4_migrate, &region_config_id, region_common_name, msg)
                            .await;
                    }
                }
            }
        });

        // (Re)subscribe loop.
        tokio::spawn({
            let region_config_id = region_config_id.to_string();
            let event_topic = if conf.event_topic.is_empty() {
                let event_topic = "gateway/+/event/+".to_string();
                if conf.topic_prefix.is_empty() {
                    event_topic
                } else {
                    format!("{}/{}", conf.topic_prefix, event_topic)
                }
            } else {
                conf.event_topic.clone()
            };
            let client = b.client.clone();
            let qos = conf.qos as i32;

            async move {
                while subscribe_rx.recv().await.is_some() {
                    info!(region_config_id = %region_config_id, event_topic = %event_topic, "Subscribing to gateway event topic");
                    if let Err(e) = client.subscribe(&event_topic, qos).await {
                        error!(region_config_id = %region_config_id, event_topic = %event_topic, error = %e, "MQTT subscribe error");
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

        if self.v4_migrate {
            df.v4_migrate();
        }

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

async fn message_callback(
    v4_migrate: bool,
    region_config_id: &str,
    region_common_name: CommonName,
    msg: mqtt::Message,
) {
    let topic = msg.topic();
    let qos = msg.qos();
    let b = msg.payload();

    let mut hasher = DefaultHasher::new();
    hasher.write(b);
    let key = redis_key(format!("gw:mqtt:lock:{:x}", hasher.finish()));
    let locked = is_locked(key).await;

    let err = || -> Result<()> {
        if locked? {
            trace!(
                region_config_id = region_config_id,
                topic = topic,
                qos = qos,
                "Message is already handled by different instance"
            );
            return Ok(());
        }

        let json = payload_is_json(b);

        info!(
            region_config_id = region_config_id,
            topic = topic,
            qos = qos,
            json = json,
            "Message received from gateway"
        );

        if topic.ends_with("/up") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "up".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(b)?,
                false => chirpstack_api::gw::UplinkFrame::decode(&mut Cursor::new(b))?,
            };

            if v4_migrate {
                event.v4_migrate();
            }

            if let Some(rx_info) = &mut event.rx_info {
                set_gateway_json(&rx_info.gateway_id, json);
                rx_info
                    .metadata
                    .insert("region_config_id".to_string(), region_config_id.to_string());
                rx_info.metadata.insert(
                    "region_common_name".to_string(),
                    region_common_name.to_string(),
                );
            }

            tokio::spawn(uplink::deduplicate_uplink(event));
        } else if topic.ends_with("/stats") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "stats".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(b)?,
                false => chirpstack_api::gw::GatewayStats::decode(&mut Cursor::new(b))?,
            };

            if v4_migrate {
                event.v4_migrate();
            }

            event
                .metadata
                .insert("region_config_id".to_string(), region_config_id.to_string());
            event.metadata.insert(
                "region_common_name".to_string(),
                region_common_name.to_string(),
            );
            set_gateway_json(&event.gateway_id, json);
            tokio::spawn(uplink::stats::Stats::handle(event));
        } else if topic.ends_with("/ack") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "ack".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(b)?,
                false => chirpstack_api::gw::DownlinkTxAck::decode(&mut Cursor::new(b))?,
            };

            if v4_migrate {
                event.v4_migrate();
            }

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
            qos = qos,
            "Processing gateway event error: {}",
            err.as_ref().unwrap()
        );
    }
}

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

fn gateway_is_json(gateway_id: &str) -> bool {
    let gw_json_r = GATEWAY_JSON.read().unwrap();
    gw_json_r.get(gateway_id).cloned().unwrap_or(false)
}

fn set_gateway_json(gateway_id: &str, is_json: bool) {
    let mut gw_json_w = GATEWAY_JSON.write().unwrap();
    gw_json_w.insert(gateway_id.to_string(), is_json);
}

fn payload_is_json(b: &[u8]) -> bool {
    String::from_utf8_lossy(b).contains("gatewayId")
}
