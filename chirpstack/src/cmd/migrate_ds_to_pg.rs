use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::{debug, info};

use crate::storage::{self, device_session, error::Error, get_async_db_conn, schema::device};
use lrwn::{DevAddr, EUI64};

pub async fn run() -> Result<()> {
    storage::setup().await?;

    info!("Migrating device-sessions from Redis to PostgreSQL");
    info!("Getting DevEUIs from PostgreSQL without device-session");

    let dev_euis: Vec<EUI64> = device::dsl::device
        .select(device::dsl::dev_eui)
        .filter(device::dsl::device_session.is_null())
        .load(&mut get_async_db_conn().await?)
        .await?;

    info!(
        "There are {} devices in PostgreSQL without device-session set",
        dev_euis.len()
    );

    for dev_eui in &dev_euis {
        debug!(dev_eui = %dev_eui, "Migrating device-session");

        let ds = match device_session::get(dev_eui).await {
            Ok(v) => v,
            Err(e) => match e {
                Error::NotFound(_) => {
                    debug!(dev_eui = %dev_eui, "Device does not have a device-session");
                    continue;
                }
                _ => {
                    return Err(anyhow::Error::new(e));
                }
            },
        };

        storage::device::partial_update(
            *dev_eui,
            &storage::device::DeviceChangeset {
                dev_addr: Some(Some(DevAddr::from_slice(&ds.dev_addr)?)),
                device_session: Some(Some(ds.into())),
                ..Default::default()
            },
        )
        .await?;

        debug!(dev_eui = %dev_eui, "Device-session migrated");
    }

    Ok(())
}
