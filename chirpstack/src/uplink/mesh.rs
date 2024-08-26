use std::str::FromStr;

use anyhow::Result;
use chrono::{DateTime, Utc};
use tracing::{error, span, trace, warn, Instrument, Level};

use chirpstack_api::gw;

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{
    error::Error,
    gateway::{self, RelayId},
};
use lrwn::EUI64;

pub struct MeshHeartbeat {
    gateway_id: EUI64,
    relay_id: RelayId,
    mesh_stats: gw::MeshHeartbeat,
}

impl MeshHeartbeat {
    pub async fn handle(s: gw::MeshHeartbeat) {
        let gateway_id = match EUI64::from_str(&s.gateway_id) {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e.full(), "Decode gateway_id error");
                return;
            }
        };

        let relay_id = match RelayId::from_str(&s.relay_id) {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e.full(), "Decode relay_id error");
                return;
            }
        };

        let span = span!(Level::INFO, "mesh_stats", gateway_id = %gateway_id, relay_id = %relay_id);

        if let Err(e) = MeshHeartbeat::_handle(gateway_id, relay_id, s)
            .instrument(span)
            .await
        {
            match e.downcast_ref::<Error>() {
                Some(Error::NotFound(_)) => {
                    let conf = config::get();
                    if !conf.gateway.allow_unknown_gateways {
                        error!(error = %e.full(), "Handle mesh-stats error");
                    }
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle mesh-stats error");
                }
            }
        }
    }

    async fn _handle(gateway_id: EUI64, relay_id: RelayId, s: gw::MeshHeartbeat) -> Result<()> {
        let mut ctx = MeshHeartbeat {
            gateway_id,
            relay_id,
            mesh_stats: s,
        };

        ctx.update_or_create_relay_gateway().await?;

        Ok(())
    }

    async fn update_or_create_relay_gateway(&mut self) -> Result<()> {
        trace!("Getting Border Gateway");
        let border_gw = gateway::get(&self.gateway_id).await?;

        let ts: DateTime<Utc> = match &self.mesh_stats.time {
            Some(v) => (*v)
                .try_into()
                .map_err(|e| anyhow!("Convert time error: {}", e))?,
            None => {
                warn!("Stats message does not have time field set");
                return Ok(());
            }
        };

        match gateway::get_relay_gateway(border_gw.tenant_id.into(), self.relay_id).await {
            Ok(mut v) => {
                if let Some(last_seen_at) = v.last_seen_at {
                    if last_seen_at > ts {
                        warn!("Time is less than last seen timestamp, ignoring stats");
                        return Ok(());
                    }
                }

                v.last_seen_at = Some(ts);
                v.region_config_id = border_gw
                    .properties
                    .get("region_config_id")
                    .cloned()
                    .unwrap_or_default();
                gateway::update_relay_gateway(v).await?;
            }
            Err(_) => {
                let _ = gateway::create_relay_gateway(gateway::RelayGateway {
                    tenant_id: border_gw.tenant_id,
                    relay_id: self.relay_id,
                    name: self.relay_id.to_string(),
                    last_seen_at: Some(ts),
                    ..Default::default()
                })
                .await?;
            }
        }

        Ok(())
    }
}
