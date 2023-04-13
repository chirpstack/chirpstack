use std::fmt;
use std::time::Duration;

use anyhow::Result;
use chirpstack_api::{common, gw};
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use reqwest::Client;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::gpstime::ToGpsTime;
use crate::uplink::helpers;
use lrwn::EUI64;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No location")]
    NoLocation,

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

pub struct ApiClient {
    uri: String,
    token: String,
    timeout: Duration,
}

impl ApiClient {
    pub fn new(uri: &str, token: &str) -> ApiClient {
        ApiClient {
            uri: uri.to_string(),
            token: token.to_string(),
            timeout: Duration::from_secs(5),
        }
    }

    pub async fn tdoa_single_frame(
        &self,
        rx_info: &[gw::UplinkRxInfo],
    ) -> Result<common::Location> {
        let req = TdoaSingleFrameRequest::new(rx_info);
        let resp = self
            .request("/api/v1/solve/tdoa", &serde_json::to_string(&req)?)
            .await?;
        Ok(resp.into_location(common::LocationSource::GeoResolverTdoa)?)
    }

    pub async fn tdoa_multi_frame(
        &self,
        rx_info: &[Vec<gw::UplinkRxInfo>],
    ) -> Result<common::Location> {
        let req = TdoaMultiFrameRequest::new(rx_info);
        let resp = self
            .request(
                "/api/v1/solve/tdoaMultiframe",
                &serde_json::to_string(&req)?,
            )
            .await?;
        Ok(resp.into_location(common::LocationSource::GeoResolverTdoa)?)
    }

    pub async fn rssi_single_frame(
        &self,
        rx_info: &[gw::UplinkRxInfo],
    ) -> Result<common::Location> {
        let req = RssiSingleFrameRequest::new(rx_info);
        let resp = self
            .request("/api/v2/rssi", &serde_json::to_string(&req)?)
            .await?;
        Ok(resp.into_location(common::LocationSource::GeoResolverRssi)?)
    }

    pub async fn rssi_multi_frame(
        &self,
        rx_info: &[Vec<gw::UplinkRxInfo>],
    ) -> Result<common::Location> {
        let req = RssiMultiFrameRequest::new(rx_info);
        let resp = self
            .request(
                "/api/v1/solve/rssiMultiframe",
                &serde_json::to_string(&req)?,
            )
            .await?;
        Ok(resp.into_location(common::LocationSource::GeoResolverRssi)?)
    }

    pub async fn wifi_tdoa_single_frame(
        &self,
        rx_info: &[gw::UplinkRxInfo],
        aps: &[WifiAccessPoint],
    ) -> Result<common::Location> {
        let req = WifiTdoaSingleFrameRequest::new(rx_info, aps);
        let resp = self
            .request("/api/v1/solve/loraWifi", &serde_json::to_string(&req)?)
            .await?;
        Ok(resp.into_location(common::LocationSource::GeoResolverWifi)?)
    }

    pub async fn gnss_lr1110_single_frame(
        &self,
        rx_info: &[gw::UplinkRxInfo],
        use_rx_time: bool,
        pl: &[u8],
    ) -> Result<common::Location> {
        let req = GnssLr1110SingleFrameRequest::new(rx_info, use_rx_time, pl);
        let resp = self
            .v3_request(
                "/api/v1/solve/gnss_lr1110_singleframe",
                &serde_json::to_string(&req)?,
            )
            .await?;
        Ok(resp.into_location(common::LocationSource::GeoResolverGnss)?)
    }

    pub async fn uplink_send(&self, req: &UplinkRequest) -> Result<UplinkResponse> {
        let endpoint = format!("{}/api/v1/device/send", self.uri);
        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            HeaderName::try_from("Ocp-Apim-Subscription-Key").unwrap(),
            self.token.parse()?,
        );

        let res = client
            .post(endpoint)
            .headers(headers)
            .json(req)
            .send()
            .await?;
        let res = res.error_for_status()?;

        Ok(res.json::<UplinkResponse>().await?)
    }

    async fn request(&self, endpoint: &str, body: &str) -> Result<Response> {
        let endpoint = format!("{}{}", self.uri, endpoint);
        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            HeaderName::try_from("Ocp-Apim-Subscription-Key").unwrap(),
            self.token.parse()?,
        );

        let res = client
            .post(endpoint)
            .body(body.to_string())
            .headers(headers)
            .send()
            .await?;

        let res = res.error_for_status()?;

        Ok(res.json::<Response>().await?)
    }

    async fn v3_request(&self, endpoint: &str, body: &str) -> Result<V3Response> {
        let endpoint = format!("{}{}", self.uri, endpoint);
        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            HeaderName::try_from("Ocp-Apim-Subscription-Key").unwrap(),
            self.token.parse()?,
        );

        let res = client
            .post(endpoint)
            .body(body.to_string())
            .headers(headers)
            .send()
            .await?;

        let res = res.error_for_status()?;
        Ok(res.json::<V3Response>().await?)
    }
}

#[derive(Serialize, Clone)]
pub struct TdoaSingleFrameRequest {
    pub lorawan: Vec<UplinkTdoa>,
}

impl TdoaSingleFrameRequest {
    pub fn new(rx_info: &[gw::UplinkRxInfo]) -> Self {
        TdoaSingleFrameRequest {
            lorawan: rx_info.iter().map(UplinkTdoa::new).collect(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct TdoaMultiFrameRequest {
    pub lorawan: Vec<Vec<UplinkTdoa>>,
}

impl TdoaMultiFrameRequest {
    pub fn new(rx_info: &[Vec<gw::UplinkRxInfo>]) -> Self {
        TdoaMultiFrameRequest {
            lorawan: rx_info
                .iter()
                .map(|i| i.iter().map(UplinkTdoa::new).collect())
                .collect(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct RssiSingleFrameRequest {
    pub lorawan: Vec<UplinkRssi>,
}

impl RssiSingleFrameRequest {
    pub fn new(rx_info: &[gw::UplinkRxInfo]) -> Self {
        RssiSingleFrameRequest {
            lorawan: rx_info.iter().map(UplinkRssi::new).collect(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct RssiMultiFrameRequest {
    pub lorawan: Vec<Vec<UplinkRssi>>,
}

impl RssiMultiFrameRequest {
    pub fn new(rx_info: &[Vec<gw::UplinkRxInfo>]) -> Self {
        RssiMultiFrameRequest {
            lorawan: rx_info
                .iter()
                .map(|i| i.iter().map(UplinkRssi::new).collect())
                .collect(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct WifiTdoaSingleFrameRequest {
    pub lorawan: Vec<UplinkTdoa>,
    #[serde(rename = "wifiAccessPoints")]
    pub wifi_access_points: Vec<WifiAccessPoint>,
}

impl WifiTdoaSingleFrameRequest {
    pub fn new(rx_info: &[gw::UplinkRxInfo], aps: &[WifiAccessPoint]) -> Self {
        WifiTdoaSingleFrameRequest {
            lorawan: rx_info.iter().map(UplinkTdoa::new).collect(),
            wifi_access_points: aps.to_vec(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct GnssLr1110SingleFrameRequest {
    pub payload: String,
    #[serde(rename = "gnss_capture_time", skip_serializing_if = "Option::is_none")]
    pub gnss_capture_time: Option<f64>,
    #[serde(
        rename = "gnss_capture_time_accuracy",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnss_capture_time_accuracy: Option<f64>,
    #[serde(
        rename = "gnss_assist_position",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnss_assist_position: Option<Vec<f64>>,
    #[serde(
        rename = "gnss_assist_altitude",
        skip_serializing_if = "Option::is_none"
    )]
    pub gnss_assist_altitude: Option<f64>,
    #[serde(rename = "gnss_use_2D_solver")]
    pub gnss_use_2d_solver: bool,
}

impl GnssLr1110SingleFrameRequest {
    pub fn new(rx_info: &[gw::UplinkRxInfo], use_rx_time: bool, pl: &[u8]) -> Self {
        GnssLr1110SingleFrameRequest {
            payload: hex::encode(pl),
            gnss_capture_time: match use_rx_time {
                false => None,
                true => match helpers::get_time_since_gps_epoch(rx_info) {
                    Some(v) => Some(v.as_secs_f64()),
                    None => Some(
                        chrono::Utc::now()
                            .to_gps_time()
                            .to_std()
                            .unwrap_or_default()
                            .as_secs_f64(),
                    ),
                },
            },
            gnss_capture_time_accuracy: None,
            gnss_assist_position: helpers::get_start_location(rx_info)
                .map(|loc| vec![loc.latitude, loc.longitude]),
            gnss_assist_altitude: helpers::get_start_location(rx_info).map(|loc| loc.altitude),
            gnss_use_2d_solver: false,
        }
    }
}

#[derive(Default, Deserialize, Clone)]
#[serde(default)]
pub struct Response {
    pub result: Option<LocationResult>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Response {
    fn into_location(self, source: common::LocationSource) -> Result<common::Location, Error> {
        if !self.errors.is_empty() {
            return Err(Error::AnyhowError(anyhow!(
                "api returned errors: {}",
                self.errors.join(", ")
            )));
        }

        if let Some(loc) = &self.result {
            return Ok(common::Location {
                latitude: loc.latitude,
                longitude: loc.longitude,
                altitude: loc.altitude,
                source: source.into(),
                accuracy: loc.accuracy.unwrap_or_default() as f32,
            });
        }

        Err(Error::NoLocation)
    }
}

#[derive(Default, Deserialize, Clone)]
#[serde(default)]
pub struct V3Response {
    pub result: Option<LocationSolverResult>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl V3Response {
    fn into_location(self, source: common::LocationSource) -> Result<common::Location, Error> {
        if !self.errors.is_empty() {
            return Err(Error::AnyhowError(anyhow!(
                "api returned errors: {}",
                self.errors.join(", ")
            )));
        }

        if let Some(loc) = &self.result {
            if loc.llh.len() != 3 {
                return Err(Error::AnyhowError(anyhow!("LLH must contain 3 items")));
            }

            return Ok(common::Location {
                latitude: loc.llh[0],
                longitude: loc.llh[1],
                altitude: loc.llh[2],
                source: source.into(),
                accuracy: loc.accuracy as f32,
            });
        }

        Err(Error::NoLocation)
    }
}

#[derive(Deserialize, Clone)]
pub struct LocationResult {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub accuracy: Option<f64>, // documented as INT, but JSON does (sometimes) contain x.y value. Also, this value can be null.
    #[serde(rename = "algorithmType")]
    pub algorithm_type: String,
    #[serde(rename = "numberOfGatewaysReceived")]
    pub number_of_gateways_received: f64, // documented as INT, but JSON does (sometimes) contain x.y value.
    #[serde(rename = "numberOfGatewaysUsed")]
    pub number_of_gateways_used: f64, // documented as INT, but JSON does (sometimes) contain x.y value.
}

#[derive(Deserialize, Clone)]
pub struct LocationSolverResult {
    pub ecef: Vec<f64>,
    pub llh: Vec<f64>,
    pub gdop: f64,
    pub accuracy: f64,
    #[serde(rename = "capture_time_gps")]
    pub capture_time_gps: f64,
    #[serde(rename = "capture_time_utc")]
    pub capture_time_utc: f64,
}

#[derive(Serialize, Clone)]
pub struct UplinkTdoa {
    #[serde(rename = "gatewayId")]
    pub gateway_id: String,
    pub rssi: f64,
    pub snr: f32,
    pub toa: u32,
    #[serde(rename = "antennaId")]
    pub antenna_id: u32,
    #[serde(rename = "antennaLocation")]
    pub antenna_location: AntennaLocation,
}

impl UplinkTdoa {
    pub fn new(rx_info: &gw::UplinkRxInfo) -> Self {
        UplinkTdoa {
            gateway_id: hex::encode(&rx_info.gateway_id),
            rssi: rx_info.rssi.into(),
            snr: rx_info.snr,
            antenna_id: rx_info.antenna,
            antenna_location: match &rx_info.location {
                Some(loc) => AntennaLocation {
                    latitude: loc.latitude,
                    longitude: loc.longitude,
                    altitude: loc.altitude,
                },
                None => AntennaLocation {
                    latitude: 0.0,
                    longitude: 0.0,
                    altitude: 0.0,
                },
            },
            toa: match &rx_info.fine_time_since_gps_epoch {
                Some(v) => v.nanos as u32,
                None => 0,
            },
        }
    }
}

#[derive(Serialize, Clone)]
pub struct UplinkRssi {
    #[serde(rename = "gatewayId")]
    pub gateway_id: String,
    pub rssi: f64,
    pub snr: f32,
    #[serde(rename = "antennaId")]
    pub antenna_id: u32,
    #[serde(rename = "antennaLocation")]
    pub antenna_location: AntennaLocation,
}

impl UplinkRssi {
    pub fn new(rx_info: &gw::UplinkRxInfo) -> Self {
        UplinkRssi {
            gateway_id: hex::encode(&rx_info.gateway_id),
            rssi: rx_info.rssi.into(),
            snr: rx_info.snr,
            antenna_id: rx_info.antenna,
            antenna_location: match &rx_info.location {
                Some(loc) => AntennaLocation {
                    latitude: loc.latitude,
                    longitude: loc.longitude,
                    altitude: loc.altitude,
                },
                None => AntennaLocation {
                    latitude: 0.0,
                    longitude: 0.0,
                    altitude: 0.0,
                },
            },
        }
    }
}

#[derive(Clone, Serialize, Default)]
pub struct WifiAccessPoint {
    #[serde(rename = "macAddress")]
    pub mac_address: String,
    #[serde(rename = "signalStrength")]
    pub signal_strength: isize,
}

#[derive(Serialize, Clone)]
pub struct AntennaLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[derive(Clone)]
pub enum UplinkMsg {
    UpDf(UplinkMsgUpDf),
    Gnss(UplinkMsgGnss),
    Wifi(UplinkMsgWifi),
    Joining(UplinkMsgJoining),
}

impl Serialize for UplinkMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UplinkMsg::UpDf(v) => v.serialize(serializer),
            UplinkMsg::Gnss(v) => v.serialize(serializer),
            UplinkMsg::Wifi(v) => v.serialize(serializer),
            UplinkMsg::Joining(v) => v.serialize(serializer),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Eui64Wrapper(EUI64);

impl Eui64Wrapper {
    pub fn new(eui64: &EUI64) -> Self {
        Eui64Wrapper(*eui64)
    }
}

impl Serialize for Eui64Wrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut parts: Vec<String> = Vec::new();
        for b in &self.0.to_be_bytes() {
            parts.push(hex::encode(vec![*b]));
        }

        serializer.serialize_str(&parts.join("-"))
    }
}

impl<'de> Deserialize<'de> for Eui64Wrapper {
    fn deserialize<D>(deserialize: D) -> Result<Eui64Wrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(Eui64WrapperVisitor)
    }
}

struct Eui64WrapperVisitor;

impl<'de> Visitor<'de> for Eui64WrapperVisitor {
    type Value = Eui64Wrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An EUI64 in the format of 01-02-03-04-05-06-07-08 is expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let s = value.to_string().replace('-', "");
        let b = hex::decode(s).map_err(|e| E::custom(format!("{}", e)))?;
        let eui64 = EUI64::from_slice(&b).map_err(|e| E::custom(format!("{}", e)))?;
        Ok(Eui64Wrapper(eui64))
    }
}

// UplinkMsgUpDf implements the LoRa Cloud UplinkMsg object.
#[derive(Default, Serialize, Clone)]
pub struct UplinkMsgUpDf {
    #[serde(rename = "msgtype")]
    pub msg_type: String, // must be set to "updf"
    #[serde(rename = "fcnt")]
    pub f_cnt: u32,
    pub port: u8,
    pub dr: u8,
    pub freq: u32,
    pub timestamp: f64, // senconds since UTC
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_capture_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_capture_time_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_assist_position: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_assist_altitude: Option<f64>,
}

// UplinkMsgGnss implements the LoRa Cloud UplinkMsg object containing a gnss payload.
#[derive(Serialize, Clone)]
pub struct UplinkMsgGnss {
    #[serde(rename = "msgtype")]
    pub msg_type: String, // must be set to "GNSS"
    pub payload: String, // HEX format
    pub timestamp: f64,  // seconds since UTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_capture_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_capture_time_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_assist_position: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_assist_altitude: Option<f64>,
}

// UplinkMsgWifi implements the LoRa Cloud UplinkMsg object containing a wifi payload.
#[derive(Serialize, Clone)]
pub struct UplinkMsgWifi {
    #[serde(rename = "msgtype")]
    pub msg_type: String, // must be set to "wifi"
    pub payload: String, // HEX
    pub timestamp: f64,  // seconds since UTC
}

// UplinkMsgJoining implements the LoRa Cloud UplinkMsg object indicating a session reset.
#[derive(Serialize, Clone)]
pub struct UplinkMsgJoining {
    #[serde(rename = "msgtype")]
    pub msg_type: String, // must be set to "joining"
    pub timestamp: f64, // seconds since UTC
}

// UplinkResponse holds the response for a single DevEUI.
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UplinkResponse {
    pub result: UplinkResponseResult,
    pub error: String,
}

// UplinkResponseResult holds the response result.
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UplinkResponseResult {
    pub file: serde_json::Value,
    pub stream_records: Option<StreamUpdate>,
    pub position_solution: Option<PositionSolution>,
    pub fulfilled_requests: serde_json::Value,
    #[serde(rename = "fports")]
    pub f_ports: serde_json::Value,
    pub info_fields: serde_json::Value,
    pub pending_requests: serde_json::Value,
    pub log_messages: serde_json::Value,
    #[serde(rename = "dnlink")]
    pub downlink: Option<LoraDownlink>,
}

// StreamUpdate lists both the signals and the fully-assembled streaming records that are received by the decoder.
// Each entry denotes an assembled packet with application data and record offset.
pub type StreamUpdate = Vec<Vec<serde_json::Value>>;

// LoRaDownlink implements the LoRa Cloud LoRaDownlink object.
#[derive(Serialize, Deserialize, Clone)]
pub struct LoraDownlink {
    pub port: u8,
    pub payload: String, // HEX
}

// PositionSolution implements the Positition Solution object.
#[derive(Serialize, Deserialize, Clone)]
pub struct PositionSolution {
    pub algorithm_type: Option<String>,
    pub ecef: Option<Vec<f64>>,
    pub llh: Vec<f64>,
    pub capture_time_gps: Option<f64>,
    pub gdop: Option<f64>,
    pub accuracy: Option<f32>,
    pub timestamp: f64,
}

// UplinkRequest implements the LoRa Cloud uplink/send request.
#[derive(Serialize, Clone)]
pub struct UplinkRequest {
    #[serde(rename = "deveui")]
    pub dev_eui: Eui64Wrapper,
    pub uplink: UplinkMsg,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_uplink_request_serizalization() {
        let updf = UplinkRequest {
            dev_eui: Eui64Wrapper::new(&EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8])),
            uplink: UplinkMsg::UpDf(UplinkMsgUpDf {
                msg_type: "updf".into(),
                f_cnt: 10,
                port: 2,
                dr: 1,
                freq: 868100000,
                timestamp: 12345.0,
                payload: "".into(),
                ..Default::default()
            }),
        };
        let json_s = serde_json::to_string(&updf).unwrap();

        assert_eq!("{\"deveui\":\"01-02-03-04-05-06-07-08\",\"uplink\":{\"msgtype\":\"updf\",\"fcnt\":10,\"port\":2,\"dr\":1,\"freq\":868100000,\"timestamp\":12345.0,\"payload\":\"\"}}", json_s);
    }
}
