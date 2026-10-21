use std::sync::{LazyLock, RwLock};
use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};

use crate::monitoring::prometheus;
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::{Object as DeadpoolObject, Pool as DeadpoolPool};
use diesel_async::pooled_connection::{
    AsyncDieselConnectionManager, ManagerConfig, RecyclingMethod,
};
use futures::{FutureExt, future::BoxFuture};
use prometheus_client::metrics::histogram::{Histogram, exponential_buckets};

use crate::config;
use crate::helpers::tls::{get_root_certs, load_cert, load_key};

pub type AsyncPgPool = DeadpoolPool<AsyncPgConnection>;
pub type AsyncPgPoolConnection = DeadpoolObject<AsyncPgConnection>;

static ASYNC_PG_POOL: LazyLock<RwLock<Option<AsyncPgPool>>> = LazyLock::new(|| RwLock::new(None));
static STORAGE_PG_CONN_GET: LazyLock<Histogram> = LazyLock::new(|| {
    let histogram = Histogram::new(exponential_buckets(0.001, 2.0, 12));
    prometheus::register(
        "storage_pg_conn_get_duration_seconds",
        "Time between requesting a PostgreSQL connection and the connection-pool returning it",
        histogram.clone(),
    );
    histogram
});

pub fn setup(conf: &config::Postgresql) -> Result<()> {
    info!("Setting up PostgreSQL connection pool");
    let mut config = ManagerConfig::default();

    let conf_clone = conf.clone();
    config.custom_setup = Box::new(move |_url| {
        pg_establish_connection(conf_clone.clone())
    });

    // Set recycling method based on configuration
    config.recycling_method = match conf.connection_recycling_method.to_lowercase().as_str() {
        "fast" => {
            info!("Using Fast connection recycling method (no validation query)");
            RecyclingMethod::Fast
        }
        "verified" => {
            info!("Using Verified connection recycling method (SELECT 1 validation query)");
            RecyclingMethod::Verified
        }
        _ => {
            error!(
                method = %conf.connection_recycling_method,
                "Invalid connection_recycling_method, defaulting to 'verified'. Valid options: 'fast', 'verified'"
            );
            RecyclingMethod::Verified
        }
    };

    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(&conf.dsn, config);
    let pool = DeadpoolPool::builder(mgr)
        .max_size(conf.max_open_connections as usize)
        .build()?;
    set_async_db_pool(pool);

    Ok(())
}

// Source:
// https://github.com/weiznich/diesel_async/blob/main/examples/postgres/pooled-with-rustls/src/main.rs
fn pg_establish_connection(conf: config::Postgresql) -> BoxFuture<'static, ConnectionResult<AsyncPgConnection>> {
    let fut = async move {
        let root_certs = get_root_certs(if conf.ca_cert.is_empty() {
            None
        } else {
            Some(conf.ca_cert.clone())
        }).map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

        let rustls_config = if !conf.tls_cert.is_empty() && !conf.tls_key.is_empty() {
            info!(
                "Configuring postgresql with client TLS certificate, ca_cert: {}, tls_cert: {}, tls_key: {}",
                conf.ca_cert, conf.tls_cert, conf.tls_key
            );

            rustls::ClientConfig::builder()
                .with_root_certificates(root_certs.clone())
                .with_client_auth_cert(
                    load_cert(&conf.tls_cert).await.map_err(|e| ConnectionError::BadConnection(e.to_string()))?,
                    load_key(&conf.tls_key).await.map_err(|e| ConnectionError::BadConnection(e.to_string()))?
            ).map_err(|e| ConnectionError::BadConnection(e.to_string()))?
        } else {
            rustls::ClientConfig::builder()
                 .with_root_certificates(root_certs.clone())
                .with_no_client_auth()
        };

        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = conf.dsn
            .parse::<tokio_postgres::Config>()
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?
            .application_name(if conf.tls_cert.is_empty() { "chirpstack".into() } else { conf.tls_cert.clone() })
            .connect(tls)
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
