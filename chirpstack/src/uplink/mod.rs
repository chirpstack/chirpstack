use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::Cursor;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use prost::Message;
use tokio::task;
use tokio::time::sleep;
use tracing::{debug, error, info, span, trace, warn, Instrument, Level};
use uuid::Uuid;

use crate::config;
use crate::framelog;
use crate::storage::{
    device, device_profile, error::Error as StorageError, gateway, get_redis_conn, redis_key,
};
use chirpstack_api::{api, common, gw, internal};
use lrwn::region::CommonName;
use lrwn::{ForwardUplinkReq, MType, PhyPayload, EUI64};

mod data;
mod data_fns;
pub mod data_sns;
mod error;
pub mod helpers;
pub mod join;
pub mod join_fns;
pub mod join_sns;
pub mod stats;

#[derive(Clone)]
pub struct RelayContext {
    pub req: ForwardUplinkReq,
    pub device: device::Device,
    pub device_profile: device_profile::DeviceProfile,
    pub device_session: internal::DeviceSession,
    pub must_ack: bool,
}

#[derive(Clone)]
pub struct UplinkFrameSet {
    pub uplink_set_id: Uuid,
    pub dr: u8,
    pub ch: usize,
    pub phy_payload: PhyPayload,
    pub tx_info: gw::UplinkTxInfo,
    pub rx_info_set: Vec<gw::UplinkRxInfo>,
    pub gateway_private_up_map: HashMap<EUI64, bool>,
    pub gateway_private_down_map: HashMap<EUI64, bool>,
    pub gateway_tenant_id_map: HashMap<EUI64, Uuid>,
    pub region_common_name: CommonName,
    pub region_config_id: String,
    pub roaming_meta_data: Option<RoamingMetaData>,
}

impl TryFrom<&UplinkFrameSet> for api::UplinkFrameLog {
    type Error = anyhow::Error;

    fn try_from(ufs: &UplinkFrameSet) -> std::result::Result<api::UplinkFrameLog, Self::Error> {
        let mut ufl = api::UplinkFrameLog {
            phy_payload: ufs.phy_payload.to_vec()?,
            tx_info: Some(ufs.tx_info.clone()),
            rx_info: ufs.rx_info_set.clone(),
            m_type: match ufs.phy_payload.mhdr.m_type {
                lrwn::MType::JoinRequest => common::MType::JoinRequest,
                lrwn::MType::RejoinRequest => common::MType::RejoinRequest,
                lrwn::MType::UnconfirmedDataUp => common::MType::UnconfirmedDataUp,
                lrwn::MType::ConfirmedDataUp => common::MType::ConfirmedDataUp,
                lrwn::MType::Proprietary => common::MType::Proprietary,
                _ => {
                    return Err(anyhow!(
                        "Unexpected m_type: {}",
                        ufs.phy_payload.mhdr.m_type
                    ));
                }
            }
            .into(),
            dev_addr: match &ufs.phy_payload.payload {
                lrwn::Payload::MACPayload(v) => v.fhdr.devaddr.to_string(),
                _ => "".to_string(),
            },
            dev_eui: match &ufs.phy_payload.payload {
                lrwn::Payload::JoinRequest(v) => v.dev_eui.to_string(),
                _ => "".to_string(),
            },
            time: None, // is set below
            plaintext_f_opts: false,
            plaintext_frm_payload: false,
        };

        for rx_info in &ufl.rx_info {
            if rx_info.time.is_some() {
                let time = rx_info.time.as_ref().unwrap();
                ufl.time = Some(prost_types::Timestamp {
                    seconds: time.seconds,
                    nanos: time.nanos,
                });
            }
        }

        if ufl.time.is_none() {
            ufl.time = Some(SystemTime::now().into());
        }

        Ok(ufl)
    }
}

#[derive(Clone)]
pub struct RoamingMetaData {
    pub base_payload: backend::BasePayload,
    pub ul_meta_data: backend::ULMetaData,
}

pub async fn deduplicate_uplink(event: gw::UplinkFrame) {
    if let Err(e) = _deduplicate_uplink(event).await {
        error!(error = %e, "Deduplication error");
    }
}

async fn _deduplicate_uplink(event: gw::UplinkFrame) -> Result<()> {
    let phy_str = hex::encode(&event.phy_payload);
    let tx_info_str = match &event.tx_info {
        Some(tx_info) => hex::encode(tx_info.encode_to_vec()),
        None => "".to_string(),
    };

    let key = redis_key(format!("up:collect:{}:{}", tx_info_str, phy_str));
    let lock_key = redis_key(format!("up:collect:{}:{}:lock", tx_info_str, phy_str));

    let dedup_delay = config::get().network.deduplication_delay;
    let mut dedup_ttl = dedup_delay * 2;
    if dedup_ttl < Duration::from_millis(200) {
        dedup_ttl = Duration::from_millis(200);
    }

    trace!(
        key = key.as_str(),
        "Adding uplink event to deduplication set"
    );
    deduplicate_put(&key, dedup_ttl, &event).await?;

    trace!(
        lock_key = lock_key.as_str(),
        "Requesting deduplication lock"
    );
    if deduplicate_locked(&lock_key, dedup_ttl).await? {
        trace!(
            lock_key = lock_key.as_str(),
            "Deduplication is already locked by an other process"
        );
        return Ok(());
    }

    trace!(
        key = key.as_str(),
        "Waiting for more uplink events to receive"
    );
    sleep(dedup_delay).await;

    trace!(key = key.as_str(), "Collecting received uplink events");
    let uplink = deduplicate_collect(&key).await?;

    let deduplication_id = Uuid::new_v4();
    let span = span!(Level::INFO, "up", deduplication_id = %deduplication_id);
    handle_uplink(deduplication_id, uplink)
        .instrument(span)
        .await?;

    Ok(())
}

async fn deduplicate_put(key: &str, ttl: Duration, event: &gw::UplinkFrame) -> Result<()> {
    task::spawn_blocking({
        let key = key.to_string();
        let event_b = event.encode_to_vec();
        move || -> Result<()> {
            let mut c = get_redis_conn()?;

            c.new_pipeline()
                .atomic()
                .cmd("SADD")
                .arg(&key)
                .arg(event_b)
                .ignore()
                .cmd("PEXPIRE")
                .arg(&key)
                .arg(ttl.as_millis() as usize)
                .ignore()
                .query(&mut c)?;

            Ok(())
        }
    })
    .await?
}

async fn deduplicate_locked(key: &str, ttl: Duration) -> Result<bool> {
    task::spawn_blocking({
        let key = key.to_string();
        move || -> Result<bool> {
            let mut c = get_redis_conn()?;

            let set: bool = redis::cmd("SET")
                .arg(key)
                .arg("lock")
                .arg("PX")
                .arg(ttl.as_millis() as usize)
                .arg("NX")
                .query(&mut *c)?;

            Ok(!set)
        }
    })
    .await?
}

async fn deduplicate_collect(key: &str) -> Result<gw::UplinkFrameSet> {
    task::spawn_blocking({
        let key = key.to_string();
        move || -> Result<gw::UplinkFrameSet> {
            let mut c = get_redis_conn()?;
            let items_b: Vec<Vec<u8>> = redis::cmd("SMEMBERS").arg(&key).query(&mut *c)?;

            if items_b.is_empty() {
                return Err(anyhow!("Zero items in collect set"));
            }

            let mut pl = gw::UplinkFrameSet {
                ..Default::default()
            };

            for b in items_b {
                let event = gw::UplinkFrame::decode(&mut Cursor::new(b))?;

                if event.tx_info.is_none() {
                    warn!("tx_info of uplink event is empty, skipping");
                    continue;
                }
                if event.rx_info.is_none() {
                    warn!("rx_info of uplink event is empty, skipping");
                    continue;
                }

                pl.tx_info = event.tx_info;
                pl.rx_info.push(event.rx_info.unwrap());
                pl.phy_payload = event.phy_payload;
            }

            Ok(pl)
        }
    })
    .await?
}

pub async fn handle_uplink(deduplication_id: Uuid, uplink: gw::UplinkFrameSet) -> Result<()> {
    let rx_info = &uplink
        .rx_info
        .get(0)
        .context("Unable to get first item from rx_info")?;

    let region_config_id = rx_info
        .metadata
        .get("region_config_id")
        .cloned()
        .unwrap_or_default();

    let common_name = rx_info
        .metadata
        .get("region_common_name")
        .cloned()
        .unwrap_or_default();

    let common_name = CommonName::from_str(&common_name)?;

    let mut uplink = UplinkFrameSet {
        uplink_set_id: deduplication_id,
        region_config_id,
        region_common_name: common_name,
        dr: 0,
        ch: 0,
        phy_payload: PhyPayload::from_slice(&uplink.phy_payload)?,
        tx_info: uplink.tx_info.context("tx_info must not be None")?,
        rx_info_set: uplink.rx_info,
        gateway_private_up_map: HashMap::new(),
        gateway_private_down_map: HashMap::new(),
        gateway_tenant_id_map: HashMap::new(),
        roaming_meta_data: None,
    };

    uplink.dr = helpers::get_uplink_dr(&uplink.region_config_id, &uplink.tx_info)?;
    uplink.ch = helpers::get_uplink_ch(
        &uplink.region_config_id,
        uplink.tx_info.frequency,
        uplink.dr,
    )?;

    info!(
        m_type = uplink.phy_payload.mhdr.m_type.to_string().as_str(),
        "Uplink received"
    );

    debug!("Updating gateway meta-data for uplink frame-set");
    update_gateway_metadata(&mut uplink).await?;

    debug!("Logging uplink frame to Redis Stream");
    let ufl: api::UplinkFrameLog = (&uplink).try_into()?;
    framelog::log_uplink_for_gateways(&ufl)
        .await
        .context("log_uplink_for_gateways error")?;

    match uplink.phy_payload.mhdr.m_type {
        MType::JoinRequest => join::JoinRequest::handle(uplink).await,
        MType::UnconfirmedDataUp | MType::ConfirmedDataUp => data::Data::handle(uplink).await,
        _ => {
            return Err(anyhow!(
                "Unexpected m_type: {}",
                uplink.phy_payload.mhdr.m_type
            ))
        }
    }

    Ok(())
}

async fn update_gateway_metadata(ufs: &mut UplinkFrameSet) -> Result<()> {
    let conf = config::get();
    for rx_info in &mut ufs.rx_info_set {
        let gw_id = EUI64::from_str(&rx_info.gateway_id)?;
        let gw_meta = match gateway::get_meta(&gw_id).await {
            Ok(v) => v,
            Err(e) => {
                if conf.gateway.allow_unknown_gateways {
                    if let StorageError::NotFound(_) = e {
                        ufs.gateway_private_up_map.insert(gw_id, false);
                        ufs.gateway_private_down_map.insert(gw_id, false);
                        continue;
                    }
                }

                error!(
                    gateway_id = gw_id.to_string().as_str(),
                    error = format!("{}", e).as_str(),
                    "Getting gateway meta-data failed"
                );
                continue;
            }
        };

        // Do not overwrite the location if it is already set. In case of a 'virtual' it is
        // possible that the location is already set in the RxInfo. Overwriting this with the
        // location of the 'virtual' gateway would mean we will get the wrong location.
        if rx_info.location.is_none() {
            rx_info.location = Some(common::Location {
                latitude: gw_meta.latitude,
                longitude: gw_meta.longitude,
                altitude: gw_meta.altitude as f64,
                ..Default::default()
            });
        }

        ufs.gateway_private_up_map
            .insert(gw_id, gw_meta.is_private_up);
        ufs.gateway_private_down_map
            .insert(gw_id, gw_meta.is_private_down);
        ufs.gateway_tenant_id_map.insert(gw_id, gw_meta.tenant_id);
    }

    Ok(())
}

fn filter_rx_info_by_tenant_id(tenant_id: Uuid, uplink: &mut UplinkFrameSet) -> Result<()> {
    let mut rx_info_set: Vec<gw::UplinkRxInfo> = Vec::new();

    for rx_info in &uplink.rx_info_set {
        let gateway_id = EUI64::from_str(&rx_info.gateway_id)?;
        let region_config_id = rx_info
            .metadata
            .get("region_config_id")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("No region_config_id in rx_info metadata"))?;
        let force_gws_private = config::get_force_gws_private(&region_config_id)?;

        if !(uplink
            .gateway_private_up_map
            .get(&gateway_id)
            .cloned()
            .unwrap_or(true)
            || force_gws_private)
            || uplink
                .gateway_tenant_id_map
                .get(&gateway_id)
                .cloned()
                .unwrap_or_else(Uuid::new_v4)
                == tenant_id
        {
            rx_info_set.push(rx_info.clone());
        }
    }

    uplink.rx_info_set = rx_info_set;
    if uplink.rx_info_set.is_empty() {
        return Err(anyhow!("RxInfo set is empty after applying filters"));
    }

    Ok(())
}

fn filter_rx_info_by_public_only(uplink: &mut UplinkFrameSet) -> Result<()> {
    let mut rx_info_set: Vec<gw::UplinkRxInfo> = Vec::new();

    for rx_info in &uplink.rx_info_set {
        let gateway_id = EUI64::from_str(&rx_info.gateway_id)?;
        if !(*uplink
            .gateway_private_up_map
            .get(&gateway_id)
            .ok_or_else(|| anyhow!("gateway_id missing in gateway_private_up_map"))?)
        {
            rx_info_set.push(rx_info.clone());
        }
    }

    uplink.rx_info_set = rx_info_set;
    if uplink.rx_info_set.is_empty() {
        return Err(anyhow!("rx_info_set has no items"));
    }

    Ok(())
}

fn filter_rx_info_by_region_config_id(
    region_config_id: &str,
    uplink: &mut UplinkFrameSet,
) -> Result<()> {
    let mut rx_info_set: Vec<gw::UplinkRxInfo> = Vec::new();

    for rx_info in &uplink.rx_info_set {
        if let Some(v) = rx_info.metadata.get("region_config_id") {
            if v == region_config_id {
                rx_info_set.push(rx_info.clone());
            }
        }
    }

    uplink.rx_info_set = rx_info_set;
    if uplink.rx_info_set.is_empty() {
        return Err(anyhow!("rx_info_set has no items"));
    }

    Ok(())
}
