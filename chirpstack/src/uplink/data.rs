use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, Utc};
use tracing::{debug, error, info, span, trace, warn, Instrument, Level};

use super::error::Error;
use super::{
    data_fns, filter_rx_info_by_region_config_id, filter_rx_info_by_tenant_id, helpers,
    RelayContext, UplinkFrameSet,
};
use crate::api::helpers::ToProto;
use crate::backend::roaming;
use crate::helpers::errors::PrintFullError;
use crate::storage::error::Error as StorageError;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_gateway, device_profile, device_queue, fields,
    helpers::get_all_device_data,
    metrics, tenant,
};
use crate::{codec, config, downlink, integration, maccommand, region, stream};
use chirpstack_api::{common, integration as integration_pb, internal, stream as stream_pb};
use lrwn::{AES128Key, EUI64};

pub struct Data {
    uplink_frame_set: UplinkFrameSet,
    relay_context: Option<RelayContext>,

    // We need a separate copy of the PhyPayload as we will be dealing with two PhyPayloads in case
    // of a Relay. In this case uplink_frame_set.phy_payload contains the uplink from the Relay,
    // and relay_context.req.payload will contain the wrapped phy_payload.
    // To avoid reimplementing all functions that read or modify the phy_payload, we copy it in a
    // separate value.
    phy_payload: lrwn::PhyPayload,

    reset: bool,
    retransmission: bool,
    f_cnt_up_full: u32,
    tenant: Option<tenant::Tenant>,
    device: Option<device::Device>,
    device_profile: Option<device_profile::DeviceProfile>,
    application: Option<application::Application>,
    device_info: Option<integration_pb::DeviceInfo>,
    relay_rx_info: Option<integration_pb::UplinkRelayRxInfo>,
    uplink_event: Option<integration_pb::UplinkEvent>,
    must_send_downlink: bool,
    downlink_mac_commands: Vec<lrwn::MACCommandSet>,
    device_gateway_rx_info: Option<internal::DeviceGatewayRxInfo>,
    device_changeset: device::DeviceChangeset,
}

impl Data {
    pub async fn handle(ufs: UplinkFrameSet) {
        let span = span!(Level::INFO, "data_up", dev_eui = tracing::field::Empty);

        if let Err(e) = Data::_handle(ufs).instrument(span).await {
            match e.downcast_ref::<Error>() {
                Some(Error::Abort) => {
                    // nothing to do
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle uplink error");
                }
            }
        }
    }

    pub async fn handle_relayed(
        relay_ctx: RelayContext,
        dev_gw_rx_info: internal::DeviceGatewayRxInfo,
        ufs: UplinkFrameSet,
    ) {
        let span = span!(Level::INFO, "data_up_relayed");

        if let Err(e) = Data::_handle_relayed(relay_ctx, dev_gw_rx_info, ufs)
            .instrument(span)
            .await
        {
            match e.downcast_ref::<Error>() {
                Some(Error::Abort) => {
                    // nothing to do
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle relayed uplink error");
                }
            }
        }
    }

    pub async fn _handle(ufs: UplinkFrameSet) -> Result<()> {
        let mut ctx = Data {
            phy_payload: ufs.phy_payload.clone(),
            uplink_frame_set: ufs,
            relay_context: None,
            f_cnt_up_full: 0,
            reset: false,
            retransmission: false,
            tenant: None,
            device: None,
            device_profile: None,
            application: None,
            device_info: None,
            relay_rx_info: None,
            uplink_event: None,
            must_send_downlink: false,
            downlink_mac_commands: Vec::new(),
            device_gateway_rx_info: None,
            device_changeset: Default::default(),
        };

        ctx.handle_passive_roaming_device().await?;
        ctx.get_device_for_phy_payload().await?;
        ctx.get_device_data().await?;
        ctx.check_roaming_allowed()?;

        // Add dev_eui to span
        let span = tracing::Span::current();
        span.record("dev_eui", ctx.device.as_ref().unwrap().dev_eui.to_string());

        if !ctx._is_roaming() {
            // In case of roaming we do not know the gateways and therefore it must not be
            // filtered.
            ctx.filter_rx_info_by_tenant().await?;
            ctx.filter_rx_info_by_region_config_id()?;
        }
        ctx.set_device_info()?;
        ctx.set_device_gateway_rx_info()?;
        ctx.handle_retransmission_reset().await?;
        ctx.set_scheduler_run_after().await?;
        ctx.decrypt_f_opts_mac_commands()?;
        ctx.decrypt_frm_payload()?;
        ctx.log_uplink_frame_set().await?;
        ctx.set_adr()?;
        ctx.set_uplink_data_rate().await?;
        ctx.handle_class_b_beacon_locked().await?;
        ctx.log_uplink_meta().await?;
        ctx.reset_channels_on_adr_ack_req()?;
        ctx.handle_mac_commands().await?;
        if !ctx._is_roaming() {
            ctx.save_device_gateway_rx_info().await?;
        }
        ctx.append_meta_data_to_uplink_history()?;
        ctx.send_uplink_event().await?;
        ctx.detect_and_save_measurements().await?;
        ctx.sync_uplink_f_cnt()?;
        ctx.set_region_config_id()?;
        ctx.update_device().await?;
        ctx.handle_uplink_ack().await?;
        ctx.save_metrics().await?;

        if ctx._is_relay() {
            ctx.handle_forward_uplink_req().await?;
        } else {
            ctx.start_downlink_data_flow().await?;
        }

        Ok(())
    }

    async fn _handle_relayed(
        relay_ctx: RelayContext,
        dev_gw_rx_info: internal::DeviceGatewayRxInfo,
        ufs: UplinkFrameSet,
    ) -> Result<()> {
        let mut ctx = Data {
            phy_payload: *relay_ctx.req.payload.clone(),
            uplink_frame_set: ufs,
            relay_context: Some(relay_ctx),
            device_gateway_rx_info: Some(dev_gw_rx_info),
            f_cnt_up_full: 0,
            reset: false,
            retransmission: false,
            tenant: None,
            device: None,
            device_profile: None,
            application: None,
            device_info: None,
            relay_rx_info: None,
            uplink_event: None,
            must_send_downlink: false,
            downlink_mac_commands: Vec::new(),
            device_changeset: Default::default(),
        };

        ctx.get_device_for_phy_payload_relayed().await?;
        ctx.get_device_data().await?;
        ctx.set_device_info()?;
        ctx.set_relay_rx_info()?;
        ctx.handle_retransmission_reset().await?;
        ctx.decrypt_f_opts_mac_commands()?;
        ctx.decrypt_frm_payload()?;
        ctx.set_adr()?;
        ctx.set_uplink_data_rate_relayed().await?;
        ctx.handle_class_b_beacon_locked().await?;
        ctx.reset_channels_on_adr_ack_req()?;
        ctx.handle_mac_commands().await?;
        ctx.append_meta_data_to_uplink_history_relayed()?;
        ctx.send_uplink_event().await?;
        ctx.detect_and_save_measurements().await?;
        ctx.sync_uplink_f_cnt()?;
        ctx.set_region_config_id()?;
        ctx.update_device().await?;
        ctx.handle_uplink_ack().await?;
        ctx.save_metrics_relayed().await?;
        ctx.start_downlink_data_flow_relayed().await?;

        Ok(())
    }

    async fn handle_passive_roaming_device(&mut self) -> Result<(), Error> {
        trace!("Handling passive-roaming device");
        let mac = if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            pl
        } else {
            return Err(Error::Anyhow(anyhow!("Expected MacPayload")));
        };

        if roaming::is_roaming_dev_addr(mac.fhdr.devaddr) {
            debug!(dev_addr = %mac.fhdr.devaddr, "DevAddr does not match NetID, assuming roaming device");
            data_fns::Data::handle(self.uplink_frame_set.clone(), mac.clone()).await;

            return Err(Error::Abort);
        }

        Ok(())
    }

    async fn get_device_for_phy_payload(&mut self) -> Result<(), Error> {
        trace!("Getting device for PhyPayload");

        let dev_addr = if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            pl.fhdr.devaddr
        } else {
            return Err(Error::Anyhow(anyhow!("No MacPayload in PhyPayload")));
        };

        match device::get_for_phypayload_and_incr_f_cnt_up(
            false,
            &mut self.phy_payload,
            self.uplink_frame_set.dr,
            self.uplink_frame_set.ch as u8,
        )
        .await
        {
            Ok(v) => match v {
                device::ValidationStatus::Ok(f_cnt, d) => {
                    self.device = Some(d);
                    self.f_cnt_up_full = f_cnt;
                }
                device::ValidationStatus::Retransmission(f_cnt, d) => {
                    self.retransmission = true;
                    self.device = Some(d);
                    self.f_cnt_up_full = f_cnt;
                }
                device::ValidationStatus::Reset(f_cnt, d) => {
                    self.reset = true;
                    self.device = Some(d);
                    self.f_cnt_up_full = f_cnt;
                }
            },
            Err(e) => match e {
                StorageError::NotFound(s) => {
                    info!(dev_addr = %s, "No device-session exists for dev_addr");
                    return Err(Error::Abort);
                }
                StorageError::InvalidMIC => {
                    info!(dev_addr = %dev_addr, "None of the device-sessions for dev_addr resulted in valid MIC");

                    // Log uplink for null DevEUI.
                    let mut ufl: stream_pb::UplinkFrameLog = (&self.uplink_frame_set).try_into()?;
                    ufl.dev_eui = "0000000000000000".to_string();
                    stream::frame::log_uplink_for_device(&ufl).await?;

                    return Err(Error::Abort);
                }
                _ => {
                    return Err(Error::Anyhow(
                        anyhow::Error::new(e).context("Get device-session"),
                    ));
                }
            },
        };

        Ok(())
    }

    async fn get_device_for_phy_payload_relayed(&mut self) -> Result<(), Error> {
        trace!("Getting device for PhyPayload (relayed)");

        let relay_ctx = self.relay_context.as_ref().unwrap();

        let dev_addr = if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            pl.fhdr.devaddr
        } else {
            return Err(Error::Anyhow(anyhow!("No MacPayload in PhyPayload")));
        };

        let dr = relay_ctx.req.metadata.dr;
        let ch = helpers::get_uplink_ch(
            &self.uplink_frame_set.region_config_id,
            relay_ctx.req.frequency,
            dr,
        )? as u8;

        match device::get_for_phypayload_and_incr_f_cnt_up(true, &mut self.phy_payload, dr, ch)
            .await
        {
            Ok(v) => match v {
                device::ValidationStatus::Ok(f_cnt, d) => {
                    self.device = Some(d);
                    self.f_cnt_up_full = f_cnt;
                }
                device::ValidationStatus::Retransmission(f_cnt, d) => {
                    self.retransmission = true;
                    self.device = Some(d);
                    self.f_cnt_up_full = f_cnt;
                }
                device::ValidationStatus::Reset(f_cnt, d) => {
                    self.reset = true;
                    self.device = Some(d);
                    self.f_cnt_up_full = f_cnt;
                }
            },
            Err(e) => match e {
                StorageError::NotFound(s) => {
                    info!(dev_addr = %s, "No device-session exists for dev_addr");
                    return Err(Error::Abort);
                }
                StorageError::InvalidMIC => {
                    info!(dev_addr = %dev_addr, "None of the device-sessions for dev_addr resulted in valid MIC");
                    return Err(Error::Abort);
                }
                _ => {
                    return Err(Error::Anyhow(
                        anyhow::Error::new(e).context("Get device-session"),
                    ));
                }
            },
        }

        Ok(())
    }

    async fn get_device_data(&mut self) -> Result<()> {
        trace!("Getting device data");

        let dev_eui = self.device.as_ref().unwrap().dev_eui;
        let (_, app, t, dp) = get_all_device_data(dev_eui).await?;

        if dp.region != self.uplink_frame_set.region_common_name {
            return Err(anyhow!("Invalid device-profile region"));
        }

        self.tenant = Some(t);
        self.application = Some(app);
        self.device_profile = Some(dp);

        Ok(())
    }

    fn check_roaming_allowed(&self) -> Result<(), Error> {
        trace!("Check if roaming is allowed for this device");
        if self._is_roaming() {
            let dp = self.device_profile.as_ref().unwrap();
            if !dp.allow_roaming {
                return Err(Error::RoamingIsNotAllowed);
            }
        }

        Ok(())
    }

    fn set_device_info(&mut self) -> Result<()> {
        trace!("Setting device-info");

        let tenant = self.tenant.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        let mut tags = (*app.tags).clone();
        tags.extend((*dp.tags).clone());
        tags.extend((*dev.tags).clone());

        self.device_info = Some(integration_pb::DeviceInfo {
            tenant_id: tenant.id.to_string(),
            tenant_name: tenant.name.clone(),
            application_id: app.id.to_string(),
            application_name: app.name.to_string(),
            device_profile_id: dp.id.to_string(),
            device_profile_name: dp.name.clone(),
            device_name: dev.name.clone(),
            device_class_enabled: dev.enabled_class.to_proto().into(),
            dev_eui: dev.dev_eui.to_string(),
            tags,
        });

        Ok(())
    }

    fn set_relay_rx_info(&mut self) -> Result<()> {
        let relay_ctx = self.relay_context.as_ref().unwrap();

        self.relay_rx_info = Some(integration_pb::UplinkRelayRxInfo {
            dev_eui: relay_ctx.device.dev_eui.to_string(),
            frequency: relay_ctx.req.frequency,
            dr: relay_ctx.req.metadata.dr as u32,
            snr: relay_ctx.req.metadata.snr as i32,
            rssi: relay_ctx.req.metadata.rssi as i32,
            wor_channel: relay_ctx.req.metadata.wor_channel as u32,
        });

        Ok(())
    }

    fn set_device_gateway_rx_info(&mut self) -> Result<()> {
        trace!("Setting gateway rx-info for device");
        let d = self.device.as_ref().unwrap();

        self.device_gateway_rx_info = Some(internal::DeviceGatewayRxInfo {
            dev_eui: d.dev_eui.to_vec(),
            dr: self.uplink_frame_set.dr as u32,
            items: self
                .uplink_frame_set
                .rx_info_set
                .iter()
                .map(|rx_info| {
                    let gw_id = EUI64::from_str(&rx_info.gateway_id).unwrap_or_default();

                    internal::DeviceGatewayRxInfoItem {
                        gateway_id: gw_id.to_vec(),
                        rssi: rx_info.rssi,
                        lora_snr: rx_info.snr,
                        antenna: rx_info.antenna,
                        board: rx_info.board,
                        context: rx_info.context.clone(),
                        is_private_up: self
                            .uplink_frame_set
                            .gateway_private_up_map
                            .get(&gw_id)
                            .cloned()
                            .unwrap_or_default(),
                        is_private_down: self
                            .uplink_frame_set
                            .gateway_private_down_map
                            .get(&gw_id)
                            .cloned()
                            .unwrap_or_default(),
                        tenant_id: self
                            .uplink_frame_set
                            .gateway_tenant_id_map
                            .get(&gw_id)
                            .map(|v| v.into_bytes().to_vec())
                            .unwrap_or_default(),
                    }
                })
                .collect(),
        });

        Ok(())
    }

    async fn handle_retransmission_reset(&self) -> Result<(), Error> {
        trace!("Handle retransmission and reset");
        let dev = self.device.as_ref().unwrap();

        if (!self.retransmission && !self.reset) || dev.skip_fcnt_check {
            return Ok(());
        }

        let app = self.application.as_ref().unwrap();
        let ts: DateTime<Utc> =
            helpers::get_rx_timestamp(&self.uplink_frame_set.rx_info_set).into();

        if self.retransmission {
            let pl = integration_pb::LogEvent {
                time: Some(ts.into()),
                device_info: self.device_info.clone(),
                level: integration_pb::LogLevel::Warning.into(),
                code: integration_pb::LogCode::UplinkFCntRetransmission.into(),
                description:
                    "Uplink was flagged as re-transmission / frame-counter did not increment".into(),
                context: [(
                    "deduplication_id".to_string(),
                    self.uplink_frame_set.uplink_set_id.to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
            };
            integration::log_event(app.id.into(), &dev.variables, &pl).await;
        }

        if self.reset {
            let pl = integration_pb::LogEvent {
                time: Some(ts.into()),
                device_info: self.device_info.clone(),
                level: integration_pb::LogLevel::Warning.into(),
                code: integration_pb::LogCode::UplinkFCntReset.into(),
                description: "Frame-counter reset or rollover detected".into(),
                context: [(
                    "deduplication_id".to_string(),
                    self.uplink_frame_set.uplink_set_id.to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
            };
            integration::log_event(app.id.into(), &dev.variables, &pl).await;
        }

        Err(Error::Abort)
    }

    // For Class-B and Class-C devices, set the scheduler_run_after timestamp to avoid collisions with
    // the Class-A downlink and Class-B/C scheduler.
    async fn set_scheduler_run_after(&mut self) -> Result<()> {
        let dev = self.device.as_mut().unwrap();
        let conf = config::get();

        if dev.enabled_class == DeviceClass::B || dev.enabled_class == DeviceClass::C {
            trace!("Setting scheduler_run_after for device");
            let scheduler_run_after =
                Utc::now() + Duration::from_std(conf.network.scheduler.class_a_lock_duration)?;

            // Only set the new scheduler_run_after if it is currently None
            // or when the current value is before the calculated scheduler_run_after.
            if dev.scheduler_run_after.is_none()
                || scheduler_run_after > dev.scheduler_run_after.unwrap()
            {
                *dev = device::partial_update(
                    dev.dev_eui,
                    &device::DeviceChangeset {
                        scheduler_run_after: Some(Some(scheduler_run_after)),
                        ..Default::default()
                    },
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn filter_rx_info_by_tenant(&mut self) -> Result<()> {
        trace!("Filtering rx_info by tenant_id");

        match filter_rx_info_by_tenant_id(
            self.application.as_ref().unwrap().tenant_id.into(),
            &mut self.uplink_frame_set,
        ) {
            Ok(_) => Ok(()),
            Err(v) => {
                // Restore the device-session in case of an error (no gateways available).
                // This is because during the fcnt validation, we immediately store the
                // device-session with incremented fcnt to avoid race conditions.
                let d = self.device.as_ref().unwrap();
                device::partial_update(
                    d.dev_eui,
                    &device::DeviceChangeset {
                        device_session: Some(d.device_session.clone()),
                        ..Default::default()
                    },
                )
                .await?;

                Err(v)
            }
        }
    }

    fn filter_rx_info_by_region_config_id(&mut self) -> Result<()> {
        trace!("Filtering rx_info by region_config_id");

        let dp = self.device_profile.as_ref().unwrap();
        if let Some(v) = &dp.region_config_id {
            filter_rx_info_by_region_config_id(v, &mut self.uplink_frame_set)?;
        }

        Ok(())
    }

    fn decrypt_f_opts_mac_commands(&mut self) -> Result<()> {
        trace!("Decrypting mac-commands");
        let ds = self.device.as_ref().unwrap().get_device_session()?;
        if ds.mac_version().to_string().starts_with("1.0") {
            if let Err(e) = self.phy_payload.decode_f_opts_to_mac_commands() {
                // This avoids failing in case of a corrupted mac-command in the frm_payload.
                warn!(error = %e.full(), "Decoding f_opts mac-commands failed");
            }
        } else {
            let nwk_s_enc_key = AES128Key::from_slice(&ds.nwk_s_enc_key)?;
            if let Err(e) = self.phy_payload.decrypt_f_opts(&nwk_s_enc_key) {
                // This avoids failing in case of a corrupted mac-command in the frm_payload.
                warn!(error = %e.full(), "Decrypting f_opts mac-commands failed");
            }
        }

        Ok(())
    }

    fn decrypt_frm_payload(&mut self) -> Result<()> {
        trace!("Decrypting FRMPayload");
        let ds = self.device.as_ref().unwrap().get_device_session()?;
        let mut f_port = 0;

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            f_port = pl.f_port.unwrap_or(0);
        }

        // Mac-commands (f_port=0) or Relay payload (f_port=226).
        if f_port == 0 || f_port == lrwn::LA_FPORT_RELAY {
            let nwk_s_enc_key = AES128Key::from_slice(&ds.nwk_s_enc_key)?;
            if let Err(e) = self.phy_payload.decrypt_frm_payload(&nwk_s_enc_key) {
                // This avoids failing in case of a corrupted mac-command in the frm_payload.
                warn!(error = %e.full(), "Decrypting frm_payload failed");
            }
        } else if !self._is_end_to_end_encrypted() {
            if let Some(app_s_key) = &ds.app_s_key {
                let app_s_key = AES128Key::from_slice(&app_s_key.aes_key)?;
                self.phy_payload.decrypt_frm_payload(&app_s_key)?;
            }
        }

        Ok(())
    }

    async fn log_uplink_frame_set(&self) -> Result<()> {
        trace!("Logging uplink frame-set");
        let mut ufl: stream_pb::UplinkFrameLog = (&self.uplink_frame_set).try_into()?;
        ufl.dev_eui = self.device.as_ref().unwrap().dev_eui.to_string();

        // self.phy_payload holds the decrypted payload.
        ufl.plaintext_f_opts = true;
        ufl.plaintext_frm_payload = true;
        ufl.phy_payload = self.phy_payload.to_vec()?;

        stream::frame::log_uplink_for_device(&ufl).await?;
        Ok(())
    }

    fn set_adr(&mut self) -> Result<()> {
        trace!("Set ADR flag in device-session");
        let ds = self.device.as_mut().unwrap().get_device_session_mut()?;
        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            ds.adr = pl.fhdr.f_ctrl.adr;
        }
        Ok(())
    }

    async fn set_uplink_data_rate(&mut self) -> Result<()> {
        trace!("Set uplink data-rate and reset tx-power on change");
        let device = self.device.as_mut().unwrap();

        self.device_changeset.last_seen_at = Some(Some(Utc::now()));
        if device.dr.is_none() || self.uplink_frame_set.dr as i16 != device.dr.unwrap_or_default() {
            self.device_changeset.dr = Some(Some(self.uplink_frame_set.dr.into()));
        }

        let ds = device.get_device_session_mut()?;
        // The node changed its data-rate. Possibly the node did also reset its
        // tx-power to max power. Because of this, we need to reset the tx-power
        // and the uplink history at the network-server side too.
        if ds.dr != self.uplink_frame_set.dr as u32 {
            ds.tx_power_index = 0;
            ds.uplink_adr_history = Vec::new();
        }
        ds.dr = self.uplink_frame_set.dr as u32;

        Ok(())
    }

    async fn set_uplink_data_rate_relayed(&mut self) -> Result<()> {
        trace!("Set relayed uplink data-rate and reset tx-power on change");
        let device = self.device.as_mut().unwrap();
        let relay_ctx = self.relay_context.as_ref().unwrap();

        self.device_changeset.last_seen_at = Some(Some(Utc::now()));
        if device.dr.is_none() || self.uplink_frame_set.dr as i16 != device.dr.unwrap_or_default() {
            self.device_changeset.dr = Some(Some(self.uplink_frame_set.dr.into()));
        }

        let ds = device.get_device_session_mut()?;
        // The node changed its data-rate. Possibly the node did also reset its
        // tx-power to max power. Because of this, we need to reset the tx-power
        // and the uplink history at the network-server side too.
        if ds.dr != relay_ctx.req.metadata.dr as u32 {
            ds.tx_power_index = 0;
            ds.uplink_adr_history = Vec::new();
        }
        ds.dr = self.uplink_frame_set.dr as u32;
        Ok(())
    }

    async fn handle_class_b_beacon_locked(&mut self) -> Result<()> {
        trace!("Handle Class-B beacon locked");
        let dev = self.device.as_mut().unwrap();
        let dp = self.device_profile.as_ref().unwrap();

        let mut enabled_class = dev.enabled_class;

        if dp.supports_class_b {
            if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
                let locked = pl.fhdr.f_ctrl.class_b;
                enabled_class = match locked {
                    true => DeviceClass::B,
                    false => DeviceClass::A,
                };
            }
        }

        // Update if the enabled class has changed.
        if dev.enabled_class != enabled_class {
            self.device_changeset.enabled_class = Some(enabled_class);
        }

        Ok(())
    }

    async fn log_uplink_meta(&self) -> Result<()> {
        trace!("Logging uplink meta");

        if let lrwn::Payload::MACPayload(mac_pl) = &self.phy_payload.payload {
            let um = stream_pb::UplinkMeta {
                dev_eui: self.device.as_ref().unwrap().dev_eui.to_string(),
                tx_info: Some(self.uplink_frame_set.tx_info.clone()),
                rx_info: self.uplink_frame_set.rx_info_set.clone(),
                phy_payload_byte_count: self.phy_payload.to_vec()?.len() as u32,
                mac_command_byte_count: {
                    if mac_pl.f_port == Some(0) {
                        if let Some(lrwn::FRMPayload::MACCommandSet(v)) = &mac_pl.frm_payload {
                            v.size()?
                        } else {
                            0
                        }
                    } else {
                        mac_pl.fhdr.f_opts.size()?
                    }
                } as u32,
                application_payload_byte_count: {
                    if mac_pl.f_port.unwrap_or_default() > 0 {
                        if let Some(lrwn::FRMPayload::Raw(b)) = &mac_pl.frm_payload {
                            b.len()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } as u32,
                message_type: self.phy_payload.mhdr.m_type.to_proto().into(),
            };

            stream::meta::log_uplink(&um).await?;
        }

        Ok(())
    }

    // This is needed as in case the device sets the ADRAckReq bit, we do not know if the device
    // has reset its channels / channel-mask or not, as there is no explicit signalling in case
    // this happens. This way, we make sure that the channels are always in sync, although it could
    // lead to a small bit of overhead (e.g. re-sending the channels / channel-mask even if the
    // device did not reset these).
    fn reset_channels_on_adr_ack_req(&mut self) -> Result<()> {
        trace!("Reset channels on adr ack req");
        let d = self.device.as_mut().unwrap();

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            if pl.fhdr.f_ctrl.adr_ack_req {
                let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;
                let ds = d.get_device_session_mut()?;

                // We reset the device-session enabled_uplink_channel_indices and
                // extra_uplink_channels. On the downlink path, the mac-command handling will
                // detect that the device is out-of-sync with the NS configuration and will send
                // mac-commands to re-sync.
                ds.enabled_uplink_channel_indices = region_conf
                    .get_default_uplink_channel_indices()
                    .iter()
                    .map(|i| *i as u32)
                    .collect();
                ds.extra_uplink_channels = HashMap::new();
            }
        }

        Ok(())
    }

    async fn handle_mac_commands(&mut self) -> Result<()> {
        trace!("Handling uplink mac-commands");

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            if !(*pl.fhdr.f_opts).is_empty() {
                trace!("Mac-commands in f_opts");
                let (mac_response, must_send_downlink) = maccommand::handle_uplink(
                    &self.uplink_frame_set,
                    &pl.fhdr.f_opts,
                    self.tenant.as_ref().unwrap(),
                    self.application.as_ref().unwrap(),
                    self.device_profile.as_ref().unwrap(),
                    self.device.as_mut().unwrap(),
                )
                .await
                .context("Handle uplink mac-commands")?;

                self.must_send_downlink = must_send_downlink;
                self.downlink_mac_commands = mac_response;
            }

            if let Some(lrwn::FRMPayload::MACCommandSet(v)) = &pl.frm_payload {
                trace!("Mac-commands in frmPayload");
                let (mac_response, must_send_downlink) = maccommand::handle_uplink(
                    &self.uplink_frame_set,
                    v,
                    self.tenant.as_ref().unwrap(),
                    self.application.as_ref().unwrap(),
                    self.device_profile.as_ref().unwrap(),
                    self.device.as_mut().unwrap(),
                )
                .await
                .context("Handle uplink mac-commands")?;

                self.must_send_downlink = must_send_downlink;
                self.downlink_mac_commands = mac_response;
            }
        }

        Ok(())
    }

    async fn save_device_gateway_rx_info(&mut self) -> Result<()> {
        trace!("Saving gateway rx-info for device");

        device_gateway::save_rx_info(self.device_gateway_rx_info.as_ref().unwrap())
            .await
            .context("Save gatewa rx-info for device")?;

        Ok(())
    }

    fn append_meta_data_to_uplink_history(&mut self) -> Result<()> {
        let ds = self.device.as_mut().unwrap().get_device_session_mut()?;

        // ignore re-transmissions we don't know the source of the
        // re-transmission (it might be a replay-attack)
        if !ds.uplink_adr_history.is_empty()
            && ds.uplink_adr_history[ds.uplink_adr_history.len() - 1].f_cnt == self.f_cnt_up_full
        {
            return Ok(());
        }

        let mut max_snr = 0.0;
        for (i, rx_info) in self.uplink_frame_set.rx_info_set.iter().enumerate() {
            if i == 0 || rx_info.snr > max_snr {
                max_snr = rx_info.snr;
            }
        }

        let mut max_rssi = 0;
        for (i, rx_info) in self.uplink_frame_set.rx_info_set.iter().enumerate() {
            if i == 0 || rx_info.rssi > max_rssi {
                max_rssi = rx_info.rssi;
            }
        }

        ds.uplink_adr_history.push(internal::UplinkAdrHistory {
            f_cnt: self.f_cnt_up_full,
            max_snr,
            max_rssi,
            tx_power_index: ds.tx_power_index,
            gateway_count: self.uplink_frame_set.rx_info_set.len() as u32,
        });

        if ds.uplink_adr_history.len() > 20 {
            ds.uplink_adr_history = ds
                .uplink_adr_history
                .drain((ds.uplink_adr_history.len() - 20)..)
                .collect();
        }

        Ok(())
    }

    fn append_meta_data_to_uplink_history_relayed(&mut self) -> Result<()> {
        trace!("Apping meta-data of relayed uplink to upink history");

        let ds = self.device.as_mut().unwrap().get_device_session_mut()?;
        let relay_ctx = self.relay_context.as_ref().unwrap();

        // ignore re-transmissions we don't know the source of the
        // re-transmission (it might be a replay-attack)
        if !ds.uplink_adr_history.is_empty()
            && ds.uplink_adr_history[ds.uplink_adr_history.len() - 1].f_cnt == self.f_cnt_up_full
        {
            return Ok(());
        }

        ds.uplink_adr_history.push(internal::UplinkAdrHistory {
            f_cnt: self.f_cnt_up_full,
            max_snr: relay_ctx.req.metadata.snr as f32,
            max_rssi: relay_ctx.req.metadata.rssi as i32,
            tx_power_index: ds.tx_power_index,
            gateway_count: 1,
        });

        if ds.uplink_adr_history.len() > 20 {
            ds.uplink_adr_history = ds
                .uplink_adr_history
                .drain((ds.uplink_adr_history.len() - 20)..)
                .collect();
        }

        Ok(())
    }

    async fn send_uplink_event(&mut self) -> Result<()> {
        trace!("Sending uplink event");

        let ts: DateTime<Utc> =
            helpers::get_rx_timestamp(&self.uplink_frame_set.rx_info_set).into();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();
        let ds = dev.get_device_session()?;
        let mac = if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            pl
        } else {
            return Err(anyhow!("Expected MacPayload"));
        };

        let mut pl = integration_pb::UplinkEvent {
            deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
            time: Some(ts.into()),
            device_info: self.device_info.clone(),
            relay_rx_info: self.relay_rx_info.clone(),
            dev_addr: mac.fhdr.devaddr.to_string(),
            adr: mac.fhdr.f_ctrl.adr,
            dr: self.uplink_frame_set.dr as u32,
            f_cnt: self.f_cnt_up_full,
            f_port: mac.f_port.unwrap_or(0) as u32,
            confirmed: self.phy_payload.mhdr.m_type == lrwn::MType::ConfirmedDataUp,
            data: match &mac.frm_payload {
                Some(lrwn::FRMPayload::Raw(b)) => b.clone(),
                _ => Vec::new(),
            },
            object: None,
            rx_info: self.uplink_frame_set.rx_info_set.clone(),
            tx_info: Some(self.uplink_frame_set.tx_info.clone()),
            join_server_context: if self._is_end_to_end_encrypted() {
                Some(common::JoinServerContext {
                    session_key_id: hex::encode(&ds.js_session_key_id),
                    app_s_key: ds.app_s_key.clone(),
                })
            } else {
                None
            },
        };

        if !self._is_end_to_end_encrypted() {
            pl.object = match codec::binary_to_struct(
                dp.payload_codec_runtime,
                ts,
                mac.f_port.unwrap_or(0),
                &dev.variables,
                &dp.payload_codec_script,
                &pl.data,
            )
            .await
            {
                Ok(v) => v,
                Err(e) => {
                    integration::log_event(
                        app.id.into(),
                        &dev.variables,
                        &integration_pb::LogEvent {
                            time: Some(Utc::now().into()),
                            device_info: self.device_info.clone(),
                            level: integration_pb::LogLevel::Error.into(),
                            code: integration_pb::LogCode::UplinkCodec.into(),
                            description: format!("{:#}", e),
                            context: [(
                                "deduplication_id".to_string(),
                                pl.deduplication_id.clone(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        },
                    )
                    .await;
                    None
                }
            };
        }

        integration::uplink_event(app.id.into(), &dev.variables, &pl).await;

        self.uplink_event = Some(pl);

        Ok(())
    }

    async fn detect_and_save_measurements(&mut self) -> Result<()> {
        trace!("Detecing and saving measurements");

        let dp = self.device_profile.as_ref().unwrap();
        let up_event = self.uplink_event.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        let data_measurements: HashMap<String, pbjson_types::value::Kind> = match &up_event.object {
            None => HashMap::new(),
            Some(v) => codec::get_measurements(v),
        };

        let mut measurements = dp.measurements.clone();
        let mut update_dp_measurements = false;

        for (k, v) in data_measurements {
            if let Some(dp_m) = measurements.get(&k) {
                if dp_m.kind == fields::MeasurementKind::UNKNOWN {
                    continue;
                }

                // Only Number, String and BoolValues are expected.
                match v {
                    pbjson_types::value::Kind::NumberValue(v) => {
                        let record = metrics::Record {
                            time: DateTime::<Utc>::try_from(
                                up_event.time.as_ref().unwrap().clone(),
                            )
                            .map_err(anyhow::Error::msg)?
                            .with_timezone(&Local),
                            kind: match dp_m.kind {
                                fields::MeasurementKind::COUNTER => metrics::Kind::COUNTER,
                                fields::MeasurementKind::ABSOLUTE => metrics::Kind::ABSOLUTE,
                                fields::MeasurementKind::GAUGE => metrics::Kind::GAUGE,
                                _ => {
                                    continue;
                                }
                            },
                            metrics: [("value".to_string(), v)].iter().cloned().collect(),
                        };

                        metrics::save(
                            &format!("device:{}:{}", dev.dev_eui, k),
                            &record,
                            &metrics::Aggregation::default_aggregations(),
                        )
                        .await?;
                    }
                    pbjson_types::value::Kind::StringValue(v) => {
                        metrics::save_state(
                            &format!("device:{}:{}", dev.dev_eui, k),
                            &v.to_string(),
                        )
                        .await?;
                    }
                    pbjson_types::value::Kind::BoolValue(v) => {
                        metrics::save_state(
                            &format!("device:{}:{}", dev.dev_eui, k),
                            &format!("{}", v),
                        )
                        .await?;
                    }
                    _ => {}
                }
            } else if dp.auto_detect_measurements {
                update_dp_measurements = true;
                measurements.insert(
                    k.clone(),
                    fields::Measurement {
                        kind: fields::MeasurementKind::UNKNOWN,
                        name: "".to_string(),
                    },
                );
            }
        }

        if update_dp_measurements {
            self.device_profile =
                Some(device_profile::set_measurements(dp.id.into(), &measurements).await?);
        }

        Ok(())
    }

    // for "normal" uplinks, this is already set by the get_for_phypayload_and_incr_f_cnt_up
    // function, however in case of retransmission or reset (if skip_fcnt_check) this is still
    // required.
    fn sync_uplink_f_cnt(&mut self) -> Result<()> {
        trace!("Syncing uplink frame-counter");
        let d = self.device.as_mut().unwrap();
        let ds = d.get_device_session_mut()?;
        ds.f_cnt_up = self.f_cnt_up_full + 1;
        Ok(())
    }

    // This is called on every uplink as the device might switch between different regions (e.g.
    // US915 8 channels to US915 16 channels). As well with ABP devices on ABP activation this is
    // value is not set initially.
    fn set_region_config_id(&mut self) -> Result<()> {
        trace!("Setting region_config_id to device-session");
        let d = self.device.as_mut().unwrap();
        let ds = d.get_device_session_mut()?;
        ds.region_config_id
            .clone_from(&self.uplink_frame_set.region_config_id);
        Ok(())
    }

    async fn update_device(&mut self) -> Result<()> {
        trace!("Updating device");

        let d = self.device.as_mut().unwrap();
        self.device_changeset.device_session = Some(d.device_session.clone());

        *d = device::partial_update(d.dev_eui, &self.device_changeset).await?;
        Ok(())
    }

    async fn handle_uplink_ack(&self) -> Result<()> {
        let mac = if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            pl
        } else {
            return Err(anyhow!("Expected MacPayload"));
        };
        if !mac.fhdr.f_ctrl.ack {
            return Ok(());
        }

        info!("Handling uplink ack");

        let tenant = self.tenant.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();
        let ts: DateTime<Utc> =
            helpers::get_rx_timestamp(&self.uplink_frame_set.rx_info_set).into();

        let qi = match device_queue::get_pending_for_dev_eui(&dev.dev_eui).await {
            Ok(v) => v,
            Err(e) => {
                warn!(dev_eui = %dev.dev_eui, error = %e.full(), "Get pending queue-item error");
                return Ok(());
            }
        };

        device_queue::delete_item(&qi.id).await?;

        let mut tags = (*app.tags).clone();
        tags.extend((*dp.tags).clone());
        tags.extend((*dev.tags).clone());

        integration::ack_event(
            app.id.into(),
            &dev.variables,
            &integration_pb::AckEvent {
                deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
                time: Some(ts.into()),
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_id: tenant.id.to_string(),
                    tenant_name: tenant.name.clone(),
                    application_id: app.id.to_string(),
                    application_name: app.name.to_string(),
                    device_profile_id: dp.id.to_string(),
                    device_profile_name: dp.name.clone(),
                    device_name: dev.name.clone(),
                    device_class_enabled: dev.enabled_class.to_proto().into(),
                    dev_eui: dev.dev_eui.to_string(),
                    tags,
                }),
                queue_item_id: qi.id.to_string(),
                acknowledged: true,
                f_cnt_down: qi.f_cnt_down.unwrap_or(0) as u32,
            },
        )
        .await;

        Ok(())
    }

    async fn save_metrics(&self) -> Result<()> {
        trace!("Saving device metrics");
        let mut max_rssi: i32 = 0;
        let mut max_snr: f32 = 0.0;

        for (i, rx_info) in self.uplink_frame_set.rx_info_set.iter().enumerate() {
            if i == 0 {
                max_rssi = rx_info.rssi;
                max_snr = rx_info.snr;
            }

            if rx_info.rssi > max_rssi {
                max_rssi = rx_info.rssi;
            }

            if rx_info.snr > max_snr {
                max_snr = rx_info.snr;
            }
        }

        let mut record = metrics::Record {
            time: Local::now(),
            kind: metrics::Kind::ABSOLUTE,
            metrics: HashMap::new(),
        };

        record.metrics.insert("rx_count".into(), 1.0);
        record.metrics.insert("gw_rssi_sum".into(), max_rssi as f64);
        record.metrics.insert("gw_snr_sum".into(), max_snr as f64);
        record.metrics.insert(
            format!("rx_freq_{}", self.uplink_frame_set.tx_info.frequency),
            1.0,
        );
        record
            .metrics
            .insert(format!("rx_dr_{}", self.uplink_frame_set.dr), 1.0);

        let dev = self.device.as_ref().unwrap();

        metrics::save(
            &format!("device:{}", dev.dev_eui),
            &record,
            &metrics::Aggregation::default_aggregations(),
        )
        .await?;

        Ok(())
    }

    async fn save_metrics_relayed(&self) -> Result<()> {
        trace!("Saving relayed device metrics");
        let relay_ctx = self.relay_context.as_ref().unwrap();

        let mut record = metrics::Record {
            time: Local::now(),
            kind: metrics::Kind::ABSOLUTE,
            metrics: HashMap::new(),
        };

        record.metrics.insert("rx_count".into(), 1.0);
        record
            .metrics
            .insert("gw_rssi_sum".into(), relay_ctx.req.metadata.rssi as f64);
        record
            .metrics
            .insert("gw_snr_sum".into(), relay_ctx.req.metadata.snr as f64);
        record
            .metrics
            .insert(format!("rx_freq_{}", relay_ctx.req.frequency), 1.0);
        record
            .metrics
            .insert(format!("rx_dr_{}", relay_ctx.req.metadata.dr), 1.0);

        let dev = self.device.as_ref().unwrap();

        metrics::save(
            &format!("device:{}", dev.dev_eui),
            &record,
            &metrics::Aggregation::default_aggregations(),
        )
        .await?;

        Ok(())
    }

    async fn start_downlink_data_flow(&mut self) -> Result<()> {
        trace!("Starting downlink data flow");

        // We sleep get_downlink_data_delay to give the end-user application some time
        // to enqueue data before the downlink flow starts. In case the user has increased
        // the RX1 Delay relative to the system RX1 Delay, then we add the additional
        // seconds to this wait.
        {
            let conf = config::get();
            let ds = self.device.as_ref().unwrap().get_device_session()?;
            let network_conf = config::get_region_network(&ds.region_config_id)?;

            let dev_rx1_delay = ds.rx1_delay as u8;
            let sys_rx1_delay = network_conf.rx1_delay;

            let rx1_delay_increase = dev_rx1_delay.checked_sub(sys_rx1_delay).unwrap_or_default();
            let rx1_delay_increase = std::time::Duration::from_secs(rx1_delay_increase.into());

            tokio::time::sleep(conf.network.get_downlink_data_delay + rx1_delay_increase).await;
        }

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            downlink::data::Data::handle_response(
                self.uplink_frame_set.clone(),
                self.device_gateway_rx_info.as_ref().cloned().unwrap(),
                self.tenant.as_ref().cloned().unwrap(),
                self.application.as_ref().cloned().unwrap(),
                self.device_profile.as_ref().cloned().unwrap(),
                self.device.as_ref().cloned().unwrap(),
                pl.fhdr.f_ctrl.adr_ack_req || self.must_send_downlink,
                self.phy_payload.mhdr.m_type == lrwn::MType::ConfirmedDataUp,
                self.downlink_mac_commands.clone(),
            )
            .await?;
        }

        Ok(())
    }

    async fn start_downlink_data_flow_relayed(&mut self) -> Result<()> {
        trace!("Starting relayed downlink data flow");

        // We sleep get_downlink_data_delay to give the end-user application some time
        // to enqueue data before the downlink flow starts. In case the user has increased
        // the RX1 Delay relative to the system RX1 Delay, then we add the additional
        // seconds to this wait.
        // Note: In this case we use the RX1 Delay from the Relay device-session.
        {
            let conf = config::get();
            let relay_ctx = self.relay_context.as_ref().unwrap();
            let ds = relay_ctx.device.get_device_session()?;

            let network_conf = config::get_region_network(&ds.region_config_id)?;

            let dev_rx1_delay = ds.rx1_delay as u8;
            let sys_rx1_delay = network_conf.rx1_delay;

            let rx1_delay_increase = dev_rx1_delay.checked_sub(sys_rx1_delay).unwrap_or_default();
            let rx1_delay_increase = std::time::Duration::from_secs(rx1_delay_increase.into());

            tokio::time::sleep(conf.network.get_downlink_data_delay + rx1_delay_increase).await;
        }

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            downlink::data::Data::handle_response_relayed(
                self.relay_context.as_ref().cloned().unwrap(),
                self.uplink_frame_set.clone(),
                self.device_gateway_rx_info.as_ref().cloned().unwrap(),
                self.tenant.as_ref().cloned().unwrap(),
                self.application.as_ref().cloned().unwrap(),
                self.device_profile.as_ref().cloned().unwrap(),
                self.device.as_ref().cloned().unwrap(),
                pl.fhdr.f_ctrl.adr_ack_req || self.must_send_downlink,
                self.phy_payload.mhdr.m_type == lrwn::MType::ConfirmedDataUp,
                self.downlink_mac_commands.clone(),
            )
            .await?;
        }

        Ok(())
    }

    async fn handle_forward_uplink_req(&self) -> Result<()> {
        trace!("Handling ForwardUplinkReq");

        if let lrwn::Payload::MACPayload(relay_pl) = &self.phy_payload.payload {
            if let Some(lrwn::FRMPayload::ForwardUplinkReq(pl)) = &relay_pl.frm_payload {
                match pl.payload.mhdr.m_type {
                    lrwn::MType::JoinRequest => {
                        super::join::JoinRequest::handle_relayed(
                            super::RelayContext {
                                req: pl.clone(),
                                device: self.device.as_ref().unwrap().clone(),
                                device_profile: self.device_profile.as_ref().unwrap().clone(),
                                must_ack: self.phy_payload.mhdr.m_type
                                    == lrwn::MType::ConfirmedDataUp,
                                must_send_downlink: relay_pl.fhdr.f_ctrl.adr_ack_req,
                            },
                            self.uplink_frame_set.clone(),
                        )
                        .await
                    }
                    lrwn::MType::UnconfirmedDataUp | lrwn::MType::ConfirmedDataUp => {
                        Data::handle_relayed(
                            super::RelayContext {
                                req: pl.clone(),
                                device: self.device.as_ref().unwrap().clone(),
                                device_profile: self.device_profile.as_ref().unwrap().clone(),
                                must_ack: self.phy_payload.mhdr.m_type
                                    == lrwn::MType::ConfirmedDataUp,
                                must_send_downlink: relay_pl.fhdr.f_ctrl.adr_ack_req,
                            },
                            self.device_gateway_rx_info.as_ref().unwrap().clone(),
                            self.uplink_frame_set.clone(),
                        )
                        .await
                    }
                    _ => {
                        return Err(anyhow!(
                            "Handling ForwardUplinkReq for MType {} supported",
                            pl.payload.mhdr.m_type
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn _is_roaming(&self) -> bool {
        self.uplink_frame_set.roaming_meta_data.is_some()
    }

    fn _is_relay(&self) -> bool {
        let dp = self.device_profile.as_ref().unwrap();

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.payload {
            if dp.is_relay && pl.f_port.unwrap_or(0) == lrwn::LA_FPORT_RELAY {
                return true;
            }
        }

        false
    }

    fn _is_end_to_end_encrypted(&self) -> bool {
        let ds = match self.device.as_ref().unwrap().get_device_session() {
            Ok(v) => v,
            Err(_) => return false,
        };

        if !ds.js_session_key_id.is_empty() {
            return true;
        }

        if let Some(app_s_key) = &ds.app_s_key {
            if !app_s_key.kek_label.is_empty() {
                return true;
            }
        }

        false
    }
}
