use std::convert::Infallible;
use std::net::SocketAddr;

use anyhow::{Context, Result};
use diesel_async::RunQueryDsl;
use tracing::info;
use warp::{http::Response, http::StatusCode, Filter};

use crate::config;
use crate::monitoring::prometheus;
use crate::storage::{get_async_db_conn, get_async_redis_conn};

pub async fn setup() {
    let conf = config::get();
    if conf.monitoring.bind.is_empty() {
        return;
    }

    let addr: SocketAddr = conf.monitoring.bind.parse().unwrap();
    info!(bind = %conf.monitoring.bind, "Setting up monitoring endpoint");

    let prom_endpoint = warp::get()
        .and(warp::path!("metrics"))
        .and_then(prometheus_handler);

    let health_endpoint = warp::get()
        .and(warp::path!("health"))
        .and_then(health_handler);

    let routes = prom_endpoint.or(health_endpoint);

    warp::serve(routes).run(addr).await;
}

async fn prometheus_handler() -> Result<impl warp::Reply, Infallible> {
    let body = prometheus::encode_to_string().unwrap_or_default();
    Ok(Response::builder().body(body))
}

async fn health_handler() -> Result<impl warp::Reply, Infallible> {
    if let Err(e) = _health_handler().await {
        return Ok(warp::reply::with_status(
            e.to_string(),
            StatusCode::SERVICE_UNAVAILABLE,
        ));
    }

    Ok(warp::reply::with_status("OK".to_string(), StatusCode::OK))
}

async fn _health_handler() -> Result<()> {
    diesel::sql_query("select 1")
        .execute(&mut get_async_db_conn().await?)
        .await
        .context("PostgreSQL connection error")?;

    let mut r = get_async_redis_conn().await?;
    let _: String = redis::cmd("PING").query_async(&mut r).await?;

    Ok(())
}
