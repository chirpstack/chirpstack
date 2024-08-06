use std::convert::TryInto;
use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use tracing::{error, info, span, trace, warn, Instrument, Level};

use lrwn::{
    keys, AES128Key, CFList, DLSettings, JoinAcceptPayload, JoinRequestPayload, JoinType, MType,
    Major, Payload, PhyPayload, MHDR,
};

use super::error::Error;
use super::join_fns;
use super::{
    filter_rx_info_by_region_config_id, filter_rx_info_by_tenant_id, helpers, RelayContext,
    UplinkFrameSet,
};

use crate::api::{backend::get_async_receiver, helpers::ToProto};
use crate::backend::{joinserver, keywrap, roaming};
use crate::helpers::errors::PrintFullError;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_keys, device_profile, device_queue,
    error::Error as StorageError,
    helpers::get_all_device_data,
    metrics, tenant,
};
use crate::{config, devaddr::get_random_dev_addr, downlink, integration, region, stream};
use chirpstack_api::{common, integration as integration_pb, internal, stream as stream_pb};

pub struct JoinRequest {
    uplink_frame_set: UplinkFrameSet,
    relay_context: Option<RelayContext>,

    js_client: Option<Arc<backend::Client>>,
    join_request: Option<JoinRequestPayload>,
    join_accept: Option<PhyPayload>,
    device: Option<device::Device>,
    application: Option<application::Application>,
    tenant: Option<tenant::Tenant>,
    device_profile: Option<device_profile::DeviceProfile>,
    device_keys: Option<device_keys::DeviceKeys>,
    device_info: Option<integration_pb::DeviceInfo>,
    relay_rx_info: Option<integration_pb::UplinkRelayRxInfo>,
    f_nwk_s_int_key: Option<AES128Key>,
    s_nwk_s_int_key: Option<AES128Key>,
    nwk_s_enc_key: Option<AES128Key>,
    app_s_key: Option<common::KeyEnvelope>,
    js_session_key_id: Vec<u8>,
}

impl JoinRequest {
    pub async fn handle(ufs: UplinkFrameSet) {
        let span = span!(Level::INFO, "join_request", dev_eui = tracing::field::Empty);

        if let Err(e) = JoinRequest::_handle(ufs).instrument(span).await {
            match e.downcast_ref::<Error>() {
                Some(Error::Abort) => {
                    // nothing to do
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle join-request error");
                }
            }
        }
    }

    pub async fn handle_relayed(relay_ctx: RelayContext, ufs: UplinkFrameSet) {
        let span = span!(Level::INFO, "join_request_relayed");

        if let Err(e) = JoinRequest::_handle_relayed(relay_ctx, ufs)
            .instrument(span)
            .await
        {
            match e.downcast_ref::<Error>() {
                Some(Error::Abort) => {
                    // nothing to do
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle relayed join-request error");
                }
            }
        }
    }

    async fn _handle(ufs: UplinkFrameSet) -> Result<()> {
        let mut ctx = JoinRequest {
            uplink_frame_set: ufs,
            relay_context: None,
            js_client: None,
            join_request: None,
            device: None,
            application: None,
            tenant: None,
            device_profile: None,
            device_keys: None,
            join_accept: None,
            device_info: None,
            relay_rx_info: None,
            f_nwk_s_int_key: None,
            s_nwk_s_int_key: None,
            nwk_s_enc_key: None,
            app_s_key: None,
            js_session_key_id: vec![],
        };

        ctx.get_join_request_payload()?;

        // Add resolved DevEUI to the span
        let span = tracing::Span::current();
        span.record(
            "dev_eui",
            ctx.join_request.as_ref().unwrap().dev_eui.to_string(),
        );

        ctx.get_device_data_or_try_pr_roaming().await?;
        ctx.get_device_keys_or_js_client().await?; // used to validate MIC + if we need external JS
        ctx.set_device_info()?;
        ctx.filter_rx_info_by_tenant()?;
        ctx.filter_rx_info_by_region_config_id()?;
        ctx.abort_on_device_is_disabled()?;
        ctx.abort_on_relay_only_comm()?;
        ctx.log_uplink_frame_set().await?;
        ctx.abort_on_otaa_is_disabled()?;
        ctx.set_random_dev_addr()?;
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
        ctx.set_device_session().await?;
        ctx.flush_device_queue().await?;
        ctx.set_device_mode().await?;
        ctx.update_device().await?;
        ctx.start_downlink_join_accept_flow().await?;
        ctx.send_join_event().await?;

        Ok(())
    }

    async fn _handle_relayed(relay_ctx: RelayContext, ufs: UplinkFrameSet) -> Result<()> {
        let mut ctx = JoinRequest {
            uplink_frame_set: ufs,
            relay_context: Some(relay_ctx),
            js_client: None,
            join_request: None,
            device: None,
            application: None,
            tenant: None,
            device_profile: None,
            device_keys: None,
            join_accept: None,
            device_info: None,
            relay_rx_info: None,
            f_nwk_s_int_key: None,
            s_nwk_s_int_key: None,
            nwk_s_enc_key: None,
            app_s_key: None,
            js_session_key_id: vec![],
        };

        ctx.get_join_request_payload_relayed()?;
        ctx.get_device_data().await?;
        ctx.get_device_keys_or_js_client().await?;
        ctx.set_device_info()?;
        ctx.set_relay_rx_info()?;
        ctx.abort_on_device_is_disabled()?;
        ctx.abort_on_otaa_is_disabled()?;
        ctx.abort_on_relay_only_comm()?;
        ctx.set_random_dev_addr()?;
        if ctx.js_client.is_some() {
            // Using join-server
            ctx.get_join_accept_from_js().await?;
        } else {
            // Using internal keys
            ctx.validate_mic().await?;
            ctx.validate_dev_nonce_and_get_device_keys().await?;
            ctx.construct_join_accept_and_set_keys()?;
        }
        ctx.set_device_session().await?;
        ctx.flush_device_queue().await?;
        ctx.set_device_mode().await?;
        ctx.update_device().await?;
        ctx.start_downlink_join_accept_flow_relayed().await?;
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

    fn get_join_request_payload_relayed(&mut self) -> Result<()> {
        trace!("Getting JoinRequestPayload from relayed");

        let relay_ctx = self.relay_context.as_ref().unwrap();
        self.join_request = Some(match relay_ctx.req.payload.payload {
            Payload::JoinRequest(pl) => pl,
            _ => {
                return Err(anyhow!(
                    "Relay PhyPayload does not contain JoinRequest payload"
                ));
            }
        });
        Ok(())
    }

    async fn get_device_data(&mut self) -> Result<()> {
        trace!("Getting device data");
        let jr = self.join_request.as_ref().unwrap();

        let (dev, app, t, dp) = get_all_device_data(jr.dev_eui).await?;

        if dp.region != self.uplink_frame_set.region_common_name {
            return Err(anyhow!("Invalid device-profile region"));
        }

        self.tenant = Some(t);
        self.application = Some(app);
        self.device_profile = Some(dp);
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

        if self.device_keys.is_none() {
            trace!(join_eui = %jr.join_eui, "Getting Join Server client");
            self.js_client = Some(joinserver::get(jr.join_eui).await?);
        }

        Ok(())
    }

    async fn get_device_data_or_try_pr_roaming(&mut self) -> Result<()> {
        trace!("Getting device");
        let jr = self.join_request.as_ref().unwrap();
        let (dev, app, t, dp) = match get_all_device_data(jr.dev_eui).await {
            Ok(v) => v,
            Err(e) => {
                if let StorageError::NotFound(_) = e {
                    if !roaming::is_enabled() {
                        warn!(dev_eui = %jr.dev_eui, "Unknown device");
                        return Err(anyhow::Error::new(Error::Abort));
                    }

                    info!(dev_eui = %jr.dev_eui, join_eui = %jr.join_eui, "Unknown device, trying passive-roaming activation");
                    join_fns::JoinRequest::start_pr(self.uplink_frame_set.clone(), *jr).await?;
                    return Err(anyhow::Error::new(Error::Abort));
                } else {
                    return Err(anyhow::Error::new(e));
                }
            }
        };

        if dp.region != self.uplink_frame_set.region_common_name {
            return Err(anyhow!("Invalid device-profile region"));
        }

        self.tenant = Some(t);
        self.application = Some(app);
        self.device_profile = Some(dp);
        self.device = Some(dev);

        Ok(())
    }

    fn set_device_info(&mut self) -> Result<()> {
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

    fn filter_rx_info_by_tenant(&mut self) -> Result<()> {
        trace!("Filtering rx_info by tenant_id");

        filter_rx_info_by_tenant_id(
            self.application.as_ref().unwrap().tenant_id.into(),
            &mut self.uplink_frame_set,
        )?;
        Ok(())
    }

    fn filter_rx_info_by_region_config_id(&mut self) -> Result<()> {
        trace!("Filtering rx_info by region_config_id");

        let dp = self.device_profile.as_ref().unwrap();
        if let Some(v) = &dp.region_config_id {
            filter_rx_info_by_region_config_id(v, &mut self.uplink_frame_set)?;
        }

        Ok(())
    }

    async fn log_uplink_frame_set(&self) -> Result<()> {
        trace!("Logging uplink frame-set");
        let ufl: stream_pb::UplinkFrameLog = (&self.uplink_frame_set).try_into()?;
        stream::frame::log_uplink_for_device(&ufl).await?;
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

    fn abort_on_relay_only_comm(&self) -> Result<(), Error> {
        // In case the relay context is not set and relay_ed_relay_only is set, abort.
        if self.relay_context.is_none() && self.device_profile.as_ref().unwrap().relay_ed_relay_only
        {
            info!(dev_eui = %self.device.as_ref().unwrap().dev_eui, "Only communication through relay is allowed");
            return Err(Error::Abort);
        }
        Ok(())
    }

    async fn validate_mic(&self) -> Result<()> {
        let device_keys = self.device_keys.as_ref().unwrap();

        if let Some(relay_ctx) = self.relay_context.as_ref() {
            if relay_ctx
                .req
                .payload
                .validate_join_request_mic(&device_keys.nwk_key)?
            {
                return Ok(());
            }
        } else if self
            .uplink_frame_set
            .phy_payload
            .validate_join_request_mic(&device_keys.nwk_key)?
        {
            return Ok(());
        }

        let app = self.application.as_ref().unwrap();
        let dev = self.device.as_ref().unwrap();

        integration::log_event(
            app.id.into(),
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
            &metrics::Aggregation::default_aggregations(),
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
                            app.id.into(),
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
                            &metrics::Aggregation::default_aggregations(),
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

    fn set_random_dev_addr(&mut self) -> Result<()> {
        trace!("Setting random DevAddr");
        let d = self.device.as_mut().unwrap();
        d.dev_addr = Some(get_random_dev_addr());
        Ok(())
    }

    async fn get_join_accept_from_js(&mut self) -> Result<()> {
        trace!("Getting join-accept from Join Server");

        let js_client = self.js_client.as_ref().unwrap();
        let jr = self.join_request.as_ref().unwrap();
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

        let dl_settings = DLSettings {
            opt_neg,
            rx2_dr: region_network.rx2_dr,
            rx1_dr_offset: region_network.rx1_dr_offset,
        };

        let mut join_req_pl = backend::JoinReqPayload {
            mac_version: dp.mac_version.to_string(),
            phy_payload: phy_b,
            dev_eui: dev.dev_eui.to_vec(),
            dev_addr: dev.dev_addr.unwrap().to_vec(),
            dl_settings: dl_settings.to_le_bytes()?.to_vec(),
            rx_delay: region_network.rx1_delay,
            cf_list: match region_conf.get_cf_list(dp.mac_version) {
                Some(v) => v.to_bytes()?.to_vec(),
                None => Vec::new(),
            },
            ..Default::default()
        };

        let async_receiver = match js_client.is_async() {
            false => None,
            true => Some(
                get_async_receiver(
                    join_req_pl.base.transaction_id,
                    js_client.get_async_timeout(),
                )
                .await?,
            ),
        };

        let join_ans_pl = js_client
            .join_req(jr.join_eui.to_vec(), &mut join_req_pl, async_receiver)
            .await?;

        if let Some(v) = &join_ans_pl.app_s_key {
            self.app_s_key = Some(common::KeyEnvelope {
                kek_label: v.kek_label.clone(),
                aes_key: v.aes_key.clone(),
            });
        }
        self.js_session_key_id
            .clone_from(&join_ans_pl.session_key_id);

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
        let region_network = config::get_region_network(&self.uplink_frame_set.region_config_id)?;
        let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;
        let join_request = self.join_request.as_ref().unwrap();

        let d = self.device.as_ref().unwrap();
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

        let mut phy = PhyPayload {
            mhdr: MHDR {
                m_type: MType::JoinAccept,
                major: Major::LoRaWANR1,
            },
            payload: Payload::JoinAccept(JoinAcceptPayload {
                join_nonce: join_nonce as u32,
                home_netid: conf.network.net_id,
                devaddr: d.dev_addr.unwrap(),
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
            let js_int_key = keys::get_js_int_key(&join_request.dev_eui, &dk.nwk_key)?;
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

        let um = stream_pb::UplinkMeta {
            dev_eui: self.device.as_ref().unwrap().dev_eui.to_string(),
            tx_info: Some(self.uplink_frame_set.tx_info.clone()),
            rx_info: self.uplink_frame_set.rx_info_set.clone(),
            message_type: common::MType::JoinRequest.into(),
            phy_payload_byte_count: self.uplink_frame_set.phy_payload.to_vec()?.len() as u32,
            ..Default::default()
        };

        stream::meta::log_uplink(&um).await?;

        Ok(())
    }

    async fn set_device_session(&mut self) -> Result<()> {
        trace!("Setting device-session");

        let region_conf = region::get(&self.uplink_frame_set.region_config_id)?;
        let region_network = config::get_region_network(&self.uplink_frame_set.region_config_id)?;

        let device = self.device.as_mut().unwrap();
        let device_profile = self.device_profile.as_ref().unwrap();

        let mut ds = internal::DeviceSession {
            region_config_id: self.uplink_frame_set.region_config_id.clone(),
            dev_addr: device.dev_addr.unwrap().to_be_bytes().to_vec(),
            f_nwk_s_int_key: self.f_nwk_s_int_key.as_ref().unwrap().to_vec(),
            s_nwk_s_int_key: self.s_nwk_s_int_key.as_ref().unwrap().to_vec(),
            nwk_s_enc_key: self.nwk_s_enc_key.as_ref().unwrap().to_vec(),
            app_s_key: self.app_s_key.clone(),
            js_session_key_id: self.js_session_key_id.clone(),
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

        match region_conf.get_cf_list(device_profile.mac_version) {
            Some(CFList::Channels(channels)) => {
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
            Some(CFList::ChannelMask(masks)) => {
                ds.enabled_uplink_channel_indices = vec![];

                for (block_i, block) in masks.iter().enumerate() {
                    for (channel_i, enabled) in block.into_iter().enumerate() {
                        if enabled {
                            ds.enabled_uplink_channel_indices
                                .push((channel_i + (block_i * 16)) as u32);
                        }
                    }
                }
            }
            None => {}
        }

        device.device_session = Some(ds.into());

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
        let d = self.device.as_mut().unwrap();

        // LoRaWAN 1.1 devices send a mac-command when changing to Class-C.
        if dp.supports_class_c && dp.mac_version.to_string().starts_with("1.0") {
            d.enabled_class = DeviceClass::C;
        } else {
            d.enabled_class = DeviceClass::A;
        }

        Ok(())
    }

    async fn update_device(&mut self) -> Result<()> {
        trace!("Updating device");

        let req = self.join_request.as_ref().unwrap();
        let d = self.device.as_mut().unwrap();

        *d = device::partial_update(
            d.dev_eui,
            &device::DeviceChangeset {
                enabled_class: Some(d.enabled_class),
                dev_addr: Some(d.dev_addr),
                secondary_dev_addr: Some(None),
                join_eui: Some(req.join_eui),
                device_session: Some(d.device_session.clone()),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    async fn start_downlink_join_accept_flow(&self) -> Result<()> {
        trace!("Starting downlink join-accept flow");
        downlink::join::JoinAccept::handle(
            &self.uplink_frame_set,
            self.tenant.as_ref().unwrap(),
            self.device.as_ref().unwrap(),
            self.join_accept.as_ref().unwrap(),
        )
        .await?;
        Ok(())
    }

    async fn start_downlink_join_accept_flow_relayed(&self) -> Result<()> {
        trace!("Starting relayed downlink join-accept flow");
        downlink::join::JoinAccept::handle_relayed(
            self.relay_context.as_ref().unwrap(),
            &self.uplink_frame_set,
            self.tenant.as_ref().unwrap(),
            self.device.as_ref().unwrap(),
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
            relay_rx_info: self.relay_rx_info.clone(),
            dev_addr: dev.dev_addr.unwrap().to_string(),
            join_server_context: if !self.js_session_key_id.is_empty() {
                Some(common::JoinServerContext {
                    app_s_key: None,
                    session_key_id: hex::encode(&self.js_session_key_id),
                })
            } else if let Some(app_s_key) = &self.app_s_key {
                if app_s_key.kek_label.is_empty() {
                    None
                } else {
                    Some(common::JoinServerContext {
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: app_s_key.kek_label.clone(),
                            aes_key: app_s_key.aes_key.clone(),
                        }),
                        session_key_id: "".into(),
                    })
                }
            } else {
                None
            },
        };

        integration::join_event(app.id.into(), &dev.variables, &pl).await;
        Ok(())
    }
}
