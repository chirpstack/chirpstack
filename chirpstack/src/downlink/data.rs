use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rand::Rng;
use tracing::{error, span, trace, warn, Instrument, Level};

use crate::api::backend::get_async_receiver;
use crate::api::helpers::{FromProto, ToProto};
use crate::backend::roaming;
use crate::downlink::{classb, helpers, tx_ack};
use crate::gpstime::{ToDateTime, ToGpsTime};
use crate::storage;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_gateway, device_profile, device_queue, device_session, downlink_frame, mac_command,
    relay, tenant,
};
use crate::uplink::{RelayContext, UplinkFrameSet};
use crate::{adr, config, gateway, integration, maccommand, region, sensitivity};
use chirpstack_api::{gw, integration as integration_pb, internal};
use lrwn::{keys, AES128Key, DevAddr, NetID};

struct DownlinkFrameItem {
    downlink_frame_item: gw::DownlinkFrameItem,
    remaining_payload_size: usize,
}

pub struct Data {
    relay_context: Option<RelayContext>,
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

    #[allow(clippy::too_many_arguments)]
    pub async fn handle_response_relayed(
        relay_ctx: RelayContext,
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

        Data::_handle_response_relayed(
            relay_ctx,
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

        let network_conf = config::get_region_network(&device_session.region_config_id)
            .context("Get network config for region")?;
        let region_conf = region::get(&device_session.region_config_id)
            .context("Get region config for region")?;

        let mut ctx = Data {
            relay_context: None,
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
            if ctx._is_roaming() {
                ctx.save_device_session().await?;
                ctx.send_downlink_frame_passive_roaming().await?;
                ctx.handle_passive_roaming_tx_ack().await?;
            } else {
                // Some mac-commands set their state (e.g. last requested) to the device-session.
                ctx.save_device_session().await?;
                ctx.send_downlink_frame().await?;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn _handle_response_relayed(
        relay_ctx: RelayContext,
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
        trace!("Downlink relayed response flow");

        let network_conf = config::get_region_network(&device_session.region_config_id)
            .context("Get network config for region")?;
        let region_conf = region::get(&device_session.region_config_id)
            .context("Get region config for region")?;

        let mut ctx = Data {
            relay_context: Some(relay_ctx),
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
        ctx.set_tx_info_relayed()?;
        ctx.get_next_device_queue_item().await?;
        ctx.set_mac_commands().await?;
        if ctx._something_to_send() {
            ctx.set_phy_payloads()?;
            ctx.wrap_phy_payloads_in_forward_downlink_req()?;
            ctx.save_downlink_frame_relayed().await?;
            ctx.save_device_session().await?;
            ctx.send_downlink_frame().await?;
        }

        Ok(())
    }

    async fn _handle_schedule_next_queue_item(dev: device::Device) -> Result<()> {
        trace!("Handle schedule next-queue item flow");

        let dp = device_profile::get(&dev.device_profile_id).await?;
        let app = application::get(&dev.application_id).await?;
        let ten = tenant::get(&app.tenant_id).await?;
        let ds = device_session::get(&dev.dev_eui).await?;
        let rc = region::get(&ds.region_config_id)?;
        let rn = config::get_region_network(&ds.region_config_id)?;
        let dev_gw = device_gateway::get_rx_info(&dev.dev_eui).await?;

        let mut ctx = Data {
            relay_context: None,
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
            ctx.check_for_first_uplink()?;
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
            Some(self.tenant.id),
            &self.device_session.region_config_id,
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
            self.set_tx_info_for_rx1()?;
        } else {
            // RX1
            if [0, 1].contains(&self.network_conf.rx_window) {
                self.set_tx_info_for_rx1()?;
            }

            // RX2
            if [0, 2].contains(&self.network_conf.rx_window) {
                self.set_tx_info_for_rx2()?;
            }
        }

        Ok(())
    }

    fn set_tx_info_relayed(&mut self) -> Result<()> {
        let mut prefer_rx2_over_rx1 = self._prefer_rx2_dr()?;
        if self.network_conf.rx2_prefer_on_link_budget {
            prefer_rx2_over_rx1 = prefer_rx2_over_rx1 || self._prefer_rx2_link_budget()?;
        }

        // RX2 is prefered and the RX window is set to automatic.
        if prefer_rx2_over_rx1 && self.network_conf.rx_window == 0 {
            // RX2
            self.set_tx_info_for_rx2_relayed()?;

            // RX1
            self.set_tx_info_for_rx1_relayed()?;
        } else {
            // RX1
            if [0, 1].contains(&self.network_conf.rx_window) {
                self.set_tx_info_for_rx1_relayed()?;
            }

            // RX2
            if [0, 2].contains(&self.network_conf.rx_window) {
                self.set_tx_info_for_rx2_relayed()?;
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

            // The queue item:
            // * should fit within the max payload size
            // * should not be pending
            // * in case encrypted, should have a valid FCntDown
            if qi.data.len() <= max_payload_size
                && !qi.is_pending
                && !(qi.is_encrypted
                    && (qi.f_cnt_down.unwrap_or_default() as u32)
                        < self.device_session.get_a_f_cnt_down())
            {
                trace!(id = %qi.id, more_in_queue = more_in_queue, "Found device queue-item for downlink");
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
                device_class_enabled: self.device.enabled_class.to_proto().into(),
                dev_eui: self.device.dev_eui.to_string(),
                tags: {
                    let mut tags = (*self.device_profile.tags).clone();
                    tags.extend((*self.device.tags).clone());
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

                integration::ack_event(self.application.id, &self.device.variables, &pl).await;
                warn!(dev_eui = %self.device.dev_eui, device_queue_item_id = %qi.id, "Device queue-item discarded because of timeout");

                continue;
            }

            // Handle payload size.
            if qi.data.len() > max_payload_size {
                device_queue::delete_item(&qi.id)
                    .await
                    .context("Delete device queue-item")?;

                let pl = integration_pb::LogEvent {
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
                        ("queue_item_id".to_string(), qi.id.to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                integration::log_event(self.application.id, &self.device.variables, &pl).await;
                warn!(dev_eui = %self.device.dev_eui, device_queue_item_id = %qi.id, "Device queue-item discarded because of max. payload size");

                continue;
            }

            // Handling encrypted payload with invalid FCntDown
            if qi.is_encrypted
                && (qi.f_cnt_down.unwrap_or_default() as u32)
                    < self.device_session.get_a_f_cnt_down()
            {
                device_queue::delete_item(&qi.id)
                    .await
                    .context("Delete device queue-item")?;

                let pl = integration_pb::LogEvent {
                    time: Some(Utc::now().into()),
                    device_info: Some(device_info.clone()),
                    level: integration_pb::LogLevel::Error.into(),
                    code: integration_pb::LogCode::FCntDown.into(),
                    description: "Device queue-item discarded because the frame-counter is invalid"
                        .to_string(),
                    context: [
                        (
                            "device_f_cnt_down".to_string(),
                            self.device_session.get_a_f_cnt_down().to_string(),
                        ),
                        (
                            "queue_item_f_cnt_down".to_string(),
                            qi.f_cnt_down.unwrap_or_default().to_string(),
                        ),
                        ("queue_item_id".to_string(), qi.id.to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                integration::log_event(self.application.id, &self.device.variables, &pl).await;
                warn!(dev_eui = %self.device.dev_eui, device_queue_item_id = %qi.id, "Device queue-item discarded because of invalid frame-counter");

                continue;
            }
        }
    }

    async fn set_mac_commands(&mut self) -> Result<()> {
        let conf = config::get();
        if conf.network.mac_commands_disabled {
            return Ok(());
        }

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

        if self.device_profile.is_relay {
            self._update_relay_conf().await?;
            self._update_filter_list().await?;
            self._update_uplink_list().await?;
            self._request_ctrl_uplink_list().await?;
            self._configure_fwd_limit_req().await?;
        }

        if self.device_profile.is_relay_ed {
            self._update_end_device_conf().await?;
        }

        self.mac_commands = filter_mac_commands(&self.device_session, &self.mac_commands);

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
        self.device.enabled_class == DeviceClass::A
    }

    fn _is_class_b(&self) -> bool {
        self.device.enabled_class == DeviceClass::B
    }

    fn _is_class_c(&self) -> bool {
        self.device.enabled_class == DeviceClass::C
    }

    fn _is_roaming(&self) -> bool {
        self.uplink_frame_set
            .as_ref()
            .unwrap()
            .roaming_meta_data
            .is_some()
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
                        mac_pl.fhdr.f_cnt = match qi.is_encrypted {
                            true => qi.f_cnt_down.unwrap_or_default() as u32,
                            false => self.device_session.get_a_f_cnt_down(),
                        };
                        mac_pl.frm_payload = Some(lrwn::FRMPayload::Raw(qi.data.clone()));

                        if qi.confirmed {
                            mhdr.m_type = lrwn::MType::ConfirmedDataDown;
                        }

                        item.remaining_payload_size -= qi.data.len();
                    } else {
                        f_pending = true;
                        mac_pl.fhdr.f_ctrl.f_pending = true;
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
            let qi_encrypted = match &self.device_queue_item {
                Some(v) => v.is_encrypted,
                None => false,
            };

            if mac_size > 15 {
                // Encrypt mac-commands.
                phy.encrypt_frm_payload(&lrwn::AES128Key::from_slice(
                    &self.device_session.nwk_s_enc_key,
                )?)
                .context("Encrypt frm_payload mac-commands")?;
            } else if self.device_queue_item.is_some() && !qi_encrypted {
                // Encrypt application payload.
                if let Some(key_env) = &self.device_session.app_s_key {
                    let app_s_key = lrwn::AES128Key::from_slice(&key_env.aes_key)?;
                    phy.encrypt_frm_payload(&app_s_key)
                        .context("Encrypt frm_payload application payload")?;
                }
            }

            // Encrypt f_opts mac-commands (LoRaWAN 1.1)
            if !self
                .device_session
                .mac_version()
                .to_string()
                .starts_with("1.0")
            {
                phy.encrypt_f_opts(&lrwn::AES128Key::from_slice(
                    &self.device_session.nwk_s_enc_key,
                )?)
                .context("Encrypt f_opts")?;
            }

            // Set MIC.
            // If this is an ACK, then FCntUp has already been incremented by one. If
            // this is not an ACK, then DownlinkDataMIC will zero out ConfFCnt.
            phy.set_downlink_data_mic(
                self.device_session.mac_version().from_proto(),
                self.device_session.f_cnt_up.overflowing_sub(1).0,
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

    fn wrap_phy_payloads_in_forward_downlink_req(&mut self) -> Result<()> {
        trace!("Wrap PhyPayloads in ForwardDownlinkReq");

        let relay_ctx = self.relay_context.as_ref().unwrap();

        for item in self.downlink_frame.items.iter_mut() {
            let mut relay_phy = lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataDown,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_slice(&relay_ctx.device_session.dev_addr)?,
                        f_cnt: relay_ctx.device_session.get_a_f_cnt_down(),
                        f_ctrl: lrwn::FCtrl {
                            adr: !self.network_conf.adr_disabled,
                            ack: relay_ctx.must_ack,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![]),
                    },
                    f_port: Some(lrwn::LA_FPORT_RELAY),
                    frm_payload: Some(lrwn::FRMPayload::Raw(item.phy_payload.clone())),
                }),
                mic: None,
            };

            relay_phy.encrypt_frm_payload(&lrwn::AES128Key::from_slice(
                &relay_ctx.device_session.nwk_s_enc_key,
            )?)?;

            // Set MIC.
            // If this is an ACK, then FCntUp has already been incremented by one. If
            // this is not an ACK, then DownlinkDataMIC will zero out ConfFCnt.
            relay_phy.set_downlink_data_mic(
                relay_ctx.device_session.mac_version().from_proto(),
                relay_ctx.device_session.f_cnt_up - 1,
                &lrwn::AES128Key::from_slice(&relay_ctx.device_session.s_nwk_s_int_key)?,
            )?;

            let relay_phy_b = relay_phy.to_vec()?;
            item.phy_payload = relay_phy_b;
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

            // Do not update the frame-counter in case the queue-item is encrypted.
            if !qi.is_encrypted {
                qi.f_cnt_down = Some(self.device_session.get_a_f_cnt_down() as i64);
                *qi = device_queue::update_item(qi.clone()).await?;
            }
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
            n_f_cnt_down: self.device_session.n_f_cnt_down,
            a_f_cnt_down: match &self.device_queue_item {
                Some(v) => match v.is_encrypted {
                    true => v.f_cnt_down.unwrap_or_default() as u32,
                    false => self.device_session.get_a_f_cnt_down(),
                },
                None => self.device_session.get_a_f_cnt_down(),
            },
            ..Default::default()
        })
        .await
        .context("Save downlink frame")?;

        Ok(())
    }

    async fn save_downlink_frame_relayed(&self) -> Result<()> {
        trace!("Saving ForwardDownlinkReq frame");

        let relay_ctx = self.relay_context.as_ref().unwrap();

        downlink_frame::save(&internal::DownlinkFrame {
            downlink_id: self.downlink_frame.downlink_id,
            dev_eui: relay_ctx.device.dev_eui.to_vec(),
            dev_eui_relayed: self.device.dev_eui.to_vec(),
            device_queue_item_id: match &self.device_queue_item {
                Some(qi) => qi.id.as_bytes().to_vec(),
                None => vec![],
            },
            nwk_s_enc_key: relay_ctx.device_session.nwk_s_enc_key.clone(),
            downlink_frame: Some(self.downlink_frame.clone()),
            n_f_cnt_down: relay_ctx.device_session.n_f_cnt_down,
            a_f_cnt_down: relay_ctx.device_session.get_a_f_cnt_down(),
            ..Default::default()
        })
        .await?;

        Ok(())
    }

    async fn send_downlink_frame(&self) -> Result<()> {
        trace!("Sending downlink frame");

        gateway::backend::send_downlink(
            &self.device_session.region_config_id,
            &self.downlink_frame,
        )
        .await
        .context("Send downlink frame")?;

        Ok(())
    }

    fn check_for_first_uplink(&self) -> Result<()> {
        trace!("Checking if device has sent its first uplink already");

        if self.device_session.f_cnt_up == 0 {
            return Err(anyhow!("Device must send its first uplink first"));
        }

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

    async fn send_downlink_frame_passive_roaming(&self) -> Result<()> {
        trace!("Sending downlink-frame using passive-roaming");

        let ufs = self.uplink_frame_set.as_ref().unwrap();

        let roaming_meta = ufs.roaming_meta_data.as_ref().unwrap();

        let net_id = NetID::from_slice(&roaming_meta.base_payload.sender_id)?;
        let client = roaming::get(&net_id)?;

        let mut req = backend::XmitDataReqPayload {
            phy_payload: self.downlink_frame.items[0].phy_payload.clone(),
            dl_meta_data: Some(backend::DLMetaData {
                class_mode: Some("A".to_string()),
                dev_eui: self.device_session.dev_eui.clone(),
                f_ns_ul_token: roaming_meta.ul_meta_data.f_ns_ul_token.clone(),
                dl_freq_1: {
                    let rx1_freq = self
                        .region_conf
                        .get_rx1_frequency_for_uplink_frequency(ufs.tx_info.frequency)?;
                    Some(rx1_freq as f64 / 1_000_000.0)
                },
                dl_freq_2: Some(self.device_session.rx2_frequency as f64 / 1_000_000.0),
                data_rate_1: {
                    let rx1_dr = self.region_conf.get_rx1_data_rate_index(
                        self.device_session.dr as u8,
                        self.device_session.rx1_dr_offset as usize,
                    )?;
                    Some(rx1_dr)
                },
                data_rate_2: Some(self.device_session.rx2_dr as u8),
                rx_delay_1: Some(self.device_session.rx1_delay as usize),
                gw_info: roaming_meta
                    .ul_meta_data
                    .gw_info
                    .iter()
                    .filter(|gw| gw.dl_allowed.unwrap_or_default())
                    .map(|gw| backend::GWInfoElement {
                        ul_token: gw.ul_token.clone(),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        };

        #[cfg(test)]
        {
            req.base.transaction_id = 1234
        }

        let async_receiver = match client.is_async() {
            false => None,
            true => {
                Some(get_async_receiver(req.base.transaction_id, client.get_async_timeout()).await?)
            }
        };
        client
            .xmit_data_req(backend::Role::FNS, &mut req, async_receiver)
            .await?;

        Ok(())
    }

    async fn handle_passive_roaming_tx_ack(&self) -> Result<()> {
        trace!("Handle passive-roaming tx-ack");

        tx_ack::TxAck::handle(gw::DownlinkTxAck {
            downlink_id: self.downlink_frame.downlink_id,
            items: vec![gw::DownlinkTxAckItem {
                status: gw::TxAckStatus::Ok.into(),
            }],
            ..Default::default()
        })
        .await;

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
            region_config_id: ufs.region_config_id.clone(),
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
            skip_f_cnt_check: self.device_session.skip_f_cnt_check,
            device_variables: self.device.variables.into_hashmap(),
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
                let ts: DateTime<Utc> = ts.clone().try_into().map_err(anyhow::Error::msg)?;
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

    async fn _update_uplink_list(&mut self) -> Result<()> {
        trace!("Updating Relay uplink list");

        if self.device_session.relay.is_none() {
            self.device_session.relay = Some(internal::Relay::default());
        }

        // Get a copy of the current relay state.
        let relay = self.device_session.relay.as_ref().unwrap().clone();

        // Get devices that must be configured on the relay.
        let relay_devices = relay::list_devices(
            15,
            0,
            &relay::DeviceFilters {
                relay_dev_eui: Some(self.device.dev_eui),
            },
        )
        .await?;

        //  Calculate unused slots.
        let used_slots: Vec<u32> = relay.devices.iter().map(|d| d.index).collect();
        let free_slots: Vec<u32> = (0..15).filter(|x| !used_slots.contains(x)).collect();

        // Iterate over the list of devices under this relay.
        for device in &relay_devices {
            // We need a dev_addr for the filter. Ignore devices that do not have a DevAddr (e.g.
            // they have never been activated).
            if let Some(dev_addr) = device.dev_addr {
                let mut found = false;

                for rd in &mut self.device_session.relay.as_mut().unwrap().devices {
                    if rd.dev_eui == device.dev_eui.to_vec() {
                        found = true;

                        // The device has not yet been provisioned, or
                        // the settings must be updated
                        if !rd.provisioned
                            || rd.dev_addr != dev_addr.to_vec()
                            || rd.uplink_limit_bucket_size
                                != device.relay_ed_uplink_limit_bucket_size as u32
                            || rd.uplink_limit_reload_rate
                                != device.relay_ed_uplink_limit_reload_rate as u32
                        {
                            let ds = match device_session::get(&device.dev_eui).await {
                                Ok(v) => v,
                                Err(_) => {
                                    // It is valid that the device is no longer activated.
                                    continue;
                                }
                            };
                            let root_wor_s_key = keys::get_root_wor_s_key(&AES128Key::from_slice(
                                &ds.nwk_s_enc_key,
                            )?)?;

                            let set = lrwn::MACCommandSet::new(vec![
                                lrwn::MACCommand::UpdateUplinkListReq(
                                    lrwn::UpdateUplinkListReqPayload {
                                        uplink_list_idx: rd.index as u8,
                                        uplink_limit: lrwn::UplinkLimitPL {
                                            bucket_size: device.relay_ed_uplink_limit_bucket_size
                                                as u8,
                                            reload_rate: device.relay_ed_uplink_limit_reload_rate
                                                as u8,
                                        },
                                        dev_addr,
                                        w_fcnt: ds.relay.map(|v| v.w_f_cnt).unwrap_or(0),
                                        root_wor_s_key,
                                    },
                                ),
                            ]);
                            mac_command::set_pending(
                                &self.device.dev_eui,
                                lrwn::CID::UpdateUplinkListReq,
                                &set,
                            )
                            .await?;
                            self.mac_commands.push(set);

                            rd.dev_addr = dev_addr.to_vec();
                            rd.root_wor_s_key = root_wor_s_key.to_vec();
                            rd.uplink_limit_bucket_size =
                                device.relay_ed_uplink_limit_bucket_size as u32;
                            rd.uplink_limit_reload_rate =
                                device.relay_ed_uplink_limit_reload_rate as u32;
                            rd.provisioned = false;

                            // Return because we can't add multiple sets and if we would combine
                            // multiple commands as a single set, it might not fit in a single
                            // downlink.
                            return Ok(());
                        }
                    }
                }

                // The device was not found in the list. This means we must add it (using the first
                // available slot).
                if !found {
                    if free_slots.is_empty() {
                        error!(relay_dev_eui = %self.device.dev_eui, "Relay does not have any free UpdateUplinkListReq slots");
                        continue;
                    }

                    let ds = match device_session::get(&device.dev_eui).await {
                        Ok(v) => v,
                        Err(_) => {
                            // It is valid that the device is no longer activated.
                            continue;
                        }
                    };
                    let root_wor_s_key =
                        keys::get_root_wor_s_key(&AES128Key::from_slice(&ds.nwk_s_enc_key)?)?;

                    let set =
                        lrwn::MACCommandSet::new(vec![lrwn::MACCommand::UpdateUplinkListReq(
                            lrwn::UpdateUplinkListReqPayload {
                                uplink_list_idx: free_slots[0] as u8,
                                uplink_limit: lrwn::UplinkLimitPL {
                                    bucket_size: device.relay_ed_uplink_limit_bucket_size as u8,
                                    reload_rate: device.relay_ed_uplink_limit_reload_rate as u8,
                                },
                                dev_addr,
                                w_fcnt: ds.relay.map(|v| v.w_f_cnt).unwrap_or(0),
                                root_wor_s_key,
                            },
                        )]);
                    mac_command::set_pending(
                        &self.device.dev_eui,
                        lrwn::CID::UpdateUplinkListReq,
                        &set,
                    )
                    .await?;
                    self.mac_commands.push(set);

                    self.device_session.relay.as_mut().unwrap().devices.push(
                        internal::RelayDevice {
                            index: free_slots[0],
                            join_eui: vec![],
                            dev_eui: device.dev_eui.to_vec(),
                            dev_addr: dev_addr.to_vec(),
                            root_wor_s_key: root_wor_s_key.to_vec(),
                            uplink_limit_bucket_size: device.relay_ed_uplink_limit_bucket_size
                                as u32,
                            uplink_limit_reload_rate: device.relay_ed_uplink_limit_reload_rate
                                as u32,
                            provisioned: false,
                            w_f_cnt_last_request: Some(Utc::now().into()),
                        },
                    );

                    // Return because we can't add multiple sets and if we would combine
                    // multiple commands as a single set, it might not fit in a single
                    // downlink.
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    async fn _request_ctrl_uplink_list(&mut self) -> Result<()> {
        trace!("Requesting CtrlUplinkList to sync WFCnt");

        if self.device_session.relay.is_none() {
            self.device_session.relay = Some(internal::Relay::default());
        }

        // Get a copy of the current relay state.
        let mut relay = self.device_session.relay.as_ref().unwrap().clone();

        // Get devices that must be configured on the relay.
        let relay_devices = relay::list_devices(
            15,
            0,
            &relay::DeviceFilters {
                relay_dev_eui: Some(self.device.dev_eui),
            },
        )
        .await?;

        // Get DevEUIs of Relay EDs.
        let relay_devices_dev_euis: Vec<Vec<u8>> =
            relay_devices.iter().map(|d| d.dev_eui.to_vec()).collect();

        // Calculate removed slots.
        let removed_slots: Vec<u32> = relay
            .devices
            .iter()
            .filter(|d| !relay_devices_dev_euis.contains(&d.dev_eui))
            .map(|f| f.index)
            .collect();

        let max_count = 3;
        let mut counter = 0;
        let mut commands: Vec<lrwn::MACCommand> = vec![];

        // Delete end-device from trusted list.
        for slot in &removed_slots {
            if counter < max_count {
                counter += 1;
                commands.push(lrwn::MACCommand::CtrlUplinkListReq(
                    lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: *slot as u8,
                            ctrl_uplink_action: 1,
                        },
                    },
                ));
            }
        }

        // Sync WFCnt.
        for rd in &mut relay.devices {
            if removed_slots.contains(&rd.index) {
                continue;
            }

            match &rd.w_f_cnt_last_request {
                Some(v) => {
                    let last_req: DateTime<Utc> =
                        v.clone().try_into().map_err(anyhow::Error::msg)?;
                    if last_req
                        < Utc::now()
                            .checked_sub_signed(chrono::Duration::hours(24))
                            .unwrap()
                        && counter < max_count
                    {
                        counter += 1;
                        commands.push(lrwn::MACCommand::CtrlUplinkListReq(
                            lrwn::CtrlUplinkListReqPayload {
                                ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                                    uplink_list_idx: rd.index as u8,
                                    ctrl_uplink_action: 0,
                                },
                            },
                        ));

                        rd.w_f_cnt_last_request = Some(Utc::now().into());
                    }
                }
                None => {
                    if counter < max_count {
                        counter += 1;
                        commands.push(lrwn::MACCommand::CtrlUplinkListReq(
                            lrwn::CtrlUplinkListReqPayload {
                                ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                                    uplink_list_idx: rd.index as u8,
                                    ctrl_uplink_action: 0,
                                },
                            },
                        ));

                        rd.w_f_cnt_last_request = Some(Utc::now().into());
                    }
                }
            }
        }

        self.device_session.relay = Some(relay);

        if !commands.is_empty() {
            let set = lrwn::MACCommandSet::new(commands);
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::CtrlUplinkListReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        Ok(())
    }

    async fn _configure_fwd_limit_req(&mut self) -> Result<()> {
        trace!("Configuring Relay Fwd Limit");

        // Get the current relay state.
        let relay = if let Some(r) = &self.device_session.relay {
            r.clone()
        } else {
            internal::Relay::default()
        };

        if relay.join_req_limit_reload_rate
            != self.device_profile.relay_join_req_limit_reload_rate as u32
            || relay.notify_limit_reload_rate
                != self.device_profile.relay_notify_limit_reload_rate as u32
            || relay.global_uplink_limit_reload_rate
                != self.device_profile.relay_global_uplink_limit_reload_rate as u32
            || relay.overall_limit_reload_rate
                != self.device_profile.relay_overall_limit_reload_rate as u32
            || relay.join_req_limit_bucket_size
                != self.device_profile.relay_join_req_limit_bucket_size as u32
            || relay.notify_limit_bucket_size
                != self.device_profile.relay_notify_limit_bucket_size as u32
            || relay.global_uplink_limit_bucket_size
                != self.device_profile.relay_global_uplink_limit_bucket_size as u32
            || relay.overall_limit_bucket_size
                != self.device_profile.relay_overall_limit_bucket_size as u32
        {
            let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::ConfigureFwdLimitReq(
                lrwn::ConfigureFwdLimitReqPayload {
                    reload_rate: lrwn::FwdLimitReloadRatePL {
                        overall_reload_rate: self.device_profile.relay_overall_limit_reload_rate
                            as u8,
                        global_uplink_reload_rate: self
                            .device_profile
                            .relay_global_uplink_limit_reload_rate
                            as u8,
                        notify_reload_rate: self.device_profile.relay_notify_limit_reload_rate
                            as u8,
                        join_req_reload_rate: self.device_profile.relay_join_req_limit_reload_rate
                            as u8,
                        reset_limit_counter: lrwn::ResetLimitCounter::NoChange,
                    },
                    load_capacity: lrwn::FwdLimitLoadCapacityPL {
                        overall_limit_size: self.device_profile.relay_overall_limit_bucket_size
                            as u8,
                        global_uplink_limit_size: self
                            .device_profile
                            .relay_global_uplink_limit_bucket_size
                            as u8,
                        notify_limit_size: self.device_profile.relay_notify_limit_bucket_size as u8,
                        join_req_limit_size: self.device_profile.relay_join_req_limit_bucket_size
                            as u8,
                    },
                },
            )]);
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::ConfigureFwdLimitReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        self.device_session.relay = Some(relay);

        Ok(())
    }

    async fn _update_filter_list(&mut self) -> Result<()> {
        trace!("Updating Relay filter list");

        if self.device_session.relay.is_none() {
            self.device_session.relay = Some(internal::Relay::default());
        }

        // Get a copy of the current relay state.
        let relay = self.device_session.relay.as_ref().unwrap().clone();

        // Get devices that must be configured on the relay.
        let relay_devices = relay::list_devices(
            15,
            0,
            &relay::DeviceFilters {
                relay_dev_eui: Some(self.device.dev_eui),
            },
        )
        .await?;

        // Get DevEUIs of Relay EDs.
        let relay_devices_dev_euis: Vec<Vec<u8>> =
            relay_devices.iter().map(|d| d.dev_eui.to_vec()).collect();

        // Calculate removed slots.
        let removed_slots: Vec<u32> = relay
            .filters
            .iter()
            .filter(|f| f.index != 0 && !relay_devices_dev_euis.contains(&f.dev_eui))
            .map(|f| f.index)
            .collect();

        // Calculate free slots.
        // Note that the first slot is used as "catch-all" filter.
        let used_slots: Vec<u32> = relay
            .filters
            .iter()
            .filter(|f| f.index == 0 || relay_devices_dev_euis.contains(&f.dev_eui))
            .map(|f| f.index)
            .collect();
        let free_slots: Vec<u32> = (1..15).filter(|x| !used_slots.contains(x)).collect();

        // Unset slots of devices that are no longer configured.
        if !removed_slots.is_empty() {
            let mut commands: Vec<lrwn::MACCommand> = Vec::new();
            for slot in removed_slots {
                commands.push(lrwn::MACCommand::FilterListReq(
                    lrwn::FilterListReqPayload {
                        filter_list_idx: slot as u8,
                        filter_list_action: lrwn::FilterListAction::NoRule,
                        filter_list_eui: vec![],
                    },
                ));

                if commands.len() > 5 {
                    commands.drain(5..);
                }
            }

            let set = lrwn::MACCommandSet::new(commands);
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::FilterListReq, &set).await?;
            self.mac_commands.push(set);

            // The deletes needs to be processed before we can add new entries.
            return Ok(());
        }

        // Make sure the first item contains the "catch-all" filter.
        // This is needed to make sure that only the rest of the filter items are allowed to join
        // through the Relay.
        if let Some(relay) = self.device_session.relay.as_mut() {
            if relay.filters.is_empty() {
                relay.filters.push(internal::RelayFilter {
                    index: 0,
                    action: 2,
                    provisioned: false,
                    ..Default::default()
                });
            }

            if let Some(filter) = relay.filters.first() {
                if !filter.provisioned {
                    let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListReq(
                        lrwn::FilterListReqPayload {
                            filter_list_idx: 0,
                            filter_list_action: lrwn::FilterListAction::Filter,
                            filter_list_eui: vec![],
                        },
                    )]);
                    mac_command::set_pending(&self.device.dev_eui, lrwn::CID::FilterListReq, &set)
                        .await?;
                    self.mac_commands.push(set);

                    // Return because we can't add multiple sets and if we would combine
                    // multiple commands as a single set, it might not fit in a single
                    // downlink.
                    return Ok(());
                }
            }
        }

        // Iterate over the list of devices under this relay.
        for device in &relay_devices {
            let mut found = false;

            for f in &mut self.device_session.relay.as_mut().unwrap().filters {
                if f.dev_eui == device.dev_eui.to_vec() {
                    found = true;

                    // The device has not yet been provisioned, or
                    // the device has a new JoinEUI, we must update it (same index).
                    if !f.provisioned || f.join_eui != device.join_eui.to_vec() {
                        let mut eui = device.join_eui.to_vec();
                        eui.extend_from_slice(&device.dev_eui.to_vec());

                        let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListReq(
                            lrwn::FilterListReqPayload {
                                filter_list_idx: f.index as u8,
                                filter_list_action: lrwn::FilterListAction::Forward,
                                filter_list_eui: eui,
                            },
                        )]);
                        mac_command::set_pending(
                            &self.device.dev_eui,
                            lrwn::CID::FilterListReq,
                            &set,
                        )
                        .await?;
                        self.mac_commands.push(set);

                        f.join_eui = device.join_eui.to_vec();
                        f.provisioned = false;

                        // Return because we can't add multiple sets and if we would combine
                        // multiple commands as a single set, it might not fit in a single
                        // downlink.
                        return Ok(());
                    }
                }
            }

            // The device was not found in the list. This means we must add it (using the first
            // available slot).
            if !found {
                if free_slots.is_empty() {
                    error!(relay_dev_eui = %self.device.dev_eui, "Relay does have have any free FilterListReq slots");
                    continue;
                }

                let mut eui = device.join_eui.to_vec();
                eui.extend_from_slice(&device.dev_eui.to_vec());

                let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListReq(
                    lrwn::FilterListReqPayload {
                        filter_list_idx: free_slots[0] as u8,
                        filter_list_action: lrwn::FilterListAction::Forward,
                        filter_list_eui: eui,
                    },
                )]);
                mac_command::set_pending(&self.device.dev_eui, lrwn::CID::FilterListReq, &set)
                    .await?;
                self.mac_commands.push(set);

                self.device_session
                    .relay
                    .as_mut()
                    .unwrap()
                    .filters
                    .push(internal::RelayFilter {
                        index: free_slots[0],
                        action: 1,
                        join_eui: device.join_eui.to_vec(),
                        dev_eui: device.dev_eui.to_vec(),
                        provisioned: false,
                    });

                // Return because we can't add multiple sets and if we would combine
                // multiple commands as a single set, it might not fit in a single
                // downlink.
                return Ok(());
            }
        }

        Ok(())
    }

    async fn _update_relay_conf(&mut self) -> Result<()> {
        trace!("Updating Relay Conf");

        // Get the current relay state.
        let relay = if let Some(r) = &self.device_session.relay {
            r.clone()
        } else {
            internal::Relay::default()
        };

        if relay.enabled != self.device_profile.relay_enabled
            || relay.cad_periodicity != self.device_profile.relay_cad_periodicity as u32
            || relay.default_channel_index != self.device_profile.relay_default_channel_index as u32
            || relay.second_channel_freq != self.device_profile.relay_second_channel_freq as u32
            || relay.second_channel_dr != self.device_profile.relay_second_channel_dr as u32
            || relay.second_channel_ack_offset
                != self.device_profile.relay_second_channel_ack_offset as u32
        {
            let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RelayConfReq(
                lrwn::RelayConfReqPayload {
                    channel_settings_relay: lrwn::ChannelSettingsRelay {
                        start_stop: match self.device_profile.relay_enabled {
                            true => 1,
                            false => 0,
                        },
                        cad_periodicity: self.device_profile.relay_cad_periodicity as u8,
                        default_ch_idx: self.device_profile.relay_default_channel_index as u8,
                        second_ch_idx: if self.device_profile.relay_second_channel_freq > 0 {
                            1
                        } else {
                            0
                        },
                        second_ch_dr: self.device_profile.relay_second_channel_dr as u8,
                        second_ch_ack_offset: self.device_profile.relay_second_channel_ack_offset
                            as u8,
                    },
                    second_ch_freq: self.device_profile.relay_second_channel_freq as u32,
                },
            )]);
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::RelayConfReq, &set).await?;
            self.mac_commands.push(set);
        }

        self.device_session.relay = Some(relay);

        Ok(())
    }

    async fn _update_end_device_conf(&mut self) -> Result<()> {
        trace!("Updating End Device Conf");

        // Get the current relay state.
        let relay = if let Some(r) = &self.device_session.relay {
            r.clone()
        } else {
            internal::Relay::default()
        };

        if relay.ed_activation_mode != self.device_profile.relay_ed_activation_mode.to_u8() as u32
            || relay.ed_smart_enable_level != self.device_profile.relay_ed_smart_enable_level as u32
            || relay.ed_back_off != self.device_profile.relay_ed_back_off as u32
            || relay.second_channel_freq != self.device_profile.relay_second_channel_freq as u32
            || relay.second_channel_dr != self.device_profile.relay_second_channel_dr as u32
            || relay.second_channel_ack_offset
                != self.device_profile.relay_second_channel_ack_offset as u32
        {
            let set = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::EndDeviceConfReq(
                lrwn::EndDeviceConfReqPayload {
                    activation_relay_mode: lrwn::ActivationRelayMode {
                        relay_mode_activation: self.device_profile.relay_ed_activation_mode,
                        smart_enable_level: self.device_profile.relay_ed_smart_enable_level as u8,
                    },
                    channel_settings_ed: lrwn::ChannelSettingsED {
                        second_ch_ack_offset: self.device_profile.relay_second_channel_ack_offset
                            as u8,
                        second_ch_dr: self.device_profile.relay_second_channel_dr as u8,
                        second_ch_idx: if self.device_profile.relay_second_channel_freq > 0 {
                            1
                        } else {
                            0
                        },
                        backoff: self.device_profile.relay_ed_back_off as u8,
                    },
                    second_ch_freq: self.device_profile.relay_second_channel_freq as u32,
                },
            )]);
            mac_command::set_pending(&self.device.dev_eui, lrwn::CID::EndDeviceConfReq, &set)
                .await?;
            self.mac_commands.push(set);
        }

        self.device_session.relay = Some(relay);

        Ok(())
    }

    fn set_tx_info_for_rx1(&mut self) -> Result<()> {
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

    fn set_tx_info_for_rx1_relayed(&mut self) -> Result<()> {
        trace!("Setting tx-info for relayed RX1");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let relay_ctx = self.relay_context.as_ref().unwrap();

        let mut tx_info = gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // get RX1 DR.
        let rx1_dr_index_relay = self.region_conf.get_rx1_data_rate_index(
            self.uplink_frame_set.as_ref().unwrap().dr,
            relay_ctx.device_session.rx1_dr_offset as usize,
        )?;
        let rx1_dr_relay = self.region_conf.get_data_rate(rx1_dr_index_relay)?;

        // set DR to tx_info.
        helpers::set_tx_info_data_rate(&mut tx_info, &rx1_dr_relay)?;

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
        let delay = if relay_ctx.device_session.rx1_delay > 0 {
            Duration::from_secs(relay_ctx.device_session.rx1_delay as u64)
        } else {
            self.region_conf.get_defaults().rx1_delay
        };
        tx_info.timing = Some(gw::Timing {
            parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                delay: Some(pbjson_types::Duration::from(delay)),
            })),
        });

        // get remaining payload size (relay)
        let max_pl_size_relay = self.region_conf.get_max_payload_size(
            relay_ctx.device_session.mac_version().from_proto(),
            relay_ctx.device_profile.reg_params_revision,
            rx1_dr_index_relay,
        )?;

        // Get remaining payload size (end-device)
        let rx1_dr_index_ed = self.region_conf.get_rx1_data_rate_index(
            relay_ctx.req.metadata.dr,
            self.device_session.rx1_dr_offset as usize,
        )?;
        let max_pl_size_ed = self.region_conf.get_max_payload_size(
            self.device_session.mac_version().from_proto(),
            self.device_profile.reg_params_revision,
            rx1_dr_index_ed,
        )?;

        // Take the smallest payload size to make sure it can be sent using the relay downlink DR
        // and the end-device downlink DR (repeated by the relay).
        let max_pl_size = if max_pl_size_relay.n < max_pl_size_ed.n {
            max_pl_size_relay
        } else {
            max_pl_size_ed
        };

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
            frequency: if self.device_session.rx2_frequency == 0 {
                self.region_conf.get_defaults().rx2_frequency
            } else {
                self.device_session.rx2_frequency
            },
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

    fn set_tx_info_for_rx2_relayed(&mut self) -> Result<()> {
        trace!("Setting tx-info for relayed RX2");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let relay_ctx = self.relay_context.as_ref().unwrap();

        let mut tx_info = gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            frequency: relay_ctx.device_session.rx2_frequency,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // Set DR to tx-info.
        let rx2_dr_relay = self
            .region_conf
            .get_data_rate(relay_ctx.device_session.rx2_dr as u8)?;
        helpers::set_tx_info_data_rate(&mut tx_info, &rx2_dr_relay)?;

        // set tx power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // set timestamp
        if !self.immediately {
            let delay = if relay_ctx.device_session.rx1_delay > 0 {
                Duration::from_secs(relay_ctx.device_session.rx1_delay as u64 + 1)
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

        // get remaining payload size (relay).
        let max_pl_size_relay = self.region_conf.get_max_payload_size(
            relay_ctx.device_session.mac_version().from_proto(),
            relay_ctx.device_profile.reg_params_revision,
            relay_ctx.device_session.rx2_dr as u8,
        )?;

        // get remaining payload size (end-device).
        let max_pl_size_ed = self.region_conf.get_max_payload_size(
            self.device_session.mac_version().from_proto(),
            self.device_profile.reg_params_revision,
            self.device_session.rx2_dr as u8,
        )?;

        // Take the smallest payload size to make sure it can be sent using the relay downlink DR
        // and the end-device downlink DR (repeated by the relay).
        let max_pl_size = if max_pl_size_relay.n < max_pl_size_ed.n {
            max_pl_size_relay
        } else {
            max_pl_size_ed
        };

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

fn filter_mac_commands(
    device_session: &internal::DeviceSession,
    mac_commands: &[lrwn::MACCommandSet],
) -> Vec<lrwn::MACCommandSet> {
    let incompatible_macs: HashMap<lrwn::CID, Vec<lrwn::CID>> = [
        (lrwn::CID::NewChannelReq, vec![lrwn::CID::LinkADRReq]),
        (lrwn::CID::LinkADRReq, vec![lrwn::CID::NewChannelReq]),
    ]
    .iter()
    .cloned()
    .collect();

    let mut filtered_mac_commands: Vec<lrwn::MACCommandSet> = Vec::new();

    'outer: for mac_command_set in mac_commands {
        for mac_command in &**mac_command_set {
            // Check if it doesn't exceed the max error error count.
            if device_session
                .mac_command_error_count
                .get(&(mac_command.cid().to_u8() as u32))
                .cloned()
                .unwrap_or_default()
                > 1
            {
                continue 'outer;
            }

            // Check if there aren't any conflicting mac-commands.
            if let Some(incompatible_cids) = incompatible_macs.get(&mac_command.cid()) {
                for mac_command_set in &filtered_mac_commands {
                    for mac_command in &**mac_command_set {
                        if incompatible_cids.contains(&mac_command.cid()) {
                            continue 'outer;
                        }
                    }
                }
            }
        }

        filtered_mac_commands.push(mac_command_set.clone());
    }

    filtered_mac_commands
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test;
    use lrwn::EUI64;
    use tokio::time::sleep;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_next_device_queue_item() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "dp".into(),
            tenant_id: t.id,
            is_relay: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let d = device::create(device::Device {
            dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            name: "dev".into(),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let ds = internal::DeviceSession {
            n_f_cnt_down: 10,
            a_f_cnt_down: 10,
            ..Default::default()
        };

        struct Test {
            name: String,
            max_payload_size: usize,
            queue_items: Vec<device_queue::DeviceQueueItem>,
            expected_queue_item: Option<device_queue::DeviceQueueItem>,
            expected_ack_event: Option<integration_pb::AckEvent>,
            expected_log_event: Option<integration_pb::LogEvent>,
        }

        let qi_id = Uuid::new_v4();

        let tests = vec![
            Test {
                name: "max payload size error".into(),
                max_payload_size: 10,
                queue_items: vec![device_queue::DeviceQueueItem {
                    id: qi_id,
                    dev_eui: d.dev_eui,
                    f_port: 1,
                    data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                    ..Default::default()
                }],
                expected_queue_item: None,
                expected_ack_event: None,
                expected_log_event: Some(integration_pb::LogEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_name: d.name.clone(),
                        dev_eui: d.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    level: integration_pb::LogLevel::Error.into(),
                    code: integration_pb::LogCode::DownlinkPayloadSize.into(),
                    description:
                        "Device queue-item discarded because it exceeds the max. payload size"
                            .into(),
                    context: [
                        ("item_size".to_string(), "11".to_string()),
                        ("queue_item_id".to_string(), qi_id.to_string()),
                        ("max_payload_size".to_string(), "10".to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    ..Default::default()
                }),
            },
            Test {
                name: "is pending".into(),
                max_payload_size: 10,
                queue_items: vec![device_queue::DeviceQueueItem {
                    id: qi_id,
                    dev_eui: d.dev_eui,
                    f_port: 1,
                    f_cnt_down: Some(10),
                    data: vec![1, 2, 3],
                    is_pending: true,
                    ..Default::default()
                }],
                expected_queue_item: None,
                expected_log_event: None,
                expected_ack_event: Some(integration_pb::AckEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_name: d.name.clone(),
                        dev_eui: d.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    queue_item_id: qi_id.to_string(),
                    acknowledged: false,
                    f_cnt_down: 10,
                    ..Default::default()
                }),
            },
            Test {
                name: "invalid frame-counter".into(),
                max_payload_size: 10,
                queue_items: vec![device_queue::DeviceQueueItem {
                    id: qi_id,
                    dev_eui: d.dev_eui,
                    f_port: 1,
                    data: vec![1, 2, 3],
                    f_cnt_down: Some(5),
                    is_encrypted: true,
                    ..Default::default()
                }],
                expected_queue_item: None,
                expected_ack_event: None,
                expected_log_event: Some(integration_pb::LogEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_name: d.name.clone(),
                        dev_eui: d.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    level: integration_pb::LogLevel::Error.into(),
                    code: integration_pb::LogCode::FCntDown.into(),
                    description: "Device queue-item discarded because the frame-counter is invalid"
                        .into(),
                    context: [
                        ("queue_item_id".to_string(), qi_id.to_string()),
                        ("device_f_cnt_down".to_string(), "10".to_string()),
                        ("queue_item_f_cnt_down".to_string(), "5".to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    ..Default::default()
                }),
            },
            Test {
                name: "valid payload".into(),
                max_payload_size: 10,
                queue_items: vec![device_queue::DeviceQueueItem {
                    id: qi_id,
                    dev_eui: d.dev_eui,
                    f_port: 1,
                    data: vec![1, 2, 3],
                    ..Default::default()
                }],
                expected_queue_item: Some(device_queue::DeviceQueueItem {
                    id: qi_id,
                    dev_eui: d.dev_eui,
                    f_port: 1,
                    data: vec![1, 2, 3],
                    ..Default::default()
                }),
                expected_log_event: None,
                expected_ack_event: None,
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);

            integration::set_mock().await;
            integration::mock::reset().await;

            device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();
            for qi in &tst.queue_items {
                device_queue::enqueue_item(qi.clone()).await.unwrap();
            }

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: t.clone(),
                application: app.clone(),
                device_profile: dp.clone(),
                device: d.clone(),
                device_session: ds.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![DownlinkFrameItem {
                    downlink_frame_item: Default::default(),
                    remaining_payload_size: tst.max_payload_size,
                }],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx.get_next_device_queue_item().await.unwrap();

            // Integrations are handled async.
            sleep(Duration::from_millis(100)).await;

            if let Some(log) = &tst.expected_log_event {
                let mut event = integration::mock::get_log_event().await.unwrap();
                assert_ne!(None, event.time);
                event.time = None;
                assert_eq!(log, &event);
            }

            if let Some(ack) = &tst.expected_ack_event {
                let mut event = integration::mock::get_ack_event().await.unwrap();
                assert_ne!(None, event.time);
                event.time = None;
                assert_eq!(ack, &event);
            }

            if let Some(qi) = &tst.expected_queue_item {
                assert_ne!(None, ctx.device_queue_item);
                assert_eq!(qi.id, ctx.device_queue_item.as_ref().unwrap().id);
            }
        }
    }

    #[test]
    fn test_filter_mac_commands() {
        struct Test {
            device_session: internal::DeviceSession,
            mac_commands: Vec<lrwn::MACCommandSet>,
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
        }

        let tests = vec![
            // No mac-commands set.
            Test {
                device_session: internal::DeviceSession {
                    ..Default::default()
                },
                mac_commands: Vec::new(),
                expected_mac_commands: Vec::new(),
            },
            // One LinkADRReq, no errors.
            Test {
                device_session: internal::DeviceSession {
                    mac_command_error_count: [(lrwn::CID::LinkADRReq.to_u8() as u32, 0)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        ch_mask: lrwn::ChMask::new([false; 16]),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                    }),
                ])],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        ch_mask: lrwn::ChMask::new([false; 16]),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                    }),
                ])],
            },
            // One LinkADRReq, 0 errors (HashMap contains the CID, but count = 0).
            Test {
                device_session: internal::DeviceSession {
                    mac_command_error_count: [(lrwn::CID::LinkADRReq.to_u8() as u32, 0)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        ch_mask: lrwn::ChMask::new([false; 16]),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                    }),
                ])],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        ch_mask: lrwn::ChMask::new([false; 16]),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                    }),
                ])],
            },
            // One LinkADRReq, exceeding error count.
            Test {
                device_session: internal::DeviceSession {
                    mac_command_error_count: [(lrwn::CID::LinkADRReq.to_u8() as u32, 2)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        ch_mask: lrwn::ChMask::new([false; 16]),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                    }),
                ])],
                expected_mac_commands: Vec::new(),
            },
            // NewChannelReq + LinkADRReq
            Test {
                device_session: Default::default(),
                mac_commands: vec![
                    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::NewChannelReq(
                        lrwn::NewChannelReqPayload {
                            ch_index: 3,
                            freq: 867100000,
                            min_dr: 0,
                            max_dr: 5,
                        },
                    )]),
                    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                        lrwn::LinkADRReqPayload {
                            dr: 0,
                            tx_power: 0,
                            ch_mask: lrwn::ChMask::new([false; 16]),
                            redundancy: lrwn::Redundancy {
                                ch_mask_cntl: 0,
                                nb_rep: 0,
                            },
                        },
                    )]),
                ],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 3,
                        freq: 867100000,
                        min_dr: 0,
                        max_dr: 5,
                    }),
                ])],
            },
            // LinkADRReq + NewChannelReq (this order should never happen)
            Test {
                device_session: Default::default(),
                mac_commands: vec![
                    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                        lrwn::LinkADRReqPayload {
                            dr: 0,
                            tx_power: 0,
                            ch_mask: lrwn::ChMask::new([false; 16]),
                            redundancy: lrwn::Redundancy {
                                ch_mask_cntl: 0,
                                nb_rep: 0,
                            },
                        },
                    )]),
                    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::NewChannelReq(
                        lrwn::NewChannelReqPayload {
                            ch_index: 3,
                            freq: 867100000,
                            min_dr: 0,
                            max_dr: 5,
                        },
                    )]),
                ],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        ch_mask: lrwn::ChMask::new([false; 16]),
                        redundancy: lrwn::Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                    }),
                ])],
            },
        ];

        for test in &tests {
            let out = filter_mac_commands(&test.device_session, &test.mac_commands);
            assert_eq!(test.expected_mac_commands, out);
        }
    }

    #[tokio::test]
    async fn test_update_uplink_list() {
        struct Test {
            name: String,
            device_session: internal::DeviceSession,
            relay_devices: Vec<(EUI64, DevAddr)>,
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
            expected_device_session: internal::DeviceSession,
        }

        let tests = vec![
            Test {
                name: "no devices".into(),
                device_session: internal::DeviceSession {
                    ..Default::default()
                },
                relay_devices: vec![],
                expected_mac_commands: vec![],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay::default()),
                    ..Default::default()
                },
            },
            Test {
                name: "already in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            join_eui: vec![],
                            dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            dev_addr: vec![1, 2, 3, 4],
                            root_wor_s_key: vec![],
                            provisioned: true,
                            uplink_limit_bucket_size: 2,
                            uplink_limit_reload_rate: 1,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![(
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
                    DevAddr::from_be_bytes([1, 2, 3, 4]),
                )],
                expected_mac_commands: vec![],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            join_eui: vec![],
                            dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            dev_addr: vec![1, 2, 3, 4],
                            root_wor_s_key: vec![],
                            provisioned: true,
                            uplink_limit_bucket_size: 2,
                            uplink_limit_reload_rate: 1,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            Test {
                name: "add device".into(),
                device_session: internal::DeviceSession {
                    ..Default::default()
                },
                relay_devices: vec![(
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
                    DevAddr::from_be_bytes([1, 2, 3, 4]),
                )],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::UpdateUplinkListReq(lrwn::UpdateUplinkListReqPayload {
                        uplink_list_idx: 0,
                        uplink_limit: lrwn::UplinkLimitPL {
                            reload_rate: 1,
                            bucket_size: 2,
                        },
                        dev_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
                        w_fcnt: 0,
                        root_wor_s_key: AES128Key::from_bytes([
                            0x47, 0x71, 0x18, 0x16, 0xe9, 0x1d, 0x6f, 0xf0, 0x59, 0xbb, 0xbf, 0x2b,
                            0xf5, 0x8e, 0x0f, 0xd3,
                        ]),
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 0,
                            dev_addr: vec![1, 2, 3, 4],
                            dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            join_eui: vec![],
                            root_wor_s_key: vec![
                                0x47, 0x71, 0x18, 0x16, 0xe9, 0x1d, 0x6f, 0xf0, 0x59, 0xbb, 0xbf,
                                0x2b, 0xf5, 0x8e, 0x0f, 0xd3,
                            ],
                            provisioned: false,
                            uplink_limit_reload_rate: 1,
                            uplink_limit_bucket_size: 2,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            Test {
                name: "update device".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            join_eui: vec![],
                            dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            dev_addr: vec![1, 2, 3, 4],
                            root_wor_s_key: vec![],
                            provisioned: true,
                            uplink_limit_bucket_size: 2,
                            uplink_limit_reload_rate: 1,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![(
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
                    DevAddr::from_be_bytes([2, 2, 3, 4]),
                )],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::UpdateUplinkListReq(lrwn::UpdateUplinkListReqPayload {
                        uplink_list_idx: 1,
                        uplink_limit: lrwn::UplinkLimitPL {
                            reload_rate: 1,
                            bucket_size: 2,
                        },
                        dev_addr: DevAddr::from_be_bytes([2, 2, 3, 4]),
                        w_fcnt: 0,
                        root_wor_s_key: AES128Key::from_bytes([
                            0x47, 0x71, 0x18, 0x16, 0xe9, 0x1d, 0x6f, 0xf0, 0x59, 0xbb, 0xbf, 0x2b,
                            0xf5, 0x8e, 0x0f, 0xd3,
                        ]),
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            join_eui: vec![],
                            dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            dev_addr: vec![2, 2, 3, 4],
                            root_wor_s_key: vec![
                                0x47, 0x71, 0x18, 0x16, 0xe9, 0x1d, 0x6f, 0xf0, 0x59, 0xbb, 0xbf,
                                0x2b, 0xf5, 0x8e, 0x0f, 0xd3,
                            ],
                            provisioned: false,
                            uplink_limit_reload_rate: 1,
                            uplink_limit_bucket_size: 2,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            Test {
                name: "add second device".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            join_eui: vec![],
                            dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            dev_addr: vec![1, 2, 3, 4],
                            root_wor_s_key: vec![],
                            provisioned: true,
                            uplink_limit_bucket_size: 2,
                            uplink_limit_reload_rate: 1,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![
                    (
                        EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
                        DevAddr::from_be_bytes([1, 2, 3, 4]),
                    ),
                    (
                        EUI64::from_be_bytes([3, 3, 3, 3, 3, 3, 3, 3]),
                        DevAddr::from_be_bytes([2, 2, 3, 4]),
                    ),
                ],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::UpdateUplinkListReq(lrwn::UpdateUplinkListReqPayload {
                        uplink_list_idx: 0,
                        uplink_limit: lrwn::UplinkLimitPL {
                            reload_rate: 1,
                            bucket_size: 2,
                        },
                        dev_addr: DevAddr::from_be_bytes([2, 2, 3, 4]),
                        w_fcnt: 0,
                        root_wor_s_key: AES128Key::from_bytes([
                            0x47, 0x71, 0x18, 0x16, 0xe9, 0x1d, 0x6f, 0xf0, 0x59, 0xbb, 0xbf, 0x2b,
                            0xf5, 0x8e, 0x0f, 0xd3,
                        ]),
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![
                            internal::RelayDevice {
                                index: 1,
                                join_eui: vec![],
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                                dev_addr: vec![1, 2, 3, 4],
                                root_wor_s_key: vec![],
                                provisioned: true,
                                uplink_limit_reload_rate: 1,
                                uplink_limit_bucket_size: 2,
                                w_f_cnt_last_request: None,
                            },
                            internal::RelayDevice {
                                index: 0,
                                join_eui: vec![],
                                dev_eui: vec![3, 3, 3, 3, 3, 3, 3, 3],
                                dev_addr: vec![2, 2, 3, 4],
                                root_wor_s_key: vec![
                                    0x47, 0x71, 0x18, 0x16, 0xe9, 0x1d, 0x6f, 0xf0, 0x59, 0xbb,
                                    0xbf, 0x2b, 0xf5, 0x8e, 0x0f, 0xd3,
                                ],
                                provisioned: false,
                                uplink_limit_reload_rate: 1,
                                uplink_limit_bucket_size: 2,
                                w_f_cnt_last_request: None,
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
        ];

        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let dp_relay = device_profile::create(device_profile::DeviceProfile {
            name: "dp-relay".into(),
            tenant_id: t.id,
            is_relay: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp_ed = device_profile::create(device_profile::DeviceProfile {
            name: "dp-ed".into(),
            tenant_id: t.id,
            is_relay_ed: true,
            relay_ed_uplink_limit_bucket_size: 2,
            relay_ed_uplink_limit_reload_rate: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let d_relay = device::create(device::Device {
            dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            name: "relay".into(),
            application_id: app.id,
            device_profile_id: dp_relay.id,
            ..Default::default()
        })
        .await
        .unwrap();

        for test in &tests {
            println!("> {}", test.name);

            // create devices + add to relay
            for (dev_eui, dev_addr) in &test.relay_devices {
                let d = device::create(device::Device {
                    name: dev_eui.to_string(),
                    dev_eui: *dev_eui,
                    dev_addr: Some(*dev_addr),
                    application_id: app.id,
                    device_profile_id: dp_ed.id,
                    ..Default::default()
                })
                .await
                .unwrap();

                let _ = device_session::save(&internal::DeviceSession {
                    dev_addr: dev_addr.to_vec(),
                    dev_eui: dev_eui.to_vec(),
                    nwk_s_enc_key: vec![0; 16],
                    ..Default::default()
                })
                .await
                .unwrap();

                relay::add_device(d_relay.dev_eui, d.dev_eui).await.unwrap();
            }

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: t.clone(),
                application: app.clone(),
                device_profile: dp_relay.clone(),
                device: d_relay.clone(),
                device_session: test.device_session.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx._update_uplink_list().await.unwrap();

            // cleanup devices
            for (dev_eui, _) in &test.relay_devices {
                device::delete(dev_eui).await.unwrap();
            }

            // We can not predict the w_f_cnt_last_request timestamp.
            if let Some(relay) = &mut ctx.device_session.relay {
                for rd in &mut relay.devices {
                    rd.w_f_cnt_last_request = None;
                }
            }

            assert_eq!(test.expected_mac_commands, ctx.mac_commands);
            assert_eq!(test.expected_device_session, ctx.device_session);
        }
    }

    #[tokio::test]
    async fn test_update_filter_list() {
        struct Test {
            name: String,
            device_session: internal::DeviceSession,
            relay_devices: Vec<(EUI64, EUI64)>, // DevEUI + JoinEUI
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
            expected_device_session: internal::DeviceSession,
        }

        let tests = vec![
            Test {
                name: "no devices + empty ds".into(),
                device_session: internal::DeviceSession::default(),
                relay_devices: vec![],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 0,
                        filter_list_action: lrwn::FilterListAction::Filter,
                        filter_list_eui: vec![],
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 0,
                            action: 2,
                            provisioned: false,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            Test {
                name: "no devices + unprovisioned filter".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 0,
                            action: 2,
                            provisioned: false,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 0,
                        filter_list_action: lrwn::FilterListAction::Filter,
                        filter_list_eui: vec![],
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 0,
                            action: 2,
                            provisioned: false,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            Test {
                name: "no devices + provisioned filter".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 0,
                            action: 2,
                            provisioned: true,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![],
                expected_mac_commands: vec![],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 0,
                            action: 2,
                            provisioned: true,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            Test {
                name: "device in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![(
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 0]),
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 1]),
                )],
                expected_mac_commands: vec![],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                        ],
                        ..Default::default()
                    }),

                    ..Default::default()
                },
            },
            Test {
                name: "update device".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![(
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 0]),
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 3]),
                )],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 1,
                        filter_list_action: lrwn::FilterListAction::Forward,
                        filter_list_eui: vec![2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 0],
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 3],
                                provisioned: false,
                            },
                        ],
                        ..Default::default()
                    }),

                    ..Default::default()
                },
            },
            Test {
                name: "add device".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![
                    (
                        EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 0]),
                        EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 1]),
                    ),
                    (
                        EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
                        EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 3]),
                    ),
                ],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 2,
                        filter_list_action: lrwn::FilterListAction::Forward,
                        filter_list_eui: vec![2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2],
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                            internal::RelayFilter {
                                index: 2,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 2],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 3],
                                provisioned: false,
                            },
                        ],
                        ..Default::default()
                    }),

                    ..Default::default()
                },
            },
            Test {
                name: "remove filter".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                relay_devices: vec![],
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 1,
                        filter_list_action: lrwn::FilterListAction::NoRule,
                        filter_list_eui: vec![],
                    }),
                ])],
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![
                            internal::RelayFilter {
                                index: 0,
                                action: 2,
                                provisioned: true,
                                ..Default::default()
                            },
                            internal::RelayFilter {
                                index: 1,
                                action: 1,
                                dev_eui: vec![2, 2, 2, 2, 2, 2, 2, 0],
                                join_eui: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                provisioned: true,
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
        ];

        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let dp_relay = device_profile::create(device_profile::DeviceProfile {
            name: "dp-relay".into(),
            tenant_id: t.id,
            is_relay: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp_ed = device_profile::create(device_profile::DeviceProfile {
            name: "dp-ed".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let d_relay = device::create(device::Device {
            dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            name: "relay".into(),
            application_id: app.id,
            device_profile_id: dp_relay.id,
            ..Default::default()
        })
        .await
        .unwrap();

        for test in &tests {
            println!("> {}", test.name);

            // create devices + add to relay
            for (dev_eui, join_eui) in &test.relay_devices {
                let d = device::create(device::Device {
                    name: dev_eui.to_string(),
                    dev_eui: *dev_eui,
                    join_eui: *join_eui,
                    application_id: app.id,
                    device_profile_id: dp_ed.id,
                    ..Default::default()
                })
                .await
                .unwrap();

                relay::add_device(d_relay.dev_eui, d.dev_eui).await.unwrap();
            }

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: t.clone(),
                application: app.clone(),
                device_profile: dp_relay.clone(),
                device: d_relay.clone(),
                device_session: test.device_session.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx._update_filter_list().await.unwrap();

            // cleanup devices
            for (dev_eui, _) in &test.relay_devices {
                device::delete(dev_eui).await.unwrap();
            }

            assert_eq!(test.expected_mac_commands, ctx.mac_commands);
            assert_eq!(test.expected_device_session, ctx.device_session);
        }
    }

    #[tokio::test]
    async fn test_update_relay_conf() {
        struct Test {
            name: String,
            device_session: internal::DeviceSession,
            device_profile: device_profile::DeviceProfile,
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
        }

        let tests = vec![
            Test {
                name: "relay in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        enabled: true,
                        cad_periodicity: 1,
                        default_channel_index: 0,
                        second_channel_freq: 868300000,
                        second_channel_dr: 3,
                        second_channel_ack_offset: 2,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_profile: device_profile::DeviceProfile {
                    is_relay: true,
                    relay_enabled: true,
                    relay_cad_periodicity: 1,
                    relay_default_channel_index: 0,
                    relay_second_channel_freq: 868300000,
                    relay_second_channel_dr: 3,
                    relay_second_channel_ack_offset: 2,
                    ..Default::default()
                },
                expected_mac_commands: vec![],
            },
            Test {
                name: "relay out of sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        enabled: true,
                        cad_periodicity: 1,
                        default_channel_index: 0,
                        second_channel_freq: 868300000,
                        second_channel_dr: 3,
                        second_channel_ack_offset: 2,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_profile: device_profile::DeviceProfile {
                    is_relay: true,
                    relay_enabled: true,
                    relay_cad_periodicity: 1,
                    relay_default_channel_index: 0,
                    relay_second_channel_freq: 868500000,
                    relay_second_channel_dr: 3,
                    relay_second_channel_ack_offset: 2,
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RelayConfReq(lrwn::RelayConfReqPayload {
                        channel_settings_relay: lrwn::ChannelSettingsRelay {
                            start_stop: 1,
                            cad_periodicity: 1,
                            default_ch_idx: 0,
                            second_ch_idx: 1,
                            second_ch_dr: 3,
                            second_ch_ack_offset: 2,
                        },
                        second_ch_freq: 868500000,
                    }),
                ])],
            },
        ];

        let _guard = test::prepare().await;

        for test in &tests {
            println!("> {}", test.name);

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: tenant::Tenant::default(),
                application: application::Application::default(),
                device_profile: test.device_profile.clone(),
                device: device::Device::default(),
                device_session: test.device_session.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx._update_relay_conf().await.unwrap();

            assert_eq!(test.expected_mac_commands, ctx.mac_commands);
        }
    }

    #[tokio::test]
    async fn test_update_end_device_conf() {
        struct Test {
            name: String,
            device_session: internal::DeviceSession,
            device_profile: device_profile::DeviceProfile,
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
        }

        let tests = vec![
            Test {
                name: "device is in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        ed_activation_mode: 1,
                        ed_smart_enable_level: 1,
                        ed_back_off: 16,
                        second_channel_freq: 868100000,
                        second_channel_dr: 3,
                        second_channel_ack_offset: 4,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_profile: device_profile::DeviceProfile {
                    relay_ed_activation_mode: lrwn::RelayModeActivation::EnableRelayMode,
                    relay_ed_smart_enable_level: 1,
                    relay_ed_back_off: 16,
                    relay_second_channel_freq: 868100000,
                    relay_second_channel_dr: 3,
                    relay_second_channel_ack_offset: 4,
                    ..Default::default()
                },
                expected_mac_commands: vec![],
            },
            Test {
                name: "device is not in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        ed_activation_mode: 0,
                        ed_smart_enable_level: 1,
                        ed_back_off: 16,
                        second_channel_freq: 868100000,
                        second_channel_dr: 3,
                        second_channel_ack_offset: 4,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_profile: device_profile::DeviceProfile {
                    relay_ed_activation_mode: lrwn::RelayModeActivation::EnableRelayMode,
                    relay_ed_smart_enable_level: 1,
                    relay_ed_back_off: 16,
                    relay_second_channel_freq: 868100000,
                    relay_second_channel_dr: 3,
                    relay_second_channel_ack_offset: 4,
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::EndDeviceConfReq(lrwn::EndDeviceConfReqPayload {
                        activation_relay_mode: lrwn::ActivationRelayMode {
                            relay_mode_activation: lrwn::RelayModeActivation::EnableRelayMode,
                            smart_enable_level: 1,
                        },
                        channel_settings_ed: lrwn::ChannelSettingsED {
                            second_ch_ack_offset: 4,
                            second_ch_dr: 3,
                            second_ch_idx: 1,
                            backoff: 16,
                        },
                        second_ch_freq: 868100000,
                    }),
                ])],
            },
        ];

        let _guard = test::prepare().await;

        for test in &tests {
            println!("> {}", test.name);

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: tenant::Tenant::default(),
                application: application::Application::default(),
                device_profile: test.device_profile.clone(),
                device: device::Device::default(),
                device_session: test.device_session.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx._update_end_device_conf().await.unwrap();

            assert_eq!(test.expected_mac_commands, ctx.mac_commands);
        }
    }

    #[tokio::test]
    async fn test_configure_fwd_limit_req() {
        struct Test {
            name: String,
            device_session: internal::DeviceSession,
            device_profile: device_profile::DeviceProfile,
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
        }

        let tests = vec![
            Test {
                name: "relay is in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        join_req_limit_reload_rate: 10,
                        join_req_limit_bucket_size: 0,
                        notify_limit_reload_rate: 15,
                        notify_limit_bucket_size: 1,
                        global_uplink_limit_reload_rate: 20,
                        global_uplink_limit_bucket_size: 2,
                        overall_limit_reload_rate: 25,
                        overall_limit_bucket_size: 3,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_profile: device_profile::DeviceProfile {
                    relay_join_req_limit_reload_rate: 10,
                    relay_join_req_limit_bucket_size: 0,
                    relay_notify_limit_reload_rate: 15,
                    relay_notify_limit_bucket_size: 1,
                    relay_global_uplink_limit_reload_rate: 20,
                    relay_global_uplink_limit_bucket_size: 2,
                    relay_overall_limit_reload_rate: 25,
                    relay_overall_limit_bucket_size: 3,
                    ..Default::default()
                },
                expected_mac_commands: vec![],
            },
            Test {
                name: "relay is not in sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        join_req_limit_reload_rate: 5,
                        join_req_limit_bucket_size: 0,
                        notify_limit_reload_rate: 15,
                        notify_limit_bucket_size: 1,
                        global_uplink_limit_reload_rate: 20,
                        global_uplink_limit_bucket_size: 2,
                        overall_limit_reload_rate: 25,
                        overall_limit_bucket_size: 3,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_profile: device_profile::DeviceProfile {
                    relay_join_req_limit_reload_rate: 10,
                    relay_join_req_limit_bucket_size: 0,
                    relay_notify_limit_reload_rate: 15,
                    relay_notify_limit_bucket_size: 1,
                    relay_global_uplink_limit_reload_rate: 20,
                    relay_global_uplink_limit_bucket_size: 2,
                    relay_overall_limit_reload_rate: 25,
                    relay_overall_limit_bucket_size: 3,
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::ConfigureFwdLimitReq(lrwn::ConfigureFwdLimitReqPayload {
                        reload_rate: lrwn::FwdLimitReloadRatePL {
                            join_req_reload_rate: 10,
                            notify_reload_rate: 15,
                            global_uplink_reload_rate: 20,
                            overall_reload_rate: 25,
                            reset_limit_counter: lrwn::ResetLimitCounter::NoChange,
                        },
                        load_capacity: lrwn::FwdLimitLoadCapacityPL {
                            join_req_limit_size: 0,
                            notify_limit_size: 1,
                            global_uplink_limit_size: 2,
                            overall_limit_size: 3,
                        },
                    }),
                ])],
            },
        ];

        let _guard = test::prepare().await;

        for test in &tests {
            println!("> {}", test.name);

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: tenant::Tenant::default(),
                application: application::Application::default(),
                device_profile: test.device_profile.clone(),
                device: device::Device::default(),
                device_session: test.device_session.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx._configure_fwd_limit_req().await.unwrap();

            assert_eq!(test.expected_mac_commands, ctx.mac_commands);
        }
    }

    #[tokio::test]
    async fn test_request_ctrl_uplink_list() {
        struct Test {
            name: String,
            relay_devices: Vec<EUI64>,
            device_session: internal::DeviceSession,
            expected_mac_commands: Vec<lrwn::MACCommandSet>,
        }

        let tests = vec![
            Test {
                name: "w_f_cnt has been recently requested".into(),
                relay_devices: vec![EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2])],
                device_session: internal::DeviceSession {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            w_f_cnt_last_request: Some(Utc::now().into()),
                            dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_mac_commands: vec![],
            },
            Test {
                name: "w_f_cnt has never been requested".into(),
                relay_devices: vec![EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2])],
                device_session: internal::DeviceSession {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            w_f_cnt_last_request: None,
                            dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 1,
                            ctrl_uplink_action: 0,
                        },
                    }),
                ])],
            },
            Test {
                name: "w_f_cnt has been requested two days ago".into(),
                relay_devices: vec![EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2])],
                device_session: internal::DeviceSession {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            w_f_cnt_last_request: Some(
                                Utc::now()
                                    .checked_sub_signed(chrono::Duration::hours(48))
                                    .unwrap()
                                    .into(),
                            ),
                            dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 1,
                            ctrl_uplink_action: 0,
                        },
                    }),
                ])],
            },
            Test {
                name: "more than three devices have outdated w_f_cnt".into(),
                relay_devices: vec![
                    EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2]),
                    EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 3]),
                    EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 4]),
                    EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 5]),
                ],
                device_session: internal::DeviceSession {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    relay: Some(internal::Relay {
                        devices: vec![
                            internal::RelayDevice {
                                index: 1,
                                w_f_cnt_last_request: None,
                                dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                                ..Default::default()
                            },
                            internal::RelayDevice {
                                index: 2,
                                w_f_cnt_last_request: None,
                                dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 3],
                                ..Default::default()
                            },
                            internal::RelayDevice {
                                index: 3,
                                w_f_cnt_last_request: None,
                                dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 4],
                                ..Default::default()
                            },
                            internal::RelayDevice {
                                index: 4,
                                w_f_cnt_last_request: None,
                                dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 5],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 1,
                            ctrl_uplink_action: 0,
                        },
                    }),
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 2,
                            ctrl_uplink_action: 0,
                        },
                    }),
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 3,
                            ctrl_uplink_action: 0,
                        },
                    }),
                    // The 4th is truncated
                ])],
            },
            Test {
                name: "device has been removed".into(),
                relay_devices: vec![],
                device_session: internal::DeviceSession {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            w_f_cnt_last_request: None,
                            dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_mac_commands: vec![lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 1,
                            ctrl_uplink_action: 1,
                        },
                    }),
                ])],
            },
        ];

        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let dp_relay = device_profile::create(device_profile::DeviceProfile {
            name: "dp-relay".into(),
            tenant_id: t.id,
            is_relay: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp_ed = device_profile::create(device_profile::DeviceProfile {
            name: "dp-ed".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let d_relay = device::create(device::Device {
            dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            name: "relay".into(),
            application_id: app.id,
            device_profile_id: dp_relay.id,
            ..Default::default()
        })
        .await
        .unwrap();

        for test in &tests {
            println!("> {}", test.name);

            // create devices + add to relay
            for dev_eui in &test.relay_devices {
                let d = device::create(device::Device {
                    name: dev_eui.to_string(),
                    dev_eui: *dev_eui,
                    application_id: app.id,
                    device_profile_id: dp_ed.id,
                    ..Default::default()
                })
                .await
                .unwrap();

                relay::add_device(d_relay.dev_eui, d.dev_eui).await.unwrap();
            }

            let mut ctx = Data {
                relay_context: None,
                uplink_frame_set: None,
                tenant: tenant::Tenant::default(),
                application: application::Application::default(),
                device_profile: device_profile::DeviceProfile::default(),
                device: d_relay.clone(),
                device_session: test.device_session.clone(),
                network_conf: config::get_region_network("eu868").unwrap(),
                region_conf: region::get("eu868").unwrap(),
                must_send: false,
                must_ack: false,
                mac_commands: vec![],
                device_gateway_rx_info: None,
                downlink_gateway: None,
                downlink_frame: Default::default(),
                downlink_frame_items: vec![],
                immediately: false,
                device_queue_item: None,
                more_device_queue_items: false,
            };

            ctx._request_ctrl_uplink_list().await.unwrap();

            // cleanup devices
            for dev_eui in &test.relay_devices {
                device::delete(dev_eui).await.unwrap();
            }

            assert_eq!(test.expected_mac_commands, ctx.mac_commands);
        }
    }
}
