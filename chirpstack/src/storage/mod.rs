use std::sync::LazyLock;
use std::sync::RwLock;
use std::time::Instant;

use anyhow::Result;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use prometheus_client::metrics::histogram::{Histogram, exponential_buckets};
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
pub mod device_queue;
pub mod device_session;
pub mod downlink_frame;
pub mod error;
pub mod fields;
pub mod fuota;
pub mod gateway;
pub mod helpers;
pub mod mac_command;
pub mod metrics;
pub mod multicast;
pub mod passive_roaming;
#[cfg(feature = "postgres")]
mod postgres;
pub mod relay;
pub mod schema;
#[cfg(feature = "postgres")]
mod schema_postgres;
#[cfg(feature = "sqlite")]
mod schema_sqlite;
pub mod search;
#[cfg(feature = "sqlite")]
mod sqlite;
pub mod tenant;
pub mod user;

use crate::monitoring::prometheus;

static ASYNC_REDIS_POOL: LazyLock<TokioRwLock<Option<AsyncRedisPool>>> =
    LazyLock::new(|| TokioRwLock::new(None));
static REDIS_PREFIX: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("".to_string()));
static STORAGE_REDIS_CONN_GET: LazyLock<Histogram> = LazyLock::new(|| {
    let histogram = Histogram::new(exponential_buckets(0.001, 2.0, 12));
    prometheus::register(
        "storage_redis_conn_get_duration_seconds",
        "Time between requesting a Redis connection and the connection-pool returning it",
        histogram.clone(),
    );
    histogram
});

#[cfg(feature = "postgres")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations_postgres");
#[cfg(feature = "sqlite")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations_sqlite");

#[cfg(feature = "postgres")]
pub use postgres::{AsyncPgPoolConnection as AsyncDbPoolConnection, get_async_db_conn};
#[cfg(feature = "sqlite")]
pub use sqlite::{AsyncSqlitePoolConnection as AsyncDbPoolConnection, get_async_db_conn};

#[derive(Clone)]
pub enum AsyncRedisPool {
    Client(deadpool_redis::Pool),
    ClusterClient(deadpool_redis::cluster::Pool),
    SentinelClient(deadpool_redis::sentinel::Pool),
}

pub enum AsyncRedisPoolConnection {
    Client(deadpool_redis::Connection),
    ClusterClient(deadpool_redis::cluster::Connection),
    SentinelClient(deadpool_redis::sentinel::Connection),
}

impl ConnectionLike for AsyncRedisPoolConnection {
    fn req_packed_command<'a>(
        &'a mut self,
        cmd: &'a redis::Cmd,
    ) -> redis::RedisFuture<'a, redis::Value> {
        match self {
            AsyncRedisPoolConnection::Client(v) => v.req_packed_command(cmd),
            AsyncRedisPoolConnection::ClusterClient(v) => v.req_packed_command(cmd),
            AsyncRedisPoolConnection::SentinelClient(v) => v.req_packed_command(cmd),
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
            AsyncRedisPoolConnection::SentinelClient(v) => {
                v.req_packed_commands(cmd, offset, count)
            }
        }
    }
    fn get_db(&self) -> i64 {
        match self {
            AsyncRedisPoolConnection::Client(v) => v.get_db(),
            AsyncRedisPoolConnection::ClusterClient(v) => v.get_db(),
            AsyncRedisPoolConnection::SentinelClient(v) => v.get_db(),
        }
    }
}

pub async fn setup() -> Result<()> {
    let conf = config::get();

    #[cfg(feature = "postgres")]
    {
        postgres::setup(&conf.postgresql)?;
    }
    #[cfg(feature = "sqlite")]
    {
        sqlite::setup(&conf.sqlite)?;
    }
    run_db_migrations().await?;

    info!("Setting up Redis client");
    if conf.redis.cluster {
        let pool = deadpool_redis::cluster::Config::from_urls(conf.redis.servers.clone())
            .builder()?
            .max_size(conf.redis.max_open_connections as usize)
            .build()?;
        set_async_redis_pool(AsyncRedisPool::ClusterClient(pool)).await;
    } else if !conf.redis.master_name.is_empty() {
        let pool = sentinel_pool(&conf.redis)?;
        set_async_redis_pool(AsyncRedisPool::SentinelClient(pool)).await;
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
        AsyncRedisPool::SentinelClient(v) => {
            AsyncRedisPoolConnection::SentinelClient(v.get().await?)
        }
    };

    STORAGE_REDIS_CONN_GET.observe(start.elapsed().as_secs_f64());

    Ok(res)
}

pub async fn run_db_migrations() -> Result<()> {
    info!("Applying schema migrations");

    let c = get_async_db_conn().await?;
    let mut c_wrapped: AsyncConnectionWrapper<AsyncDbPoolConnection> =
        AsyncConnectionWrapper::from(c);

    #[cfg(feature = "postgres")]
    let _ = task::spawn_blocking(move || -> Result<()> {
        c_wrapped
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("{}", e))?;

        Ok(())
    })
    .await?;

    #[cfg(feature = "sqlite")]
    let _ = task::spawn_blocking(move || -> Result<()> {
        use diesel::sql_types::Integer;
        use diesel::{QueryableByName, RunQueryDsl};

        #[derive(QueryableByName)]
        struct FkPragma {
            #[diesel(sql_type = Integer)]
            foreign_keys: i32,
        }

        let fk_pragma: i32 = diesel::sql_query("pragma foreign_keys")
            .get_result::<FkPragma>(&mut c_wrapped)?
            .foreign_keys;
        if fk_pragma != 0 {
            let _ = diesel::sql_query("pragma foreign_keys = 0").execute(&mut c_wrapped)?;
        }

        c_wrapped
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("{}", e))?;

        if fk_pragma != 0 {
            let _ = diesel::sql_query("pragma foreign_keys = 1").execute(&mut c_wrapped)?;
        }

        Ok(())
    })
    .await?;

    Ok(())
}

// Sentinel returns only ip:port for the master, so TLS mode and auth
// credentials must be forwarded explicitly via node_connection_info.
fn sentinel_node_connection_info(
    conf: &config::Redis,
) -> Result<deadpool_redis::sentinel::SentinelNodeConnectionInfo> {
    use redis::IntoConnectionInfo;

    let mut tls_mode = None;
    for server in &conf.servers {
        let info = server.as_str().into_connection_info()?;

        // The Sentinel URLs are used as-is to connect to the Sentinel nodes,
        // which do not support SELECT. Reject a database index early with a
        // clear error instead of failing on the first Redis command.
        if info.redis_settings().db() != 0 {
            return Err(anyhow!(
                "Redis Sentinel URLs must not contain a database index ({}), use key_prefix to separate keys",
                server
            ));
        }

        // Use TLS towards the master when the Sentinel nodes use TLS (rediss://).
        if matches!(info.addr(), redis::ConnectionAddr::TcpTls { .. }) {
            tls_mode = Some(deadpool_redis::sentinel::TlsMode::Secure);
        }
    }

    Ok(deadpool_redis::sentinel::SentinelNodeConnectionInfo {
        tls_mode,
        redis_connection_info: Some(deadpool_redis::RedisConnectionInfo {
            password: if conf.password.is_empty() {
                None
            } else {
                Some(conf.password.clone())
            },
            ..Default::default()
        }),
    })
}

fn sentinel_pool(conf: &config::Redis) -> Result<deadpool_redis::sentinel::Pool> {
    Ok(deadpool_redis::sentinel::Config::from_urls(
        conf.servers.clone(),
        conf.master_name.clone(),
        deadpool_redis::sentinel::SentinelServerType::Master,
    )
    .with_node_connection_info(Some(sentinel_node_connection_info(conf)?))
    .builder()?
    .max_size(conf.max_open_connections as usize)
    .build()?)
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
    let mut c_wrapped: AsyncConnectionWrapper<AsyncDbPoolConnection> =
        AsyncConnectionWrapper::from(c);

    tokio::task::spawn_blocking(move || -> Result<()> {
        c_wrapped
            .revert_all_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("During revert: {}", e))?;
        c_wrapped
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("During run: {}", e))?;

        Ok(())
    })
    .await?
}

#[cfg(test)]
pub async fn reset_redis() -> Result<()> {
    () = redis::cmd("FLUSHDB")
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

    fn sentinel_conf(servers: Vec<&str>, password: &str) -> config::Redis {
        config::Redis {
            servers: servers.into_iter().map(|s| s.to_string()).collect(),
            master_name: "mymaster".to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_sentinel_node_connection_info_no_password() {
        let info =
            sentinel_node_connection_info(&sentinel_conf(vec!["redis://127.0.0.1:26379"], ""))
                .unwrap();
        assert!(info.tls_mode.is_none());
        assert_eq!(None, info.redis_connection_info.unwrap().password);
    }

    #[test]
    fn test_sentinel_node_connection_info_password_forwarded() {
        let info = sentinel_node_connection_info(&sentinel_conf(
            vec!["redis://127.0.0.1:26379"],
            "s3cret",
        ))
        .unwrap();
        assert_eq!(
            Some("s3cret".to_string()),
            info.redis_connection_info.unwrap().password
        );
    }

    #[test]
    fn test_sentinel_node_connection_info_tls_from_rediss_url() {
        let info = sentinel_node_connection_info(&sentinel_conf(
            vec!["redis://127.0.0.1:26379", "rediss://127.0.0.1:26380"],
            "",
        ))
        .unwrap();
        assert!(matches!(
            info.tls_mode,
            Some(deadpool_redis::sentinel::TlsMode::Secure)
        ));
    }

    #[test]
    fn test_sentinel_node_connection_info_tls_scheme_case_insensitive() {
        let info =
            sentinel_node_connection_info(&sentinel_conf(vec!["REDISS://127.0.0.1:26379"], ""))
                .unwrap();
        assert!(matches!(
            info.tls_mode,
            Some(deadpool_redis::sentinel::TlsMode::Secure)
        ));
    }

    #[test]
    fn test_sentinel_node_connection_info_rejects_db_index() {
        let err = sentinel_node_connection_info(&sentinel_conf(
            vec!["redis://127.0.0.1:26379", "redis://127.0.0.1:26380/1"],
            "",
        ))
        .unwrap_err();
        assert!(err.to_string().contains("key_prefix"));
    }

    #[test]
    fn test_sentinel_pool_rejects_empty_server_list() {
        assert!(sentinel_pool(&sentinel_conf(vec![], "")).is_err());
    }

    #[test]
    fn test_sentinel_pool_builds_without_connecting() {
        // Building the pool must not require a reachable Sentinel.
        let pool = sentinel_pool(&sentinel_conf(
            vec!["redis://127.0.0.1:1", "redis://127.0.0.1:2"],
            "s3cret",
        ))
        .unwrap();
        assert_eq!(100, pool.status().max_size);
    }

    #[test]
    fn test_sentinel_pool_rejects_invalid_url() {
        assert!(sentinel_pool(&sentinel_conf(vec!["not a url"], "")).is_err());
    }
}
