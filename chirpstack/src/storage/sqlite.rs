use std::sync::RwLock;

use anyhow::{Context, Result};
use tracing::info;

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::deadpool::{Object as DeadpoolObject, Pool as DeadpoolPool};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;

use crate::config::Postgresql;

pub type AsyncSqlitePool = DeadpoolPool<SyncConnectionWrapper<SqliteConnection>>;
pub type AsyncSqlitePoolConnection = DeadpoolObject<SyncConnectionWrapper<SqliteConnection>>;

lazy_static! {
    static ref ASYNC_SQLITE_POOL: RwLock<Option<SqlitePool>> = RwLock::new(None);
}

pub fn setup(conf: &Postgresql) -> Result<()> {
    info!("Setting up Sqlite connection pool");
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(sqlite_establish_connection);
    let mgr =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new_with_config(
            &conf.dsn, config,
        );
    let pool = DeadpoolPool::builder(mgr)
        .max_size(conf.max_open_connections as usize)
        .build()?;
    set_async_db_pool(pool);

    Ok(())
}

fn sqlite_establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncSqliteConnection>> {
    unimplemented!()
}

fn get_async_db_pool() -> Result<AsyncSqlitePool> {
    let pool_r = ASYNC_SQLITE_POOL.read().unwrap();
    let pool: AsyncSqlitePool = pool_r
        .as_ref()
        .ok_or_else(|| anyhow!("PostgreSQL connection pool is not initialized"))?
        .clone();
    Ok(pool)
}

pub async fn get_async_db_conn() -> Result<AsyncSqlitePoolConnection> {
    let pool = get_async_db_pool()?;

    let start = Instant::now();
    let res = pool.get().await?;

    STORAGE_SQLITE_CONN_GET.observe(start.elapsed().as_secs_f64());

    Ok(res)
}

fn set_async_db_pool(p: AsyncSqlitePool) {
    let mut pool_w = ASYNC_SQLITE_POOL.write().unwrap();
    *pool_w = Some(p);
}
