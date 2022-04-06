#[macro_use]
extern crate anyhow;

use std::fs::File;
use std::io::Read;

use aes_kw::Kek;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Certificate, Identity};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::trace;

const PROTOCOL_VERSION: &str = "1.0";

pub trait BasePayloadProvider {
    fn base_payload(&self) -> &BasePayload;
}

pub struct ClientConfig {
    pub sender_id: String,
    pub receiver_id: String,
    pub server: String,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,

    // Contains the value for the Authorization header. This may
    // include a prefix, like Bearer, Key or Basic.
    pub authorization: Option<String>,

    // Holds the optional Redis database client. When set the client
    // will use the aysnc protocol scheme. In this case the client will wait
    // AsyncTimeout before returning a timeout error.
    // pub redis_client: Option<Box<dyn redis::aio::ConnectionLike>>,

    // AsyncTimeout defines the async timeout. This must be set when RedisClient
    // is set.
    pub async_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            sender_id: "".into(),
            receiver_id: "".into(),
            server: "".into(),
            ca_cert: "".into(),
            tls_cert: "".into(),
            tls_key: "".into(),
            authorization: None,
            async_timeout: Duration::zero(),
        }
    }
}

pub struct Client {
    client: reqwest::Client,
    config: ClientConfig,
    headers: HeaderMap,
}

impl Client {
    pub fn new(c: ClientConfig) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        if let Some(auth) = &c.authorization {
            headers.insert(AUTHORIZATION, auth.clone().parse()?);
        }

        let mut client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .use_rustls_tls(); // this is important as else the client-certificate does not work!

        if !c.tls_cert.is_empty() && !c.tls_key.is_empty() {
            trace!(tls_cert = %c.tls_cert, tls_key = %c.tls_key, "Reading client certificate");

            let mut b: Vec<u8> = Vec::new();
            File::open(&c.tls_key)
                .context("Open tls_key")?
                .read_to_end(&mut b)
                .context("Read tls_key")?;
            File::open(&c.tls_cert)
                .context("Open tls_cert")?
                .read_to_end(&mut b)
                .context("Read tls_cert")?;

            trace!("Parsing client certificate");
            let id = Identity::from_pem(&b).context("Parse tls_cert and tls_key")?;

            trace!("Adding client certificate as identity");
            client = client.identity(id);
        } else {
            trace!("No client certificate configured");
        }

        if !c.ca_cert.is_empty() {
            trace!(ca_cert = %c.ca_cert, "Reading CA certificate");
            let mut b: Vec<u8> = Vec::new();
            File::open(&c.ca_cert)
                .context("Open ca_cert")?
                .read_to_end(&mut b)
                .context("Read ca_cert")?;

            trace!("Parsing CA certificate");
            let cert = Certificate::from_pem(&b).context("Parse ca_cert")?;

            trace!("Adding CA certificate to root certificate bundle");
            client = client.add_root_certificate(cert);
        } else {
            trace!("No CA certificate configured");
        }

        Ok(Client {
            config: c,
            client: client.build()?,
            headers,
        })
    }

    pub fn get_sender_id(&self) -> String {
        self.config.sender_id.clone()
    }

    pub fn get_receiver_id(&self) -> String {
        self.config.receiver_id.clone()
    }

    pub fn is_async(&self) -> bool {
        false
    }

    pub fn get_random_transaction_id(&self) -> u32 {
        rand::random()
    }

    pub async fn join_req(&self, pl: &mut JoinReqPayload) -> Result<JoinAnsPayload> {
        pl.base.protocol_version = PROTOCOL_VERSION.to_string();
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::JoinReq;
        if pl.base.transaction_id == 0 {
            pl.base.transaction_id = self.get_random_transaction_id();
        }

        let mut ans: JoinAnsPayload = Default::default();
        self.request(&pl, &mut ans).await?;

        Ok(ans)
    }

    async fn request<S, D>(&self, pl: &S, ans: &mut D) -> Result<()>
    where
        S: ?Sized + serde::ser::Serialize + BasePayloadProvider,
        D: serde::de::DeserializeOwned,
    {
        let base = pl.base_payload();
        let _key = self.get_async_key(base.transaction_id);

        let (resp_tx, mut resp_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) =
            mpsc::channel(1);
        let (_err_tx, mut err_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) =
            mpsc::channel(1);

        // TODO: implement async

        let res = self
            .client
            .post(&self.config.server)
            .headers(self.headers.clone())
            .json(pl)
            .send()
            .await?
            .error_for_status()?;

        if !self.is_async() {
            resp_tx.send(res.text().await?).await?;
        }

        tokio::select! {
            err = err_rx.recv() => {
                if let Some(err) = err {
                return Err(anyhow!("{}", err));
                }
            },
            v = resp_rx.recv() => {
                if let Some(v) = v {
                *ans = serde_json::from_str(&v)?;
                }
            },
        }

        Ok(())
    }

    fn get_async_key(&self, transaction_id: u32) -> String {
        format!("backend:async:{}", transaction_id)
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    JoinReq,
    JoinAns,
    RejoinReq,
    RejoinAns,
    AppSKeyReq,
    AppSKeyAns,
    PRStartReq,
    PRStartAns,
    PRStopReq,
    PRStopAns,
    HomeNSReq,
    HomeNSAns,
    XmitDataReq,
    XmitDataAns,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::JoinReq
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResultCode {
    Success,
    MICFailed,
    JoinReqFailed,
    NoRoamingAgreement,
    DevRoamingDisallowed,
    RoamingActDisallowed,
    ActivationDisallowed,
    UnknownDevEUI,
    UnknownDevAddr,
    UnknownSender,
    UnknownReceiver,
    Deferred,
    XmitFailed,
    InvalidFPort,
    InvalidProtocolVersion,
    StaleDeviceProfile,
    MalformedRequest,
    FrameSizeError,
    Other,
}

impl Default for ResultCode {
    fn default() -> Self {
        ResultCode::Success
    }
}

#[derive(Serialize, Deserialize)]
pub enum RatePolicy {
    Drop,
    Mark,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BasePayload {
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "SenderID")]
    pub sender_id: String,
    #[serde(rename = "ReceiverID")]
    pub receiver_id: String,
    #[serde(rename = "TransactionID")]
    pub transaction_id: u32,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "SenderToken", with = "hex_encode")]
    pub sender_token: Vec<u8>,
    #[serde(rename = "ReceiverToken", with = "hex_encode")]
    pub receiver_token: Vec<u8>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BasePayloadResult {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "Result")]
    pub result: ResultPayload,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ResultPayload {
    #[serde(rename = "ResultCode")]
    pub result_code: ResultCode,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KeyEnvelope {
    #[serde(rename = "KEKLabel")]
    pub kek_label: String,
    #[serde(rename = "AESKey", with = "hex_encode")]
    pub aes_key: Vec<u8>,
}

impl KeyEnvelope {
    pub fn new(label: &str, kek: Option<&[u8; 16]>, key: &[u8; 16]) -> Result<Self> {
        if label.is_empty() || kek.is_none() {
            return Ok(KeyEnvelope {
                kek_label: "".into(),
                aes_key: key.to_vec(),
            });
        }

        let kek = Kek::from(*kek.unwrap());
        let mut cipher: Vec<u8> = vec![0; 16 + 8];
        kek.wrap(key, &mut cipher)
            .map_err(|e| anyhow!("KEK wrap failed: {}", e))?;

        Ok(KeyEnvelope {
            kek_label: label.to_string(),
            aes_key: cipher,
        })
    }

    pub fn unwrap(&self, kek: &[u8; 16]) -> Result<[u8; 16]> {
        let kek = Kek::from(*kek);
        let mut out: [u8; 16] = [0; 16];
        kek.unwrap(&self.aes_key, &mut out)
            .map_err(|e| anyhow!("KEK unwrap failed: {}", e))?;
        Ok(out)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct JoinReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "MACVersion")]
    pub mac_version: String,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "DevAddr", with = "hex_encode")]
    pub dev_addr: Vec<u8>,
    #[serde(rename = "DLSettings", with = "hex_encode")]
    pub dl_settings: Vec<u8>,
    #[serde(rename = "RxDelay")]
    pub rx_delay: u8,
    #[serde(rename = "CFList", with = "hex_encode")]
    pub cf_list: Vec<u8>,
}

impl BasePayloadProvider for &mut JoinReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct JoinAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "Lifetime")]
    pub lifetime: Option<usize>,
    #[serde(rename = "SNwkSIntKey")]
    pub s_nwk_s_int_key: Option<KeyEnvelope>,
    #[serde(rename = "FNwkSIntKey")]
    pub f_nwk_s_int_key: Option<KeyEnvelope>,
    #[serde(rename = "NwkSEncKey")]
    pub nwk_s_enc_key: Option<KeyEnvelope>,
    #[serde(rename = "NwkSKey")]
    pub nwk_s_key: Option<KeyEnvelope>,
    #[serde(rename = "AppSKey")]
    pub app_s_key: Option<KeyEnvelope>,
    #[serde(rename = "SessionKeyID", with = "hex_encode")]
    pub session_key_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct RejoinReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "MACVersion")]
    pub mac_version: String,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "DevAddr", with = "hex_encode")]
    pub dev_addr: Vec<u8>,
    #[serde(rename = "DLSettings", with = "hex_encode")]
    pub dl_settings: Vec<u8>,
    #[serde(rename = "RxDelay")]
    pub rx_delay: u8,
    #[serde(rename = "CFList", with = "hex_encode")]
    pub cf_list: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct RejoinAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "Lifetime")]
    pub lifetime: Option<usize>,
    #[serde(rename = "SNwkSIntKey")]
    pub s_nwk_s_int_key: Option<KeyEnvelope>,
    #[serde(rename = "FNwkSIntKey")]
    pub f_nwk_s_int_key: Option<KeyEnvelope>,
    #[serde(rename = "NwkSEncKey")]
    pub nwk_s_enc_key: Option<KeyEnvelope>,
    #[serde(rename = "NwkSKey")]
    pub nwk_s_key: Option<KeyEnvelope>,
    #[serde(rename = "AppSKey")]
    pub app_s_key: Option<KeyEnvelope>,
    #[serde(rename = "SessionKeyID", with = "hex_encode")]
    pub session_key_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct AppSKeyReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "SessionKeyID", with = "hex_encode")]
    pub session_key_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct AppSKeyAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "AppSKey")]
    pub app_s_key: Option<KeyEnvelope>,
    #[serde(rename = "SessionKeyID", with = "hex_encode")]
    pub session_key_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PRStartReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "ULMetaData")]
    pub ul_meta_data: ULMetaData,
}

#[derive(Serialize, Deserialize)]
pub struct PRStartAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "Lifetime")]
    pub lifetime: Option<usize>,
    #[serde(rename = "FNwkSIntKey")]
    pub f_nwk_s_int_key: Option<KeyEnvelope>,
    #[serde(rename = "NwkSKey")]
    pub nwk_s_key: Option<KeyEnvelope>,
    #[serde(rename = "FCntUp")]
    pub f_cnt_up: Option<u32>,
    #[serde(rename = "ServiceProfile")]
    pub service_profile: Option<ServiceProfile>,
    #[serde(rename = "DLMetaData")]
    pub dl_meta_data: Option<DLMetaData>,
    #[serde(rename = "DevAddr", with = "hex_encode")]
    pub dev_addr: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PRStopReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "Lifetime")]
    pub lifetime: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct PRStopAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
}

#[derive(Serialize, Deserialize)]
pub struct XmitDataReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "FRMPayload", with = "hex_encode")]
    pub frm_payload: Vec<u8>,
    #[serde(rename = "ULMetaData")]
    pub ul_meta_data: Option<ULMetaData>,
    #[serde(rename = "DLMetaData")]
    pub dl_meta_data: Option<DLMetaData>,
}

#[derive(Serialize, Deserialize)]
pub struct HomeNSReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct HomeNSAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(rename = "HNetID", with = "hex_encode")]
    pub h_net_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct XmitDataAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
}

#[derive(Serialize, Deserialize)]
pub struct ULMetaData {
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "DevAddr", with = "hex_encode")]
    pub dev_addr: Vec<u8>,
    #[serde(rename = "FPort")]
    pub f_port: Option<u8>,
    #[serde(rename = "FCntDown")]
    pub f_cnt_down: Option<u32>,
    #[serde(rename = "FCntUp")]
    pub f_cnt_up: Option<u32>,
    #[serde(rename = "Confirmed")]
    pub confirmed: bool,
    #[serde(rename = "DataRate")]
    pub data_rate: Option<usize>,
    #[serde(rename = "ULFreq")]
    pub ul_freq: Option<f64>,
    #[serde(rename = "Margin")]
    pub margin: Option<isize>,
    #[serde(rename = "Battery")]
    pub battery: Option<isize>,
    #[serde(rename = "FNSULToken", with = "hex_encode")]
    pub f_ns_ul_token: Vec<u8>,
    #[serde(rename = "RecvTime")]
    pub recv_time: DateTime<Utc>,
    #[serde(rename = "RFRegion")]
    pub rf_region: String,
    #[serde(rename = "GWCnt")]
    pub gw_cnt: Option<usize>,
    #[serde(rename = "GWInfoElement")]
    pub gw_info_element: Vec<GWInfoElement>,
}

#[derive(Serialize, Deserialize)]
pub struct GWInfoElement {
    #[serde(rename = "ID", with = "hex_encode")]
    pub id: Vec<u8>,
    #[serde(rename = "FineRecvTime")]
    pub fine_recv_time: Option<usize>,
    #[serde(rename = "RFRegion")]
    pub rf_region: String,
    #[serde(rename = "RSSI")]
    pub rssi: Option<isize>,
    #[serde(rename = "SNR")]
    pub snr: Option<f64>,
    #[serde(rename = "Lat")]
    pub lat: Option<f64>,
    #[serde(rename = "Lon")]
    pub lon: Option<f64>,
    #[serde(rename = "ULToken", with = "hex_encode")]
    pub ul_token: Vec<u8>,
    #[serde(rename = "DLAllowed")]
    pub dl_allowed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceProfile {
    #[serde(rename = "ServiceProfile")]
    pub service_profile_id: String,
    #[serde(rename = "ULRate")]
    pub ul_rate: usize,
    #[serde(rename = "ULBucketSize")]
    pub ul_bucket_size: usize,
    #[serde(rename = "ULRatePolicy")]
    pub ul_rate_policy: RatePolicy,
    #[serde(rename = "DLRate")]
    pub dl_rate: usize,
    #[serde(rename = "DLBucketSize")]
    pub dl_bucket_size: usize,
    #[serde(rename = "DLRatePolicy")]
    pub dl_rate_policy: RatePolicy,
    #[serde(rename = "AddGWMetadata")]
    pub add_gw_metadata: bool,
    #[serde(rename = "DevStatusReqFreq")]
    pub dev_status_req_freq: usize,
    #[serde(rename = "ReportDevStatusBatery")]
    pub report_dev_status_battery: bool,
    #[serde(rename = "ReportDevStatusMargin")]
    pub report_dev_status_margin: bool,
    #[serde(rename = "DRMin")]
    pub dr_min: usize,
    #[serde(rename = "DRMax")]
    pub dr_mac: usize,
    #[serde(rename = "ChannelMask", with = "hex_encode")]
    pub channel_mask: Vec<u8>,
    #[serde(rename = "PRAllowed")]
    pub pr_allowed: bool,
    #[serde(rename = "HRAllowed")]
    pub hr_allowed: bool,
    #[serde(rename = "RAAllowed")]
    pub ra_allowed: bool,
    #[serde(rename = "NwkGeoLoc")]
    pub nwk_geo_loc: bool,
    #[serde(rename = "TargetPER")]
    pub target_per: f32,
    #[serde(rename = "MinGWDiversity")]
    pub min_gw_diversity: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DLMetaData {
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "FPort")]
    pub f_port: Option<u8>,
    #[serde(rename = "FCntDown")]
    pub f_cnt_down: Option<u32>,
    #[serde(rename = "Confirmed")]
    pub confirmed: bool,
    #[serde(rename = "DLFreq1")]
    pub dl_freq_1: Option<f64>,
    #[serde(rename = "DLFreq2")]
    pub dl_freq_2: Option<f64>,
    #[serde(rename = "RXDelay1")]
    pub rx_delay_1: Option<usize>,
    #[serde(rename = "ClassMode")]
    pub class_mode: Option<String>,
    #[serde(rename = "DataRate1")]
    pub data_rate_1: Option<usize>,
    #[serde(rename = "DataRate2")]
    pub data_rate_2: Option<usize>,
    #[serde(rename = "FNSULToken", with = "hex_encode")]
    pub f_ns_ul_token: Vec<u8>,
    #[serde(rename = "GWInfo")]
    pub gw_info: Vec<GWInfoElement>,
    #[serde(rename = "HiPriorityFlag")]
    pub hi_priority_flag: bool,
}

mod hex_encode {
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(b: &[u8], serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(b))
    }

    pub fn deserialize<'a, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_key_envelope() {
        let key: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8];
        let kek: [u8; 16] = [8, 7, 6, 5, 4, 3, 2, 1, 8, 7, 6, 5, 4, 3, 2, 1];

        // no wrapping
        let ke = KeyEnvelope::new("", None, &key).unwrap();
        assert_eq!(key.to_vec(), ke.aes_key);
        assert_eq!("", ke.kek_label);

        // wrapping
        let ke = KeyEnvelope::new("test-kek", Some(&kek), &key).unwrap();
        assert_eq!(
            vec![
                0xe3, 0xd5, 0xa4, 0x7b, 0xa2, 0x5c, 0xbe, 0x6e, 0x5d, 0xa8, 0x20, 0x84, 0x6e, 0xc,
                0xb6, 0xa8, 0x2b, 0x75, 0xc, 0x59, 0xd8, 0x48, 0xec, 0x7a
            ],
            ke.aes_key
        );
        assert_eq!("test-kek", ke.kek_label);
        assert_eq!(key, ke.unwrap(&kek).unwrap());
    }
}
