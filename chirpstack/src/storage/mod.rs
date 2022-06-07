use std::ops::{Deref, DerefMut};
use std::sync::RwLock;

use anyhow::Context;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::embed_migrations;
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
pub mod schema;
pub mod search;
pub mod tenant;
pub mod user;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref PG_POOL: RwLock<Option<PgPool>> = RwLock::new(None);
    static ref REDIS_POOL: RwLock<Option<RedisPool>> = RwLock::new(None);
}

embed_migrations!("./migrations");

pub enum RedisPool {
    Client(Pool<redis::Client>),
    ClusterClient(Pool<redis::cluster::ClusterClient>),
}

pub enum RedisPoolConnection {
    Client(PooledConnection<redis::Client>),
    ClusterClient(PooledConnection<redis::cluster::ClusterClient>),
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
    let pg_conn = get_db_conn()?;

    info!("Applying schema migrations");
    embedded_migrations::run(&pg_conn).context("Run migrations error")?;

    info!("Setting up Redis client");
    if conf.redis.cluster {
        let client = redis::cluster::ClusterClientBuilder::new(conf.redis.servers.clone())
            .open()
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

    Ok(())
}

pub fn get_db_pool() -> PgPool {
    let pool_r = PG_POOL.read().unwrap();
    let pool = pool_r.as_ref().unwrap().clone();
    pool
}

pub fn get_db_conn() -> Result<PgPoolConnection> {
    let pool = get_db_pool();
    pool.get().context("Get connection from pool error")
}

pub fn get_redis_conn() -> Result<RedisPoolConnection> {
    let pool_r = REDIS_POOL.read().unwrap();
    let pool = pool_r.as_ref().unwrap();
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
    s
}

#[cfg(test)]
pub fn reset_db() -> Result<()> {
    use diesel_migrations::{revert_latest_migration, run_pending_migrations};
    let conn = get_db_conn()?;

    loop {
        match revert_latest_migration(&conn) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }

    // and forward again
    run_pending_migrations(&conn)?;

    Ok(())
}

#[cfg(test)]
pub async fn reset_redis() -> Result<()> {
    let mut c = get_redis_conn()?;
    redis::cmd("FLUSHALL").query(&mut *c)?;
    Ok(())
}
