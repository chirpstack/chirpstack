use anyhow::{Context, Result};
use rand::Rng;
use tracing::{span, trace, Instrument, Level};

use super::helpers;
use crate::backend::roaming;
use crate::storage::downlink_frame;
use crate::{gateway, region};
use chirpstack_api::{gw, internal};

pub struct Data {
    region_config_id: String,
    xmit_data_req: backend::XmitDataReqPayload,
    dl_meta_data: backend::DLMetaData,
    uplink_rx_info: Vec<gw::UplinkRxInfo>,
    downlink_frame: gw::DownlinkFrame,
}

impl Data {
    pub async fn handle(
        pl: backend::XmitDataReqPayload,
        dl_meta: backend::DLMetaData,
    ) -> Result<()> {
        let span = span!(Level::INFO, "xmit_data_req_pr");
        Data::_handle(pl, dl_meta).instrument(span).await
    }

    async fn _handle(pl: backend::XmitDataReqPayload, dl_meta: backend::DLMetaData) -> Result<()> {
        let mut uplink_rx_info = roaming::dl_meta_data_to_uplink_rx_info(&dl_meta)?;
        uplink_rx_info.sort_by(|a, b| {
            if a.snr == b.snr {
                return a.rssi.partial_cmp(&b.rssi).unwrap();
            }
            b.snr.partial_cmp(&a.snr).unwrap()
        });

        if uplink_rx_info.is_empty() {
            return Err(anyhow!("DLMetaData is not set"));
        }

        let region_config_id = uplink_rx_info[0]
            .metadata
            .get("region_config_id")
            .cloned()
            .unwrap_or_default();

        let mut ctx = Data {
            region_config_id,
            uplink_rx_info,
            xmit_data_req: pl,
            dl_meta_data: dl_meta,
            downlink_frame: gw::DownlinkFrame {
                downlink_id: rand::thread_rng().gen(),
                ..Default::default()
            },
        };

        ctx.set_downlink_frame()?;
        ctx.save_downlink_frame().await?;
        ctx.send_downlink_frame().await?;

        Ok(())
    }

    fn set_downlink_frame(&mut self) -> Result<()> {
        trace!("Setting DownlinkFrame parameters");
        let region_conf = region::get(&self.region_config_id)?;

        let rx_info = self
            .uplink_rx_info
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("rx_info is empty"))?;

        self.downlink_frame.gateway_id = rx_info.gateway_id.clone();
        if self.dl_meta_data.dl_freq_1.is_some()
            && self.dl_meta_data.data_rate_1.is_some()
            && self.dl_meta_data.rx_delay_1.is_some()
        {
            let mut tx_info = gw::DownlinkTxInfo {
                frequency: (self.dl_meta_data.dl_freq_1.unwrap() * 1_000_000.0) as u32,
                board: rx_info.board,
                antenna: rx_info.antenna,
                context: rx_info.context.clone(),
                timing: Some(gw::Timing {
                    parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                        delay: Some(pbjson_types::Duration {
                            seconds: self.dl_meta_data.rx_delay_1.unwrap() as i64,
                            nanos: 0,
                        }),
                    })),
                }),
                ..Default::default()
            };

            tx_info.power = region_conf.get_downlink_tx_power(tx_info.frequency) as i32;

            let rx1_dr = region_conf.get_data_rate(self.dl_meta_data.data_rate_1.unwrap())?;
            helpers::set_tx_info_data_rate(&mut tx_info, &rx1_dr)?;

            self.downlink_frame.items.push(gw::DownlinkFrameItem {
                phy_payload: self.xmit_data_req.phy_payload.clone(),
                tx_info: Some(tx_info),
                tx_info_legacy: None,
            });
        }

        if self.dl_meta_data.dl_freq_2.is_some()
            && self.dl_meta_data.data_rate_2.is_some()
            && self.dl_meta_data.rx_delay_1.is_some()
        {
            let mut tx_info = gw::DownlinkTxInfo {
                frequency: (self.dl_meta_data.dl_freq_2.unwrap() * 1_000_000.0) as u32,
                board: rx_info.board,
                antenna: rx_info.antenna,
                context: rx_info.context,
                timing: Some(gw::Timing {
                    parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                        delay: Some(pbjson_types::Duration {
                            seconds: self.dl_meta_data.rx_delay_1.unwrap() as i64 + 1,
                            nanos: 0,
                        }),
                    })),
                }),
                ..Default::default()
            };

            tx_info.power = region_conf.get_downlink_tx_power(tx_info.frequency) as i32;

            let rx2_dr = region_conf.get_data_rate(self.dl_meta_data.data_rate_2.unwrap())?;
            helpers::set_tx_info_data_rate(&mut tx_info, &rx2_dr)?;

            self.downlink_frame.items.push(gw::DownlinkFrameItem {
                phy_payload: self.xmit_data_req.phy_payload.clone(),
                tx_info: Some(tx_info),
                tx_info_legacy: None,
            });
        }

        Ok(())
    }

    async fn save_downlink_frame(&self) -> Result<()> {
        trace!("Saving downlink frame");

        downlink_frame::save(&internal::DownlinkFrame {
            downlink_id: self.downlink_frame.downlink_id,
            downlink_frame: Some(self.downlink_frame.clone()),
            ..Default::default()
        })
        .await
        .context("Save downlink frame")?;

        Ok(())
    }

    async fn send_downlink_frame(&self) -> Result<()> {
        trace!("Sending downlink frame");

        gateway::backend::send_downlink(&self.region_config_id, &self.downlink_frame)
            .await
            .context("Send downlink frame")?;

        Ok(())
    }
}
