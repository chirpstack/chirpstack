use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use tracing::{info, trace, warn};
use uuid::Uuid;

use super::Integration as IntegrationTrait;
use crate::gpstime::ToGpsTime;
use crate::integration::{integration_event, location_event};
use crate::storage::application::LoraCloudConfiguration;
use crate::storage::device_queue;
use crate::uplink::helpers::{get_start_location, get_time_since_gps_epoch_chrono};
use chirpstack_api::{common, gw, integration};
use lrwn::EUI64;

mod buffer;
mod client;
mod convert;

pub struct Integration {
    client: client::ApiClient,
    config: LoraCloudConfiguration,
}

impl Integration {
    pub fn new(conf: &LoraCloudConfiguration) -> Integration {
        trace!("Initializing LoRa Cloud integration");

        Integration {
            client: client::ApiClient::new(
                "https://mgs.loracloud.com",
                &conf.modem_geolocation_services.token,
            ),
            config: conf.clone(),
        }
    }

    async fn modem_joining(&self, pl: &integration::JoinEvent) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();

        info!(dev_eui = %di.dev_eui, "Forwarding join notification");
        let ts: DateTime<Utc> = pl
            .time
            .as_ref()
            .unwrap()
            .clone()
            .try_into()
            .map_err(anyhow::Error::msg)?;
        let dev_eui = EUI64::from_str(&di.dev_eui)?;

        let pl = client::UplinkRequest {
            dev_eui: client::Eui64Wrapper::new(&dev_eui),
            uplink: client::UplinkMsg::Joining(client::UplinkMsgJoining {
                msg_type: "joining".into(),
                timestamp: ts.timestamp_millis() as f64 / 1000.0,
            }),
        };

        let _ = self.client.uplink_send(&pl).await?;

        Ok(())
    }

    async fn modem_updf(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();

        info!(dev_eui = %di.dev_eui, "Forwarding updf message");
        let ts: DateTime<Utc> = pl
            .time
            .as_ref()
            .unwrap()
            .clone()
            .try_into()
            .map_err(anyhow::Error::msg)?;
        let dev_eui = EUI64::from_str(&di.dev_eui)?;

        let req = client::UplinkRequest {
            dev_eui: client::Eui64Wrapper::new(&dev_eui),
            uplink: client::UplinkMsg::UpDf({
                let mut msg_updf = client::UplinkMsgUpDf {
                    msg_type: "updf".into(),
                    f_cnt: pl.f_cnt,
                    port: pl.f_port as u8,
                    dr: pl.dr as u8,
                    freq: pl.tx_info.as_ref().unwrap().frequency,
                    timestamp: ts.timestamp_millis() as f64 / 1000.0,
                    payload: hex::encode(&pl.data),
                    gnss_capture_time: match self.config.modem_geolocation_services.gnss_use_rx_time
                    {
                        false => None,
                        true => {
                            let ts = match get_time_since_gps_epoch_chrono(&pl.rx_info) {
                                Some(v) => v,
                                None => Utc::now().to_gps_time(),
                            };

                            // Compensate for gnss scanning time and uplink.
                            let ts = ts - Duration::seconds(6);
                            Some(ts.num_seconds() as f64)
                        }
                    },
                    gnss_capture_time_accuracy: match self
                        .config
                        .modem_geolocation_services
                        .gnss_use_rx_time
                    {
                        false => None,
                        true => Some(15.0),
                    },
                    gnss_assist_position: None,
                    gnss_assist_altitude: None,
                };

                if self
                    .config
                    .modem_geolocation_services
                    .gnss_use_gateway_location
                {
                    if let Some(loc) = get_start_location(&pl.rx_info) {
                        msg_updf.gnss_assist_position = Some(vec![loc.latitude, loc.longitude]);
                        msg_updf.gnss_assist_altitude = Some(loc.altitude);
                    }
                }

                msg_updf
            }),
        };

        let resp = self.client.uplink_send(&req).await?;

        self.handle_modem_response(vars, pl, &resp, common::LocationSource::GeoResolverGnss)
            .await?;

        Ok(())
    }

    async fn modem_metadata(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, "Forwarding uplink meta-data");
        let ts: DateTime<Utc> = pl
            .time
            .as_ref()
            .unwrap()
            .clone()
            .try_into()
            .map_err(anyhow::Error::msg)?;
        let dev_eui = EUI64::from_str(&di.dev_eui)?;

        let req = client::UplinkRequest {
            dev_eui: client::Eui64Wrapper::new(&dev_eui),
            uplink: client::UplinkMsg::UpDf(client::UplinkMsgUpDf {
                msg_type: "updf".into(),
                f_cnt: pl.f_cnt,
                port: pl.f_port as u8,
                dr: pl.dr as u8,
                freq: pl.tx_info.as_ref().unwrap().frequency,
                timestamp: ts.timestamp_millis() as f64 / 1000.0,
                payload: "".into(),
                ..Default::default()
            }),
        };

        let resp = self.client.uplink_send(&req).await?;
        self.handle_modem_response(vars, pl, &resp, common::LocationSource::Unknown)
            .await?;

        Ok(())
    }

    #[async_recursion]
    async fn handle_modem_response(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
        resp: &client::UplinkResponse,
        loc_source: common::LocationSource,
    ) -> Result<()> {
        trace!("Handling modem uplink response");
        if !resp.error.is_empty() {
            return Err(anyhow!("{}", resp.error));
        }

        self.handle_response_integration_event(vars, pl, &resp.result)
            .await?;

        if self.config.modem_geolocation_services.parse_tlv && resp.result.stream_records.is_some()
        {
            self.handle_response_tlv_records(
                vars,
                pl,
                resp.result.stream_records.as_ref().unwrap(),
            )
            .await?;
        }

        if let Some(v) = &resp.result.downlink {
            self.handle_response_downlink(pl, v).await?;
        }

        if let Some(v) = &resp.result.position_solution {
            self.handle_response_position(vars, pl, v, loc_source)
                .await?;
        }

        Ok(())
    }

    async fn handle_response_tlv_records(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
        stream: &[Vec<serde_json::Value>],
    ) -> Result<()> {
        trace!("Handling TLV records");

        let mut payloads: Vec<Vec<u8>> = Vec::new();

        // Parse all payloads from the stream.
        for record in stream {
            // Sanity check, as 0 = index, 1 = payload
            if record.len() != 2 {
                continue;
            }

            if let serde_json::Value::String(record_pl) = &record[1] {
                let record_b = hex::decode(record_pl).context("Decode stream payload")?;
                payloads.push(record_b);
            }
        }

        let di = pl.device_info.as_ref().unwrap();
        let ts: DateTime<Utc> = pl
            .time
            .as_ref()
            .unwrap()
            .clone()
            .try_into()
            .map_err(anyhow::Error::msg)?;
        let dev_eui = EUI64::from_str(&di.dev_eui)?;

        for p in &payloads {
            let mut index = 0;
            // There must be at least 2 bytes to read (tag + length)
            while p.len() - index >= 2 {
                // Tag
                let t = p[index];
                // Length
                let l = p[index + 1] as usize;

                // Validate that we can at least read 'l' data
                if p.len() - index - 2 < l {
                    return Err(anyhow!("Invalid TLV record"));
                }

                // Get v
                let v = &p[index + 2..index + 2 + l];

                // increment index (2 bytes for t and l bytes + length of v)
                index = index + 2 + l;

                match t {
                    // GNSS
                    0x06 | 0x07 => {
                        let mut msg_gnss = client::UplinkMsgGnss {
                            msg_type: "gnss".into(),
                            payload: hex::encode(v),
                            timestamp: ts.timestamp_millis() as f64 / 1000.0,
                            gnss_capture_time: None,
                            gnss_capture_time_accuracy: None,
                            gnss_assist_position: None,
                            gnss_assist_altitude: None,
                        };

                        // Note: we must rely on the embedded gnss timestamp, as the frame
                        // is de-fragmented and we can not assume the scan time from the
                        // rx timestamp.

                        if let Some(loc) = get_start_location(&pl.rx_info) {
                            msg_gnss.gnss_assist_position = Some(vec![loc.latitude, loc.longitude]);
                            msg_gnss.gnss_assist_altitude = Some(loc.altitude);
                        }

                        let req = client::UplinkRequest {
                            dev_eui: client::Eui64Wrapper::new(&dev_eui),
                            uplink: client::UplinkMsg::Gnss(msg_gnss),
                        };
                        let resp = self.client.uplink_send(&req).await?;
                        self.handle_modem_response(
                            vars,
                            pl,
                            &resp,
                            common::LocationSource::GeoResolverGnss,
                        )
                        .await?;
                    }
                    // Wif (legacy)
                    0x08 => {
                        let mut vv = vec![0x01];
                        vv.extend_from_slice(v);

                        let req = client::UplinkRequest {
                            dev_eui: client::Eui64Wrapper::new(&dev_eui),
                            uplink: client::UplinkMsg::Wifi(client::UplinkMsgWifi {
                                msg_type: "wifi".into(),
                                payload: hex::encode(vv),
                                timestamp: ts.timestamp_millis() as f64 / 1000.0,
                            }),
                        };
                        let resp = self.client.uplink_send(&req).await?;
                        self.handle_modem_response(
                            vars,
                            pl,
                            &resp,
                            common::LocationSource::GeoResolverWifi,
                        )
                        .await?;
                    }
                    // Wifi
                    0x0e => {
                        // we have to skip first 5 bytes
                        if v.len() < 5 {
                            continue;
                        }

                        let mut vv = vec![0x01];
                        vv.extend_from_slice(&v[5..]);

                        let req = client::UplinkRequest {
                            dev_eui: client::Eui64Wrapper::new(&dev_eui),
                            uplink: client::UplinkMsg::Wifi(client::UplinkMsgWifi {
                                msg_type: "wifi".into(),
                                payload: hex::encode(vv),
                                timestamp: ts.timestamp_millis() as f64 / 1000.0,
                            }),
                        };
                        let resp = self.client.uplink_send(&req).await?;
                        self.handle_modem_response(
                            vars,
                            pl,
                            &resp,
                            common::LocationSource::GeoResolverWifi,
                        )
                        .await?;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_response_integration_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
        result: &client::UplinkResponseResult,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        trace!(dev_eui = %di.dev_eui, "Handling response integration event");

        let int_pl = integration::IntegrationEvent {
            deduplication_id: pl.deduplication_id.clone(),
            device_info: pl.device_info.clone(),
            time: Some(Utc::now().into()),
            integration_name: "loracloud".into(),
            event_type: "modem_UplinkResponse".into(),
            object: Some(convert::serde_json_to_pb_json(&serde_json::to_value(
                result,
            )?)),
        };

        integration_event(Uuid::from_str(&di.application_id)?, vars, &int_pl).await;
        Ok(())
    }

    async fn handle_response_downlink(
        &self,
        pl: &integration::UplinkEvent,
        result: &client::LoraDownlink,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();

        trace!(dev_eui = %di.dev_eui, "Handling downlink enqueue");
        let dev_eui = EUI64::from_str(&di.dev_eui)?;

        let _ = device_queue::enqueue_item(device_queue::DeviceQueueItem {
            dev_eui,
            f_port: match result.port {
                0 => 150,
                _ => result.port,
            } as i16,
            data: hex::decode(&result.payload)?,
            ..Default::default()
        })
        .await?;

        Ok(())
    }

    async fn handle_response_position(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
        result: &client::PositionSolution,
        source: common::LocationSource,
    ) -> Result<()> {
        if result.llh.len() != 3 {
            return Err(anyhow!("llh must contain exactly 3 items"));
        }

        let di = pl.device_info.as_ref().unwrap();

        let loc_pl = integration::LocationEvent {
            deduplication_id: pl.deduplication_id.clone(),
            device_info: pl.device_info.clone(),
            time: Some(Utc::now().into()),
            location: Some(common::Location {
                latitude: result.llh[0],
                longitude: result.llh[1],
                altitude: result.llh[2],
                source: source.into(),
                accuracy: result.accuracy.unwrap_or_default(),
            }),
        };

        location_event(Uuid::from_str(&di.application_id)?, vars, &loc_pl).await;
        Ok(())
    }

    async fn update_geoloc_buffer(
        &self,
        pl: &integration::UplinkEvent,
    ) -> Result<Vec<Vec<gw::UplinkRxInfo>>> {
        // Do not trigger geolocation if there are less than 3 gateways.
        if pl.rx_info.len() < 3 {
            return Ok(Vec::new());
        }

        let di = pl.device_info.as_ref().unwrap();
        let dev_eui = EUI64::from_str(&di.dev_eui)?;
        let ttl = Duration::seconds(
            self.config
                .modem_geolocation_services
                .geolocation_buffer_ttl as i64,
        );

        let mut buf = vec![pl.rx_info.clone()];
        buf.extend_from_slice(&buffer::get_geoloc_buffer(&dev_eui, &ttl).await?);
        buf.truncate(
            (self
                .config
                .modem_geolocation_services
                .geolocation_min_buffer_size
                + 1) as usize, // buffer + current uplink
        );

        buffer::save_geoloc_buffer(&dev_eui, &ttl, &buf).await?;

        Ok(buf)
    }

    async fn handle_geolocation(
        &self,
        pl: &integration::UplinkEvent,
        buffer: &[Vec<gw::UplinkRxInfo>],
    ) -> Result<Option<common::Location>> {
        if self.config.modem_geolocation_services.geolocation_gnss {
            let loc = self.handle_geolocation_gnss(pl).await?;
            if loc.is_some() {
                return Ok(loc);
            }
        }

        if self.config.modem_geolocation_services.geolocation_wifi {
            let loc = self.handle_geolocation_wifi(pl).await?;
            if loc.is_some() {
                return Ok(loc);
            }
        }

        if self.config.modem_geolocation_services.geolocation_tdoa {
            let loc = self.handle_geolocation_tdoa(pl, buffer).await?;
            if loc.is_some() {
                return Ok(loc);
            }
        }

        if self.config.modem_geolocation_services.geolocation_rssi {
            let loc = self.handle_geolocation_rssi(pl, buffer).await?;
            if loc.is_some() {
                return Ok(loc);
            }
        }

        Ok(None)
    }

    async fn handle_geolocation_gnss(
        &self,
        pl: &integration::UplinkEvent,
    ) -> Result<Option<common::Location>> {
        let di = pl.device_info.as_ref().unwrap();
        trace!(dev_eui = %di.dev_eui, "Trying GNSS geolocation");

        let gnss_pl_str: String = match &pl.object {
            None => {
                // object is not set
                return Ok(None);
            }
            Some(v) => {
                // retrieve gnss payload field.
                let field = match v.fields.get(
                    &self
                        .config
                        .modem_geolocation_services
                        .geolocation_gnss_payload_field,
                ) {
                    None => {
                        // object does not contain gnss payload field
                        return Ok(None);
                    }
                    Some(v) => v,
                };

                // Try to cast the field to String.
                if let Some(pbjson_types::value::Kind::StringValue(v)) = &field.kind {
                    v.to_string()
                } else {
                    return Ok(None);
                }
            }
        };

        let b = hex::decode(&gnss_pl_str).context("Decode GNSS payload field")?;
        let resp = self
            .client
            .gnss_lr1110_single_frame(
                &pl.rx_info,
                self.config
                    .modem_geolocation_services
                    .geolocation_gnss_use_rx_time,
                &b,
            )
            .await;

        match resp {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                warn!(error = %e, "GNSS geolocation failed");
                Ok(None)
            }
        }
    }

    async fn handle_geolocation_wifi(
        &self,
        pl: &integration::UplinkEvent,
    ) -> Result<Option<common::Location>> {
        let di = pl.device_info.as_ref().unwrap();
        trace!(dev_eui = %di.dev_eui, "Trying wifi geolocation");

        let wifi_aps: Vec<client::WifiAccessPoint> = match &pl.object {
            None => {
                // object is not set
                return Ok(None);
            }
            Some(v) => {
                // retrieve wifi payload field.
                let field = match v.fields.get(
                    &self
                        .config
                        .modem_geolocation_services
                        .geolocation_wifi_payload_field,
                ) {
                    None => {
                        // object does not contain ifi payload field.
                        return Ok(None);
                    }
                    Some(v) => v,
                };

                // Try to cast the field to ListValue.
                let ap_list: &Vec<pbjson_types::Value> =
                    if let Some(pbjson_types::value::Kind::ListValue(v)) = &field.kind {
                        &v.values
                    } else {
                        return Ok(None);
                    };

                // Cast ListValue to Vec<WifiAccessPoint>
                let ap_list: Vec<client::WifiAccessPoint> = ap_list
                    .iter()
                    .map(|v| {
                        if let Some(pbjson_types::value::Kind::StructValue(v)) = &v.kind {
                            let mut ap: client::WifiAccessPoint = Default::default();

                            if let Some(field) = v.fields.get("macAddress") {
                                if let Some(pbjson_types::value::Kind::StringValue(mac)) =
                                    &field.kind
                                {
                                    ap.mac_address = mac.to_string();
                                }
                            }

                            if let Some(field) = v.fields.get("signalStrength") {
                                if let Some(pbjson_types::value::Kind::NumberValue(sig)) =
                                    &field.kind
                                {
                                    ap.signal_strength = *sig as isize;
                                }
                            }

                            ap
                        } else {
                            Default::default()
                        }
                    })
                    .collect();

                ap_list
            }
        };

        let resp = self
            .client
            .wifi_tdoa_single_frame(&pl.rx_info, &wifi_aps)
            .await;
        match resp {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                warn!(error = %e, "Wifi geolocation failed");
                Ok(None)
            }
        }
    }

    async fn handle_geolocation_tdoa(
        &self,
        pl: &integration::UplinkEvent,
        buffer: &[Vec<gw::UplinkRxInfo>],
    ) -> Result<Option<common::Location>> {
        let di = pl.device_info.as_ref().unwrap();
        trace!(dev_eui = %di.dev_eui, "Trying TDOA geolocation");

        let resp = if buffer.len() == 1 {
            self.client.tdoa_single_frame(&buffer[0]).await
        } else {
            self.client.tdoa_multi_frame(buffer).await
        };

        match resp {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                warn!(error = %e, "TDOA geolocation failed");
                Ok(None)
            }
        }
    }

    async fn handle_geolocation_rssi(
        &self,
        pl: &integration::UplinkEvent,
        buffer: &[Vec<gw::UplinkRxInfo>],
    ) -> Result<Option<common::Location>> {
        let di = pl.device_info.as_ref().unwrap();
        trace!(dev_eui = %di.dev_eui, "Trying RSSI geolocation");

        let resp = if buffer.len() == 1 {
            self.client.rssi_single_frame(&buffer[0]).await
        } else {
            self.client.rssi_multi_frame(buffer).await
        };

        match resp {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                warn!(error = %e, "RSSI geolocation failed");
                Ok(None)
            }
        }
    }
}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        if self.config.modem_geolocation_services.modem_enabled {
            if self
                .config
                .modem_geolocation_services
                .forward_f_ports
                .contains(&pl.f_port)
            {
                self.modem_updf(vars, pl).await?;
            } else {
                // Only forward meta-data.
                self.modem_metadata(vars, pl).await?;
            }
        }

        // In case of TDOA or RSSI, add the rx-info to the geolocation buffer.
        let geoloc_buffer: Vec<Vec<gw::UplinkRxInfo>> =
            if self.config.modem_geolocation_services.geolocation_tdoa
                || self.config.modem_geolocation_services.geolocation_rssi
            {
                self.update_geoloc_buffer(pl).await?
            } else {
                Vec::new()
            };

        // In case geolocation is disabled, this returns None.
        let loc = self.handle_geolocation(pl, &geoloc_buffer).await?;
        if let Some(v) = loc {
            let di = pl.device_info.as_ref().unwrap();
            let loc_pl = integration::LocationEvent {
                deduplication_id: pl.deduplication_id.clone(),
                time: Some(Utc::now().into()),
                device_info: pl.device_info.clone(),
                location: Some(v),
            };

            location_event(Uuid::from_str(&di.application_id)?, vars, &loc_pl).await;
        }

        Ok(())
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        if self.config.modem_geolocation_services.modem_enabled {
            self.modem_joining(pl).await?;
        }

        Ok(())
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::AckEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::TxAckEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::LogEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::StatusEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::LocationEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        Ok(())
    }
}
