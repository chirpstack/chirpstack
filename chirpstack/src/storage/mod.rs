use std::sync::RwLock;
use std::time::Instant;

use anyhow::Result;
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::deadpool::{Object as DeadpoolObject, Pool as DeadpoolPool};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
use redis::aio::ConnectionLike;
use tokio::sync::RwLock as TokioRwLock;
use tokio::task;
use tracing::{error, info};

use crate::config;

pub mod api_key;
pub mod application;
pub mod device;
pub mod device_gateway;
pub mod device_keys;
pub mod device_profile;
pub mod device_profile_template;
pub mod device_queue;
pub mod device_session;
pub mod downlink_frame;
pub mod error;
pub mod fields;
pub mod gateway;
pub mod helpers;
pub mod mac_command;
pub mod metrics;
pub mod multicast;
pub mod passive_roaming;
pub mod relay;
pub mod schema;
pub mod search;
pub mod tenant;
pub mod user;

use crate::helpers::tls::get_root_certs;
use crate::monitoring::prometheus;

pub type AsyncPgPool = DeadpoolPool<AsyncPgConnection>;
pub type AsyncPgPoolConnection = DeadpoolObject<AsyncPgConnection>;

lazy_static! {
    static ref ASYNC_PG_POOL: RwLock<Option<AsyncPgPool>> = RwLock::new(None);
    static ref ASYNC_REDIS_POOL: TokioRwLock<Option<AsyncRedisPool>> = TokioRwLock::new(None);
    static ref REDIS_PREFIX: RwLock<String> = RwLock::new("".to_string());
    static ref STORAGE_REDIS_CONN_GET: Histogram = {
        let histogram = Histogram::new(exponential_buckets(0.001, 2.0, 12));
        prometheus::register(
            "storage_redis_conn_get_duration_seconds",
            "Time between requesting a Redis connection and the connection-pool returning it",
            histogram.clone(),
        );
        histogram
    };
    static ref STORAGE_PG_CONN_GET: Histogram = {
        let histogram = Histogram::new(exponential_buckets(0.001, 2.0, 12));
        prometheus::register(
            "storage_pg_conn_get_duration_seconds",
            "Time between requesting a PostgreSQL connection and the connection-pool returning it",
            histogram.clone(),
        );
        histogram
    };
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Clone)]
pub enum AsyncRedisPool {
    Client(deadpool_redis::Pool),
    ClusterClient(deadpool_redis::cluster::Pool),
}

pub enum AsyncRedisPoolConnection {
    Client(deadpool_redis::Connection),
    ClusterClient(deadpool_redis::cluster::Connection),
}

impl ConnectionLike for AsyncRedisPoolConnection {
    fn req_packed_command<'a>(
        &'a mut self,
        cmd: &'a redis::Cmd,
    ) -> redis::RedisFuture<'a, redis::Value> {
        match self {
            AsyncRedisPoolConnection::Client(v) => v.req_packed_command(cmd),
            AsyncRedisPoolConnection::ClusterClient(v) => v.req_packed_command(cmd),
        }
    }
    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a redis::Pipeline,
        offset: usize,
        count: usize,
    ) -> redis::RedisFuture<'a, Vec<redis::Value>> {
        match self {
            AsyncRedisPoolConnection::Client(v) => v.req_packed_commands(cmd, offset, count),
            AsyncRedisPoolConnection::ClusterClient(v) => v.req_packed_commands(cmd, offset, count),
        }
    }
    fn get_db(&self) -> i64 {
        match self {
            AsyncRedisPoolConnection::Client(v) => v.get_db(),
            AsyncRedisPoolConnection::ClusterClient(v) => v.get_db(),
        }
    }
}

pub async fn setup() -> Result<()> {
    let conf = config::get();

    info!("Setting up PostgreSQL connection pool");
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(pg_establish_connection);

    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
        &conf.postgresql.dsn,
        config,
    );
    let pool = DeadpoolPool::builder(mgr)
        .max_size(conf.postgresql.max_open_connections as usize)
        .build()?;
    set_async_db_pool(pool);
    run_db_migrations().await?;

    info!("Setting up Redis client");
    if conf.redis.cluster {
        let pool = deadpool_redis::cluster::Config::from_urls(conf.redis.servers.clone())
            .builder()?
            .max_size(conf.redis.max_open_connections as usize)
            .build()?;
        set_async_redis_pool(AsyncRedisPool::ClusterClient(pool)).await;
    } else {
        let pool = deadpool_redis::Config::from_url(conf.redis.servers[0].clone())
            .builder()?
            .max_size(conf.redis.max_open_connections as usize)
            .build()?;
        set_async_redis_pool(AsyncRedisPool::Client(pool)).await;
    }

    if !conf.redis.key_prefix.is_empty() {
        info!(prefix = %conf.redis.key_prefix, "Setting Redis prefix");
        REDIS_PREFIX
            .write()
            .unwrap()
            .clone_from(&conf.redis.key_prefix);
    }

    Ok(())
}

// Source:
// https://github.com/weiznich/diesel_async/blob/main/examples/postgres/pooled-with-rustls/src/main.rs
fn pg_establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        let conf = config::get();

        let root_certs = get_root_certs(if conf.postgresql.ca_cert.is_empty() {
            None
        } else {
            Some(conf.postgresql.ca_cert.clone())
        })
        .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        let rustls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_certs)
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                error!(error = %e, "PostgreSQL connection error");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

pub fn get_async_db_pool() -> Result<AsyncPgPool> {
    let pool_r = ASYNC_PG_POOL.read().unwrap();
    let pool: AsyncPgPool = pool_r
        .as_ref()
        .ok_or_else(|| anyhow!("PostgreSQL connection pool is not initialized"))?
        .clone();
    Ok(pool)
}

pub async fn get_async_db_conn() -> Result<AsyncPgPoolConnection> {
    let pool = get_async_db_pool()?;

    let start = Instant::now();
    let res = pool.get().await?;

    STORAGE_PG_CONN_GET.observe(start.elapsed().as_secs_f64());

    Ok(res)
}

async fn get_async_redis_pool() -> Result<AsyncRedisPool> {
    let pool_r = ASYNC_REDIS_POOL.read().await;
    let pool: AsyncRedisPool = pool_r
        .as_ref()
        .ok_or_else(|| anyhow!("Redis connection pool is not initialized"))?
        .clone();
    Ok(pool)
}

pub async fn get_async_redis_conn() -> Result<AsyncRedisPoolConnection> {
    let pool = get_async_redis_pool().await?;

    let start = Instant::now();
    let res = match pool {
        AsyncRedisPool::Client(v) => AsyncRedisPoolConnection::Client(v.get().await?),
        AsyncRedisPool::ClusterClient(v) => {
            AsyncRedisPoolConnection::ClusterClient(v.clone().get().await?)
        }
    };

    STORAGE_REDIS_CONN_GET.observe(start.elapsed().as_secs_f64());

    Ok(res)
}

pub fn set_async_db_pool(p: AsyncPgPool) {
    let mut pool_w = ASYNC_PG_POOL.write().unwrap();
    *pool_w = Some(p);
}

pub async fn run_db_migrations() -> Result<()> {
    info!("Applying schema migrations");

    let c = get_async_db_conn().await?;
    let mut c_wrapped: AsyncConnectionWrapper<AsyncPgPoolConnection> =
        AsyncConnectionWrapper::from(c);

    task::spawn_blocking(move || -> Result<()> {
        c_wrapped
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("{}", e))?;

        Ok(())
    })
    .await?
}

async fn set_async_redis_pool(p: AsyncRedisPool) {
    let mut pool_w = ASYNC_REDIS_POOL.write().await;
    *pool_w = Some(p);
}

pub fn redis_key(s: String) -> String {
    let prefix = REDIS_PREFIX.read().unwrap();
    format!("{}{}", prefix, s)
}

#[cfg(test)]
pub async fn reset_db() -> Result<()> {
    let c = get_async_db_conn().await?;
    let mut c_wrapped: AsyncConnectionWrapper<AsyncPgPoolConnection> =
        AsyncConnectionWrapper::from(c);

    tokio::task::spawn_blocking(move || -> Result<()> {
        c_wrapped
            .revert_all_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("{}", e))?;
        c_wrapped
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("{}", e))?;

        Ok(())
    })
    .await?
}

#[cfg(test)]
pub async fn reset_redis() -> Result<()> {
    redis::cmd("FLUSHDB")
        .query_async(&mut get_async_redis_conn().await?)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prefix_no_prefix() {
        *REDIS_PREFIX.write().unwrap() = "".to_string();
        assert_eq!("lora:test:key", redis_key("lora:test:key".to_string()));
    }

    #[test]
    fn test_prefix() {
        *REDIS_PREFIX.write().unwrap() = "foobar:".to_string();
        assert_eq!(
            "foobar:lora:test:key",
            redis_key("lora:test:key".to_string())
        );
    }
}
