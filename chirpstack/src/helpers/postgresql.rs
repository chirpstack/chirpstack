use std::str::FromStr;

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::AsyncPgConnection;
use tokio_postgres::NoTls;
use tokio_postgres::config::SslMode;
use tracing::error;

use crate::helpers::tls::get_root_certs;

fn bad_connection<E: std::fmt::Display>(err: E) -> ConnectionError {
    ConnectionError::BadConnection(err.to_string())
}

pub async fn establish_connection(config: &str, ca_cert: Option<String>) -> ConnectionResult<AsyncPgConnection> {
    let parsed = tokio_postgres::Config::from_str(config).map_err(bad_connection)?;
    let ssl_mode = parsed.get_ssl_mode();

    match ssl_mode {
        SslMode::Disable => {
            let (client, conn) = tokio_postgres::connect(config, NoTls)
                .await
                .map_err(bad_connection)?;

            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    error!(error = %e, "PostgreSQL connection error");
                }
            });

            AsyncPgConnection::try_from(client).await
        }
        _ => {
            let root_certs =
                get_root_certs(ca_cert.filter(|v| !v.is_empty())).map_err(bad_connection)?;
            let rustls_config = rustls::ClientConfig::builder()
                .with_root_certificates(root_certs)
                .with_no_client_auth();
            let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
            let (client, conn) = tokio_postgres::connect(config, tls)
                .await
                .map_err(bad_connection)?;

            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    error!(error = %e, "PostgreSQL connection error");
                }
            });

            AsyncPgConnection::try_from(client).await
        }
    }
}
