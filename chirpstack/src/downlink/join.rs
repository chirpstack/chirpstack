use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use rand::Rng;
use tracing::{span, trace, Instrument, Level};

use lrwn::{PhyPayload, EUI64};

use super::helpers;
use crate::api::helpers::FromProto;
use crate::gateway::backend::send_downlink;
use crate::storage::{device, downlink_frame, tenant};
use crate::uplink::{RelayContext, UplinkFrameSet};
use crate::{config, region};
use chirpstack_api::{gw, internal};

pub struct JoinAccept<'a> {
    uplink_frame_set: &'a UplinkFrameSet,
    relay_context: Option<&'a RelayContext>,
    tenant: &'a tenant::Tenant,
    device: &'a device::Device,
    device_session: &'a internal::DeviceSession,
    join_accept: &'a PhyPayload,
    network_conf: config::RegionNetwork,
    region_conf: Arc<Box<dyn lrwn::region::Region + Sync + Send>>,

    downlink_frame: chirpstack_api::gw::DownlinkFrame,
    device_gateway_rx_info: Option<chirpstack_api::internal::DeviceGatewayRxInfo>,
    downlink_gateway: Option<chirpstack_api::internal::DeviceGatewayRxInfoItem>,
}

impl JoinAccept<'_> {
    pub async fn handle(
        ufs: &UplinkFrameSet,
        tenant: &tenant::Tenant,
        device: &device::Device,
        device_session: &internal::DeviceSession,
        join_accept: &PhyPayload,
    ) -> Result<()> {
        let span = span!(Level::TRACE, "join_accept", downlink_id = %ufs.uplink_set_id);

        let fut = JoinAccept::_handle(ufs, tenant, device, device_session, join_accept);
        fut.instrument(span).await
    }

    pub async fn handle_relayed(
        relay_ctx: &RelayContext,
        ufs: &UplinkFrameSet,
        tenant: &tenant::Tenant,
        device: &device::Device,
        device_session: &internal::DeviceSession,
        join_accept: &PhyPayload,
    ) -> Result<()> {
        let span = span!(Level::TRACE, "join_accept_relayed", downlink_id = %ufs.uplink_set_id);

        let fut = JoinAccept::_handle_relayed(
            relay_ctx,
            ufs,
            tenant,
            device,
            device_session,
            join_accept,
        );
        fut.instrument(span).await
    }

    async fn _handle(
        ufs: &UplinkFrameSet,
        tenant: &tenant::Tenant,
        device: &device::Device,
        device_session: &internal::DeviceSession,
        join_accept: &PhyPayload,
    ) -> Result<()> {
        let mut ctx = JoinAccept {
            uplink_frame_set: ufs,
            relay_context: None,
            tenant,
            device,
            device_session,
            join_accept,
            network_conf: config::get_region_network(&ufs.region_config_id)?,
            region_conf: region::get(&ufs.region_config_id)?,

            downlink_frame: chirpstack_api::gw::DownlinkFrame {
                downlink_id: rand::thread_rng().gen(),
                ..Default::default()
            },
            device_gateway_rx_info: None,
            downlink_gateway: None,
        };

        ctx.set_device_gateway_rx_info()?;
        ctx.select_downlink_gateway()?;
        ctx.set_tx_info()?;
        ctx.set_downlink_frame()?;
        ctx.save_downlink_frame().await?;
        ctx.send_join_accept_response().await?;

        Ok(())
    }

    async fn _handle_relayed(
        relay_ctx: &RelayContext,
        ufs: &UplinkFrameSet,
        tenant: &tenant::Tenant,
        device: &device::Device,
        device_session: &internal::DeviceSession,
        join_accept: &PhyPayload,
    ) -> Result<()> {
        let mut ctx = JoinAccept {
            uplink_frame_set: ufs,
            relay_context: Some(relay_ctx),
            tenant,
            device,
            device_session,
            join_accept,
            network_conf: config::get_region_network(&ufs.region_config_id)?,
            region_conf: region::get(&ufs.region_config_id)?,

            downlink_frame: chirpstack_api::gw::DownlinkFrame {
                downlink_id: rand::thread_rng().gen(),
                ..Default::default()
            },
            device_gateway_rx_info: None,
            downlink_gateway: None,
        };

        ctx.set_device_gateway_rx_info()?;
        ctx.select_downlink_gateway()?;
        ctx.set_tx_info_relayed()?;
        ctx.set_downlink_frame_relayed()?;
        ctx.send_join_accept_response().await?;
        ctx.save_downlink_frame_relayed().await?;

        Ok(())
    }

    fn set_device_gateway_rx_info(&mut self) -> Result<()> {
        trace!("Set device-gateway rx-info");

        self.device_gateway_rx_info = Some(internal::DeviceGatewayRxInfo {
            dev_eui: self.device.dev_eui.to_be_bytes().to_vec(),
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
                            .unwrap_or_else(Vec::new),
                    }
                })
                .collect(),
        });

        Ok(())
    }

    fn select_downlink_gateway(&mut self) -> Result<()> {
        trace!("Select downlink gateway");

        let gw_down = helpers::select_downlink_gateway(
            Some(self.tenant.id),
            &self.uplink_frame_set.region_config_id,
            self.network_conf.gateway_prefer_min_margin,
            self.device_gateway_rx_info.as_mut().unwrap(),
        )?;

        self.downlink_frame.gateway_id = hex::encode(&gw_down.gateway_id);
        self.downlink_gateway = Some(gw_down);

        Ok(())
    }

    fn set_tx_info(&mut self) -> Result<()> {
        trace!("Setting tx-info");

        if self.network_conf.rx_window == 0 || self.network_conf.rx_window == 1 {
            self.set_tx_info_for_rx1()?;
        }

        if self.network_conf.rx_window == 0 || self.network_conf.rx_window == 2 {
            self.set_tx_info_for_rx2()?;
        }

        Ok(())
    }

    fn set_tx_info_relayed(&mut self) -> Result<()> {
        trace!("Setting tx-info for relay");

        if self.network_conf.rx_window == 0 || self.network_conf.rx_window == 1 {
            self.set_tx_info_for_rx1_relayed()?;
        }

        if self.network_conf.rx_window == 0 || self.network_conf.rx_window == 2 {
            self.set_tx_info_for_rx2_relayed()?;
        }

        Ok(())
    }

    fn set_tx_info_for_rx1(&mut self) -> Result<()> {
        trace!("Setting tx-info for RX1");
        let gw_down = self.downlink_gateway.as_ref().unwrap();

        let mut tx_info = chirpstack_api::gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // get RX1 DR.
        let rx1_dr_index = self
            .region_conf
            .get_rx1_data_rate_index(self.uplink_frame_set.dr, 0)?;
        let rx1_dr = self.region_conf.get_data_rate(rx1_dr_index)?;

        // set DR to tx_info.
        helpers::set_tx_info_data_rate(&mut tx_info, &rx1_dr)?;

        // set frequency
        tx_info.frequency = self
            .region_conf
            .get_rx1_frequency_for_uplink_frequency(self.uplink_frame_set.tx_info.frequency)?;

        // set tx power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // Set timestamp.
        tx_info.timing = Some(gw::Timing {
            parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                delay: Some(pbjson_types::Duration::from(
                    self.region_conf.get_defaults().join_accept_delay1,
                )),
            })),
        });

        // set downlink item
        self.downlink_frame
            .items
            .push(chirpstack_api::gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            });

        Ok(())
    }

    fn set_tx_info_for_rx1_relayed(&mut self) -> Result<()> {
        trace!("Setting relay tx-info for RX1");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let relay_ctx = self.relay_context.unwrap();

        let mut tx_info = chirpstack_api::gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            context: gw_down.context.clone(),
            ..Default::default()
        };

        // Get RX1 DR offset.
        let rx1_dr_offset = relay_ctx.device_session.rx1_dr_offset as usize;

        // get RX1 DR.
        let rx1_dr_index = self
            .region_conf
            .get_rx1_data_rate_index(self.uplink_frame_set.dr, rx1_dr_offset)?;
        let rx1_dr = self.region_conf.get_data_rate(rx1_dr_index)?;

        // set DR to tx_info.
        helpers::set_tx_info_data_rate(&mut tx_info, &rx1_dr)?;

        // set frequency
        tx_info.frequency = self
            .region_conf
            .get_rx1_frequency_for_uplink_frequency(self.uplink_frame_set.tx_info.frequency)?;

        // set tx power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // Set timestamp.
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

        // set downlink item
        self.downlink_frame
            .items
            .push(chirpstack_api::gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            });

        Ok(())
    }

    fn set_tx_info_for_rx2(&mut self) -> Result<()> {
        trace!("Setting tx-info for RX2");
        let gw_down = self.downlink_gateway.as_ref().unwrap();

        // Get frequency.
        let frequency = self.region_conf.get_defaults().rx2_frequency;

        let mut tx_info = chirpstack_api::gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            context: gw_down.context.clone(),
            frequency,
            ..Default::default()
        };

        // get RX2 DR
        let rx2_dr_index = self.region_conf.get_defaults().rx2_dr;
        let rx2_dr = self.region_conf.get_data_rate(rx2_dr_index)?;

        // set DR to tx_info
        helpers::set_tx_info_data_rate(&mut tx_info, &rx2_dr)?;

        // set tx-power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // Set timestamp.
        tx_info.timing = Some(gw::Timing {
            parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                delay: Some(pbjson_types::Duration::from(
                    self.region_conf.get_defaults().join_accept_delay2,
                )),
            })),
        });

        // set downlink item
        self.downlink_frame
            .items
            .push(chirpstack_api::gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            });

        Ok(())
    }

    fn set_tx_info_for_rx2_relayed(&mut self) -> Result<()> {
        trace!("Setting relay tx-info for RX2");

        let gw_down = self.downlink_gateway.as_ref().unwrap();
        let relay_ctx = self.relay_context.unwrap();

        // Get frequency.
        let frequency = relay_ctx.device_session.rx2_frequency;

        let mut tx_info = chirpstack_api::gw::DownlinkTxInfo {
            board: gw_down.board,
            antenna: gw_down.antenna,
            context: gw_down.context.clone(),
            frequency,
            ..Default::default()
        };

        // get RX2 DR
        let rx2_dr_index = relay_ctx.device_session.rx2_dr as u8;
        let rx2_dr = self.region_conf.get_data_rate(rx2_dr_index)?;

        // set DR to tx_info
        helpers::set_tx_info_data_rate(&mut tx_info, &rx2_dr)?;

        // set tx-power
        if self.network_conf.downlink_tx_power != -1 {
            tx_info.power = self.network_conf.downlink_tx_power;
        } else {
            tx_info.power = self.region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        // Set timestamp.
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

        // set downlink item
        self.downlink_frame
            .items
            .push(chirpstack_api::gw::DownlinkFrameItem {
                tx_info: Some(tx_info),
                ..Default::default()
            });

        Ok(())
    }

    fn set_downlink_frame(&mut self) -> Result<()> {
        trace!("Setting downlink frame");

        let phy_b = self.join_accept.to_vec()?;
        for i in &mut self.downlink_frame.items {
            i.phy_payload = phy_b.clone();
        }

        Ok(())
    }

    fn set_downlink_frame_relayed(&mut self) -> Result<()> {
        trace!("Setting ForwardDownlinkReq frame");

        let relay_ctx = self.relay_context.as_ref().unwrap();

        let mut relay_phy = lrwn::PhyPayload {
            mhdr: lrwn::MHDR {
                m_type: lrwn::MType::UnconfirmedDataDown,
                major: lrwn::Major::LoRaWANR1,
            },
            payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                fhdr: lrwn::FHDR {
                    devaddr: lrwn::DevAddr::from_slice(&relay_ctx.device_session.dev_addr)?,
                    f_cnt: if relay_ctx
                        .device_session
                        .mac_version()
                        .to_string()
                        .starts_with("1.0")
                    {
                        relay_ctx.device_session.n_f_cnt_down
                    } else {
                        relay_ctx.device_session.a_f_cnt_down
                    },
                    f_ctrl: lrwn::FCtrl {
                        adr: !self.network_conf.adr_disabled,
                        ack: relay_ctx.must_ack,
                        ..Default::default()
                    },
                    f_opts: lrwn::MACCommandSet::new(vec![]),
                },
                f_port: Some(lrwn::LA_FPORT_RELAY),
                frm_payload: Some(lrwn::FRMPayload::ForwardDownlinkReq(
                    lrwn::ForwardDownlinkReq {
                        payload: Box::new(self.join_accept.clone()),
                    },
                )),
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
        for i in &mut self.downlink_frame.items {
            i.phy_payload = relay_phy_b.clone();
        }

        Ok(())
    }

    async fn send_join_accept_response(&self) -> Result<()> {
        trace!("Sending join-accept response");

        send_downlink(
            &self.uplink_frame_set.region_config_id,
            &self.downlink_frame,
        )
        .await?;
        Ok(())
    }

    async fn save_downlink_frame(&self) -> Result<()> {
        trace!("Saving downlink frame");

        let df = chirpstack_api::internal::DownlinkFrame {
            dev_eui: self.device.dev_eui.to_be_bytes().to_vec(),
            downlink_id: self.downlink_frame.downlink_id,
            downlink_frame: Some(self.downlink_frame.clone()),
            nwk_s_enc_key: self.device_session.nwk_s_enc_key.clone(),
            ..Default::default()
        };

        downlink_frame::save(&df)
            .await
            .context("Saving downlink-frame error")?;

        Ok(())
    }

    async fn save_downlink_frame_relayed(&self) -> Result<()> {
        trace!("Saving ForwardDownlinkReq frame");

        let relay_ctx = self.relay_context.as_ref().unwrap();

        let df = chirpstack_api::internal::DownlinkFrame {
            dev_eui: relay_ctx.device.dev_eui.to_be_bytes().to_vec(),
            downlink_id: self.downlink_frame.downlink_id,
            downlink_frame: Some(self.downlink_frame.clone()),
            nwk_s_enc_key: relay_ctx.device_session.nwk_s_enc_key.clone(),
            a_f_cnt_down: relay_ctx.device_session.get_a_f_cnt_down(),
            n_f_cnt_down: relay_ctx.device_session.n_f_cnt_down,
            ..Default::default()
        };

        downlink_frame::save(&df).await?;
        Ok(())
    }
}
