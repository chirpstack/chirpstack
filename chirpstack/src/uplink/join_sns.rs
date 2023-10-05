use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use tracing::{span, trace, Instrument, Level};

use super::{helpers, UplinkFrameSet};
use crate::api::helpers::ToProto;
use crate::backend::{joinserver, keywrap, roaming};
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_keys, device_profile, device_queue, device_session,
    error::Error as StorageError,
    metrics, tenant,
};
use crate::{config, devaddr::get_random_dev_addr, integration, metalog, region};
use backend::{PRStartAnsPayload, PRStartReqPayload};
use chirpstack_api::{common, integration as integration_pb, internal, meta};
use lrwn::{keys, AES128Key, DevAddr, NetID};

pub struct JoinRequest {
    uplink_frame_set: UplinkFrameSet,
    pr_start_req: PRStartReqPayload,
    pr_start_ans: Option<PRStartAnsPayload>,

    join_request: Option<lrwn::JoinRequestPayload>,
    join_accept: Option<lrwn::PhyPayload>,
    device: Option<device::Device>,
    device_session: Option<internal::DeviceSession>,
    js_client: Option<Arc<backend::Client>>,
    application: Option<application::Application>,
    tenant: Option<tenant::Tenant>,
    device_profile: Option<device_profile::DeviceProfile>,
    device_keys: Option<device_keys::DeviceKeys>,
    device_info: Option<integration_pb::DeviceInfo>,
    dev_addr: Option<DevAddr>,
    f_nwk_s_int_key: Option<AES128Key>,
    s_nwk_s_int_key: Option<AES128Key>,
    nwk_s_enc_key: Option<AES128Key>,
    app_s_key: Option<common::KeyEnvelope>,
    js_session_key_id: String,
}

impl JoinRequest {
    pub async fn start_pr(
        ufs: UplinkFrameSet,
        pr_start_req: PRStartReqPayload,
    ) -> Result<PRStartAnsPayload> {
        let span = span!(Level::INFO, "start_pr");
        JoinRequest::_start_pr(ufs, pr_start_req)
            .instrument(span)
            .await
    }

    async fn _start_pr(
        ufs: UplinkFrameSet,
        pr_start_req: PRStartReqPayload,
    ) -> Result<PRStartAnsPayload> {
        let mut ctx = JoinRequest {
            uplink_frame_set: ufs,
            pr_start_req,

            pr_start_ans: None,
            join_request: None,
            join_accept: None,
            device: None,
            device_session: None,
            js_client: None,
            application: None,
            tenant: None,
            device_profile: None,
            device_keys: None,
            device_info: None,
            dev_addr: None,
            f_nwk_s_int_key: None,
            s_nwk_s_int_key: None,
            nwk_s_enc_key: None,
            app_s_key: None,
            js_session_key_id: "".to_string(),
        };

        ctx.get_join_request_payload()?;
        ctx.get_device().await?;
        ctx.get_device_keys_or_js_client().await?;
        ctx.get_application().await?;
        ctx.get_tenant().await?;
        ctx.get_device_profile().await?;
        ctx.set_device_info()?;
        ctx.abort_on_device_is_disabled()?;
        ctx.abort_on_otaa_is_disabled()?;
        ctx.get_random_dev_addr()?;
        if ctx.js_client.is_some() {
            // Using join-server
            ctx.get_join_accept_from_js().await?;
        } else {
            // Using internal keys
            ctx.validate_mic().await?;
            ctx.validate_dev_nonce_and_get_device_keys().await?;
            ctx.construct_join_accept_and_set_keys()?;
        }
        ctx.log_uplink_meta().await?;
        ctx.create_device_session().await?;
        ctx.flush_device_queue().await?;
        ctx.set_device_mode().await?;
        ctx.set_join_eui().await?;
        ctx.send_join_event().await?;
        ctx.set_pr_start_ans_payload()?;

        ctx.pr_start_ans
            .ok_or_else(|| anyhow!("PRStartAnsPayload is not set"))
    }

    fn get_join_request_payload(&mut self) -> Result<()> {
        trace!("Getting JoinRequestPayload");
        self.join_request = Some(match self.uplink_frame_set.phy_payload.payload {
            lrwn::Payload::JoinRequest(pl) => pl,
            _ => {
                return Err(anyhow!("PhyPayload does not contain JoinRequest payload"));
            }
        });
        Ok(())
    }

    async fn get_device(&mut self) -> Result<()> {
        trace!("Getting device");
        let jr = self.join_request.as_ref().unwrap();
        let dev = device::get(&jr.dev_eui).await?;
        self.device = Some(dev);
        Ok(())
    }

    // We need to get either the device-keys or a JS client. In any other case, this must return an error.
    async fn get_device_keys_or_js_client(&mut self) -> Result<()> {
        trace!("Getting device keys");
        let jr = self.join_request.as_ref().unwrap();
        self.device_keys = match device_keys::get(&jr.dev_eui).await {
            Ok(v) => Some(v),
            Err(e) => {
                if let StorageError::NotFound(_) = e {
                    None
                } else {
                    return Err(anyhow::Error::new(e));
                }
            }
        };

        if !self.device_keys.is_some() {
            trace!(join_eui = %jr.join_eui, "Getting Join Server client");
            self.js_client = Some(joinserver::get(&jr.join_eui)?);
        }

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

        let mut tags = (*dp.tags).clone();
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

    fn get_random_dev_addr(&mut self) -> Result<()> {
        self.dev_addr = Some(get_random_dev_addr());
        Ok(())
    }

    async fn get_join_accept_from_js(&mut self) -> Result<()> {
        trace!("Getting join-accept from Join Server");

        let js_client = self.js_client.as_ref().unwrap();
        let region_network = config::get_region_network(&self.uplink_frame_set.region_config_id)?;
        let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;

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

        let dl_settings = lrwn::DLSettings {
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

        let join_ans_pl = js_client.join_req(&mut join_req_pl, None).await?;

        if let Some(v) = &join_ans_pl.app_s_key {
            self.app_s_key = Some(common::KeyEnvelope {
                kek_label: v.kek_label.clone(),
                aes_key: v.aes_key.clone(),
            });
        }
        self.js_session_key_id = hex::encode(join_ans_pl.session_key_id);

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

        self.join_accept = Some(
            lrwn::PhyPayload::from_slice(&join_ans_pl.phy_payload).context("Decode PhyPayload")?,
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
            app.id,
            &dev.variables,
            &integration_pb::LogEvent {
                time: Some(Utc::now().into()),
                device_info: self.device_info.clone(),
                level: integration_pb::LogLevel::Error.into(),
                code: integration_pb::LogCode::UplinkMic.into(),
                description: "MIC of join-request is invalid, make sure keys are correct".into(),
                context: [(
                    "deduplication_id".to_string(),
                    self.uplink_frame_set.uplink_set_id.to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
            },
        )
        .await;

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

    async fn validate_dev_nonce_and_get_device_keys(&mut self) -> Result<()> {
        trace!("Validate dev-nonce and get device-keys");
        let dev = self.device.as_ref().unwrap();
        let app = self.application.as_ref().unwrap();
        let join_request = self.join_request.as_ref().unwrap();

        self.device_keys = Some(
            match device_keys::validate_incr_join_and_store_dev_nonce(
                &dev.dev_eui,
                join_request.dev_nonce as i32,
            )
            .await
            {
                Ok(v) => v,
                Err(v) => match v {
                    StorageError::InvalidDevNonce => {
                        integration::log_event(
                            app.id,
                            &dev.variables,
                            &integration_pb::LogEvent {
                                time: Some(Utc::now().into()),
                                device_info: self.device_info.clone(),
                                level: integration_pb::LogLevel::Error.into(),
                                code: integration_pb::LogCode::Otaa.into(),
                                description: "DevNonce has already been used".into(),
                                context: [(
                                    "deduplication_id".to_string(),
                                    self.uplink_frame_set.uplink_set_id.to_string(),
                                )]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )
                        .await;

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

    fn construct_join_accept_and_set_keys(&mut self) -> Result<()> {
        trace!("Constructing JoinAccept payload");

        let conf = config::get();
        let region_network = config::get_region_network(&self.uplink_frame_set.region_config_id)?;
        let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;
        let join_request = self.join_request.as_ref().unwrap();

        let dk = self.device_keys.as_mut().unwrap();

        let join_nonce = dk.join_nonce - 1; // this was incremented on validation
        if join_nonce == (1 << 24) - 1 {
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

        let mut phy = lrwn::PhyPayload {
            mhdr: lrwn::MHDR {
                m_type: lrwn::MType::JoinAccept,
                major: lrwn::Major::LoRaWANR1,
            },
            payload: lrwn::Payload::JoinAccept(lrwn::JoinAcceptPayload {
                join_nonce: join_nonce as u32,
                home_netid: conf.network.net_id,
                devaddr: self.dev_addr.unwrap(),
                dl_settings: lrwn::DLSettings {
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
            let js_int_key = keys::get_js_int_key(&join_request.dev_eui, &dk.nwk_key)?;
            phy.set_join_accept_mic(
                lrwn::JoinType::Join,
                &join_request.join_eui,
                join_request.dev_nonce,
                &js_int_key,
            )?;
        } else {
            phy.set_join_accept_mic(
                lrwn::JoinType::Join,
                &join_request.join_eui,
                join_request.dev_nonce,
                &dk.nwk_key,
            )?;
        }

        phy.encrypt_join_accept_payload(&dk.nwk_key)?;
        self.join_accept = Some(phy);

        trace!("Setting session-keys");
        let device_keys = self.device_keys.as_ref().unwrap();

        self.f_nwk_s_int_key = Some(keys::get_f_nwk_s_int_key(
            opt_neg,
            &device_keys.nwk_key,
            &conf.network.net_id,
            &join_request.join_eui,
            join_nonce as u32,
            join_request.dev_nonce,
        )?);

        self.s_nwk_s_int_key = Some(match opt_neg {
            true => keys::get_s_nwk_s_int_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                join_nonce as u32,
                join_request.dev_nonce,
            )?,
            false => keys::get_f_nwk_s_int_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                join_nonce as u32,
                join_request.dev_nonce,
            )?,
        });

        self.nwk_s_enc_key = Some(match opt_neg {
            true => keys::get_nwk_s_enc_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                join_nonce as u32,
                join_request.dev_nonce,
            )?,
            false => keys::get_f_nwk_s_int_key(
                opt_neg,
                &device_keys.nwk_key,
                &conf.network.net_id,
                &join_request.join_eui,
                join_nonce as u32,
                join_request.dev_nonce,
            )?,
        });

        self.app_s_key = Some(common::KeyEnvelope {
            kek_label: "".to_string(),
            aes_key: match opt_neg {
                true => keys::get_app_s_key(
                    opt_neg,
                    &device_keys.app_key,
                    &conf.network.net_id,
                    &join_request.join_eui,
                    join_nonce as u32,
                    join_request.dev_nonce,
                )?,
                false => keys::get_app_s_key(
                    opt_neg,
                    &device_keys.nwk_key,
                    &conf.network.net_id,
                    &join_request.join_eui,
                    join_nonce as u32,
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

        let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;
        let region_network = config::get_region_network(&self.uplink_frame_set.region_config_id)?;

        let device = self.device.as_ref().unwrap();
        let device_profile = self.device_profile.as_ref().unwrap();
        let join_request = self.join_request.as_ref().unwrap();

        let mut ds = internal::DeviceSession {
            region_config_id: self.uplink_frame_set.region_config_id.clone(),
            dev_eui: device.dev_eui.to_be_bytes().to_vec(),
            dev_addr: self.dev_addr.unwrap().to_be_bytes().to_vec(),
            join_eui: join_request.join_eui.to_be_bytes().to_vec(),
            f_nwk_s_int_key: self.f_nwk_s_int_key.as_ref().unwrap().to_vec(),
            s_nwk_s_int_key: self.s_nwk_s_int_key.as_ref().unwrap().to_vec(),
            nwk_s_enc_key: self.nwk_s_enc_key.as_ref().unwrap().to_vec(),
            app_s_key: self.app_s_key.clone(),
            rx1_delay: region_network.rx1_delay.into(),
            rx1_dr_offset: region_network.rx1_dr_offset.into(),
            rx2_dr: region_network.rx2_dr.into(),
            rx2_frequency: region_conf.get_defaults().rx2_frequency,
            enabled_uplink_channel_indices: region_conf
                .get_default_uplink_channel_indices()
                .iter()
                .map(|i| *i as u32)
                .collect(),
            skip_f_cnt_check: device.skip_fcnt_check,
            ..Default::default()
        };

        device_profile.reset_session_to_boot_params(&mut ds);

        if let Some(lrwn::CFList::Channels(channels)) =
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
            *device = device::set_enabled_class(&device.dev_eui, DeviceClass::C).await?;
        } else {
            *device = device::set_enabled_class(&device.dev_eui, DeviceClass::A).await?;
        }
        Ok(())
    }

    async fn set_join_eui(&mut self) -> Result<()> {
        trace!("Setting JoinEUI");
        let dev = self.device.as_mut().unwrap();
        let req = self.join_request.as_ref().unwrap();

        *dev = device::set_join_eui(dev.dev_eui, req.join_eui).await?;

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
            relay_rx_info: None,
            dev_addr: self.dev_addr.as_ref().unwrap().to_string(),
            join_server_context: if !self.js_session_key_id.is_empty() {
                Some(integration_pb::JoinServerContext {
                    app_s_key: None,
                    session_key_id: self.js_session_key_id.clone(),
                })
            } else if let Some(app_s_key) = &self.app_s_key {
                if app_s_key.kek_label.is_empty() {
                    None
                } else {
                    Some(integration_pb::JoinServerContext {
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: app_s_key.kek_label.clone(),
                            aes_key: app_s_key.aes_key.clone(),
                        }),
                        session_key_id: "".to_string(),
                    })
                }
            } else {
                None
            },
        };

        integration::join_event(app.id, &dev.variables, &pl).await;
        Ok(())
    }

    fn set_pr_start_ans_payload(&mut self) -> Result<()> {
        trace!("Setting PRStartAnsPayload");
        let ds = self.device_session.as_ref().unwrap();
        let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;

        let sender_id = NetID::from_slice(&self.pr_start_req.base.sender_id)?;
        let pr_lifetime = roaming::get_passive_roaming_lifetime(sender_id)?;
        let kek_label = roaming::get_passive_roaming_kek_label(sender_id)?;

        let nwk_s_key = if ds.mac_version().to_string().starts_with("1.0") {
            Some(keywrap::wrap(
                &kek_label,
                AES128Key::from_slice(&ds.nwk_s_enc_key)?,
            )?)
        } else {
            None
        };

        let f_nwk_s_int_key = if ds.mac_version().to_string().starts_with("1.0") {
            None
        } else {
            Some(keywrap::wrap(
                &kek_label,
                AES128Key::from_slice(&ds.f_nwk_s_int_key)?,
            )?)
        };

        let rx1_delay = region_conf.get_defaults().join_accept_delay1;
        let rx1_dr = region_conf.get_rx1_data_rate_index(self.uplink_frame_set.dr, 0)?;
        let rx1_freq = region_conf
            .get_rx1_frequency_for_uplink_frequency(self.uplink_frame_set.tx_info.frequency)?;

        let rx2_dr = region_conf.get_defaults().rx2_dr;
        let rx2_freq = region_conf.get_defaults().rx2_frequency;

        self.pr_start_ans = Some(PRStartAnsPayload {
            base: self
                .pr_start_req
                .base
                .to_base_payload_result(backend::ResultCode::Success, ""),
            phy_payload: self.join_accept.as_ref().unwrap().to_vec()?,
            dev_eui: ds.dev_eui.clone(),
            dev_addr: ds.dev_addr.clone(),
            lifetime: if pr_lifetime.is_zero() {
                None
            } else {
                Some(pr_lifetime.as_secs() as usize)
            },
            f_nwk_s_int_key,
            nwk_s_key,
            f_cnt_up: Some(0),
            dl_meta_data: Some(backend::DLMetaData {
                dev_eui: ds.dev_eui.clone(),
                dl_freq_1: Some(rx1_freq as f64 / 1_000_000.0),
                dl_freq_2: Some(rx2_freq as f64 / 1_000_000.0),
                rx_delay_1: Some(rx1_delay.as_secs() as usize),
                class_mode: Some("A".to_string()),
                data_rate_1: Some(rx1_dr),
                data_rate_2: Some(rx2_dr),
                f_ns_ul_token: self.pr_start_req.ul_meta_data.f_ns_ul_token.clone(),
                gw_info: self
                    .pr_start_req
                    .ul_meta_data
                    .gw_info
                    .iter()
                    .map(|gw| backend::GWInfoElement {
                        ul_token: gw.ul_token.clone(),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        });

        Ok(())
    }
}
