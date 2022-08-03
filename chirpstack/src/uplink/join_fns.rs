use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use tracing::{span, trace, Instrument, Level};
use uuid::Uuid;

use super::{filter_rx_info_by_public_only, UplinkFrameSet};
use crate::api::backend::get_async_receiver;
use crate::backend::{joinserver, keywrap, roaming};
use crate::downlink;
use crate::storage::passive_roaming;
use crate::uplink::helpers;
use backend::Client;
use chirpstack_api::internal;
use lrwn::{JoinRequestPayload, NetID};

pub struct JoinRequest {
    uplink_frame_set: UplinkFrameSet,
    join_request: JoinRequestPayload,
    home_net_id: Option<NetID>,
    client: Option<Arc<Client>>,
    pr_start_ans: Option<backend::PRStartAnsPayload>,
}

impl JoinRequest {
    pub async fn start_pr(ufs: UplinkFrameSet, jr: JoinRequestPayload) -> Result<()> {
        let span = span!(Level::INFO, "start_pr");
        JoinRequest::_start_pr(ufs, jr).instrument(span).await
    }

    async fn _start_pr(ufs: UplinkFrameSet, jr: JoinRequestPayload) -> Result<()> {
        let mut ctx = JoinRequest {
            uplink_frame_set: ufs,
            join_request: jr,
            home_net_id: None,
            client: None,
            pr_start_ans: None,
        };

        ctx.filter_rx_info_by_public_only()?;
        ctx.get_home_net_id().await?;
        ctx.get_client()?;
        ctx.start_roaming().await?;
        ctx.save_roaming_session().await?;

        Ok(())
    }

    fn filter_rx_info_by_public_only(&mut self) -> Result<()> {
        trace!("Filtering rx_info by public gateways only");
        filter_rx_info_by_public_only(&mut self.uplink_frame_set)?;

        Ok(())
    }

    async fn get_home_net_id(&mut self) -> Result<()> {
        trace!("Getting home netid");

        trace!(join_eui = %self.join_request.join_eui, "Trying to get join-server client");
        let js_client = joinserver::get(&self.join_request.join_eui)?;

        let mut home_ns_req = backend::HomeNSReqPayload {
            dev_eui: self.join_request.dev_eui.to_vec(),
            ..Default::default()
        };

        #[cfg(test)]
        {
            home_ns_req.base.transaction_id = 1234;
        }

        let async_receiver = match js_client.is_async() {
            false => None,
            true => Some(
                get_async_receiver(
                    home_ns_req.base.transaction_id,
                    js_client.get_async_timeout(),
                )
                .await?,
            ),
        };

        trace!("Requesting home netid");
        let home_ns_ans = js_client
            .home_ns_req(&mut home_ns_req, async_receiver)
            .await?;
        self.home_net_id = Some(NetID::from_slice(&home_ns_ans.h_net_id)?);

        Ok(())
    }

    fn get_client(&mut self) -> Result<()> {
        let net_id = self.home_net_id.as_ref().unwrap();
        trace!(net_id = %net_id, "Getting backend interfaces client");
        self.client = Some(roaming::get(net_id)?);
        Ok(())
    }

    async fn start_roaming(&mut self) -> Result<()> {
        trace!("Starting passive-roaming");

        let mut pr_req = backend::PRStartReqPayload {
            phy_payload: self.uplink_frame_set.phy_payload.to_vec()?,
            ul_meta_data: backend::ULMetaData {
                dev_eui: self.join_request.dev_eui.to_vec(),
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

        let client = self.client.as_ref().unwrap();
        let async_receiver = match client.is_async() {
            false => None,
            true => Some(
                get_async_receiver(pr_req.base.transaction_id, client.get_async_timeout()).await?,
            ),
        };

        let resp = client
            .pr_start_req(backend::Role::SNS, &mut pr_req, async_receiver)
            .await?;

        if let Some(dl_meta) = &resp.dl_meta_data {
            downlink::roaming::PassiveRoamingDownlink::handle(
                self.uplink_frame_set.clone(),
                resp.phy_payload.clone(),
                dl_meta.clone(),
            )
            .await?;
        } else {
            return Err(anyhow!("DLMetaData is not set"));
        }

        self.pr_start_ans = Some(resp);
        Ok(())
    }

    async fn save_roaming_session(&mut self) -> Result<()> {
        trace!("Saving roaming-session");

        let pr_start_ans = self.pr_start_ans.as_ref().unwrap();

        if pr_start_ans.dev_addr.is_empty()
            || pr_start_ans.lifetime.is_none()
            || pr_start_ans.lifetime.unwrap() == 0
        {
            return Ok(());
        }

        let sess_id = Uuid::new_v4();

        let sess = internal::PassiveRoamingDeviceSession {
            session_id: sess_id.as_bytes().to_vec(),
            net_id: self.home_net_id.unwrap().to_vec(),
            dev_addr: pr_start_ans.dev_addr.clone(),
            dev_eui: self.join_request.dev_eui.to_vec(),
            lifetime: {
                let lt = pr_start_ans.lifetime.unwrap_or_default() as i64;
                if lt == 0 {
                    None
                } else {
                    Some((Utc::now() + Duration::seconds(lt)).into())
                }
            },
            lorawan_1_1: pr_start_ans.f_nwk_s_int_key.is_some(),

            f_nwk_s_int_key: match &pr_start_ans.f_nwk_s_int_key {
                Some(ke) => keywrap::unwrap(ke)?.to_vec(),
                None => match &pr_start_ans.nwk_s_key {
                    None => Vec::new(),
                    Some(ke) => keywrap::unwrap(ke)?.to_vec(),
                },
            },
            ..Default::default()
        };

        passive_roaming::save(&sess)
            .await
            .context("Save passive-roaming device-session")
    }
}
