use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use diesel_async::RunQueryDsl;
use http::StatusCode;
use tracing::info;

use crate::config;
use crate::monitoring::prometheus;
use crate::storage::{get_async_db_conn, get_async_redis_conn};

pub async fn setup() -> Result<()> {
    let conf = config::get();
    if conf.monitoring.bind.is_empty() {
        return Ok(());
    }

    let addr: SocketAddr = conf.monitoring.bind.parse().unwrap();
    info!(bind = %conf.monitoring.bind, "Setting up monitoring endpoint");

    let app = Router::new()
        .route("/metrics", get(prometheus_handler))
        .route("/health", get(health_handler));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn prometheus_handler() -> Response {
    let body = prometheus::encode_to_string().unwrap_or_default();
    body.into_response()
}

async fn health_handler() -> Response {
    if let Err(e) = _health_handler().await {
        (StatusCode::SERVICE_UNAVAILABLE, e.to_string()).into_response()
    } else {
        (StatusCode::OK, "".to_string()).into_response()
    }
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
