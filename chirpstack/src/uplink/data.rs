use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, Utc};
use tracing::{error, info, span, trace, warn, Instrument, Level};

use super::error::Error;
use super::{filter_rx_info_by_tenant_id, helpers, UplinkFrameSet};
use crate::storage::error::Error as StorageError;
use crate::storage::{
    application, device, device_gateway, device_profile, device_queue, device_session, fields,
    metrics, tenant,
};
use crate::{codec, config, downlink, framelog, integration, maccommand};
use chirpstack_api::{api, common, integration as integration_pb, internal};
use lrwn::AES128Key;

pub struct Data {
    uplink_frame_set: UplinkFrameSet,

    reset: bool,
    retransmission: bool,
    f_cnt_up_full: u32,
    tenant: Option<tenant::Tenant>,
    device_session: Option<internal::DeviceSession>,
    device: Option<device::Device>,
    device_profile: Option<device_profile::DeviceProfile>,
    application: Option<application::Application>,
    device_info: Option<integration_pb::DeviceInfo>,
    mac_payload: Option<lrwn::MACPayload>,
    uplink_event: Option<integration_pb::UplinkEvent>,
    must_send_downlink: bool,
    downlink_mac_commands: Vec<lrwn::MACCommandSet>,
    device_gateway_rx_info: Option<internal::DeviceGatewayRxInfo>,
}

impl Data {
    pub async fn handle(ufs: UplinkFrameSet) {
        let span = span!(Level::INFO, "data_up");

        if let Err(e) = Data::_handle(ufs).instrument(span).await {
            match e.downcast_ref::<Error>() {
                Some(Error::Abort) => {
                    // nothing to do
                }
                Some(_) | None => {
                    error!(error = %e, "Handle uplink error");
                }
            }
        }
    }

    async fn _handle(ufs: UplinkFrameSet) -> Result<()> {
        let mut ctx = Data {
            uplink_frame_set: ufs,
            f_cnt_up_full: 0,
            reset: false,
            retransmission: false,
            tenant: None,
            device_session: None,
            device: None,
            device_profile: None,
            application: None,
            device_info: None,
            mac_payload: None,
            uplink_event: None,
            must_send_downlink: false,
            downlink_mac_commands: Vec::new(),
            device_gateway_rx_info: None,
        };

        ctx.get_device_session().await?;
        ctx.get_device().await?;
        ctx.get_device_profile().await?;
        ctx.get_application().await?;
        ctx.get_tenant().await?;
        ctx.abort_on_device_is_disabled().await?;
        ctx.set_device_info()?;
        ctx.handle_retransmission_reset().await?;
        ctx.set_device_lock().await?;
        ctx.set_scheduler_run_after().await?;
        ctx.filter_rx_info_by_tenant().await?;
        ctx.decrypt_f_opts_mac_commands()?;
        ctx.decrypt_frm_payload()?;
        ctx.get_mac_payload()?;
        ctx.log_uplink_frame_set().await?;
        ctx.set_adr()?;
        ctx.set_uplink_data_rate().await?;
        ctx.set_enabled_class().await?;

        // ctx.send_uplink_meta_data_to_network_controller()?;
        ctx.handle_mac_commands().await?;
        ctx.save_device_gateway_rx_info().await?;
        ctx.append_meta_data_to_uplink_history()?;
        ctx.send_uplink_event().await?;
        ctx.detect_and_save_measurements().await?;
        ctx.sync_uplink_f_cnt()?;
        ctx.set_region_name()?;
        ctx.save_device_session().await?;
        ctx.handle_uplink_ack().await?;
        ctx.save_metrics().await?;
        ctx.start_downlink_data_flow().await?;

        Ok(())
    }

    async fn get_device_session(&mut self) -> Result<(), Error> {
        trace!("Getting device-session for dev_addr");

        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            match device_session::get_for_phypayload_and_incr_f_cnt_up(
                &self.uplink_frame_set.phy_payload,
                self.uplink_frame_set.dr,
                self.uplink_frame_set.ch as u8,
            )
            .await
            {
                Ok(v) => match v {
                    device_session::ValidationStatus::Ok(f_cnt, ds) => {
                        self.device_session = Some(ds);
                        self.f_cnt_up_full = f_cnt;
                    }
                    device_session::ValidationStatus::Retransmission(f_cnt, ds) => {
                        self.retransmission = true;
                        self.device_session = Some(ds);
                        self.f_cnt_up_full = f_cnt;
                    }
                    device_session::ValidationStatus::Reset(f_cnt, ds) => {
                        self.reset = true;
                        self.device_session = Some(ds);
                        self.f_cnt_up_full = f_cnt;
                    }
                },
                Err(e) => match e {
                    StorageError::NotFound(s) => {
                        warn!(dev_addr = %s, "No device-session exists for dev_addr");
                        return Err(Error::Abort);
                    }
                    StorageError::InvalidMIC => {
                        warn!(dev_addr = %pl.fhdr.devaddr, "None of the device-sessions for dev_addr resulted in valid MIC");
                        return Err(Error::Abort);
                    }
                    _ => {
                        return Err(Error::AnyhowError(
                            anyhow::Error::new(e).context("Get device-session"),
                        ));
                    }
                },
            };
        }

        Ok(())
    }

    async fn get_device(&mut self) -> Result<()> {
        trace!("Getting device");
        let dev_eui = lrwn::EUI64::from_slice(&self.device_session.as_ref().unwrap().dev_eui)?;
        self.device = Some(device::get(&dev_eui).await?);
        Ok(())
    }

    async fn get_device_profile(&mut self) -> Result<()> {
        trace!("Getting the device-profile");
        let dp = device_profile::get(&self.device.as_ref().unwrap().device_profile_id).await?;
        if dp.region != self.uplink_frame_set.region_common_name {
            return Err(anyhow!("Invalid device-profile region"));
        }
        self.device_profile = Some(dp);

        Ok(())
    }

    async fn get_application(&mut self) -> Result<()> {
        trace!("Getting application");
        self.application =
            Some(application::get(&self.device.as_ref().unwrap().application_id).await?);
        Ok(())
    }

    async fn get_tenant(&mut self) -> Result<()> {
        trace!("Getting tenant");
        self.tenant = Some(tenant::get(&self.application.as_ref().unwrap().tenant_id).await?);
        Ok(())
    }

    fn set_device_info(&mut self) -> Result<()> {
        trace!("Setting device-info");

        let tenant = self.tenant.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        let mut tags = (&*dp.tags).clone();
        tags.clone_from(&*dev.tags);

        self.device_info = Some(integration_pb::DeviceInfo {
            tenant_id: tenant.id.to_string(),
            tenant_name: tenant.name.clone(),
            application_id: app.id.to_string(),
            application_name: app.name.to_string(),
            device_profile_id: dp.id.to_string(),
            device_profile_name: dp.name.clone(),
            device_name: dev.name.clone(),
            dev_eui: dev.dev_eui.to_string(),
            tags,
        });

        Ok(())
    }

    async fn abort_on_device_is_disabled(&self) -> Result<(), Error> {
        let device = self.device.as_ref().unwrap();

        if device.is_disabled {
            // Restore the device-session in case the device is disabled.
            // This is because during the fcnt validation, we immediately store the
            // device-session with incremented fcnt to avoid race conditions.
            device_session::save(self.device_session.as_ref().unwrap())
                .await
                .context("Savel device-session")?;

            info!(dev_eui = %device.dev_eui, "Device is disabled, aborting flow");
            return Err(Error::Abort);
        }

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
                deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
                time: Some(ts.into()),
                device_info: self.device_info.clone(),
                level: integration_pb::LogLevel::Warning.into(),
                code: integration_pb::LogCode::UplinkFCntRetransmission.into(),
                description:
                    "Uplink was flagged as re-transmission / frame-counter did not increment".into(),
                ..Default::default()
            };
            integration::log_event(&app.id, &dev.variables, &pl).await?;
        }

        if self.reset {
            let pl = integration_pb::LogEvent {
                deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
                time: Some(ts.into()),
                device_info: self.device_info.clone(),
                level: integration_pb::LogLevel::Warning.into(),
                code: integration_pb::LogCode::UplinkFCntReset.into(),
                description: "Frame-counter reset or rollover detected".into(),
                ..Default::default()
            };
            integration::log_event(&app.id, &dev.variables, &pl).await?;
        }

        Err(Error::Abort)
    }

    async fn set_device_lock(&self) -> Result<()> {
        trace!("Setting device lock");
        let dev = self.device.as_ref().unwrap();
        let conf = config::get();

        device::set_lock(
            &dev.dev_eui,
            Duration::from_std(conf.network.scheduler.class_a_lock_duration)?,
        )
        .await
    }

    // For Class-B and Class-C devices, set the scheduler_run_after timestamp to avoid collisions with
    // the Class-A downlink and Class-B/C scheduler.
    async fn set_scheduler_run_after(&mut self) -> Result<()> {
        let dev = self.device.as_mut().unwrap();
        let conf = config::get();

        if &dev.enabled_class == "B" || &dev.enabled_class == "C" {
            trace!("Setting scheduler_run_after for device");
            let scheduler_run_after =
                Utc::now() + Duration::from_std(conf.network.scheduler.class_a_lock_duration)?;

            // Only set the new scheduler_run_after if it is currently None
            // or when the current value is before the calculated scheduler_run_after.
            if dev.scheduler_run_after.is_none()
                || scheduler_run_after > dev.scheduler_run_after.unwrap()
            {
                *dev = device::set_scheduler_run_after(&dev.dev_eui, Some(scheduler_run_after))
                    .await?;
            }
        }

        Ok(())
    }

    async fn filter_rx_info_by_tenant(&mut self) -> Result<()> {
        trace!("Filtering rx_info by tenant_id");

        match filter_rx_info_by_tenant_id(
            &self.application.as_ref().unwrap().tenant_id,
            &mut self.uplink_frame_set,
        ) {
            Ok(_) => Ok(()),
            Err(v) => {
                // Restore the device-session in case of an error (no gateways available).
                // This is because during the fcnt validation, we immediately store the
                // device-session with incremented fcnt to avoid race conditions.
                device_session::save(self.device_session.as_ref().unwrap())
                    .await
                    .context("Savel device-session")?;

                Err(v)
            }
        }
    }

    fn decrypt_f_opts_mac_commands(&mut self) -> Result<()> {
        trace!("Decrypting mac-commands");
        let ds = self.device_session.as_ref().unwrap();
        match ds.mac_version() {
            common::MacVersion::Lorawan100
            | common::MacVersion::Lorawan101
            | common::MacVersion::Lorawan102
            | common::MacVersion::Lorawan103
            | common::MacVersion::Lorawan104 => {
                self.uplink_frame_set
                    .phy_payload
                    .decode_f_opts_to_mac_commands()
                    .context("Decode mac-commands")?;
            }
            common::MacVersion::Lorawan110 => {
                let nwk_s_enc_key = AES128Key::from_slice(&ds.nwk_s_enc_key)?;
                self.uplink_frame_set
                    .phy_payload
                    .decrypt_f_opts(&nwk_s_enc_key)
                    .context("Decrypt f_opts")?;
            }
        }
        Ok(())
    }

    fn decrypt_frm_payload(&mut self) -> Result<()> {
        trace!("Decrypting FRMPayload");
        let ds = self.device_session.as_ref().unwrap();
        let mut f_port = 0;

        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            f_port = pl.f_port.unwrap_or(0);
        }

        if f_port == 0 {
            let nwk_s_enc_key = AES128Key::from_slice(&ds.nwk_s_enc_key)?;
            self.uplink_frame_set
                .phy_payload
                .decrypt_frm_payload(&nwk_s_enc_key)?;
        } else if ds.app_s_key.is_some() {
            let app_s_key = AES128Key::from_slice(&ds.app_s_key.as_ref().unwrap().aes_key)?;

            self.uplink_frame_set
                .phy_payload
                .decrypt_frm_payload(&app_s_key)?;
        }

        Ok(())
    }

    fn get_mac_payload(&mut self) -> Result<()> {
        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            self.mac_payload = Some(pl.clone());
        }

        if self.mac_payload.is_none() {
            return Err(anyhow!("No MacPayload"));
        }

        Ok(())
    }

    async fn log_uplink_frame_set(&self) -> Result<()> {
        trace!("Logging uplink frame-set");
        let mut ufl: api::UplinkFrameLog = (&self.uplink_frame_set).try_into()?;
        ufl.dev_eui = self.device.as_ref().unwrap().dev_eui.to_string();
        framelog::log_uplink_for_device(&ufl).await?;
        Ok(())
    }

    fn set_adr(&mut self) -> Result<()> {
        trace!("Set ADR flag in device-session");
        let mut ds = self.device_session.as_mut().unwrap();
        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            ds.adr = pl.fhdr.f_ctrl.adr;
        }
        Ok(())
    }

    async fn set_uplink_data_rate(&mut self) -> Result<()> {
        trace!("Set uplink data-rate and reset tx-power on change");
        let device = self.device.as_mut().unwrap();
        *device = device::set_last_seen_dr(&device.dev_eui, self.uplink_frame_set.dr).await?;

        let mut ds = self.device_session.as_mut().unwrap();
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

    async fn set_enabled_class(&mut self) -> Result<()> {
        trace!("Set Class-B beacon locked");
        let dev = self.device.as_mut().unwrap();
        let dp = self.device_profile.as_ref().unwrap();

        let mut mode = match dp.supports_class_c {
            true => "C",
            false => "A",
        }
        .to_string();

        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            let locked = pl.fhdr.f_ctrl.class_b;
            mode = match locked {
                true => "B".to_string(),
                false => mode,
            };
        }

        // Update if the enabled class has changed.
        if dev.enabled_class != mode {
            *dev = device::set_enabled_class(&dev.dev_eui, &mode).await?;
        }

        Ok(())
    }

    async fn handle_mac_commands(&mut self) -> Result<()> {
        trace!("Handling uplink mac-commands");

        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            if !(*pl.fhdr.f_opts).is_empty() {
                trace!("Mac-commands in f_opts");
                let (mac_response, must_send_downlink) = maccommand::handle_uplink(
                    &self.uplink_frame_set,
                    &pl.fhdr.f_opts,
                    self.tenant.as_ref().unwrap(),
                    self.application.as_ref().unwrap(),
                    self.device_profile.as_ref().unwrap(),
                    self.device.as_ref().unwrap(),
                    self.device_session.as_mut().unwrap(),
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
                    self.device.as_ref().unwrap(),
                    self.device_session.as_mut().unwrap(),
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
        let dev_gw_rx_info = internal::DeviceGatewayRxInfo {
            dev_eui: self.device_session.as_ref().unwrap().dev_eui.clone(),
            dr: self.uplink_frame_set.dr as u32,
            items: self
                .uplink_frame_set
                .rx_info_set
                .iter()
                .map(|rx_info| internal::DeviceGatewayRxInfoItem {
                    gateway_id: hex::decode(&rx_info.gateway_id).unwrap(),
                    rssi: rx_info.rssi,
                    lora_snr: rx_info.snr,
                    antenna: rx_info.antenna,
                    board: rx_info.board,
                    context: rx_info.context.clone(),
                })
                .collect(),
        };

        device_gateway::save_rx_info(&dev_gw_rx_info)
            .await
            .context("Save rx-info")?;

        self.device_gateway_rx_info = Some(dev_gw_rx_info);

        Ok(())
    }

    fn append_meta_data_to_uplink_history(&mut self) -> Result<()> {
        let mut ds = self.device_session.as_mut().unwrap();

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

    async fn send_uplink_event(&mut self) -> Result<()> {
        trace!("Sending uplink event");

        let ts: DateTime<Utc> =
            helpers::get_rx_timestamp(&self.uplink_frame_set.rx_info_set).into();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();
        let mac = self.mac_payload.as_ref().unwrap();

        let mut pl = integration_pb::UplinkEvent {
            deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
            time: Some(ts.into()),
            device_info: self.device_info.clone(),
            dev_addr: mac.fhdr.devaddr.to_string(),
            adr: mac.fhdr.f_ctrl.adr,
            dr: self.uplink_frame_set.dr as u32,
            f_cnt_up: self.f_cnt_up_full,
            f_port: mac.f_port.unwrap_or(0) as u32,
            confirmed: self.uplink_frame_set.phy_payload.mhdr.m_type
                == lrwn::MType::ConfirmedDataUp,
            data: match &mac.frm_payload {
                Some(lrwn::FRMPayload::Raw(b)) => b.clone(),
                _ => Vec::new(),
            },
            object: None,
            rx_info: self.uplink_frame_set.rx_info_set.clone(),
            tx_info: Some(self.uplink_frame_set.tx_info.clone()),
        };

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
                    &app.id,
                    &dev.variables,
                    &integration_pb::LogEvent {
                        deduplication_id: pl.deduplication_id.clone(),
                        time: Some(Utc::now().into()),
                        device_info: self.device_info.clone(),
                        level: integration_pb::LogLevel::Error.into(),
                        code: integration_pb::LogCode::UplinkCodec.into(),
                        description: format!("{}", e),
                        ..Default::default()
                    },
                )
                .await?;
                None
            }
        };

        integration::uplink_event(&app.id, &dev.variables, &pl).await?;

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
                            )?
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

                        metrics::save(&format!("device:{}:{}", dev.dev_eui, k), &record).await?;
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
            } else {
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
                Some(device_profile::set_measurements(dp.id, &measurements).await?);
        }

        Ok(())
    }

    // for "normal" uplinks, this is already set by the get_for_phypayload_and_incr_f_cnt_up
    // function, however in case of retransmission or reset (if skip_fcnt_check) this is still
    // required.
    fn sync_uplink_f_cnt(&mut self) -> Result<()> {
        trace!("Syncing uplink frame-counter");
        let mut ds = self.device_session.as_mut().unwrap();
        ds.f_cnt_up = self.f_cnt_up_full + 1;
        Ok(())
    }

    // This is called on every uplink as the device might switch between different regions (e.g.
    // US915 8 channels to US915 16 channels). As well with ABP devices on ABP activation this is
    // value is not set initially.
    fn set_region_name(&mut self) -> Result<()> {
        trace!("Setting region_name to device-session");
        let mut ds = self.device_session.as_mut().unwrap();
        ds.region_name = self.uplink_frame_set.region_name.clone();
        Ok(())
    }

    async fn save_device_session(&self) -> Result<()> {
        trace!("Saving device-session");
        device_session::save(self.device_session.as_ref().unwrap())
            .await
            .context("Save device-session")?;
        Ok(())
    }

    async fn handle_uplink_ack(&self) -> Result<()> {
        let mac = self.mac_payload.as_ref().unwrap();
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
                warn!(dev_eui = %dev.dev_eui, error = %e, "Get pending queue-item error");
                return Ok(());
            }
        };

        device_queue::delete_item(&qi.id).await?;

        let mut tags = (&*dp.tags).clone();
        tags.clone_from(&*dev.tags);

        integration::ack_event(
            &app.id,
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
                    dev_eui: dev.dev_eui.to_string(),
                    tags,
                }),
                queue_item_id: qi.id.to_string(),
                acknowledged: true,
                f_cnt_down: qi.f_cnt_down.unwrap_or(0) as u32,
            },
        )
        .await?;

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

        metrics::save(&format!("device:{}", dev.dev_eui), &record).await?;

        Ok(())
    }

    async fn start_downlink_data_flow(&mut self) -> Result<()> {
        trace!("Starting downlink data flow");
        if let lrwn::Payload::MACPayload(pl) = &self.uplink_frame_set.phy_payload.payload {
            downlink::data::Data::handle_response(
                self.uplink_frame_set.clone(),
                self.device_gateway_rx_info.as_ref().cloned().unwrap(),
                self.tenant.as_ref().cloned().unwrap(),
                self.application.as_ref().cloned().unwrap(),
                self.device_profile.as_ref().cloned().unwrap(),
                self.device.as_ref().cloned().unwrap(),
                self.device_session.as_ref().cloned().unwrap(),
                pl.fhdr.f_ctrl.adr_ack_req || self.must_send_downlink,
                self.uplink_frame_set.phy_payload.mhdr.m_type == lrwn::MType::ConfirmedDataUp,
                self.downlink_mac_commands.clone(),
            )
            .await?;
        }

        Ok(())
    }
}
