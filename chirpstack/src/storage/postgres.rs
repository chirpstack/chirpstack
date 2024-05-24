use std::sync::RwLock;
use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};

use crate::monitoring::prometheus;
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::deadpool::{Object as DeadpoolObject, Pool as DeadpoolPool};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::AsyncPgConnection;
use futures::{future::BoxFuture, FutureExt};
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};

use crate::config;

use crate::helpers::tls::get_root_certs;

pub type AsyncPgPool = DeadpoolPool<AsyncPgConnection>;
pub type AsyncPgPoolConnection = DeadpoolObject<AsyncPgConnection>;

lazy_static! {
    static ref ASYNC_PG_POOL: RwLock<Option<AsyncPgPool>> = RwLock::new(None);
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

pub fn setup(conf: &config::Postgresql) -> Result<()> {
    info!("Setting up PostgreSQL connection pool");
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(pg_establish_connection);
    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(&conf.dsn, config);
    let pool = DeadpoolPool::builder(mgr)
        .max_size(conf.max_open_connections as usize)
        .build()?;
    set_async_db_pool(pool);

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

fn get_async_db_pool() -> Result<AsyncPgPool> {
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

fn set_async_db_pool(p: AsyncPgPool) {
    let mut pool_w = ASYNC_PG_POOL.write().unwrap();
    *pool_w = Some(p);
}
