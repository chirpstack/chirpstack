use std::sync::{LazyLock, RwLock};
use std::time::Instant;

use anyhow::Result;
use tracing::info;

use crate::monitoring::prometheus;
use diesel::sqlite::SqliteConnection;
use diesel::{Connection, ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::deadpool::{Object as DeadpoolObject, Pool as DeadpoolPool};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
use scoped_futures::ScopedBoxFuture;

use crate::config;

pub type AsyncSqlitePool = DeadpoolPool<SyncConnectionWrapper<SqliteConnection>>;
pub type AsyncSqlitePoolConnection = DeadpoolObject<SyncConnectionWrapper<SqliteConnection>>;

static ASYNC_SQLITE_POOL: LazyLock<RwLock<Option<AsyncSqlitePool>>> =
    LazyLock::new(|| RwLock::new(None));
static STORAGE_SQLITE_CONN_GET: LazyLock<Histogram> = LazyLock::new(|| {
    let histogram = Histogram::new(exponential_buckets(0.001, 2.0, 12));
    prometheus::register(
        "storage_sqlite_conn_get_duration_seconds",
        "Time between requesting a SQLite connection and the connection-pool returning it",
        histogram.clone(),
    );
    histogram
});

pub fn setup(conf: &config::Sqlite) -> Result<()> {
    info!("Setting up SQLite connection pool");
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(sqlite_establish_connection);
    let mgr =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new_with_config(
            &conf.path, config,
        );
    let pool = DeadpoolPool::builder(mgr)
        .max_size(conf.max_open_connections as usize)
        .build()?;
    set_async_db_pool(pool);

    Ok(())
}

fn sqlite_establish_connection(
    url: &str,
) -> BoxFuture<ConnectionResult<SyncConnectionWrapper<SqliteConnection>>> {
    let url = url.to_string();
    tokio::task::spawn_blocking(
        move || -> ConnectionResult<SyncConnectionWrapper<SqliteConnection>> {
            let mut conn = SqliteConnection::establish(&url)?;

            use diesel::connection::SimpleConnection;
            let conf = config::get();
            let pragmas = &conf
                .sqlite
                .pragmas
                .iter()
                .map(|p| format!("PRAGMA {};", p))
                .collect::<Vec<String>>()
                .join("");
            conn.batch_execute(&pragmas)
                .map_err(|err| ConnectionError::BadConnection(err.to_string()))?;
            Ok(SyncConnectionWrapper::new(conn))
        },
    )
    .unwrap_or_else(|err| Err(ConnectionError::BadConnection(err.to_string())))
    .boxed()
}

fn get_async_db_pool() -> Result<AsyncSqlitePool> {
    let pool_r = ASYNC_SQLITE_POOL.read().unwrap();
    let pool: AsyncSqlitePool = pool_r
        .as_ref()
        .ok_or_else(|| anyhow!("SQLite connection pool is not initialized"))?
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

pub async fn db_transaction<'a, R, E, F>(
    conn: &mut AsyncSqlitePoolConnection,
    callback: F,
) -> Result<R, E>
where
    F: for<'r> FnOnce(
            &'r mut SyncConnectionWrapper<SqliteConnection>,
        ) -> ScopedBoxFuture<'a, 'r, Result<R, E>>
        + Send
        + 'a,
    E: From<diesel::result::Error> + Send + 'a,
    R: Send + 'a,
{
    conn.immediate_transaction(callback).await
}

fn set_async_db_pool(p: AsyncSqlitePool) {
    let mut pool_w = ASYNC_SQLITE_POOL.write().unwrap();
    *pool_w = Some(p);
}
