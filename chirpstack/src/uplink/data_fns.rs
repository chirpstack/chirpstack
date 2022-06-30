use anyhow::Result;
use chrono::{Duration, Utc};
use tracing::{error, info, span, trace, Instrument, Level};
use uuid::Uuid;

use super::{error::Error, filter_rx_info_by_public_only, UplinkFrameSet};
use crate::api::backend::get_async_receiver;
use crate::backend::{keywrap, roaming};
use crate::storage::passive_roaming;
use crate::uplink::helpers;
use chirpstack_api::internal;
use lrwn::NetID;

pub struct Data {
    uplink_frame_set: UplinkFrameSet,
    mac_payload: lrwn::MACPayload,
    pr_device_sessions: Vec<internal::PassiveRoamingDeviceSession>,
}

impl Data {
    pub async fn handle(ufs: UplinkFrameSet, mac_pl: lrwn::MACPayload) {
        let span = span!(Level::INFO, "data_pr");
        if let Err(e) = Data::_handle(ufs, mac_pl).instrument(span).await {
            match e.downcast_ref::<Error>() {
                Some(Error::Abort) => {
                    // nothing to do
                }
                Some(_) | None => {
                    error!(error = %e, "Handle passive-roaming uplink error");
                }
            }
        }
    }

    async fn _handle(ufs: UplinkFrameSet, mac_pl: lrwn::MACPayload) -> Result<()> {
        let mut ctx = Data {
            uplink_frame_set: ufs,
            mac_payload: mac_pl,
            pr_device_sessions: Vec::new(),
        };

        ctx.filter_rx_info_by_public_only()?;
        ctx.get_pr_device_sessions().await?;
        ctx.start_pr_sessions().await?;
        ctx.forward_uplink_for_sessions().await?;
        ctx.save_pr_device_sessions().await?;

        Ok(())
    }

    fn filter_rx_info_by_public_only(&mut self) -> Result<()> {
        trace!("Filtering rx_info by public gateways only");
        filter_rx_info_by_public_only(&mut self.uplink_frame_set)?;
        Ok(())
    }

    async fn get_pr_device_sessions(&mut self) -> Result<()> {
        trace!("Getting passive-roaming device-sessions");
        self.pr_device_sessions =
            passive_roaming::get_for_phy_payload(&self.uplink_frame_set.phy_payload).await?;

        for ds in &mut self.pr_device_sessions {
            ds.f_cnt_up = self.mac_payload.fhdr.f_cnt + 1;
        }

        trace!(
            count = self.pr_device_sessions.len(),
            "Got passive-roaming device-sessions"
        );

        Ok(())
    }

    async fn start_pr_sessions(&mut self) -> Result<()> {
        // Skip this step when we already have active sessions.
        if !self.pr_device_sessions.is_empty() {
            return Ok(());
        }

        let net_ids = roaming::get_net_ids_for_dev_addr(self.mac_payload.fhdr.devaddr);

        trace!(net_ids = ?net_ids, "Got NetIDs");

        for net_id in net_ids {
            let ds = match self.start_pr_session(net_id).await {
                Ok(v) => v,
                Err(e) => {
                    error!(net_id = %net_id, error = %e, "Start passive-roaming error");
                    continue;
                }
            };

            // No need to store the device-session or call XmitDataReq when
            // lifetime is not set (stateless passive-roaming).
            if ds.lifetime.is_some() {
                self.pr_device_sessions.push(ds);
            }
        }

        Ok(())
    }

    async fn forward_uplink_for_sessions(&self) -> Result<()> {
        trace!("Forwarding uplink for passive-roaming sessions");

        for ds in &self.pr_device_sessions {
            let mut req = backend::XmitDataReqPayload {
                phy_payload: self.uplink_frame_set.phy_payload.to_vec()?,
                ul_meta_data: Some(backend::ULMetaData {
                    dev_addr: self.mac_payload.fhdr.devaddr.to_vec(),
                    data_rate: Some(self.uplink_frame_set.dr),
                    ul_freq: Some((self.uplink_frame_set.tx_info.frequency as f64) / 1_000_000.0),
                    recv_time: helpers::get_rx_timestamp_chrono(&self.uplink_frame_set.rx_info_set),
                    rf_region: self
                        .uplink_frame_set
                        .region_common_name
                        .to_string()
                        .replace('_', "-"),
                    gw_cnt: Some(self.uplink_frame_set.rx_info_set.len()),
                    gw_info: roaming::rx_info_to_gw_info(&self.uplink_frame_set.rx_info_set)?,
                    ..Default::default()
                }),
                ..Default::default()
            };

            let net_id = NetID::from_slice(&ds.net_id)?;
            let client = roaming::get(&net_id)?;
            let async_receiver = match client.is_async() {
                false => None,
                true => Some(
                    get_async_receiver(req.base.transaction_id, client.get_async_timeout()).await?,
                ),
            };

            if let Err(e) = client
                .xmit_data_req(backend::Role::SNS, &mut req, async_receiver)
                .await
            {
                error!(net_id = %net_id, error = %e, "XmitDataReq failed");
            }
        }

        Ok(())
    }

    async fn save_pr_device_sessions(&self) -> Result<()> {
        trace!("Saving passive-roaming device-sessions");

        for ds in &self.pr_device_sessions {
            passive_roaming::save(ds).await?;
        }

        Ok(())
    }

    async fn start_pr_session(
        &self,
        net_id: NetID,
    ) -> Result<internal::PassiveRoamingDeviceSession> {
        info!(net_id = %net_id, dev_addr = %self.mac_payload.fhdr.devaddr, "Starting passive-roaming session");

        let mut pr_req = backend::PRStartReqPayload {
            phy_payload: self.uplink_frame_set.phy_payload.to_vec()?,
            ul_meta_data: backend::ULMetaData {
                ul_freq: Some((self.uplink_frame_set.tx_info.frequency as f64) / 1_000_000.0),
                data_rate: Some(self.uplink_frame_set.dr),
                recv_time: helpers::get_rx_timestamp_chrono(&self.uplink_frame_set.rx_info_set),
                rf_region: self
                    .uplink_frame_set
                    .region_common_name
                    .to_string()
                    .replace('_', "-"),
                gw_cnt: Some(self.uplink_frame_set.rx_info_set.len()),
                gw_info: roaming::rx_info_to_gw_info(&self.uplink_frame_set.rx_info_set)?,
                ..Default::default()
            },
            ..Default::default()
        };

        #[cfg(test)]
        {
            pr_req.base.transaction_id = 1234;
        }

        let client = roaming::get(&net_id)?;
        let async_receiver = match client.is_async() {
            false => None,
            true => Some(
                get_async_receiver(pr_req.base.transaction_id, client.get_async_timeout()).await?,
            ),
        };

        let pr_start_ans = client
            .pr_start_req(backend::Role::SNS, &mut pr_req, async_receiver)
            .await?;
        let sess_id = Uuid::new_v4();

        Ok(internal::PassiveRoamingDeviceSession {
            session_id: sess_id.as_bytes().to_vec(),
            net_id: net_id.to_vec(),
            dev_addr: self.mac_payload.fhdr.devaddr.to_vec(),
            lifetime: {
                let lt = pr_start_ans.lifetime.unwrap_or_default() as i64;
                if lt == 0 {
                    None
                } else {
                    Some((Utc::now() + Duration::seconds(lt)).into())
                }
            },
            f_nwk_s_int_key: match &pr_start_ans.f_nwk_s_int_key {
                Some(ke) => keywrap::unwrap(ke)?.to_vec(),
                None => match &pr_start_ans.nwk_s_key {
                    None => Vec::new(),
                    Some(ke) => keywrap::unwrap(ke)?.to_vec(),
                },
            },
            f_cnt_up: pr_start_ans.f_cnt_up.unwrap_or_default(),
            ..Default::default()
        })
    }
}
