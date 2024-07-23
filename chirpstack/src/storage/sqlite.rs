use std::sync::RwLock;
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
use scoped_futures::ScopedBoxFuture;
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};

use crate::config;

pub type AsyncSqlitePool = DeadpoolPool<SyncConnectionWrapper<SqliteConnection>>;
pub type AsyncSqlitePoolConnection = DeadpoolObject<SyncConnectionWrapper<SqliteConnection>>;

lazy_static! {
    static ref ASYNC_SQLITE_POOL: RwLock<Option<AsyncSqlitePool>> = RwLock::new(None);
    static ref STORAGE_SQLITE_CONN_GET: Histogram = {
        let histogram = Histogram::new(exponential_buckets(0.001, 2.0, 12));
        prometheus::register(
            "storage_sqlite_conn_get_duration_seconds",
            "Time between requesting a SQLite connection and the connection-pool returning it",
            histogram.clone(),
        );
        histogram
    };
}

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
            // A few settings need to be configure:
            // - enable foreign keys since it's off by default in sqlite
            // - set busy_timeout to avoid manually managing transaction business/contention
            // see https://sqlite.org/rescode.html#busy
            // - set user journal mode
            let conf = config::get();
            let pragmas = pragma_generate(&conf.sqlite);
            conn.batch_execute(&pragmas).map_err(|err| ConnectionError::BadConnection(err.to_string()))?;
            Ok(SyncConnectionWrapper::new(conn))
        },
    )
        .unwrap_or_else(|err| Err(ConnectionError::BadConnection(err.to_string())))
        .boxed()
}

fn pragma_generate(conf: &config::Sqlite) -> String {
    let timeout = conf.busy_timeout;
    let journal_pragma = match &conf.journal_mode {
        Some(mode) if !mode.is_empty () => format!("PRAGMA journal_mode = {mode};"),
        _ => String::from(""),
    };
    format!(
        r#"
        PRAGMA busy_timeout = {timeout};
        PRAGMA foreign_keys = ON;
        {journal_pragma}
        "#,
    )
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

pub async fn db_transaction<'a, R, E, F>(conn: &mut AsyncSqlitePoolConnection, callback: F) -> Result<R, E>
where
    F: for<'r> FnOnce(&'r mut SyncConnectionWrapper<SqliteConnection>) -> ScopedBoxFuture<'a, 'r, Result<R, E>> + Send + 'a,
    E: From<diesel::result::Error> + Send + 'a,
    R: Send + 'a,
{
    conn.immediate_transaction(callback).await
}

fn set_async_db_pool(p: AsyncSqlitePool) {
    let mut pool_w = ASYNC_SQLITE_POOL.write().unwrap();
    *pool_w = Some(p);
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_pragmas(expected: &[(&str, &str)], generated: &str) {
        let mut pragma_lines = generated.lines().map(str::trim).filter(|s| !s.is_empty());
        for (key, value) in expected {
            let expected_pragma = format!("PRAGMA {key} = {value};");
            assert_eq!(expected_pragma, pragma_lines.next().unwrap());
        }
        assert!(pragma_lines.next().is_none());
    }

    #[test]
    fn default_sqlite_pragmas() {
        let expected = &[
            ("busy_timeout", "1000"),
            ("foreign_keys", "ON"),
        ];
        let conf = config::Sqlite::default();
        check_pragmas(expected, &pragma_generate(&conf));
    }

    #[test]
    fn sqlite_pragmas_with_custom_timeout() {
        let expected = &[
            ("busy_timeout", "123"),
            ("foreign_keys", "ON"),
        ];
        let mut conf = config::Sqlite::default();
        conf.busy_timeout = 123;
        check_pragmas(expected, &pragma_generate(&conf));
    }

    #[test]
    fn sqlite_pragmas_with_journal_mode_is_applied() {
        let expected = &[
            ("busy_timeout", "1000"),
            ("foreign_keys", "ON"),
            ("journal_mode", "WAL"),
        ];
        let mut conf = config::Sqlite::default();
        conf.journal_mode = Some("WAL".into());
        check_pragmas(expected, &pragma_generate(&conf));
    }

    #[test]
    fn sqlite_pragmas_with_empty_journal_mode_is_ignored() {
        let expected = &[
            ("busy_timeout", "1000"),
            ("foreign_keys", "ON"),
        ];
        let mut conf = config::Sqlite::default();
        conf.journal_mode = Some("".into());
        check_pragmas(expected, &pragma_generate(&conf));
    }
}
