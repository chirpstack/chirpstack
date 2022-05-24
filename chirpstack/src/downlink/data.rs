use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rand::Rng;
use tracing::{span, trace, warn, Instrument, Level};

use crate::api::helpers::FromProto;
use crate::downlink::{classb, helpers};
use crate::gpstime::{ToDateTime, ToGpsTime};
use crate::storage;
use crate::storage::{
    application, device, device_gateway, device_profile, device_queue, device_session,
    downlink_frame, mac_command, tenant,
};
use crate::uplink::UplinkFrameSet;
use crate::{adr, config, gateway, integration, maccommand, region, sensitivity};
use chirpstack_api::{gw, integration as integration_pb, internal};
use lrwn::DevAddr;

struct DownlinkFrameItem {
    downlink_frame_item: gw::DownlinkFrameItem,
    remaining_payload_size: usize,
}

pub struct Data {
    uplink_frame_set: Option<UplinkFrameSet>,
    tenant: tenant::Tenant,
    application: application::Application,
    device_profile: device_profile::DeviceProfile,
    device: device::Device,
    device_session: internal::DeviceSession,
    network_conf: config::RegionNetwork,
    region_conf: Arc<Box<dyn lrwn::region::Region + Sync + Send>>,
    must_send: bool,
    must_ack: bool,
    mac_commands: Vec<lrwn::MACCommandSet>,
    device_gateway_rx_info: Option<internal::DeviceGatewayRxInfo>,
    downlink_gateway: Option<internal::DeviceGatewayRxInfoItem>,
    downlink_frame: gw::DownlinkFrame,
    downlink_frame_items: Vec<DownlinkFrameItem>,
    immediately: bool,
    device_queue_item: Option<device_queue::DeviceQueueItem>,
    more_device_queue_items: bool,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub async fn handle_response(
        ufs: UplinkFrameSet,
        dev_gw_rx_info: internal::DeviceGatewayRxInfo,
        tenant: tenant::Tenant,
        application: application::Application,
        device_profile: device_profile::DeviceProfile,
        device: device::Device,
        device_session: internal::DeviceSession,
        must_send: bool,
        must_ack: bool,
        mac_commands: Vec<lrwn::MACCommandSet>,
    ) -> Result<()> {
        let span = span!(Level::TRACE, "data_down", downlink_id = %ufs.uplink_set_id);

        Data::_handle_response(
            ufs,
            dev_gw_rx_info,
            tenant,
            application,
            device_profile,
            device,
            device_session,
            must_send,
            must_ack,
            mac_commands,
        )
        .instrument(span)
        .await
    }

    pub async fn handle_schedule_next_queue_item(device: device::Device) -> Result<()> {
        let span = span!(Level::TRACE, "schedule", dev_eui = %device.dev_eui);

        Data::_handle_schedule_next_queue_item(device)
            .instrument(span)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn _handle_response(
        ufs: UplinkFrameSet,
        dev_gw_rx_info: internal::DeviceGatewayRxInfo,
        tenant: tenant::Tenant,
        application: application::Application,
        device_profile: device_profile::DeviceProfile,
        device: device::Device,
        device_session: internal::DeviceSession,
        must_send: bool,
        must_ack: bool,
        mac_commands: Vec<lrwn::MACCommandSet>,
    ) -> Result<()> {
        trace!("Downlink response flow");

        let network_conf = config::get_region_network(&device_session.region_name)
            .context("Get network config for region")?;
        let region_conf =
            region::get(&device_session.region_name).context("Get region config for region")?;

        let mut ctx = Data {
            uplink_frame_set: Some(ufs),
            tenant,
            application,
            device_profile,
            device,
            device_session,
            network_conf,
            region_conf,
            must_send,
            must_ack,
            mac_commands,
            device_gateway_rx_info: Some(dev_gw_rx_info),
            downlink_gateway: None,
            downlink_frame: gw::DownlinkFrame {
                downlink_id: rand::thread_rng().gen(),
                ..Default::default()
            },
            downlink_frame_items: Vec::new(),
            immediately: false,
            device_queue_item: None,
            more_device_queue_items: false,
        };

        ctx.select_downlink_gateway()?;
        ctx.set_tx_info()?;
        ctx.get_next_device_queue_item().await?;
        ctx.set_mac_commands().await?;

        if ctx._something_to_send() {
            ctx.set_phy_payloads()?;
            ctx.update_device_queue_item().await?;
            ctx.save_downlink_frame().await?;
            ctx.send_downlink_frame().await?;
        }

        // Some mac-commands set their state (e.g. last requested) to the device-session.
        ctx.save_device_session().await?;

        Ok(())
    }

    async fn _handle_schedule_next_queue_item(dev: device::Device) -> Result<()> {
        trace!("Handle schedule next-queue item flow");

        let dp = device_profile::get(&dev.device_profile_id).await?;
        let app = application::get(&dev.application_id).await?;
        let ten = tenant::get(&app.tenant_id).await?;
        let ds = device_session::get(&dev.dev_eui).await?;
        let rc = region::get(&ds.region_name)?;
        let rn = config::get_region_network(&ds.region_name)?;
        let dev_gw = device_gateway::get_rx_info(&dev.dev_eui).await?;

        let mut ctx = Data {
            uplink_frame_set: None,
            tenant: ten,
            application: app,
            device_profile: dp,
            device: dev,
            device_session: ds,
            network_conf: rn,
            region_conf: rc,
            must_send: false,
            must_ack: false,
            mac_commands: vec![],
            device_gateway_rx_info: Some(dev_gw),
            downlink_gateway: None,
            downlink_frame: gw::DownlinkFrame {
                downlink_id: rand::thread_rng().gen(),
                ..Default::default()
            },
            downlink_frame_items: vec![],
            immediately: false,
            device_queue_item: None,
            more_device_queue_items: false,
        };

        ctx.select_downlink_gateway()?;
        if ctx._is_class_c() {
            ctx.get_class_c_device_lock().await?;
            ctx.set_immediately()?;
            ctx.set_tx_info_for_rx2()?;
        }
        if ctx._is_class_b() {
            ctx.set_tx_info_for_class_b_and_lock_device().await?;
        }
        if ctx._is_class_a() {
            return Err(anyhow!("Invalid device-class"));
        }
        ctx.get_next_device_queue_item().await?;
        if ctx._something_to_send() {
            ctx.set_phy_payloads()?;
            ctx.update_device_queue_item().await?;
            ctx.save_downlink_frame().await?;
            ctx.send_downlink_frame().await?;
        }

        Ok(())
    }

    fn select_downlink_gateway(&mut self) -> Result<()> {
        trace!("Selecting downlink gateway");

        let gw_down = helpers::select_downlink_gateway(
            &self.device_session.region_name,
            self.network_conf.gateway_prefer_min_margin,
            self.device_gateway_rx_info.as_mut().unwrap(),
        )?;

        self.downlink_frame.gateway_id = hex::encode(&gw_down.gateway_id);
        self.downlink_gateway = Some(gw_down);

        Ok(())
    }

    fn set_tx_info(&mut self) -> Result<()> {
        let mut prefer_rx2_over_rx1 = self._prefer_rx2_dr()?;
        if self.network_conf.rx2_prefer_on_link_budget {
            prefer_rx2_over_rx1 = prefer_rx2_over_rx1 || self._prefer_rx2_link_budget()?;
        }

        // RX2 is prefered and the RX window is set to automatic.
        if prefer_rx2_over_rx1 && self.network_conf.rx_window == 0 {
            // RX2
            self.set_tx_info_for_rx2()?;

            // RX1
            self._set_tx_info_for_rx1()?;
        } else {
            // RX1
            if [0, 1].contains(&self.network_conf.rx_window) {
                self._set_tx_info_for_rx1()?;
            }

            // RX2
            if [0, 2].contains(&self.network_conf.rx_window) {
                self.set_tx_info_for_rx2()?;
            }
        }

        Ok(())
    }

    async fn get_next_device_queue_item(&mut self) -> Result<()> {
        trace!("Getting next device queue-item");

        // sanity check
        if self.downlink_frame_items.is_empty() {
            return Err(anyhow!("downlink_frame_items is empty"));
        }

        // We use the first downlink opportunity to determine the max-payload size
        // for the downlink.
        let max_payload_size = self.downlink_frame_items[0].remaining_payload_size;

        // It might require a couple of iterations to get the device-queue item.
        loop {
            let (qi, more_in_queue) =
                match device_queue::get_next_for_dev_eui(&self.device.dev_eui).await {
                    Ok(v) => v,
                    Err(e) => match e {
                        // If no queue items could be found, do not return an error.
                        storage::error::Error::NotFound(_) => {
                            return Ok(());
                        }
                        _ => {
                            return Err(e).context("Get next queue-item");
                        }
                    },
                };

            // The queue item should fit within the max payload size and should not be pending
            // already.
            if qi.data.len() <= max_payload_size && !qi.is_pending {
                self.device_queue_item = Some(qi);
                self.more_device_queue_items = more_in_queue;
                return Ok(());
            }

            // If this point is reached, the downlink queue-item can not be used
            // because of one of the reasons below.
            let device_info = integration_pb::DeviceInfo {
                tenant_id: self.tenant.id.to_string(),
                tenant_name: self.tenant.name.clone(),
                application_id: self.application.id.to_string(),
                application_name: self.application.name.to_string(),
                device_profile_id: self.device_profile.id.to_string(),
                device_profile_name: self.device_profile.name.clone(),
                device_name: self.device.name.clone(),
                dev_eui: self.device.dev_eui.to_string(),
                tags: {
                    let mut tags = (&*self.device_profile.tags).clone();
                    tags.clone_from(&*self.device.tags);
                    tags
                },
            };

            // Handle unacknowledged pending downlink.
            // Note that get_next_for_dev_eui only returns pending queue-items when they have
            // expired. For pending queue-items that have not yet expired, a NotFound is returned.
            if qi.is_pending {
                device_queue::delete_item(&qi.id)
                    .await
                    .context("Delete device queue-item")?;

                let pl = integration_pb::AckEvent {
                    deduplication_id: match &self.uplink_frame_set {
                        Some(v) => v.uplink_set_id.to_string(),
                        None => "".to_string(),
                    },
                    time: Some(Utc::now().into()),
                    device_info: Some(device_info.clone()),
                    queue_item_id: qi.id.to_string(),
                    acknowledged: false,
                    f_cnt_down: match qi.f_cnt_down {
                        Some(v) => v as u32,
                        None => 0,
                    },
                };

                integration::ack_event(&self.application.id, &self.device.variables, &pl)
                    .await
                    .context("Publish ack event")?;

                warn!(dev_eui = %self.device.dev_eui, device_queue_item_id = %qi.id, "Device queue-item discarded because of timeout");

                continue;
            }

            // Handle payload size.
            if qi.data.len() > max_payload_size {
                device_queue::delete_item(&qi.id)
                    .await
                    .context("Delete device queue-item")?;

                let pl = integration_pb::LogEvent {
                    deduplication_id: match &self.uplink_frame_set {
                        Some(v) => v.uplink_set_id.to_string(),
                        None => "".to_string(),
                    },
                    time: Some(Utc::now().into()),
                    device_info: Some(device_info.clone()),
                    level: integration_pb::LogLevel::Error.into(),
                    code: integration_pb::LogCode::DownlinkPayloadSize.into(),
                    description:
                        "Device queue-item discarded because it exceeds the max. payload size"
                            .to_string(),
                    context: [
                        ("max_payload_size".to_string(), max_payload_size.to_string()),
                        ("item_size".to_string(), qi.data.len().to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                integration::log_event(&self.application.id, &self.device.variables, &pl)
                    .await
                    .context("Publish log event")?;

                warn!(dev_eui = %self.device.dev_eui, device_queue_item_id = %qi.id, "Device queue-item discarded because of max. payload size");

                continue;
            }
        }
    }

    async fn set_mac_commands(&mut self) -> Result<()> {
        // First we set all mac-commands. This does not take the max. payload size in mind, that
        // will be taken care of in one of the next steps.
        self._request_custom_channel_reconfiguration().await?;
        self._request_channel_mask_reconfiguration().await?;
        self._request_adr_change().await?;
        self._request_device_status()?;
        self._request_rejoin_param_setup().await?;
        self._set_ping_slot_parameters().await?;
        self._set_rx_parameters().await?;
        self._set_tx_parameters().await?;

        Ok(())
    }

    fn _something_to_send(&self) -> bool {
        // No device-queue item to send, no mac-commands to send, no ACK to send
        // in reply to a confirmed-uplink and no requirement to send an empty downlink
        // (e.g. in case of ADRACKReq).
        if self.device_queue_item.is_none()
            && self.mac_commands.is_empty()
            && !self.must_ack
            && !self.must_send
        {
            return false;
        }

        true
    }

    fn _is_class_a(&self) -> bool {
        &self.device.enabled_class == "A"
    }

    fn _is_class_b(&self) -> bool {
        &self.device.enabled_class == "B"
    }

    fn _is_class_c(&self) -> bool {
        &self.device.enabled_class == "C"
    }

    fn set_phy_payloads(&mut self) -> Result<()> {
        trace!("Setting downlink PHYPayloads");
        let mut f_pending = self.more_device_queue_items;

        for item in self.downlink_frame_items.iter_mut() {
            let mut mac_size: usize = 0;
            let mut mac_commands: Vec<lrwn::MACCommand> = Vec::new();

            // collect all mac-commands up to RemainingPayloadSize bytes.
            for mac_set in &self.mac_commands {
                // get size of mac-command block
                let s = mac_set.size().context("Get mac-command size")?;

                // break if it does not fit within the RemainingPayloadSize
                if (item.remaining_payload_size as isize) - (s as isize) < 0 {
                    break;
                }

                item.remaining_payload_size -= s;
                mac_size += s;

                for mac in &**mac_set {
                    mac_commands.push(mac.clone());
                }
            }

            // LoRaWAN MHDR
            let mut mhdr = lrwn::MHDR {
                m_type: lrwn::MType::UnconfirmedDataDown,
                major: lrwn::Major::LoRaWANR1,
            };

            // LoRaWAN MAC payload
            let mut mac_pl = lrwn::MACPayload {
                fhdr: lrwn::FHDR {
                    devaddr: lrwn::DevAddr::from_slice(&self.device_session.dev_addr)?,
                    f_cnt: self.device_session.n_f_cnt_down,
                    f_ctrl: lrwn::FCtrl {
                        adr: !self.network_conf.adr_disabled,
                        ack: self.must_ack,
                        f_pending,
                        ..Default::default()
                    },
                    f_opts: lrwn::MACCommandSet::new(vec![]),
                },
                f_port: None,
                frm_payload: None,
            };

            // In this case mac-commands are sent as FRMPayload. We will not be able to
            // send a device-queue item in this case.
            if mac_size > 15 {
                // Set the FPending to true if we were planning to send a downlink
                // device-queue item.
                mac_pl.fhdr.f_ctrl.f_pending = self.device_queue_item.is_some();

                // Set the mac-commands as FRMPayload.
                mac_pl.frm_payload = Some(lrwn::FRMPayload::MACCommandSet(
                    lrwn::MACCommandSet::new(mac_commands),
                ));

                // MAC-layer FPort.
                mac_pl.f_port = Some(0);

                // Network-server FCnt.
                mac_pl.fhdr.f_cnt = self.device_session.n_f_cnt_down;

                // Unset queue-item.
                self.device_queue_item = None;
            } else {
                // In this case mac-commands are sent using the FOpts field. In case there
                // is a device-queue item, we will validate if it still fits within the
                // RemainingPayloadSize.

                // Set the mac-commands as FOpts.
                mac_pl.fhdr.f_opts = lrwn::MACCommandSet::new(mac_commands);

                // Test if we still can send a device-queue item.
                if let Some(qi) = &self.device_queue_item {
                    if qi.data.len() <= item.remaining_payload_size {
                        // Set the device-queue item.
                        mac_pl.f_port = Some(qi.f_port as u8);
                        mac_pl.fhdr.f_cnt = if self
                            .device_session
                            .mac_version()
                            .to_string()
                            .starts_with("1.0")
                        {
                            self.device_session.n_f_cnt_down
                        } else {
                            self.device_session.a_f_cnt_down
                        };
                        mac_pl.frm_payload = Some(lrwn::FRMPayload::Raw(qi.data.clone()));

                        if qi.confirmed {
                            mhdr.m_type = lrwn::MType::ConfirmedDataDown;
                        }

                        item.remaining_payload_size -= qi.data.len();
                    } else {
                        f_pending = true;
                        mac_pl.fhdr.f_ctrl.f_pending = true;
                        self.device_queue_item = None;
                    }
                }
            }

            // Construct LoRaWAN PHYPayload.
            let mut phy = lrwn::PhyPayload {
                mhdr,
                payload: lrwn::Payload::MACPayload(mac_pl),
                mic: None,
            };

            // Encrypt FRMPayload.
            if mac_size > 15 {
                // Encrypt mac-commands.
                phy.encrypt_frm_payload(&lrwn::AES128Key::from_slice(
                    &self.device_session.nwk_s_enc_key,
                )?)
                .context("Encrypt frm_payload mac-commands")?;
            } else {
                // Encrypt application payload.
                if let Some(key_env) = &self.device_session.app_s_key {
                    let app_s_key = lrwn::AES128Key::from_slice(&key_env.aes_key)?;
                    phy.encrypt_frm_payload(&app_s_key)
                        .context("Encrypt frm_payload application payload")?;
                }
            }

            // Set MIC.
            // If this is an ACK, then FCntUp has already been incremented by one. If
            // this is not an ACK, then DownlinkDataMIC will zero out ConfFCnt.
            phy.set_downlink_data_mic(
                self.device_session.mac_version().from_proto(),
                self.device_session.f_cnt_up - 1,
                &lrwn::AES128Key::from_slice(&self.device_session.s_nwk_s_int_key)?,
            )
            .context("Set downlink data MIC")?;

            let b = phy.to_vec().context("Encode PHYPayload")?;
            item.downlink_frame_item.phy_payload = b;

            self.downlink_frame
                .items
                .push(item.downlink_frame_item.clone());
        }

        Ok(())
    }

    async fn update_device_queue_item(&mut self) -> Result<()> {
        trace!("Updating device queue-item");
        if let Some(qi) = &mut self.device_queue_item {
            // Note that the is_pending is set to true after a tx acknowledgement. If it would be
            // set to true at this point, the queue-item would be removed in the following Class-A
            // case:
            // * is_pending is set to true
            // * gateway sends negative tx ack (downlink is not transmitted)
            // * get_next_device_queue_item is called on next downlink
            // * as is_pending was already set to true, a negative ack event is sent
            //   and item is popped from the queue
            qi.f_cnt_down = Some(if self
                .device_session
                .mac_version()
                .to_string()
                .starts_with("1.0")
            {
                self.device_session.n_f_cnt_down
            } else {
                self.device_session.a_f_cnt_down
            } as i64);

            *qi = device_queue::update_item(qi.clone()).await?;
        }

        Ok(())
    }

    async fn save_downlink_frame(&self) -> Result<()> {
        trace!("Saving downlink frame");

        downlink_frame::save(&internal::DownlinkFrame {
            downlink_id: self.downlink_frame.downlink_id,
            dev_eui: self.device.dev_eui.to_be_bytes().to_vec(),
            device_queue_item_id: match &self.device_queue_item {
                Some(qi) => qi.id.as_bytes().to_vec(),
                None => vec![],
            },
            encrypted_fopts: self
                .device_session
                .mac_version()
                .to_string()
                .starts_with("1.1"),
            nwk_s_enc_key: self.device_session.nwk_s_enc_key.clone(),
            downlink_frame: Some(self.downlink_frame.clone()),
            ..Default::default()
        })
        .await
        .context("Save downlink frame")?;

        Ok(())
    }

    async fn send_downlink_frame(&self) -> Result<()> {
        trace!("Sending downlink frame");

        gateway::backend::send_downlink(&self.device_session.region_name, &self.downlink_frame)
            .await
            .context("Send downlink frame")?;

        Ok(())
    }

    async fn get_class_c_device_lock(&self) -> Result<()> {
        trace!("Getting Class-C device lock");
        let conf = config::get();

        device::get_lock(
            &self.device.dev_eui,
            chrono::Duration::from_std(conf.network.scheduler.class_c_lock_duration)?,
        )
        .await
        .context("Get device lock")?;

        Ok(())
    }

    fn set_immediately(&mut self) -> Result<()> {
        trace!("Setting immediately flag");
        self.immediately = true;
        Ok(())
    }

    async fn save_device_session(&self) -> Result<()> {
        trace!("Saving device-session");

        device_session::save(&self.device_session)
            .await
            .context("Save device-session")?;
        Ok(())
    }

    async fn _request_custom_channel_reconfiguration(&mut self) -> Result<()> {
        trace!("Requesting custom channel re-configuration");
        let mut wanted_channels: HashMap<usize, lrwn::region::Channel> = HashMap::new();

        for i in self.region_conf.get_user_defined_uplink_channel_indices() {
            let c = self.region_conf.get_uplink_channel(i)?;
            wanted_channels.insert(i, c);
        }

        // cleanup channels that do not exist anydmore
        // these will be disabled by the LinkADRReq channel-mask reconfiguration
        let ds_keys: Vec<usize> = self
            .device_session
            .extra_uplink_channels
            .keys()
            .map(|k| *k as usize)
            .collect();

        for k in &ds_keys {
            if !wanted_channels.contains_key(k) {
                self.device_session
                    .extra_uplink_channels
                    .remove(&(*k as u32));
            }
        }

        let current_channels: HashMap<usize, lrwn::region::Channel> = self
            .device_session
            .extra_uplink_channels
            .iter()
            .map(|(k, v)| {
                (
                    *k as usize,
                    lrwn::region::Channel {
                        frequency: v.frequency,
                        min_dr: v.min_dr as u8,
                        max_dr: v.max_dr as u8,
                        ..Default::default()
                    },
                )
            })
            .collect();

        if let Some(block) =
            maccommand::new_channel::request(3, &current_channels, &wanted_channels)
        {
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::NewChannelReq, &block)
                .await?;
            self.mac_commands.push(block);
        }

        Ok(())
    }

    // Note: this must come before ADR!
    async fn _request_channel_mask_reconfiguration(&mut self) -> Result<()> {
        trace!("Requesting channel-mask reconfiguration");
        let enabled_uplink_channel_indices: Vec<usize> = self
            .device_session
            .enabled_uplink_channel_indices
            .iter()
            .map(|i| *i as usize)
            .collect();

        let mut payloads = self
            .region_conf
            .get_link_adr_req_payloads_for_enabled_uplink_channel_indices(
                &enabled_uplink_channel_indices,
            );

        // Nothing to do.
        if payloads.is_empty() {
            return Ok(());
        }

        let last = payloads.last_mut().unwrap();
        last.tx_power = self.device_session.tx_power_index as u8;
        last.dr = self.device_session.dr as u8;
        last.redundancy.nb_rep = self.device_session.nb_trans as u8;

        let set = lrwn::MACCommandSet::new(
            payloads
                .iter()
                .cloned()
                .map(lrwn::MACCommand::LinkADRReq)
                .collect(),
        );

        mac_command::set_pending(&self.device.dev_eui, lrwn::CID::LinkADRReq, &set).await?;
        self.mac_commands.push(set);

        Ok(())
    }

    async fn _request_adr_change(&mut self) -> Result<()> {
        trace!("Requesting ADR change");

        if self.network_conf.adr_disabled {
            return Ok(());
        }

        let dr = self
            .region_conf
            .get_data_rate(self.uplink_frame_set.as_ref().unwrap().dr)?;

        let ufs = self.uplink_frame_set.as_ref().unwrap();

        let req = adr::Request {
            region_name: ufs.region_name.clone(),
            region_common_name: ufs.region_common_name,
            dev_eui: self.device.dev_eui,
            mac_version: self.device_profile.mac_version,
            reg_params_revision: self.device_profile.reg_params_revision,
            adr: self.device_session.adr,
            dr: self.uplink_frame_set.as_ref().unwrap().dr,
            tx_power_index: self.device_session.tx_power_index as u8,
            nb_trans: self.device_session.nb_trans as u8,
            max_tx_power_index: if self.device_session.max_supported_tx_power_index != 0 {
                self.device_session.max_supported_tx_power_index as u8
            } else {
                let mut max_tx_power_index: u8 = 0;
                for n in 0..16 {
                    if self.region_conf.get_tx_power_offset(n).is_ok() {
                        max_tx_power_index = n as u8;
                    }
                }
                max_tx_power_index
            },
            required_snr_for_dr: match dr {
                lrwn::region::DataRateModulation::Lora(params) => {
                    config::get_required_snr_for_sf(params.spreading_factor)?
                }
                _ => 0.0,
            },
            installation_margin: self.network_conf.installation_margin,
            min_dr: self.network_conf.min_dr,
            max_dr: self.network_conf.max_dr,
            uplink_history: self.device_session.uplink_adr_history.clone(),
        };

        let resp = adr::handle(&self.device_profile.adr_algorithm_id, &req).await;

        // The response values are different than the request values, thus we must
        // send a LinkADRReq to the device.
        if resp.dr != req.dr
            || resp.tx_power_index != req.tx_power_index
            || resp.nb_trans != req.nb_trans
        {
            let mut adr_set = false;
            for set in self.mac_commands.iter_mut() {
                let mut is_link_adr_set = false;

                for mac in &mut **set {
                    if let lrwn::MACCommand::LinkADRReq(pl) = mac {
                        pl.dr = resp.dr;
                        pl.tx_power = resp.tx_power_index;
                        pl.redundancy.nb_rep = resp.nb_trans;

                        adr_set = true;
                        is_link_adr_set = true;
                    }
                }

                if is_link_adr_set {
                    // We need to update the pending mac-command.
                    mac_command::set_pending(&self.device.dev_eui, lrwn::CID::LinkADRReq, set)
                        .await?;
                }
            }

            // There was no existing LinkADRReq to be sent, we need to construct a new one.
            if !adr_set {
                let mut ch_mask: [bool; 16] = [false; 16];
                let mut ch_mask_cntl: Option<u8> = None;

                for i in &self.device_session.enabled_uplink_channel_indices {
                    match ch_mask_cntl {
                        None => {
                            // set the chMaskCntl
                            ch_mask_cntl = Some((i / 16) as u8);
                        }
                        Some(v) => {
                            if v != (i / 16) as u8 {
                                // break the loop as we only need to send one block of channels
                                break;
                            }
                        }
                    }

                    ch_mask[(i % 16) as usize] = true;
                }

                let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                    lrwn::LinkADRReqPayload {
                        dr: resp.dr,
                        tx_power: resp.tx_power_index,
                        ch_mask: lrwn::ChMask::new(ch_mask),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: ch_mask_cntl.unwrap(),
                            nb_rep: resp.nb_trans,
                        },
                    },
                )]);

                mac_command::set_pending(&self.device.dev_eui, lrwn::CID::LinkADRReq, &set).await?;
                self.mac_commands.push(set);
            }
        }

        Ok(())
    }

    fn _request_device_status(&mut self) -> Result<()> {
        trace!("Requesting device-status");

        if self.device_profile.device_status_req_interval == 0 {
            return Ok(());
        }

        match &self.device_session.last_device_status_request {
            None => {
                self.mac_commands.push(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::DevStatusReq,
                ]));

                self.device_session.last_device_status_request = Some(Utc::now().into());
            }
            Some(ts) => {
                let ts: DateTime<Utc> = ts.clone().try_into()?;
                let req_interval = Duration::from_secs(60 * 60 * 24)
                    / self.device_profile.device_status_req_interval as u32;

                let cur_interval: Duration = (Utc::now() - ts).to_std()?;

                if cur_interval >= req_interval {
                    self.mac_commands.push(lrwn::MACCommandSet::new(vec![
                        lrwn::MACCommand::DevStatusReq,
                    ]));

                    self.device_session.last_device_status_request = Some(Utc::now().into());
                }
            }
        }

        Ok(())
    }

    async fn _request_rejoin_param_setup(&mut self) -> Result<()> {
        trace!("Requesting rejoin param setup");

        // Rejoin-request is disabled or device does not support LoRaWAN 1.1.
        if !self.network_conf.rejoin_request.enabled
            || self
                .device_session
                .mac_version()
                .to_string()
                .starts_with("1.0")
        {
            return Ok(());
        }

        if !self.device_session.rejoin_request_enabled
            || self.device_session.rejoin_request_max_count_n as u8
                != self.network_conf.rejoin_request.max_count_n
            || self.device_session.rejoin_request_max_time_n as u8
                != self.network_conf.rejoin_request.max_time_n
        {
            let set = maccommand::rejoin_param_setup::request(
                self.network_conf.rejoin_request.max_time_n,
                self.network_conf.rejoin_request.max_count_n,
            );
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::RejoinParamSetupReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        Ok(())
    }

    async fn _set_ping_slot_parameters(&mut self) -> Result<()> {
        trace!("Setting ping-slot parameters");

        if !self.device_profile.supports_class_b {
            return Ok(());
        }

        if self.device_session.class_b_ping_slot_dr as u8 != self.network_conf.class_b.ping_slot_dr
            || self.device_session.class_b_ping_slot_freq
                != self.network_conf.class_b.ping_slot_frequency
        {
            let set = maccommand::ping_slot_channel::request(
                self.network_conf.class_b.ping_slot_dr,
                self.network_conf.class_b.ping_slot_frequency,
            );
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::PingSlotChannelReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        Ok(())
    }

    async fn _set_rx_parameters(&mut self) -> Result<()> {
        trace!("Setting rx parameters");

        if self.device_session.rx2_frequency != self.network_conf.rx2_frequency
            || self.device_session.rx2_dr as u8 != self.network_conf.rx2_dr
            || self.device_session.rx1_dr_offset as u8 != self.network_conf.rx1_dr_offset
        {
            let set = maccommand::rx_param_setup::request(
                self.network_conf.rx1_dr_offset,
                self.network_conf.rx2_frequency,
                self.network_conf.rx2_dr,
            );
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::RxParamSetupReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        let rx1_delay = self.device_session.rx1_delay as u8;
        if rx1_delay != self.network_conf.rx1_delay {
            let set = maccommand::rx_timing_setup::request(self.network_conf.rx1_delay);
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::RxTimingSetupReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        Ok(())
    }

    async fn _set_tx_parameters(&mut self) -> Result<()> {
        trace!("Setting tx parameters");

        if !self
            .region_conf
            .implements_tx_param_setup(self.device_session.mac_version().from_proto())
        {
            return Ok(());
        }

        let uplink_eirp_index =
            lrwn::get_tx_param_setup_eirp_index(self.network_conf.uplink_max_eirp);

        if self.device_session.uplink_dwell_time_400ms != self.network_conf.uplink_dwell_time_400ms
            || self.device_session.downlink_dwell_time_400ms
                != self.network_conf.downlink_dwell_time_400ms
            || self.device_session.uplink_max_eirp_index as u8 != uplink_eirp_index
        {
            let set = maccommand::tx_param_setup::request(
                self.network_conf.uplink_dwell_time_400ms,
                self.network_conf.downlink_dwell_time_400ms,
                uplink_eirp_index,
            );
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::TxParamSetupReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        Ok(())
    }

    fn _set_tx_info_for_rx1(&mut self) -> Result<()> {
        trace!("Setting tx-info for RX1");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let mut tx_info = gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // get RX1 DR.
        let rx1_dr_index = self.region_conf.get_rx1_data_rate_index(
            self.uplink_frame_set.as_ref().unwrap().dr,
            self.device_session.rx1_dr_offset as usize,
        )?;
        let rx1_dr = self.region_conf.get_data_rate(rx1_dr_index)?;

        // set DR to tx_info.
        helpers::set_tx_info_data_rate(&mut tx_info, &rx1_dr)?;

        // set frequency
        tx_info.frequency = self.region_conf.get_rx1_frequency_for_uplink_frequency(
            self.uplink_frame_set.as_ref().unwrap().tx_info.frequency,
        )?;

        // set tx power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // set timestamp
        let delay = if self.device_session.rx1_delay > 0 {
            Duration::from_secs(self.device_session.rx1_delay as u64)
        } else {
            self.region_conf.get_defaults().rx1_delay
        };
        tx_info.timing = Some(gw::Timing {
            parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                delay: Some(pbjson_types::Duration::from(delay)),
            })),
        });

        // get remaining payload size
        let max_pl_size = self.region_conf.get_max_payload_size(
            self.device_session.mac_version().from_proto(),
            self.device_profile.reg_params_revision,
            rx1_dr_index,
        )?;

        self.downlink_frame_items.push(DownlinkFrameItem {
            downlink_frame_item: gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            },
            remaining_payload_size: max_pl_size.n,
        });

        Ok(())
    }

    fn set_tx_info_for_rx2(&mut self) -> Result<()> {
        trace!("Setting tx-info for RX2");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let mut tx_info = gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            frequency: self.device_session.rx2_frequency,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // Set DR to tx-info.
        let rx2_dr = self
            .region_conf
            .get_data_rate(self.device_session.rx2_dr as u8)?;
        helpers::set_tx_info_data_rate(&mut tx_info, &rx2_dr)?;

        // set tx power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // set timestamp
        if !self.immediately {
            let delay = if self.device_session.rx1_delay > 0 {
                Duration::from_secs(self.device_session.rx1_delay as u64 + 1)
            } else {
                self.region_conf.get_defaults().rx2_delay
            };

            tx_info.timing = Some(gw::Timing {
                parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                    delay: Some(pbjson_types::Duration::from(delay)),
                })),
            });
        }

        if self.immediately {
            tx_info.timing = Some(gw::Timing {
                parameters: Some(gw::timing::Parameters::Immediately(
                    gw::ImmediatelyTimingInfo {},
                )),
            });
        }

        // get remaining payload size
        let max_pl_size = self.region_conf.get_max_payload_size(
            self.device_session.mac_version().from_proto(),
            self.device_profile.reg_params_revision,
            self.device_session.rx2_dr as u8,
        )?;

        self.downlink_frame_items.push(DownlinkFrameItem {
            downlink_frame_item: gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            },
            remaining_payload_size: max_pl_size.n,
        });

        Ok(())
    }

    async fn set_tx_info_for_class_b_and_lock_device(&mut self) -> Result<()> {
        trace!("Setting tx-info for Class-B");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let mut tx_info = gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            frequency: self.device_session.class_b_ping_slot_freq,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // Set DR to tx-info.
        let ping_dr = self
            .region_conf
            .get_data_rate(self.device_session.class_b_ping_slot_dr as u8)?;
        helpers::set_tx_info_data_rate(&mut tx_info, &ping_dr)?;

        // set tx power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // set timing
        let now_gps_ts = Utc::now().to_gps_time() + chrono::Duration::seconds(1);
        let ping_slot_ts = classb::get_next_ping_slot_after(
            now_gps_ts,
            &DevAddr::from_slice(&self.device_session.dev_addr)?,
            self.device_session.class_b_ping_slot_nb as usize,
        )?;
        trace!(gps_time_now_ts = %now_gps_ts, ping_slot_ts = %ping_slot_ts, "Calculated ping-slot timestamp");
        tx_info.timing = Some(gw::Timing {
            parameters: Some(gw::timing::Parameters::GpsEpoch(gw::GpsEpochTimingInfo {
                time_since_gps_epoch: Some(pbjson_types::Duration::from(ping_slot_ts.to_std()?)),
            })),
        });

        let scheduler_run_after_ts = ping_slot_ts.to_date_time();
        // Try to aquire the device lock.
        device::get_lock(&self.device.dev_eui, scheduler_run_after_ts - Utc::now())
            .await
            .context("Get device lock")?;

        // Update the device next scheduler run.
        trace!(scheduler_run_after = %scheduler_run_after_ts, "Setting scheduler_run_after for device");
        self.device =
            device::set_scheduler_run_after(&self.device.dev_eui, Some(scheduler_run_after_ts))
                .await?;

        // Use default frequency if not configured. Based on the configured region this will use
        // channel-hopping.
        if tx_info.frequency == 0 {
            let beacon_ts = classb::get_beacon_start(ping_slot_ts);
            let freq = self.region_conf.get_ping_slot_frequency(
                DevAddr::from_slice(&self.device_session.dev_addr)?,
                beacon_ts.to_std()?,
            )?;
            tx_info.frequency = freq;
        }

        // get remaining payload size
        let max_pl_size = self.region_conf.get_max_payload_size(
            self.device_session.mac_version().from_proto(),
            self.device_profile.reg_params_revision,
            self.device_session.class_b_ping_slot_dr as u8,
        )?;

        self.downlink_frame_items.push(DownlinkFrameItem {
            downlink_frame_item: gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            },
            remaining_payload_size: max_pl_size.n,
        });

        Ok(())
    }

    fn _prefer_rx2_dr(&self) -> Result<bool> {
        // The device has not yet been updated to the network-server RX2 parameters
        // (using mac-commands). Do not prefer RX2 over RX1 in this case.
        if self.device_session.rx2_frequency != self.network_conf.rx2_frequency
            || self.device_session.rx2_dr != self.network_conf.rx2_dr as u32
            || self.device_session.rx1_dr_offset != self.network_conf.rx1_dr_offset as u32
            || self.device_session.rx1_delay != self.network_conf.rx1_delay as u32
        {
            return Ok(false);
        }

        // get rx1 data-rate
        let dr_rx1 = self.region_conf.get_rx1_data_rate_index(
            self.uplink_frame_set.as_ref().unwrap().dr,
            self.device_session.rx1_dr_offset as usize,
        )?;

        if dr_rx1 < self.network_conf.rx2_prefer_on_rx1_dr_lt {
            return Ok(true);
        }

        Ok(false)
    }

    fn _prefer_rx2_link_budget(&self) -> Result<bool> {
        // The device has not yet been updated to the network-server RX2 parameters
        // (using mac-commands). Do not prefer RX2 over RX1 in this case.
        if self.device_session.rx2_frequency != self.network_conf.rx2_frequency
            || self.device_session.rx2_dr != self.network_conf.rx2_dr as u32
            || self.device_session.rx1_dr_offset != self.network_conf.rx1_dr_offset as u32
            || self.device_session.rx1_delay != self.network_conf.rx1_delay as u32
        {
            return Ok(false);
        }

        // get rx1 data-rate
        let dr_rx1_index = self.region_conf.get_rx1_data_rate_index(
            self.uplink_frame_set.as_ref().unwrap().dr,
            self.device_session.rx1_dr_offset as usize,
        )?;

        let rx1_dr = self.region_conf.get_data_rate(dr_rx1_index)?;
        let rx2_dr = self
            .region_conf
            .get_data_rate(self.device_session.rx2_dr as u8)?;

        // the calculation below only applies for LORA modulation
        if let lrwn::region::DataRateModulation::Lora(rx1_dr) = rx1_dr {
            if let lrwn::region::DataRateModulation::Lora(rx2_dr) = rx2_dr {
                let tx_power_rx1 = if self.network_conf.downlink_tx_power != -1 {
                    self.network_conf.downlink_tx_power
                } else {
                    self.region_conf.get_downlink_tx_power(
                        self.region_conf.get_rx1_frequency_for_uplink_frequency(
                            self.uplink_frame_set.as_ref().unwrap().tx_info.frequency,
                        )?,
                    ) as i32
                };

                let tx_power_rx2 = if self.network_conf.downlink_tx_power != -1 {
                    self.network_conf.downlink_tx_power
                } else {
                    self.region_conf
                        .get_downlink_tx_power(self.device_session.rx2_frequency)
                        as i32
                };

                let link_budget_rx1 = sensitivity::calculate_link_budget(
                    rx1_dr.bandwidth,
                    6.0,
                    config::get_required_snr_for_sf(rx1_dr.spreading_factor)?,
                    tx_power_rx1 as f32,
                );

                let link_budget_rx2 = sensitivity::calculate_link_budget(
                    rx2_dr.bandwidth,
                    6.0,
                    config::get_required_snr_for_sf(rx2_dr.spreading_factor)?,
                    tx_power_rx2 as f32,
                );

                return Ok(link_budget_rx2 > link_budget_rx1);
            }
        }

        Ok(false)
    }
}
