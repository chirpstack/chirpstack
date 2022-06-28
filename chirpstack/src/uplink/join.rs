use std::convert::TryInto;
use std::sync::Arc;

use aes::cipher::{generic_array::GenericArray, BlockEncrypt, NewBlockCipher};
use aes::{Aes128, Block};
use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use rand::RngCore;
use tracing::{error, span, trace, Instrument, Level};

use lrwn::{
    AES128Key, CFList, DLSettings, DevAddr, JoinAcceptPayload, JoinRequestPayload, JoinType, MType,
    Major, NetID, Payload, PhyPayload, EUI64, MHDR,
};

use super::{filter_rx_info_by_tenant_id, helpers, UplinkFrameSet};
use crate::api::helpers::ToProto;
use crate::backend::{joinserver, keywrap};
use crate::storage::device_session;
use crate::storage::{
    application, device, device_keys, device_profile, device_queue, error::Error as StorageError,
    metrics, tenant,
};
use crate::{config, downlink, framelog, integration, metalog, region};
use chirpstack_api::{api, common, integration as integration_pb, internal, meta};

pub struct JoinRequest {
    uplink_frame_set: UplinkFrameSet,

    js_client: Option<Arc<backend::Client>>,
    join_request: Option<JoinRequestPayload>,
    join_accept: Option<PhyPayload>,
    device: Option<device::Device>,
    device_session: Option<internal::DeviceSession>,
    application: Option<application::Application>,
    tenant: Option<tenant::Tenant>,
    device_profile: Option<device_profile::DeviceProfile>,
    device_keys: Option<device_keys::DeviceKeys>,
    dev_addr: Option<DevAddr>,
    device_info: Option<integration_pb::DeviceInfo>,
    f_nwk_s_int_key: Option<AES128Key>,
    s_nwk_s_int_key: Option<AES128Key>,
    nwk_s_enc_key: Option<AES128Key>,
    app_s_key: Option<common::KeyEnvelope>,
}

impl JoinRequest {
    pub async fn handle(ufs: UplinkFrameSet) {
        let span = span!(Level::INFO, "join_request");

        if let Err(e) = JoinRequest::_handle(ufs).instrument(span).await {
            error!(error = %e, "Handle join-request error");
        }
    }

    async fn _handle(ufs: UplinkFrameSet) -> Result<()> {
        let mut ctx = JoinRequest {
            uplink_frame_set: ufs,
            js_client: None,
            join_request: None,
            device: None,
            device_session: None,
            application: None,
            tenant: None,
            device_profile: None,
            device_keys: None,
            dev_addr: None,
            join_accept: None,
            device_info: None,
            f_nwk_s_int_key: None,
            s_nwk_s_int_key: None,
            nwk_s_enc_key: None,
            app_s_key: None,
        };

        ctx.get_join_request_payload()?;
        ctx.get_js_client()?;
        ctx.get_device().await?;
        ctx.get_application().await?;
        ctx.get_tenant().await?;
        ctx.get_device_profile().await?;
        ctx.set_device_info()?;
        ctx.filter_rx_info_by_tenant()?;
        ctx.log_uplink_frame_set().await?;
        ctx.abort_on_device_is_disabled()?;
        ctx.abort_on_otaa_is_disabled()?;
        ctx.get_random_dev_addr()?;
        if ctx.js_client.is_some() {
            // Using join-server
            ctx.get_join_accept_from_js().await?;
        } else {
            // Using internal keys
            ctx.validate_dev_nonce_and_get_device_keys().await?;
            ctx.validate_mic().await?;
            ctx.construct_join_accept_and_set_keys()?;
            ctx.save_device_keys().await?;
        }
        ctx.log_uplink_meta().await?;
        ctx.create_device_session().await?;
        ctx.flush_device_queue().await?;
        ctx.set_device_mode().await?;
        ctx.start_downlink_join_accept_flow().await?;
        ctx.send_join_event().await?;

        Ok(())
    }

    fn get_join_request_payload(&mut self) -> Result<()> {
        trace!("Getting JoinRequestPayload");
        self.join_request = Some(match self.uplink_frame_set.phy_payload.payload {
            Payload::JoinRequest(pl) => pl,
            _ => {
                return Err(anyhow!("PhyPayload does not contain JoinRequest payload"));
            }
        });
        Ok(())
    }

    fn get_js_client(&mut self) -> Result<()> {
        let jr = self.join_request.as_ref().unwrap();

        trace!(join_eui = %jr.join_eui, "Trying to get Join Server client");
        if let Ok(v) = joinserver::get(&jr.join_eui) {
            trace!("Found Join Server client");
            self.js_client = Some(v);
        } else {
            trace!("Join Server client does not exist");
        }

        Ok(())
    }

    async fn get_device(&mut self) -> Result<()> {
        trace!("Getting device");
        self.device = Some(device::get(&self.join_request.unwrap().dev_eui).await?);
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

    async fn get_device_profile(&mut self) -> Result<()> {
        trace!("Getting device-profile");

        let dp = device_profile::get(&self.device.as_ref().unwrap().device_profile_id).await?;
        if dp.region != self.uplink_frame_set.region_common_name {
            return Err(anyhow!("Invalid device-profile region"));
        }

        self.device_profile = Some(dp);
        Ok(())
    }

    fn set_device_info(&mut self) -> Result<()> {
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

    fn filter_rx_info_by_tenant(&mut self) -> Result<()> {
        trace!("Filtering rx_info by tenant_id");

        filter_rx_info_by_tenant_id(
            &self.application.as_ref().unwrap().tenant_id,
            &mut self.uplink_frame_set,
        )?;
        Ok(())
    }

    async fn log_uplink_frame_set(&self) -> Result<()> {
        trace!("Logging uplink frame-set");
        let ufl: api::UplinkFrameLog = (&self.uplink_frame_set).try_into()?;
        framelog::log_uplink_for_device(&ufl).await?;
        Ok(())
    }

    fn abort_on_device_is_disabled(&self) -> Result<()> {
        if self.device.as_ref().unwrap().is_disabled {
            return Err(anyhow!("Device is disabled"));
        }
        Ok(())
    }

    fn abort_on_otaa_is_disabled(&self) -> Result<()> {
        if !self.device_profile.as_ref().unwrap().supports_otaa {
            return Err(anyhow!("OTAA is disabled in device-profile"));
        }
        Ok(())
    }

    async fn validate_dev_nonce_and_get_device_keys(&mut self) -> Result<()> {
        trace!("Validate dev-nonce and get device-keys");
        let dev = self.device.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let join_request = self.join_request.as_ref().unwrap();

        self.device_keys = Some(
            match device_keys::validate_and_store_dev_nonce(
                &dev.dev_eui,
                join_request.dev_nonce as i32,
            )
            .await
            {
                Ok(v) => v,
                Err(v) => match v {
                    StorageError::InvalidDevNonce => {
                        integration::log_event(
                            &app.id,
                            &dev.variables,
                            &integration_pb::LogEvent {
                                deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
                                time: Some(Utc::now().into()),
                                device_info: self.device_info.clone(),
                                level: integration_pb::LogLevel::Error.into(),
                                code: integration_pb::LogCode::Otaa.into(),
                                description: "DevNonce has already been used".into(),
                                ..Default::default()
                            },
                        )
                        .await?;

                        metrics::save(
                            &format!("device:{}", dev.dev_eui),
                            &metrics::Record {
                                time: Local::now(),
                                kind: metrics::Kind::ABSOLUTE,
                                metrics: [("error_OTAA".into(), 1f64)].iter().cloned().collect(),
                            },
                        )
                        .await?;

                        return Err(v.into());
                    }
                    _ => {
                        return Err(v.into());
                    }
                },
            },
        );

        Ok(())
    }

    async fn validate_mic(&self) -> Result<()> {
        let device_keys = self.device_keys.as_ref().unwrap();
        if self
            .uplink_frame_set
            .phy_payload
            .validate_join_request_mic(&device_keys.nwk_key)?
        {
            return Ok(());
        }

        let app = self.application.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        integration::log_event(
            &app.id,
            &dev.variables,
            &integration_pb::LogEvent {
                deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
                time: Some(Utc::now().into()),
                device_info: self.device_info.clone(),
                level: integration_pb::LogLevel::Error.into(),
                code: integration_pb::LogCode::UplinkMic.into(),
                description: "MIC of join-request is invalid, make sure keys are correct".into(),
                ..Default::default()
            },
        )
        .await?;

        metrics::save(
            &format!("device:{}", dev.dev_eui),
            &metrics::Record {
                time: Local::now(),
                kind: metrics::Kind::ABSOLUTE,
                metrics: [("error_UPLINK_MIC".into(), 1f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        )
        .await?;

        Err(anyhow!("Invalid MIC"))
    }

    fn get_random_dev_addr(&mut self) -> Result<()> {
        let conf = config::get();

        let mut dev_addr: [u8; 4] = [0; 4];

        rand::thread_rng().fill_bytes(&mut dev_addr);

        #[cfg(test)]
        {
            dev_addr = [1, 2, 3, 4];
        }

        let mut dev_addr = DevAddr::from_be_bytes(dev_addr);
        dev_addr.set_addr_prefix(&conf.network.net_id);
        self.dev_addr = Some(dev_addr);

        Ok(())
    }

    async fn get_join_accept_from_js(&mut self) -> Result<()> {
        trace!("Getting join-accept from Join Server");

        let js_client = self.js_client.as_ref().unwrap();
        let region_network = config::get_region_network(&self.uplink_frame_set.region_name)?;
        let region_conf = region::get(&self.uplink_frame_set.region_name)?;

        let phy_b = self.uplink_frame_set.phy_payload.to_vec()?;
        let dp = self.device_profile.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        // The opt_neg flag is set for devices other than 1.0.x.
        let opt_neg = !self
            .device_profile
            .as_ref()
            .unwrap()
            .mac_version
            .to_string()
            .starts_with("1.0");

        let dl_settings = DLSettings {
            opt_neg,
            rx2_dr: region_network.rx2_dr,
            rx1_dr_offset: region_network.rx1_dr_offset,
        };

        let mut join_req_pl = backend::JoinReqPayload {
            mac_version: dp.mac_version.to_string(),
            phy_payload: phy_b,
            dev_eui: dev.dev_eui.to_vec(),
            dev_addr: self.dev_addr.unwrap().to_vec(),
            dl_settings: dl_settings.to_le_bytes()?.to_vec(),
            rx_delay: region_network.rx1_delay,
            cf_list: match region_conf.get_cf_list(dp.mac_version) {
                Some(v) => v.to_bytes()?.to_vec(),
                None => Vec::new(),
            },
            ..Default::default()
        };

        let join_ans_pl = js_client.join_req(&mut join_req_pl).await?;

        if let Some(v) = &join_ans_pl.app_s_key {
            self.app_s_key = Some(common::KeyEnvelope {
                kek_label: v.kek_label.clone(),
                aes_key: v.aes_key.clone(),
            });
        }

        if let Some(v) = &join_ans_pl.nwk_s_key {
            let key = keywrap::unwrap(v).context("Unwrap nwk_s_key")?;
            self.s_nwk_s_int_key = Some(key);
            self.f_nwk_s_int_key = Some(key);
            self.nwk_s_enc_key = Some(key);
        }

        if let Some(v) = &join_ans_pl.s_nwk_s_int_key {
            let key = keywrap::unwrap(v).context("Unwrap s_nwk_s_int_key")?;
            self.s_nwk_s_int_key = Some(key);
        }

        if let Some(v) = &join_ans_pl.f_nwk_s_int_key {
            let key = keywrap::unwrap(v).context("Unwrap f_nwk_s_int_key")?;
            self.f_nwk_s_int_key = Some(key);
        }

        if let Some(v) = &join_ans_pl.nwk_s_enc_key {
            let key = keywrap::unwrap(v).context("Unwrap nwk_s_enc_key")?;
            self.nwk_s_enc_key = Some(key);
        }

        self.join_accept =
            Some(PhyPayload::from_slice(&join_ans_pl.phy_payload).context("Decode PhyPayload")?);

        Ok(())
    }

    fn construct_join_accept_and_set_keys(&mut self) -> Result<()> {
        trace!("Constructing JoinAccept payload");

        let conf = config::get();
        let region_network = config::get_region_network(&self.uplink_frame_set.region_name)?;
        let region_conf = region::get(&self.uplink_frame_set.region_name)?;
        let join_request = self.join_request.as_ref().unwrap();

        let dk = self.device_keys.as_mut().unwrap();
        if dk.join_nonce == (1 << 24) - 1 {
            return Err(anyhow!("Join-nonce overflow"));
        }

        // The opt_neg flag is set for devices other than 1.0.x.
        let opt_neg = !self
            .device_profile
            .as_ref()
            .unwrap()
            .mac_version
            .to_string()
            .starts_with("1.0");

        let mut phy = PhyPayload {
            mhdr: MHDR {
                m_type: MType::JoinAccept,
                major: Major::LoRaWANR1,
            },
            payload: Payload::JoinAccept(JoinAcceptPayload {
                join_nonce: dk.join_nonce as u32,
                home_netid: conf.network.net_id,
                devaddr: self.dev_addr.unwrap(),
                dl_settings: DLSettings {
                    opt_neg,
                    rx2_dr: region_network.rx2_dr,
                    rx1_dr_offset: region_network.rx1_dr_offset,
                },
                rx_delay: region_network.rx1_delay,
                cflist: region_conf.get_cf_list(self.device_profile.as_ref().unwrap().mac_version),
            }),
            mic: None, // we need to calculate this
        };

        if opt_neg {
            let js_int_key = get_js_int_key(&join_request.dev_eui, &dk.nwk_key)?;
            phy.set_join_accept_mic(
                JoinType::Join,
                &join_request.join_eui,
                join_request.dev_nonce,
                &js_int_key,
            )?;
        } else {
            phy.set_join_accept_mic(
                JoinType::Join,
                &join_request.join_eui,
                join_request.dev_nonce,
                &dk.nwk_key,
            )?;
        }

        phy.encrypt_join_accept_payload(&dk.nwk_key)?;
        self.join_accept = Some(phy);

        trace!("Setting session-keys");
        let device_keys = self.device_keys.as_ref().unwrap();

        self.f_nwk_s_int_key = Some(get_f_nwk_s_int_key(
            opt_neg,
            &device_keys.nwk_key,
            &conf.network.net_id,
            &join_request.join_eui,
            device_keys.join_nonce as u32,
            join_request.dev_nonce,
        )?);

        self.s_nwk_s_int_key = Some(match opt_neg {
            true => get_s_nwk_s_int_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                device_keys.join_nonce as u32,
                join_request.dev_nonce,
            )?,
            false => get_f_nwk_s_int_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                device_keys.join_nonce as u32,
                join_request.dev_nonce,
            )?,
        });

        self.nwk_s_enc_key = Some(match opt_neg {
            true => get_nwk_s_enc_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                device_keys.join_nonce as u32,
                join_request.dev_nonce,
            )?,
            false => get_f_nwk_s_int_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                device_keys.join_nonce as u32,
                join_request.dev_nonce,
            )?,
        });

        self.app_s_key = Some(common::KeyEnvelope {
            kek_label: "".to_string(),
            aes_key: match opt_neg {
                true => get_app_s_key(
                    opt_neg,
                    &device_keys.app_key,
                    &conf.network.net_id,
                    &join_request.join_eui,
                    device_keys.join_nonce as u32,
                    join_request.dev_nonce,
                )?,
                false => get_app_s_key(
                    opt_neg,
                    &device_keys.nwk_key,
                    &conf.network.net_id,
                    &join_request.join_eui,
                    device_keys.join_nonce as u32,
                    join_request.dev_nonce,
                )?,
            }
            .to_vec(),
        });

        Ok(())
    }

    async fn log_uplink_meta(&self) -> Result<()> {
        trace!("Logging uplink meta");

        let req = meta::UplinkMeta {
            dev_eui: self.device.as_ref().unwrap().dev_eui.to_string(),
            tx_info: Some(self.uplink_frame_set.tx_info.clone()),
            rx_info: self.uplink_frame_set.rx_info_set.clone(),
            message_type: common::MType::JoinRequest.into(),
            phy_payload_byte_count: self.uplink_frame_set.phy_payload.to_vec()?.len() as u32,
            ..Default::default()
        };

        metalog::log_uplink(&req).await?;

        Ok(())
    }

    async fn create_device_session(&mut self) -> Result<()> {
        trace!("Creating device-session");

        let region_conf = region::get(&self.uplink_frame_set.region_name)?;
        let region_network = config::get_region_network(&self.uplink_frame_set.region_name)?;

        let device = self.device.as_ref().unwrap();
        let device_profile = self.device_profile.as_ref().unwrap();
        let join_request = self.join_request.as_ref().unwrap();

        let mut ds = internal::DeviceSession {
            region_name: self.uplink_frame_set.region_name.clone(),
            dev_eui: device.dev_eui.to_be_bytes().to_vec(),
            dev_addr: self.dev_addr.unwrap().to_be_bytes().to_vec(),
            join_eui: join_request.join_eui.to_be_bytes().to_vec(),
            mac_version: device_profile.mac_version.to_proto().into(),
            f_nwk_s_int_key: self.f_nwk_s_int_key.as_ref().unwrap().to_vec(),
            s_nwk_s_int_key: self.s_nwk_s_int_key.as_ref().unwrap().to_vec(),
            nwk_s_enc_key: self.nwk_s_enc_key.as_ref().unwrap().to_vec(),
            app_s_key: self.app_s_key.clone(),
            f_cnt_up: 0,
            n_f_cnt_down: 0,
            a_f_cnt_down: 0,
            conf_f_cnt: 0,
            rx1_delay: region_network.rx1_delay.into(),
            rx1_dr_offset: region_network.rx1_dr_offset.into(),
            rx2_dr: region_network.rx2_dr.into(),
            rx2_frequency: region_conf.get_defaults().rx2_frequency,
            enabled_uplink_channel_indices: region_conf
                .get_default_uplink_channel_indices()
                .iter()
                .map(|i| *i as u32)
                .collect(),
            class_b_ping_slot_dr: device_profile.class_b_ping_slot_dr as u32,
            class_b_ping_slot_freq: device_profile.class_b_ping_slot_freq as u32,
            nb_trans: 1,
            skip_f_cnt_check: device.skip_fcnt_check,
            ..Default::default()
        };

        if let Some(CFList::Channels(channels)) =
            region_conf.get_cf_list(device_profile.mac_version)
        {
            for f in channels.iter().cloned() {
                if f == 0 {
                    continue;
                }

                let i = region_conf
                    .get_uplink_channel_index(f, true)
                    .context("Unknown cf_list frequency")?;

                ds.enabled_uplink_channel_indices.push(i as u32);

                // add extra channel to extra uplink channels, so that we can
                // keep track on frequency and data-rate changes
                let c = region_conf
                    .get_uplink_channel(i)
                    .context("Get uplink channel error")?;

                ds.extra_uplink_channels.insert(
                    i as u32,
                    internal::DeviceSessionChannel {
                        frequency: c.frequency,
                        min_dr: c.min_dr as u32,
                        max_dr: c.max_dr as u32,
                    },
                );
            }
        }

        device_session::save(&ds)
            .await
            .context("Saving device-session failed")?;

        self.device_session = Some(ds);

        Ok(())
    }

    async fn flush_device_queue(&self) -> Result<()> {
        let dp = self.device_profile.as_ref().unwrap();
        if !dp.flush_queue_on_activate {
            return Ok(());
        }

        trace!("Flushing device-queue");
        let dev = self.device.as_ref().unwrap();
        device_queue::flush_for_dev_eui(&dev.dev_eui).await?;
        Ok(())
    }

    async fn set_device_mode(&mut self) -> Result<()> {
        let dp = self.device_profile.as_ref().unwrap();
        let device = self.device.as_mut().unwrap();

        // LoRaWAN 1.1 devices send a mac-command when changing to Class-C.
        if dp.supports_class_c && dp.mac_version.to_string().starts_with("1.0") {
            *device = device::set_enabled_class(&device.dev_eui, "C").await?;
        } else {
            *device = device::set_enabled_class(&device.dev_eui, "A").await?;
        }
        Ok(())
    }

    async fn save_device_keys(&mut self) -> Result<()> {
        trace!("Updating device-keys");
        let mut dk = self.device_keys.as_mut().unwrap();
        dk.join_nonce += 1;

        *dk = device_keys::update(dk.clone()).await?;
        Ok(())
    }

    async fn start_downlink_join_accept_flow(&self) -> Result<()> {
        trace!("Starting downlink join-accept flow");
        downlink::join::JoinAccept::handle(
            &self.uplink_frame_set,
            self.device.as_ref().unwrap(),
            self.device_session.as_ref().unwrap(),
            self.join_accept.as_ref().unwrap(),
        )
        .await?;
        Ok(())
    }

    async fn send_join_event(&self) -> Result<()> {
        trace!("Sending join event");

        let ts: DateTime<Utc> =
            helpers::get_rx_timestamp(&self.uplink_frame_set.rx_info_set).into();

        let app = self.application.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        let pl = integration_pb::JoinEvent {
            deduplication_id: self.uplink_frame_set.uplink_set_id.to_string(),
            time: Some(ts.into()),
            device_info: self.device_info.clone(),
            dev_addr: self.dev_addr.as_ref().unwrap().to_string(),
        };

        integration::join_event(&app.id, &dev.variables, &pl).await?;
        Ok(())
    }
}

// For LoRaWAN 1.0: SNwkSIntKey = NwkSEncKey = FNwkSIntKey = NwkSKey
fn get_f_nwk_s_int_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x01, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

fn get_app_s_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x02, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

fn get_s_nwk_s_int_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x03, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

fn get_nwk_s_enc_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x04, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

fn get_js_enc_key(dev_eui: &EUI64, nwk_key: &AES128Key) -> Result<AES128Key> {
    get_js_key(0x05, dev_eui, nwk_key)
}

pub fn get_js_int_key(dev_eui: &EUI64, nwk_key: &AES128Key) -> Result<AES128Key> {
    get_js_key(0x06, dev_eui, nwk_key)
}

fn get_s_key(
    opt_neg: bool,
    typ: u8,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    let key_bytes = nwk_key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b: [u8; 16] = [0; 16];

    b[0] = typ;
    if opt_neg {
        b[1..4].clone_from_slice(&join_nonce.to_le_bytes()[0..3]);
        b[4..12].clone_from_slice(&join_eui.to_le_bytes());
        b[12..14].clone_from_slice(&dev_nonce.to_le_bytes()[0..2]);
    } else {
        b[1..4].clone_from_slice(&join_nonce.to_le_bytes()[0..3]);
        b[4..7].clone_from_slice(&net_id.to_le_bytes());
        b[7..9].clone_from_slice(&dev_nonce.to_le_bytes()[0..2]);
    }

    let block = Block::from_mut_slice(&mut b);
    cipher.encrypt_block(block);

    Ok(AES128Key::from_slice(block)?)
}

fn get_js_key(typ: u8, dev_eui: &EUI64, nwk_key: &AES128Key) -> Result<AES128Key> {
    let key_bytes = nwk_key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b: [u8; 16] = [0; 16];
    b[0] = typ;
    b[1..9].clone_from_slice(&dev_eui.to_le_bytes());

    let block = Block::from_mut_slice(&mut b);
    cipher.encrypt_block(block);

    Ok(AES128Key::from_slice(block)?)
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn nwk_key() -> AES128Key {
        AES128Key::from_bytes([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08,
        ])
    }

    fn app_key() -> AES128Key {
        AES128Key::from_bytes([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ])
    }

    fn dev_eui() -> EUI64 {
        EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08])
    }

    fn join_eui() -> EUI64 {
        EUI64::from_be_bytes([0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])
    }

    fn join_nonce() -> u32 {
        65536
    }

    fn dev_nonce() -> u16 {
        258
    }

    fn net_id() -> NetID {
        NetID::from_be_bytes([0x01, 0x02, 0x03])
    }

    #[test]
    fn lorawan_1_0() {
        let nwk_s_key = get_f_nwk_s_int_key(
            false,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let app_s_key = get_app_s_key(
            false,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        assert_eq!(
            AES128Key::from_bytes([
                223, 83, 195, 95, 48, 52, 204, 206, 208, 255, 53, 76, 112, 222, 4, 223,
            ]),
            nwk_s_key
        );

        assert_eq!(
            AES128Key::from_bytes([
                146, 123, 156, 145, 17, 131, 207, 254, 76, 178, 255, 75, 117, 84, 95, 109
            ]),
            app_s_key
        )
    }

    #[test]
    fn lorawan_1_1() {
        let app_s_key = get_app_s_key(
            true,
            &app_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let f_nwk_s_int_key = get_f_nwk_s_int_key(
            true,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let s_nwk_s_int_key = get_s_nwk_s_int_key(
            true,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let nwk_s_enc_key = get_nwk_s_enc_key(
            true,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        assert_eq!(
            AES128Key::from_bytes([
                1, 98, 18, 21, 209, 202, 8, 254, 191, 12, 96, 44, 194, 173, 144, 250
            ]),
            app_s_key,
        );

        assert_eq!(
            AES128Key::from_bytes([
                83, 127, 138, 174, 137, 108, 121, 224, 21, 209, 2, 208, 98, 134, 53, 78
            ]),
            f_nwk_s_int_key,
        );

        assert_eq!(
            AES128Key::from_bytes([
                88, 148, 152, 153, 48, 146, 207, 219, 95, 210, 224, 42, 199, 81, 11, 241
            ]),
            s_nwk_s_int_key,
        );

        assert_eq!(
            AES128Key::from_bytes([
                152, 152, 40, 60, 79, 102, 235, 108, 111, 213, 22, 88, 130, 4, 108, 64
            ]),
            nwk_s_enc_key,
        );
    }
}
