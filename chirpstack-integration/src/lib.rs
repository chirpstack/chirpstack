#[macro_use]
extern crate lazy_static;

use std::io::Cursor;
use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::{error, info, warn, Level};
use tracing_subscriber::{filter, prelude::*};

use chirpstack_api::{integration as integration_pb, prost::Message};

lazy_static! {
    static ref INTEGRATION: RwLock<Option<Box<dyn IntegrationTrait + Sync + Send>>> =
        RwLock::new(None);
}

#[derive(Default, Deserialize, Clone)]
#[serde(default)]
pub struct Configuration {
    pub logging: Logging,
    pub redis: Redis,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Logging {
    pub level: String,
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "info".into(),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Redis {
    pub servers: Vec<String>,
    pub cluster: bool,
    pub key_prefix: String,
    pub consumer_group: String,
    pub consumer_name: String,
}

impl Default for Redis {
    fn default() -> Self {
        Redis {
            servers: vec!["redis://127.0.0.1/".into()],
            cluster: false,
            key_prefix: "".into(),
            consumer_group: "integration_pulsar".into(),
            consumer_name: "main".into(),
        }
    }
}

#[async_trait]
pub trait IntegrationTrait {
    async fn uplink_event(&self, pl: &integration_pb::UplinkEvent) -> Result<()>;

    async fn join_event(&self, pl: &integration_pb::JoinEvent) -> Result<()>;

    async fn ack_event(&self, pl: &integration_pb::AckEvent) -> Result<()>;

    async fn txack_event(&self, pl: &integration_pb::TxAckEvent) -> Result<()>;

    async fn log_event(&self, pl: &integration_pb::LogEvent) -> Result<()>;

    async fn status_event(&self, pl: &integration_pb::StatusEvent) -> Result<()>;

    async fn location_event(&self, pl: &integration_pb::LocationEvent) -> Result<()>;

    async fn integration_event(&self, pl: &integration_pb::IntegrationEvent) -> Result<()>;
}

struct Integration {
    redis_client: RedisClient,
    key_prefix: String,
    consumer_group: String,
    consumer_name: String,
}

enum RedisClient {
    Client(redis::Client),
    ClusterClient(redis::cluster::ClusterClient),
}

impl RedisClient {
    async fn get_async_connection(&self) -> Result<RedisConnection> {
        match self {
            RedisClient::Client(c) => Ok(RedisConnection::Client(c.get_async_connection().await?)),
            RedisClient::ClusterClient(c) => Ok(RedisConnection::ClusterClient(
                c.get_async_connection().await?,
            )),
        }
    }
}

enum RedisConnection {
    Client(redis::aio::Connection),
    ClusterClient(redis::cluster_async::ClusterConnection),
}

impl redis::aio::ConnectionLike for RedisConnection {
    fn get_db(&self) -> i64 {
        match self {
            RedisConnection::Client(c) => c.get_db(),
            RedisConnection::ClusterClient(c) => c.get_db(),
        }
    }

    fn req_packed_command<'a>(
        &'a mut self,
        cmd: &'a redis::Cmd,
    ) -> redis::RedisFuture<'a, redis::Value> {
        match self {
            RedisConnection::Client(c) => c.req_packed_command(cmd),
            RedisConnection::ClusterClient(c) => c.req_packed_command(cmd),
        }
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a redis::Pipeline,
        offset: usize,
        count: usize,
    ) -> redis::RedisFuture<'a, Vec<redis::Value>> {
        match self {
            RedisConnection::Client(c) => c.req_packed_commands(cmd, offset, count),
            RedisConnection::ClusterClient(c) => c.req_packed_commands(cmd, offset, count),
        }
    }
}

impl Integration {
    fn new(conf: &Configuration) -> Result<Self> {
        info!("Initializing ChirpStack Integration backend");

        let redis_client = if conf.redis.cluster {
            info!("Setting up Redis Cluster client");
            RedisClient::ClusterClient(
                redis::cluster::ClusterClientBuilder::new(conf.redis.servers.clone()).build()?,
            )
        } else {
            info!(server = %conf.redis.servers[0], "Setting up Redis client");
            RedisClient::Client(redis::Client::open(conf.redis.servers[0].clone())?)
        };

        Ok(Integration {
            redis_client,
            key_prefix: conf.redis.key_prefix.clone(),
            consumer_group: conf.redis.consumer_group.clone(),
            consumer_name: conf.redis.consumer_name.clone(),
        })
    }

    async fn start(&self) -> Result<()> {
        info!("Getting Redis connection");
        let mut redis_conn = self.redis_client.get_async_connection().await?;

        let key = format!("{}device:stream:event", self.key_prefix);

        // Try to create the consumer group. This will fail in case the consumer group already exists.
        let _: usize = match redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(&key)
            .arg(&self.consumer_group)
            .arg(0)
            .arg("MKSTREAM")
            .query_async(&mut redis_conn)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e, "Could not create Redis consumer group, ignore this error if the group already exists");
                0
            }
        };

        loop {
            let srr: redis::streams::StreamReadReply = redis::cmd("XREADGROUP")
                .arg("GROUP")
                .arg(&self.consumer_group)
                .arg(&self.consumer_name)
                .arg("COUNT")
                .arg(10)
                .arg("BLOCK")
                .arg(1000)
                .arg("STREAMS")
                .arg(&key)
                .arg(">")
                .query_async(&mut redis_conn)
                .await?;

            for stream_key in &srr.keys {
                for stream_id in &stream_key.ids {
                    redis::cmd("XACK")
                        .arg(&key)
                        .arg(&self.consumer_group)
                        .arg(&stream_id.id)
                        .query_async(&mut redis_conn)
                        .await?;

                    for (k, v) in &stream_id.map {
                        let res = || -> Result<()> {
                            info!(key = %k, "Event received from Redis stream");
                            match k.as_ref() {
                                "up" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl = integration_pb::UplinkEvent::decode(
                                            &mut Cursor::new(b),
                                        )?;
                                        tokio::spawn(uplink_event(pl));
                                    }
                                }
                                "join" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl =
                                            integration_pb::JoinEvent::decode(&mut Cursor::new(b))?;
                                        tokio::spawn(join_event(pl));
                                    }
                                }
                                "ack" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl =
                                            integration_pb::AckEvent::decode(&mut Cursor::new(b))?;
                                        tokio::spawn(ack_event(pl));
                                    }
                                }
                                "txack" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl = integration_pb::TxAckEvent::decode(
                                            &mut Cursor::new(b),
                                        )?;
                                        tokio::spawn(txack_event(pl));
                                    }
                                }
                                "status" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl = integration_pb::StatusEvent::decode(
                                            &mut Cursor::new(b),
                                        )?;
                                        tokio::spawn(status_event(pl));
                                    }
                                }
                                "log" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl =
                                            integration_pb::LogEvent::decode(&mut Cursor::new(b))?;
                                        tokio::spawn(log_event(pl));
                                    }
                                }
                                "location" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl = integration_pb::LocationEvent::decode(
                                            &mut Cursor::new(b),
                                        )?;
                                        tokio::spawn(location_event(pl));
                                    }
                                }
                                "integration" => {
                                    if let redis::Value::Data(b) = v {
                                        let pl = integration_pb::IntegrationEvent::decode(
                                            &mut Cursor::new(b),
                                        )?;
                                        tokio::spawn(integration_event(pl));
                                    }
                                }
                                _ => {
                                    error!(key = %k, "Unexpected event key");
                                }
                            }

                            Ok(())
                        }();

                        if let Err(e) = res {
                            error!(key = %k, error = %e, "Parsing event error");
                        }
                    }
                }
            }
        }
    }
}

pub fn setup_log(conf: &Configuration) -> Result<()> {
    let filter = filter::LevelFilter::from_level(Level::from_str(&conf.logging.level).unwrap());

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    info!(
        "Starting {} (version: {}, docs: {})",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_HOMEPAGE")
    );

    Ok(())
}

pub async fn register(integration: Box<dyn IntegrationTrait + Sync + Send>) {
    let mut int = INTEGRATION.write().await;
    *int = Some(integration);
}

pub async fn start(conf: Configuration) -> Result<()> {
    let int = Integration::new(&conf)?;
    int.start().await
}

async fn uplink_event(pl: integration_pb::UplinkEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().uplink_event(&pl).await {
        error!(error = %e, "Uplink event error");
    }
}

async fn join_event(pl: integration_pb::JoinEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().join_event(&pl).await {
        error!(error = %e, "Join event error");
    }
}

async fn ack_event(pl: integration_pb::AckEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().ack_event(&pl).await {
        error!(error = %e, "Ack event error");
    }
}

async fn txack_event(pl: integration_pb::TxAckEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().txack_event(&pl).await {
        error!(error = %e, "Tx ack event error");
    }
}

async fn status_event(pl: integration_pb::StatusEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().status_event(&pl).await {
        error!(error = %e, "Status event error");
    }
}

async fn log_event(pl: integration_pb::LogEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().log_event(&pl).await {
        error!(error = %e, "Log event error");
    }
}

async fn location_event(pl: integration_pb::LocationEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().location_event(&pl).await {
        error!(error = %e, "Location event error");
    }
}

async fn integration_event(pl: integration_pb::IntegrationEvent) {
    let integration = INTEGRATION.read().await;
    if let Err(e) = integration.as_ref().unwrap().integration_event(&pl).await {
        error!(error = %e, "Integration event error");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::time::Duration;

    use tokio::sync::RwLock;
    use tokio::time::sleep;

    lazy_static! {
        static ref UPLINK_EVENTS: RwLock<Vec<integration_pb::UplinkEvent>> =
            RwLock::new(Vec::new());
        static ref JOIN_EVENTS: RwLock<Vec<integration_pb::JoinEvent>> = RwLock::new(Vec::new());
        static ref ACK_EVENTS: RwLock<Vec<integration_pb::AckEvent>> = RwLock::new(Vec::new());
        static ref TXACK_EVENTS: RwLock<Vec<integration_pb::TxAckEvent>> = RwLock::new(Vec::new());
        static ref LOG_EVENTS: RwLock<Vec<integration_pb::LogEvent>> = RwLock::new(Vec::new());
        static ref STATUS_EVENTS: RwLock<Vec<integration_pb::StatusEvent>> =
            RwLock::new(Vec::new());
        static ref LOCATION_EVENTS: RwLock<Vec<integration_pb::LocationEvent>> =
            RwLock::new(Vec::new());
        static ref INTEGRATION_EVENTS: RwLock<Vec<integration_pb::IntegrationEvent>> =
            RwLock::new(Vec::new());
    }

    struct MockIntegration {}

    #[async_trait]
    impl IntegrationTrait for MockIntegration {
        async fn uplink_event(&self, pl: &integration_pb::UplinkEvent) -> Result<()> {
            UPLINK_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn join_event(&self, pl: &integration_pb::JoinEvent) -> Result<()> {
            JOIN_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn ack_event(&self, pl: &integration_pb::AckEvent) -> Result<()> {
            ACK_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn txack_event(&self, pl: &integration_pb::TxAckEvent) -> Result<()> {
            TXACK_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn log_event(&self, pl: &integration_pb::LogEvent) -> Result<()> {
            LOG_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn status_event(&self, pl: &integration_pb::StatusEvent) -> Result<()> {
            STATUS_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn location_event(&self, pl: &integration_pb::LocationEvent) -> Result<()> {
            LOCATION_EVENTS.write().await.push(pl.clone());
            Ok(())
        }

        async fn integration_event(&self, pl: &integration_pb::IntegrationEvent) -> Result<()> {
            INTEGRATION_EVENTS.write().await.push(pl.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_integration() {
        let redis_url = env::var("TEST_REDIS_URL").unwrap_or("redis://127.0.0.1/1".to_string());

        setup_log(&Configuration::default()).unwrap();
        register(Box::new(MockIntegration {})).await;

        let conf = Configuration {
            redis: Redis {
                servers: vec![redis_url.clone()],
                consumer_group: "test_group".into(),
                consumer_name: "test_consumer".into(),
                ..Default::default()
            },
            ..Default::default()
        };

        tokio::spawn(start(conf));
        sleep(Duration::from_millis(100)).await;

        let redis_client = redis::Client::open(redis_url).unwrap();
        let mut redis_conn = redis_client.get_async_connection().await.unwrap();

        println!("Uplink");

        // uplink
        let pl = integration_pb::UplinkEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("up")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = UPLINK_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::UplinkEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("Join");

        // join
        let pl = integration_pb::JoinEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("join")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = JOIN_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::JoinEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("Ack");

        // ack
        let pl = integration_pb::AckEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("ack")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = ACK_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::AckEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("TxAck");

        // txack
        let pl = integration_pb::TxAckEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("txack")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = TXACK_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::TxAckEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("Log");

        // log
        let pl = integration_pb::LogEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("log")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = LOG_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::LogEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("Status");

        // status
        let pl = integration_pb::StatusEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("status")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = STATUS_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::StatusEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("Location");

        // location
        let pl = integration_pb::LocationEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("location")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = LOCATION_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::LocationEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);

        println!("Integration");

        // integration
        let pl = integration_pb::IntegrationEvent::default();
        let _: String = redis::cmd("XADD")
            .arg("device:stream:event")
            .arg("MAXLEN")
            .arg(1)
            .arg("*")
            .arg("integration")
            .arg(pl.encode_to_vec())
            .query_async(&mut redis_conn)
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        let pl_recv = INTEGRATION_EVENTS
            .write()
            .await
            .drain(0..1)
            .collect::<Vec<integration_pb::IntegrationEvent>>()
            .first()
            .cloned()
            .unwrap();

        assert_eq!(pl, pl_recv);
    }
}
