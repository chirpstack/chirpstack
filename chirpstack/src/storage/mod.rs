use std::ops::{Deref, DerefMut};
use std::sync::RwLock;

use anyhow::Context;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::info;

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
pub mod mac_command;
pub mod metrics;
pub mod multicast;
pub mod passive_roaming;
pub mod relay;
pub mod schema;
pub mod search;
pub mod tenant;
pub mod user;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref PG_POOL: RwLock<Option<PgPool>> = RwLock::new(None);
    static ref REDIS_POOL: RwLock<Option<RedisPool>> = RwLock::new(None);
    static ref REDIS_PREFIX: RwLock<String> = RwLock::new("".to_string());
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub enum RedisPool {
    Client(Pool<redis::Client>),
    ClusterClient(Pool<redis::cluster::ClusterClient>),
}

pub enum RedisPoolConnection {
    Client(PooledConnection<redis::Client>),
    ClusterClient(PooledConnection<redis::cluster::ClusterClient>),
}

impl RedisPoolConnection {
    pub fn new_pipeline(&self) -> RedisPipeline {
        match self {
            RedisPoolConnection::Client(_) => RedisPipeline::Pipeline(redis::pipe()),
            RedisPoolConnection::ClusterClient(_) => {
                RedisPipeline::ClusterPipeline(redis::cluster::cluster_pipe())
            }
        }
    }
}

impl Deref for RedisPoolConnection {
    type Target = dyn redis::ConnectionLike;

    fn deref(&self) -> &Self::Target {
        match self {
            RedisPoolConnection::Client(v) => v.deref() as &dyn redis::ConnectionLike,
            RedisPoolConnection::ClusterClient(v) => v.deref() as &dyn redis::ConnectionLike,
        }
    }
}

impl DerefMut for RedisPoolConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            RedisPoolConnection::Client(v) => v.deref_mut() as &mut dyn redis::ConnectionLike,
            RedisPoolConnection::ClusterClient(v) => {
                v.deref_mut() as &mut dyn redis::ConnectionLike
            }
        }
    }
}

pub enum RedisPipeline {
    Pipeline(redis::Pipeline),
    ClusterPipeline(redis::cluster::ClusterPipeline),
}

impl RedisPipeline {
    pub fn cmd(&mut self, name: &str) -> &mut Self {
        match self {
            RedisPipeline::Pipeline(p) => {
                p.cmd(name);
            }
            RedisPipeline::ClusterPipeline(p) => {
                p.cmd(name);
            }
        }
        self
    }

    pub fn arg<T: redis::ToRedisArgs>(&mut self, arg: T) -> &mut Self {
        match self {
            RedisPipeline::Pipeline(p) => {
                p.arg(arg);
            }
            RedisPipeline::ClusterPipeline(p) => {
                p.arg(arg);
            }
        }
        self
    }

    pub fn ignore(&mut self) -> &mut Self {
        match self {
            RedisPipeline::Pipeline(p) => {
                p.ignore();
            }
            RedisPipeline::ClusterPipeline(p) => {
                p.ignore();
            }
        }
        self
    }

    pub fn atomic(&mut self) -> &mut Self {
        match self {
            RedisPipeline::Pipeline(p) => {
                p.atomic();
            }
            RedisPipeline::ClusterPipeline(_) => {
                // TODO: ClusterPipeline does not (yet?) provide .atomic() method.
                // https://github.com/redis-rs/redis-rs/issues/731
            }
        }
        self
    }

    pub fn query<T: redis::FromRedisValue>(
        &mut self,
        con: &mut RedisPoolConnection,
    ) -> redis::RedisResult<T> {
        match self {
            RedisPipeline::Pipeline(p) => {
                if let RedisPoolConnection::Client(c) = con {
                    p.query(&mut **c)
                } else {
                    panic!("Mismatch between RedisPipeline and RedisPoolConnection")
                }
            }
            RedisPipeline::ClusterPipeline(p) => {
                if let RedisPoolConnection::ClusterClient(c) = con {
                    p.query(c)
                } else {
                    panic!("Mismatch between RedisPipeline and RedisPoolConnection")
                }
            }
        }
    }
}

pub async fn setup() -> Result<()> {
    let conf = config::get();

    info!("Setting up PostgreSQL connection pool");
    let pg_pool = PgPool::builder()
        .max_size(conf.postgresql.max_open_connections)
        .min_idle(match conf.postgresql.min_idle_connections {
            0 => None,
            _ => Some(conf.postgresql.min_idle_connections),
        })
        .build(ConnectionManager::new(&conf.postgresql.dsn))
        .context("Setup PostgreSQL connection pool error")?;
    set_db_pool(pg_pool);
    let mut pg_conn = get_db_conn()?;

    info!("Applying schema migrations");
    pg_conn
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("{}", e))?;

    info!("Setting up Redis client");
    if conf.redis.cluster {
        let client = redis::cluster::ClusterClientBuilder::new(conf.redis.servers.clone())
            .build()
            .context("ClusterClient open")?;
        let pool: r2d2::Pool<redis::cluster::ClusterClient> = r2d2::Pool::builder()
            .max_size(conf.redis.max_open_connections)
            .min_idle(match conf.redis.min_idle_connections {
                0 => None,
                _ => Some(conf.redis.min_idle_connections),
            })
            .build(client)
            .context("Building Redis pool")?;
        set_redis_pool(RedisPool::ClusterClient(pool));
    } else {
        let client = redis::Client::open(conf.redis.servers[0].clone()).context("Redis client")?;
        let pool: r2d2::Pool<redis::Client> = r2d2::Pool::builder()
            .max_size(conf.redis.max_open_connections)
            .min_idle(match conf.redis.min_idle_connections {
                0 => None,
                _ => Some(conf.redis.min_idle_connections),
            })
            .build(client)
            .context("Building Redis pool")?;
        set_redis_pool(RedisPool::Client(pool));
    }

    if !conf.redis.key_prefix.is_empty() {
        info!(prefix = %conf.redis.key_prefix, "Setting Redis prefix");
        *REDIS_PREFIX.write().unwrap() = conf.redis.key_prefix.clone();
    }

    Ok(())
}

pub fn get_db_pool() -> Result<PgPool> {
    let pool_r = PG_POOL.read().unwrap();
    let pool = pool_r
        .as_ref()
        .ok_or_else(|| anyhow!("PostgreSQL connection pool is not initialized (yet)"))?
        .clone();
    Ok(pool)
}

pub fn get_db_conn() -> Result<PgPoolConnection> {
    let pool = get_db_pool()?;
    Ok(pool.get()?)
}

pub fn get_redis_conn() -> Result<RedisPoolConnection> {
    let pool_r = REDIS_POOL.read().unwrap();
    let pool = pool_r
        .as_ref()
        .ok_or_else(|| anyhow!("Redis connection pool is not initialized (yet)"))?;
    Ok(match pool {
        RedisPool::Client(v) => RedisPoolConnection::Client(v.get()?),
        RedisPool::ClusterClient(v) => RedisPoolConnection::ClusterClient(v.get()?),
    })
}

pub fn set_db_pool(p: PgPool) {
    let mut pool_w = PG_POOL.write().unwrap();
    *pool_w = Some(p);
}

pub fn set_redis_pool(p: RedisPool) {
    let mut pool_w = REDIS_POOL.write().unwrap();
    *pool_w = Some(p);
}

pub fn redis_key(s: String) -> String {
    let prefix = REDIS_PREFIX.read().unwrap();
    format!("{}{}", prefix, s)
}

#[cfg(test)]
pub fn reset_db() -> Result<()> {
    let mut conn = get_db_conn()?;
    conn.revert_all_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("{}", e))?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("{}", e))?;

    Ok(())
}

#[cfg(test)]
pub async fn reset_redis() -> Result<()> {
    let mut c = get_redis_conn()?;
    redis::cmd("FLUSHDB").query(&mut *c)?;
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
