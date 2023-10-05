use anyhow::Result;
use chrono::{Duration, Utc};
use std::time::SystemTime;
use tracing::{error, info, span, trace, Instrument, Level};
use uuid::Uuid;

use lrwn::{AES128Key, MType, Payload, PhyPayload, EUI64};

use crate::api::helpers::ToProto;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_profile, device_queue, device_session, downlink_frame, multicast, tenant,
};
use crate::{framelog, integration, metalog};
use chirpstack_api::{api, common, gw, integration as integration_pb, internal, meta};

pub struct TxAck {
    downlink_tx_ack: gw::DownlinkTxAck,
    downlink_tx_ack_status: gw::TxAckStatus,
    downlink_id: u32,

    downlink_frame: Option<internal::DownlinkFrame>,
    downlink_frame_item: Option<gw::DownlinkFrameItem>,
    phy_payload: Option<PhyPayload>,
    phy_payload_relayed: Option<PhyPayload>,
    device_session: Option<internal::DeviceSession>,
    device_session_relayed: Option<internal::DeviceSession>,
    tenant: Option<tenant::Tenant>,
    tenant_relayed: Option<tenant::Tenant>,
    application: Option<application::Application>,
    application_relayed: Option<application::Application>,
    device_profile: Option<device_profile::DeviceProfile>,
    device_profile_relayed: Option<device_profile::DeviceProfile>,
    device: Option<device::Device>,
    device_relayed: Option<device::Device>,
    device_queue_item: Option<device_queue::DeviceQueueItem>,
}

impl TxAck {
    pub async fn handle(tx_ack: gw::DownlinkTxAck) {
        let span = span!(Level::TRACE, "tx_ack", downlink_id = tx_ack.downlink_id);
        if let Err(e) = TxAck::_handle(tx_ack).instrument(span).await {
            error!(error = %e, "Handling tx ack error");
        }
    }

    async fn _handle(tx_ack: gw::DownlinkTxAck) -> Result<()> {
        if tx_ack.items.is_empty() {
            return Err(anyhow!("Zero items in tx ack"));
        }

        let mut ctx = TxAck {
            downlink_tx_ack_status: {
                let mut status = gw::TxAckStatus::default();
                for item in &tx_ack.items {
                    status = item.status();
                    if item.status() == gw::TxAckStatus::Ok {
                        break;
                    }
                }
                status
            },
            downlink_id: tx_ack.downlink_id,
            downlink_tx_ack: tx_ack,
            downlink_frame: None,
            downlink_frame_item: None,
            phy_payload: None,
            phy_payload_relayed: None,
            device_session: None,
            device_session_relayed: None,
            tenant: None,
            tenant_relayed: None,
            application: None,
            application_relayed: None,
            device_profile: None,
            device_profile_relayed: None,
            device: None,
            device_relayed: None,
            device_queue_item: None,
        };

        ctx.get_downlink_frame().await?;
        ctx.decode_phy_payload()?;

        if ctx.is_relay_payload() {
            return ctx._handle_relayed().await;
        }

        if ctx.is_error() {
            if ctx.is_application_payload() || ctx.is_mac_only_downlink() {
                ctx.get_device().await?;
                ctx.get_device_profile().await?;
                ctx.get_application().await?;
                ctx.get_tenant().await?;
                ctx.log_tx_ack_error().await?;
            }

            if ctx.is_multicast_downlink() {
                ctx.delete_multicast_group_queue_item().await?;
            }
        } else {
            if ctx.is_application_payload() {
                ctx.get_device().await?;
                ctx.get_device_profile().await?;
                ctx.get_application().await?;
                ctx.get_tenant().await?;
                ctx.get_device_session().await?;
                ctx.get_device_queue_item().await?;
                if ctx.is_unconfirmed_downlink() {
                    ctx.delete_device_queue_item().await?;
                }

                if ctx.is_confirmed_downlink() {
                    ctx.set_device_queue_item_pending().await?;
                    ctx.set_device_session_conf_f_cnt()?;
                }

                ctx.increment_a_f_cnt_down()?;
                ctx.save_device_session().await?;
                ctx.send_tx_ack_event().await?;
            }

            if ctx.is_mac_only_downlink() {
                ctx.get_device_session().await?;
                ctx.increment_n_f_cnt_down()?;
                ctx.save_device_session().await?;
            }

            if ctx.is_multicast_downlink() {
                ctx.delete_multicast_group_queue_item().await?;
            }

            // log downlink frame and meta-data.
            ctx.log_downlink_frame().await?;
            ctx.log_downlink_meta().await?;
        }

        Ok(())
    }

    async fn _handle_relayed(&mut self) -> Result<()> {
        self.get_phy_payload_relayed()?;

        if self.is_error() {
            // We log the tx ack error under the relay as this is the device to which the downlink
            // is sent.
            self.get_device().await?;
            self.get_device_profile().await?;
            self.get_application().await?;
            self.get_tenant().await?;
            self.log_tx_ack_error().await?;
        } else {
            // First handle the relay frame-counter increment.
            self.get_device_session().await?;
            self.increment_a_f_cnt_down()?;
            self.save_device_session().await?;

            // Handle end-device frame-counter increment + queue item.
            if self.is_application_payload_relayed() {
                self.get_device_session_relayed().await?;
                self.get_device_queue_item().await?;
                if self.is_unconfirmed_downlink_relayed() {
                    self.delete_device_queue_item().await?;
                }

                if self.is_confirmed_downlink_relayed() {
                    self.set_device_queue_item_pending().await?;
                    self.set_device_session_conf_f_cnt_relayed()?;
                }

                self.increment_a_f_cnt_down_relayed()?;
                self.save_device_session_relayed().await?;

                // Log tx ack event.
                self.get_device_relayed().await?;
                self.get_device_profile_relayed().await?;
                self.get_application_relayed().await?;
                self.get_tenant_relayed().await?;
                self.send_tx_ack_event_relayed().await?;
            } else if self.is_mac_only_downlink_relayed() {
                self.get_device_session_relayed().await?;
                self.increment_n_f_cnt_down_relayed()?;
                self.save_device_session_relayed().await?;
            }

            // Log downlink frame and meta-data.
            // This will log the downlink under the relay as this is the device to which the
            // downlink is sent.
            self.log_downlink_frame().await?;
            self.log_downlink_meta().await?;
        }

        Ok(())
    }

    async fn get_downlink_frame(&mut self) -> Result<()> {
        trace!("Get downlink-frame from Redis");
        let df = downlink_frame::get(self.downlink_id).await?;
        let gw_df = &df
            .downlink_frame
            .as_ref()
            .ok_or_else(|| anyhow!("downlink_frame is None"))?;

        // Validate that we don't receive more ack items than downlink items that were
        // sent to the gateway. Receiving less acks is valid, e.g. the gateway might
        // ack the first item only.
        if self.downlink_tx_ack.items.len() > gw_df.items.len() {
            return Err(anyhow!("More items in tx ack than in downlink-frame"));
        }

        for (i, tx_ack) in self.downlink_tx_ack.items.iter().enumerate() {
            if tx_ack.status() == gw::TxAckStatus::Ok {
                self.downlink_frame_item = Some(gw_df.items[i].clone());
            }
        }

        // take last negative ack if there is no positive ack
        if self.downlink_frame_item.is_none() {
            self.downlink_frame_item =
                Some(gw_df.items[self.downlink_tx_ack.items.len() - 1].clone());
        }

        self.downlink_frame = Some(df);

        Ok(())
    }

    async fn get_device_session(&mut self) -> Result<()> {
        trace!("Getting device-session");
        let dev_eui = EUI64::from_slice(&self.downlink_frame.as_ref().unwrap().dev_eui)?;
        self.device_session = Some(device_session::get(&dev_eui).await?);

        Ok(())
    }

    async fn get_device_session_relayed(&mut self) -> Result<()> {
        trace!("Getting relayed device-session");
        let dev_eui = EUI64::from_slice(&self.downlink_frame.as_ref().unwrap().dev_eui_relayed)?;
        self.device_session_relayed = Some(device_session::get(&dev_eui).await?);

        Ok(())
    }

    async fn get_device(&mut self) -> Result<()> {
        trace!("Getting device");
        let dev_eui = EUI64::from_slice(&self.downlink_frame.as_ref().unwrap().dev_eui)?;
        self.device = Some(device::get(&dev_eui).await?);
        Ok(())
    }

    async fn get_device_relayed(&mut self) -> Result<()> {
        trace!("Getting relayed device");
        let dev_eui = EUI64::from_slice(&self.downlink_frame.as_ref().unwrap().dev_eui_relayed)?;
        self.device_relayed = Some(device::get(&dev_eui).await?);
        Ok(())
    }

    async fn get_device_profile(&mut self) -> Result<()> {
        trace!("Getting device-profile");
        self.device_profile =
            Some(device_profile::get(&self.device.as_ref().unwrap().device_profile_id).await?);
        Ok(())
    }

    async fn get_device_profile_relayed(&mut self) -> Result<()> {
        trace!("Getting relayed device-profile");
        self.device_profile_relayed = Some(
            device_profile::get(&self.device_relayed.as_ref().unwrap().device_profile_id).await?,
        );
        Ok(())
    }

    async fn get_application(&mut self) -> Result<()> {
        trace!("Getting application");
        self.application =
            Some(application::get(&self.device.as_ref().unwrap().application_id).await?);
        Ok(())
    }

    async fn get_application_relayed(&mut self) -> Result<()> {
        trace!("Getting relayed application");
        self.application_relayed =
            Some(application::get(&self.device_relayed.as_ref().unwrap().application_id).await?);
        Ok(())
    }

    async fn get_tenant(&mut self) -> Result<()> {
        trace!("Getting tenant");
        self.tenant = Some(tenant::get(&self.application.as_ref().unwrap().tenant_id).await?);
        Ok(())
    }

    async fn get_tenant_relayed(&mut self) -> Result<()> {
        trace!("Getting relayed tenant");
        self.tenant_relayed =
            Some(tenant::get(&self.application_relayed.as_ref().unwrap().tenant_id).await?);
        Ok(())
    }

    async fn get_device_queue_item(&mut self) -> Result<()> {
        trace!("Getting device queue-item");
        self.device_queue_item = Some(
            device_queue::get_item(&Uuid::from_slice(
                &self.downlink_frame.as_ref().unwrap().device_queue_item_id,
            )?)
            .await?,
        );
        Ok(())
    }

    async fn delete_device_queue_item(&self) -> Result<()> {
        trace!("Deleting device queue-item");
        device_queue::delete_item(&Uuid::from_slice(
            &self.downlink_frame.as_ref().unwrap().device_queue_item_id,
        )?)
        .await?;
        Ok(())
    }

    async fn delete_multicast_group_queue_item(&self) -> Result<()> {
        trace!("Deleting multicast-group queue item");
        multicast::delete_queue_item(&Uuid::from_slice(
            &self
                .downlink_frame
                .as_ref()
                .unwrap()
                .multicast_group_queue_item_id,
        )?)
        .await?;

        Ok(())
    }

    async fn set_device_queue_item_pending(&mut self) -> Result<()> {
        trace!("Setting device queue-item pending");

        let dev = self.device.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let qi = self.device_queue_item.as_mut().unwrap();

        qi.is_pending = true;

        if dev.enabled_class == DeviceClass::C {
            let timeout = Utc::now() + Duration::seconds(dp.class_c_timeout as i64);
            qi.timeout_after = Some(timeout);
        }

        *qi = device_queue::update_item(qi.clone()).await?;

        Ok(())
    }

    fn set_device_session_conf_f_cnt(&mut self) -> Result<()> {
        trace!("Setting device-session conf_f_cnt");

        let ds = self.device_session.as_mut().unwrap();
        let qi = self.device_queue_item.as_ref().unwrap();

        ds.conf_f_cnt = match qi.f_cnt_down {
            Some(v) => v as u32,
            None => {
                error!("Expected device queue-item f_cnt_down to be set");
                0
            }
        };

        Ok(())
    }

    fn set_device_session_conf_f_cnt_relayed(&mut self) -> Result<()> {
        trace!("Setting relayed device-session conf_f_cnt");

        let ds = self.device_session_relayed.as_mut().unwrap();
        let qi = self.device_queue_item.as_ref().unwrap();

        ds.conf_f_cnt = match qi.f_cnt_down {
            Some(v) => v as u32,
            None => {
                error!("Expected device queue-item f_cnt_down to be set");
                0
            }
        };

        Ok(())
    }

    fn increment_a_f_cnt_down(&mut self) -> Result<()> {
        trace!("Incrementing a_f_cnt_down");

        let ds = self.device_session.as_mut().unwrap();
        ds.set_a_f_cnt_down(self.downlink_frame.as_ref().unwrap().a_f_cnt_down + 1);

        Ok(())
    }

    fn increment_a_f_cnt_down_relayed(&mut self) -> Result<()> {
        trace!("Incrementing relayed a_f_cnt_down");

        let ds = self.device_session_relayed.as_mut().unwrap();
        ds.set_a_f_cnt_down(ds.get_a_f_cnt_down() + 1);

        Ok(())
    }

    fn increment_n_f_cnt_down(&mut self) -> Result<()> {
        trace!("Incrementing n_f_cnt_down");

        let ds = self.device_session.as_mut().unwrap();
        ds.n_f_cnt_down += 1;

        Ok(())
    }

    fn increment_n_f_cnt_down_relayed(&mut self) -> Result<()> {
        trace!("Incrementing relayed n_f_cnt_down");

        let ds = self.device_session_relayed.as_mut().unwrap();
        ds.n_f_cnt_down += 1;

        Ok(())
    }

    async fn save_device_session(&self) -> Result<()> {
        trace!("Saving device-session");
        device_session::save(self.device_session.as_ref().unwrap()).await?;
        Ok(())
    }

    async fn save_device_session_relayed(&self) -> Result<()> {
        trace!("Saving relayed device-session");
        device_session::save(self.device_session_relayed.as_ref().unwrap()).await?;
        Ok(())
    }

    async fn log_tx_ack_error(&self) -> Result<()> {
        trace!("Logging tx ack error");

        let tenant = self.tenant.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        let mut tags = (*dp.tags).clone();
        tags.extend((*dev.tags).clone());

        let pl = integration_pb::LogEvent {
            time: Some(Utc::now().into()),
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
            level: integration_pb::LogLevel::Error.into(),
            code: integration_pb::LogCode::DownlinkGateway.into(),
            description: self.downlink_tx_ack_status.into(),
            ..Default::default()
        };

        integration::log_event(app.id, &dev.variables, &pl).await;

        Ok(())
    }

    async fn send_tx_ack_event(&self) -> Result<()> {
        trace!("Sending tx ack event");

        let tenant = self.tenant.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();
        let qi = self.device_queue_item.as_ref().unwrap();

        let mut tags = (*dp.tags).clone();
        tags.extend((*dev.tags).clone());

        let downlink_id = self.downlink_frame.as_ref().unwrap().downlink_id;
        let gateway_id = self
            .downlink_frame
            .as_ref()
            .unwrap()
            .downlink_frame
            .as_ref()
            .unwrap()
            .gateway_id
            .clone();

        let pl = integration_pb::TxAckEvent {
            downlink_id,
            time: Some(Utc::now().into()),
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
            f_cnt_down: qi.f_cnt_down.unwrap_or(0) as u32,
            gateway_id,
            tx_info: self.downlink_frame_item.as_ref().unwrap().tx_info.clone(),
        };

        integration::txack_event(app.id, &dev.variables, &pl).await;

        Ok(())
    }

    async fn send_tx_ack_event_relayed(&self) -> Result<()> {
        trace!("Sending relayed tx ack event");

        let tenant = self.tenant_relayed.as_ref().unwrap();
        let app = self.application_relayed.as_ref().unwrap();
        let dp = self.device_profile_relayed.as_ref().unwrap();
        let dev = self.device_relayed.as_ref().unwrap();
        let qi = self.device_queue_item.as_ref().unwrap();

        let mut tags = (*dp.tags).clone();
        tags.extend((*dev.tags).clone());

        let downlink_id = self.downlink_frame.as_ref().unwrap().downlink_id;
        let gateway_id = self
            .downlink_frame
            .as_ref()
            .unwrap()
            .downlink_frame
            .as_ref()
            .unwrap()
            .gateway_id
            .clone();

        let pl = integration_pb::TxAckEvent {
            downlink_id,
            time: Some(Utc::now().into()),
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
            f_cnt_down: qi.f_cnt_down.unwrap_or(0) as u32,
            gateway_id,
            tx_info: self.downlink_frame_item.as_ref().unwrap().tx_info.clone(),
        };

        integration::txack_event(app.id, &dev.variables, &pl).await;

        Ok(())
    }

    fn decode_phy_payload(&mut self) -> Result<()> {
        trace!("Decoding PhyPayload");
        let phy =
            lrwn::PhyPayload::from_slice(&self.downlink_frame_item.as_ref().unwrap().phy_payload)?;
        self.phy_payload = Some(phy);

        Ok(())
    }

    fn get_phy_payload_relayed(&mut self) -> Result<()> {
        trace!("Getting relayed PhyPayload");
        let df = self.downlink_frame.as_ref().unwrap();

        let mut phy = self.phy_payload.as_ref().unwrap().clone();
        let nwk_s_enc_key = AES128Key::from_slice(&df.nwk_s_enc_key)?;

        // We need to set the full AFCntDown to decrypt the FRMPayload holding the
        // ForwardDownlinkReq.
        if let Payload::MACPayload(pl) = &mut phy.payload {
            pl.fhdr.f_cnt = df.a_f_cnt_down;
        }
        phy.decrypt_frm_payload(&nwk_s_enc_key)?;

        if let Payload::MACPayload(pl) = phy.payload {
            if let Some(lrwn::FRMPayload::ForwardDownlinkReq(pl)) = pl.frm_payload {
                self.phy_payload_relayed = Some(*pl.payload);
            }
        }

        Ok(())
    }

    async fn log_downlink_frame(&mut self) -> Result<()> {
        trace!("Logging downlink frame");
        let df = self.downlink_frame.as_ref().unwrap();
        let gw_df = df
            .downlink_frame
            .as_ref()
            .ok_or_else(|| anyhow!("downlink_frame is None"))?;
        let dfi = self.downlink_frame_item.as_ref().unwrap();
        let phy = self.phy_payload.as_mut().unwrap();

        let dfl = api::DownlinkFrameLog {
            time: Some(SystemTime::now().into()),
            phy_payload: dfi.phy_payload.clone(),
            tx_info: dfi.tx_info.clone(),
            downlink_id: gw_df.downlink_id,
            gateway_id: gw_df.gateway_id.clone(),
            m_type: match &phy.mhdr.m_type {
                MType::JoinAccept => common::MType::JoinAccept,
                MType::UnconfirmedDataDown => common::MType::UnconfirmedDataDown,
                MType::ConfirmedDataDown => common::MType::ConfirmedDataDown,
                MType::Proprietary => common::MType::Proprietary,
                _ => {
                    return Err(anyhow!("Unepxected MType: {}", phy.mhdr.m_type));
                }
            }
            .into(),
            dev_addr: match &phy.payload {
                Payload::MACPayload(pl) => pl.fhdr.devaddr.to_string(),
                _ => "".to_string(),
            },
            dev_eui: {
                if !df.dev_eui.is_empty() {
                    EUI64::from_slice(&df.dev_eui)?.to_string()
                } else {
                    "".to_string()
                }
            },
            plaintext_f_opts: false,
            plaintext_frm_payload: false,
        };

        // Log for gateway (with potentially encrypted mac-commands).
        info!(gateway_id = %dfl.gateway_id, "Log downlink-frame for gateway");
        framelog::log_downlink_for_gateway(&dfl).await?;

        // Downlink is not related to a device / DevEUI, e.g. it could be a multicast
        // or proprietary downlink. Therefore we can't log it for a specific DevEUI.
        if df.dev_eui.is_empty() {
            trace!("Downlink does not have dev_eui set (multicast downlink)");
            return Ok(());
        }

        let nwk_s_enc_key = AES128Key::from_slice(&df.nwk_s_enc_key)?;

        // Set the full frame-counter value.
        // This is needed to decrypt the mac-commands and / or FrmPayload (in case of Relay).
        if let Payload::MACPayload(pl) = &mut phy.payload {
            if pl.f_port.unwrap_or(0) == 0 {
                // We need to set the full NFcntDown to decrypt the mac-commands.
                pl.fhdr.f_cnt = df.n_f_cnt_down;
            } else {
                pl.fhdr.f_cnt = df.a_f_cnt_down;
            }
        }

        let mut plaintext_frm_payload = false;
        if let Payload::MACPayload(pl) = &phy.payload {
            // f_port must be either 0 or 226 (Relay).
            if vec![0, lrwn::LA_FPORT_RELAY].contains(&pl.f_port.unwrap_or(0)) {
                phy.decrypt_frm_payload(&nwk_s_enc_key)?;
                plaintext_frm_payload = true;
            }
        }

        // Decrypt f_opts mac-commands (LoRaWAN 1.1)
        if df.encrypted_fopts {
            phy.decrypt_f_opts(&nwk_s_enc_key)?;
        }

        let dfl = api::DownlinkFrameLog {
            time: dfl.time.clone(),
            phy_payload: phy.to_vec()?,
            tx_info: dfl.tx_info.clone(),
            downlink_id: dfl.downlink_id,
            gateway_id: dfl.gateway_id.clone(),
            m_type: dfl.m_type,
            dev_addr: dfl.dev_addr.clone(),
            dev_eui: dfl.dev_eui.clone(),
            plaintext_f_opts: true,
            plaintext_frm_payload,
        };

        // Log for device.
        info!(device_eui = %dfl.dev_eui, "Log downlink-frame for device");
        framelog::log_downlink_for_device(&dfl).await?;

        Ok(())
    }

    async fn log_downlink_meta(&self) -> Result<()> {
        trace!("Logging downlink meta");

        let df = self.downlink_frame.as_ref().unwrap();
        let dfi = self.downlink_frame_item.as_ref().unwrap();
        let phy = self.phy_payload.as_ref().unwrap();

        let dm = meta::DownlinkMeta {
            dev_eui: if !df.dev_eui.is_empty() {
                EUI64::from_slice(&df.dev_eui)?.to_string()
            } else {
                "".to_string()
            },
            multicast_group_id: if !df.multicast_group_id.is_empty() {
                Uuid::from_slice(&df.multicast_group_id)?.to_string()
            } else {
                "".to_string()
            },
            tx_info: dfi.tx_info.clone(),
            phy_payload_byte_count: phy.to_vec()?.len() as u32,
            mac_command_byte_count: if let lrwn::Payload::MACPayload(mac_pl) = &phy.payload {
                if mac_pl.f_port == Some(0) {
                    if let Some(lrwn::FRMPayload::MACCommandSet(v)) = &mac_pl.frm_payload {
                        v.size()?
                    } else {
                        0
                    }
                } else {
                    mac_pl.fhdr.f_opts.size()?
                }
            } else {
                0
            } as u32,
            application_payload_byte_count: if let lrwn::Payload::MACPayload(mac_pl) = &phy.payload
            {
                if mac_pl.f_port.unwrap_or_default() > 0 {
                    if let Some(lrwn::FRMPayload::Raw(b)) = &mac_pl.frm_payload {
                        b.len()
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            } as u32,
            message_type: phy.mhdr.m_type.to_proto().into(),
            gateway_id: df.downlink_frame.as_ref().unwrap().gateway_id.clone(),
        };

        metalog::log_downlink(&dm).await
    }

    fn is_error(&self) -> bool {
        self.downlink_tx_ack_status != gw::TxAckStatus::Ok
    }

    // Returns true if the downlink_frame is associated to a dev_eui and if the f_port > 0.
    // In the case the downlink is multicast, the f_port > 0, but the dev_eui is not set.
    fn is_application_payload(&self) -> bool {
        if self.downlink_frame.as_ref().unwrap().dev_eui.is_empty() {
            return false;
        }

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.as_ref().unwrap().payload {
            if pl.f_port.unwrap_or(0) != 0 {
                return true;
            }
        }
        false
    }

    fn is_application_payload_relayed(&self) -> bool {
        if self.downlink_frame.as_ref().unwrap().dev_eui.is_empty() {
            return false;
        }

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload_relayed.as_ref().unwrap().payload {
            if pl.f_port.unwrap_or(0) != 0 {
                return true;
            }
        }

        false
    }

    fn is_relay_payload(&self) -> bool {
        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.as_ref().unwrap().payload {
            if pl.f_port.unwrap_or(0) == lrwn::LA_FPORT_RELAY {
                return true;
            }
        }
        false
    }

    fn is_mac_only_downlink(&self) -> bool {
        if self.downlink_frame.as_ref().unwrap().dev_eui.is_empty() {
            return false;
        }

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload.as_ref().unwrap().payload {
            if pl.f_port.unwrap_or(0) == 0 {
                return true;
            }
        }
        false
    }

    fn is_mac_only_downlink_relayed(&self) -> bool {
        if self.downlink_frame.as_ref().unwrap().dev_eui.is_empty() {
            return false;
        }

        if let lrwn::Payload::MACPayload(pl) = &self.phy_payload_relayed.as_ref().unwrap().payload {
            if pl.f_port.unwrap_or(0) == 0 {
                return true;
            }
        }
        false
    }

    fn is_multicast_downlink(&self) -> bool {
        let df = self.downlink_frame.as_ref().unwrap();
        if !df.multicast_group_id.is_empty() && !df.multicast_group_queue_item_id.is_empty() {
            return true;
        }
        false
    }

    fn is_unconfirmed_downlink(&self) -> bool {
        if self.phy_payload.as_ref().unwrap().mhdr.m_type == lrwn::MType::UnconfirmedDataDown {
            return true;
        }
        false
    }

    fn is_unconfirmed_downlink_relayed(&self) -> bool {
        if self.phy_payload_relayed.as_ref().unwrap().mhdr.m_type
            == lrwn::MType::UnconfirmedDataDown
        {
            return true;
        }
        false
    }

    fn is_confirmed_downlink(&self) -> bool {
        if self.phy_payload.as_ref().unwrap().mhdr.m_type == lrwn::MType::ConfirmedDataDown {
            return true;
        }
        false
    }

    fn is_confirmed_downlink_relayed(&self) -> bool {
        if self.phy_payload_relayed.as_ref().unwrap().mhdr.m_type == lrwn::MType::ConfirmedDataDown
        {
            return true;
        }
        false
    }
}
