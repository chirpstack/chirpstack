use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::info;

use crate::storage::{self, device_gateway, get_async_db_conn, schema::device};
use chirpstack_api::internal;
use lrwn::EUI64;

pub async fn run() -> Result<()> {
    storage::setup().await?;

    info!("Migrating gateway <> device rx-info to device-session");
    info!("Getting DevEUIs from database");

    let dev_euis: Vec<EUI64> = device::table
        .select(device::dev_eui)
        .filter(device::device_session.is_not_null())
        .load(&mut get_async_db_conn().await?)
        .await?;

    for (i, dev_eui) in dev_euis.iter().enumerate() {
        if i % 1000 == 0 {
            info!(count = i, total_count = dev_euis.len(), "migrating data");
        }

        if let Ok(rx_info) = device_gateway::get_rx_info(dev_eui).await {
            let d = storage::device::get(dev_eui).await?;
            let mut ds = d.get_device_session()?.clone();

            // Skip if already migrated or filled
            if !ds.gateway_rx_info_history.is_empty() {
                continue;
            }

            ds.gateway_rx_info_history = vec![internal::GatewayRxInfoHistory {
                dr: rx_info.dr,
                items: rx_info
                    .items
                    .iter()
                    .map(|i| internal::GatewayRxInfoHistoryItem {
                        gateway_id: i.gateway_id.clone(),
                        rssi: i.rssi,
                        lora_snr: i.lora_snr,
                        antenna: i.antenna,
                        board: i.board,
                        context: i.context.clone(),
                        is_private_up: i.is_private_up,
                        is_private_down: i.is_private_down,
                        tenant_id: i.tenant_id.clone(),
                    })
                    .collect(),
            }];

            storage::device::partial_update(
                *dev_eui,
                &storage::device::DeviceChangeset {
                    device_session: Some(Some(storage::fields::DeviceSession::new(ds))),
                    ..Default::default()
                },
            )
            .await?;
        }
    }

    Ok(())
}
