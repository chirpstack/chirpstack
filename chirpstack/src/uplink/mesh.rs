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

pub struct Mesh {
    gateway_id: EUI64,
    relay_id: RelayId,
    time: DateTime<Utc>,
    mesh_event: gw::MeshEvent,
}

impl Mesh {
    pub async fn handle(s: gw::MeshEvent) {
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

        let span = span!(Level::INFO, "mesh", gateway_id = %gateway_id, relay_id = %relay_id);

        if let Err(e) = Mesh::_handle(gateway_id, relay_id, s)
            .instrument(span)
            .await
        {
            match e.downcast_ref::<Error>() {
                Some(Error::NotFound(_)) => {
                    let conf = config::get();
                    if !conf.gateway.allow_unknown_gateways {
                        error!(error = %e.full(), "Handle mesh error");
                    }
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle mesh error");
                }
            }
        }
    }

    async fn _handle(gateway_id: EUI64, relay_id: RelayId, s: gw::MeshEvent) -> Result<()> {
        let ctx = Mesh {
            gateway_id,
            relay_id,
            time: s
                .time
                .ok_or_else(|| anyhow!("Time field is empty"))?
                .try_into()
                .map_err(|e| anyhow!("Covert time error: {}", e))?,
            mesh_event: s,
        };

        ctx.handle_events().await?;

        Ok(())
    }

    async fn handle_events(&self) -> Result<()> {
        trace!("Handling mesh events");

        for event in &self.mesh_event.events {
            match &event.event {
                Some(gw::mesh_event_item::Event::Proprietary(_)) | None => continue,
                Some(gw::mesh_event_item::Event::Heartbeat(v)) => self._handle_heartbeat(v).await?,
            }
        }

        Ok(())
    }

    async fn _handle_heartbeat(&self, _pl: &gw::MeshEventHeartbeat) -> Result<()> {
        trace!("Handling heartbeat event");

        let border_gw = gateway::get(&self.gateway_id).await?;

        match gateway::get_relay_gateway(border_gw.tenant_id.into(), self.relay_id).await {
            Ok(mut v) => {
                if let Some(last_seen_at) = v.last_seen_at {
                    if last_seen_at > self.time {
                        warn!("Time is less than last seen timestamp, ignoring heartbeat");
                        return Ok(());
                    }
                }

                v.last_seen_at = Some(self.time);
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
                    last_seen_at: Some(self.time),
                    ..Default::default()
                })
                .await?;
            }
        }

        Ok(())
    }
}
