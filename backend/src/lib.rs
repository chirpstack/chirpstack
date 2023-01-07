#[macro_use]
extern crate anyhow;

use std::fs::File;
use std::io::Read;
use std::time::Duration;

use aes_kw::Kek;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Certificate, Identity};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Receiver;
use tracing::{debug, error, info, trace};

const PROTOCOL_VERSION: &str = "1.0";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    FNS,
    HNS,
    SNS,
}

pub trait BasePayloadProvider {
    fn base_payload(&self) -> &BasePayload;
}

pub trait BasePayloadResultProvider {
    fn base_payload(&self) -> &BasePayloadResult;
}

pub struct ClientConfig {
    pub sender_id: Vec<u8>,
    pub receiver_id: Vec<u8>,
    pub server: String,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,

    // Contains the value for the Authorization header. This may
    // include a prefix, like Bearer, Key or Basic.
    pub authorization: Option<String>,

    // AsyncTimeout defines the async timeout. This must be set when RedisClient
    // is set.
    pub async_timeout: Duration,

    // Use target-role URL suffix (e.g. /fns, /sns, ...).
    pub use_target_role_suffix: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            sender_id: vec![],
            receiver_id: vec![],
            server: "".into(),
            ca_cert: "".into(),
            tls_cert: "".into(),
            tls_key: "".into(),
            authorization: None,
            async_timeout: Duration::from_secs(0),
            use_target_role_suffix: false,
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

    pub fn get_sender_id(&self) -> Vec<u8> {
        self.config.sender_id.clone()
    }

    pub fn get_receiver_id(&self) -> Vec<u8> {
        self.config.receiver_id.clone()
    }

    pub fn is_async(&self) -> bool {
        !self.config.async_timeout.is_zero()
    }

    pub fn get_async_timeout(&self) -> Duration {
        self.config.async_timeout
    }

    pub async fn join_req(
        &self,
        pl: &mut JoinReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<JoinAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::JoinReq;

        let mut ans: JoinAnsPayload = Default::default();
        self.request(None, &pl, &mut ans, async_resp).await?;
        Ok(ans)
    }

    pub async fn rejoin_req(
        &self,
        pl: &mut RejoinReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<RejoinAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::RejoinReq;

        let mut ans: RejoinAnsPayload = Default::default();
        self.request(None, &pl, &mut ans, async_resp).await?;
        Ok(ans)
    }

    pub async fn app_s_key_req(
        &self,
        pl: &mut AppSKeyReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<AppSKeyAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::AppSKeyReq;

        let mut ans: AppSKeyAnsPayload = Default::default();
        self.request(None, &pl, &mut ans, async_resp).await?;
        Ok(ans)
    }

    pub async fn pr_start_req(
        &self,
        target_role: Role,
        pl: &mut PRStartReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<PRStartAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::PRStartReq;

        let mut ans: PRStartAnsPayload = Default::default();
        self.request(Some(target_role), &pl, &mut ans, async_resp)
            .await?;
        Ok(ans)
    }

    pub async fn pr_start_ans(&self, target_role: Role, pl: &PRStartAnsPayload) -> Result<()> {
        self.response_request(Some(target_role), pl).await
    }

    pub async fn pr_stop_req(
        &self,
        target_role: Role,
        pl: &mut PRStopReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<PRStopAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::PRStopReq;

        let mut ans: PRStopAnsPayload = Default::default();
        self.request(Some(target_role), &pl, &mut ans, async_resp)
            .await?;
        Ok(ans)
    }

    pub async fn pr_stop_ans(&self, target_role: Role, pl: &PRStopAnsPayload) -> Result<()> {
        self.response_request(Some(target_role), pl).await
    }

    pub async fn home_ns_req(
        &self,
        pl: &mut HomeNSReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<HomeNSAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::HomeNSReq;

        let mut ans: HomeNSAnsPayload = Default::default();
        self.request(None, &pl, &mut ans, async_resp).await?;
        Ok(ans)
    }

    pub async fn xmit_data_req(
        &self,
        target_role: Role,
        pl: &mut XmitDataReqPayload,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<XmitDataAnsPayload> {
        pl.base.sender_id = self.config.sender_id.clone();
        pl.base.receiver_id = self.config.receiver_id.clone();
        pl.base.message_type = MessageType::XmitDataReq;

        let mut ans: XmitDataAnsPayload = Default::default();
        self.request(Some(target_role), &pl, &mut ans, async_resp)
            .await?;
        Ok(ans)
    }

    pub async fn xmit_data_ans(&self, target_role: Role, pl: &XmitDataAnsPayload) -> Result<()> {
        self.response_request(Some(target_role), pl).await
    }

    async fn response_request<S>(&self, target_role: Option<Role>, pl: &S) -> Result<()>
    where
        S: ?Sized + serde::ser::Serialize + BasePayloadResultProvider,
    {
        let server = if self.config.use_target_role_suffix {
            match target_role {
                Some(Role::FNS) => format!("{}/fns", self.config.server),
                Some(Role::SNS) => format!("{}/sns", self.config.server),
                Some(Role::HNS) => format!("{}/hns", self.config.server),
                None => self.config.server.clone(),
            }
        } else {
            self.config.server.clone()
        };

        let bp = pl.base_payload();
        let body = serde_json::to_string(&pl)?;

        info!(receiver_id = %hex::encode(&bp.base.receiver_id), transaction_id = bp.base.transaction_id, message_type = ?bp.base.message_type, server = %server, "Making request");
        debug!("JSON: {}", body);

        self.client
            .post(&server)
            .headers(self.headers.clone())
            .body(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn request<S, D>(
        &self,
        target_role: Option<Role>,
        pl: &S,
        ans: &mut D,
        async_resp: Option<Receiver<Vec<u8>>>,
    ) -> Result<()>
    where
        S: ?Sized + serde::ser::Serialize + BasePayloadProvider,
        D: serde::de::DeserializeOwned + BasePayloadResultProvider,
    {
        let server = if self.config.use_target_role_suffix {
            match target_role {
                Some(Role::FNS) => format!("{}/fns", self.config.server),
                Some(Role::SNS) => format!("{}/sns", self.config.server),
                Some(Role::HNS) => format!("{}/hns", self.config.server),
                None => self.config.server.clone(),
            }
        } else {
            self.config.server.clone()
        };

        let bp = pl.base_payload().clone();
        let body = serde_json::to_string(&pl)?;

        info!(receiver_id = %hex::encode(&bp.receiver_id), transaction_id = bp.transaction_id, message_type = ?bp.message_type, server = %server, async_interface = %async_resp.is_some(), "Making request");
        debug!("JSON: {}", body);

        let res = self
            .client
            .post(&server)
            .headers(self.headers.clone())
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        let resp_json = match async_resp {
            Some(rx) => {
                let sleep = tokio::time::sleep(self.config.async_timeout);

                tokio::select! {
                    rx_ans = rx => {
                        String::from_utf8(rx_ans?)?
                    }
                    _ = sleep => {
                        error!(receiver_id = %hex::encode(&bp.receiver_id), transaction_id = bp.transaction_id, message_type = ?bp.message_type, "Async request timeout");
                        return Err(anyhow!("Async timeout"));
                    }
                }
            }
            None => res.text().await?,
        };

        debug!("JSON: {}", resp_json);
        let base: BasePayloadResult = serde_json::from_str(&resp_json)?;
        if base.result.result_code != ResultCode::Success {
            error!(result_code = ?base.result.result_code, description = %base.result.description, receiver_id = %hex::encode(&bp.receiver_id), transaction_id = bp.transaction_id, message_type = ?bp.message_type, "Response error");
            return Err(anyhow!(
                "Response error, code: {:?}, description: {:?}",
                base.result.result_code,
                base.result.description
            ));
        }

        *ans = serde_json::from_str(&resp_json)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum RatePolicy {
    Drop,
    Mark,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct BasePayload {
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "SenderID", with = "hex_encode")]
    pub sender_id: Vec<u8>,
    #[serde(rename = "ReceiverID", with = "hex_encode")]
    pub receiver_id: Vec<u8>,
    #[serde(rename = "TransactionID")]
    pub transaction_id: u32,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(
        default,
        rename = "SenderToken",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sender_token: Vec<u8>,
    #[serde(
        default,
        rename = "ReceiverToken",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub receiver_token: Vec<u8>,
}

impl BasePayload {
    pub fn to_base_payload_result(
        &self,
        res_code: ResultCode,
        description: &str,
    ) -> BasePayloadResult {
        BasePayloadResult {
            base: BasePayload {
                protocol_version: self.protocol_version.clone(),
                sender_id: self.receiver_id.clone(),
                receiver_id: self.sender_id.clone(),
                transaction_id: self.transaction_id,
                message_type: match self.message_type {
                    MessageType::PRStartReq => MessageType::PRStartAns,
                    MessageType::PRStopReq => MessageType::PRStopAns,
                    MessageType::XmitDataReq => MessageType::XmitDataAns,
                    _ => self.message_type,
                },
                sender_token: self.receiver_token.clone(),
                receiver_token: self.sender_token.clone(),
            },
            result: ResultPayload {
                result_code: res_code,
                description: description.to_string(),
            },
        }
    }

    pub fn is_answer(&self) -> bool {
        match self.message_type {
            MessageType::JoinAns
            | MessageType::RejoinAns
            | MessageType::AppSKeyAns
            | MessageType::PRStartAns
            | MessageType::PRStopAns
            | MessageType::HomeNSAns
            | MessageType::XmitDataAns => true,

            MessageType::JoinReq
            | MessageType::RejoinReq
            | MessageType::AppSKeyReq
            | MessageType::PRStartReq
            | MessageType::PRStopReq
            | MessageType::HomeNSReq
            | MessageType::XmitDataReq => false,
        }
    }
}

impl Default for BasePayload {
    fn default() -> Self {
        BasePayload {
            protocol_version: PROTOCOL_VERSION.into(),
            sender_id: "".into(),
            receiver_id: "".into(),
            transaction_id: rand::random(),
            message_type: MessageType::default(),
            sender_token: vec![],
            receiver_token: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct BasePayloadResult {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "Result")]
    pub result: ResultPayload,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct ResultPayload {
    #[serde(rename = "ResultCode")]
    pub result_code: ResultCode,
    #[serde(
        default,
        rename = "Description",
        skip_serializing_if = "String::is_empty"
    )]
    pub description: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct KeyEnvelope {
    #[serde(default, rename = "KEKLabel")]
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

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
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
    #[serde(
        default,
        rename = "CFList",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cf_list: Vec<u8>,
}

impl BasePayloadProvider for &mut JoinReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
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
    #[serde(
        default,
        rename = "SessionKeyID",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub session_key_id: Vec<u8>,
}

impl BasePayloadResultProvider for JoinAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
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
    #[serde(
        default,
        rename = "CFList",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cf_list: Vec<u8>,
}

impl BasePayloadProvider for &mut RejoinReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
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
    #[serde(
        default,
        rename = "SessionKeyID",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub session_key_id: Vec<u8>,
}

impl BasePayloadResultProvider for RejoinAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AppSKeyReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "SessionKeyID", with = "hex_encode")]
    pub session_key_id: Vec<u8>,
}

impl BasePayloadProvider for &mut AppSKeyReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
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

impl BasePayloadResultProvider for AppSKeyAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct PRStartReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "PHYPayload", with = "hex_encode")]
    pub phy_payload: Vec<u8>,
    #[serde(rename = "ULMetaData")]
    pub ul_meta_data: ULMetaData,
}

impl BasePayloadProvider for &mut PRStartReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct PRStartAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(
        default,
        rename = "PHYPayload",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub phy_payload: Vec<u8>,
    #[serde(
        default,
        rename = "DevEUI",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        default,
        rename = "DevAddr",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dev_addr: Vec<u8>,
}

impl BasePayloadResultProvider for PRStartAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PRStopReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "Lifetime")]
    pub lifetime: Option<usize>,
}

impl BasePayloadProvider for &mut PRStopReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct PRStopAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
}

impl BasePayloadResultProvider for PRStopAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct XmitDataReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(
        default,
        rename = "PHYPayload",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub phy_payload: Vec<u8>,
    #[serde(
        default,
        rename = "FRMPayload",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub frm_payload: Vec<u8>,
    #[serde(rename = "ULMetaData")]
    pub ul_meta_data: Option<ULMetaData>,
    #[serde(rename = "DLMetaData")]
    pub dl_meta_data: Option<DLMetaData>,
}

impl BasePayloadProvider for &mut XmitDataReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct XmitDataAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
}

impl BasePayloadResultProvider for XmitDataAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct HomeNSReqPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    #[serde(rename = "DevEUI", with = "hex_encode")]
    pub dev_eui: Vec<u8>,
}

impl BasePayloadProvider for &mut HomeNSReqPayload {
    fn base_payload(&self) -> &BasePayload {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct HomeNSAnsPayload {
    #[serde(flatten)]
    pub base: BasePayloadResult,
    #[serde(
        default,
        rename = "HNetID",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub h_net_id: Vec<u8>,
}

impl BasePayloadResultProvider for HomeNSAnsPayload {
    fn base_payload(&self) -> &BasePayloadResult {
        &self.base
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ULMetaData {
    #[serde(
        default,
        rename = "DevEUI",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dev_eui: Vec<u8>,
    #[serde(
        default,
        rename = "DevAddr",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dev_addr: Vec<u8>,
    #[serde(rename = "FPort")]
    pub f_port: Option<u8>,
    #[serde(rename = "FCntDown")]
    pub f_cnt_down: Option<u32>,
    #[serde(rename = "FCntUp")]
    pub f_cnt_up: Option<u32>,
    #[serde(rename = "Confirmed")]
    pub confirmed: Option<bool>,
    #[serde(rename = "DataRate")]
    pub data_rate: Option<u8>,
    #[serde(rename = "ULFreq")]
    pub ul_freq: Option<f64>,
    #[serde(rename = "Margin")]
    pub margin: Option<isize>,
    #[serde(rename = "Battery")]
    pub battery: Option<isize>,
    #[serde(
        default,
        rename = "FNSULToken",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub f_ns_ul_token: Vec<u8>,
    #[serde(rename = "RecvTime")]
    pub recv_time: DateTime<Utc>,
    #[serde(
        default,
        rename = "RFRegion",
        with = "rf_region_encode",
        skip_serializing_if = "String::is_empty"
    )]
    pub rf_region: String,
    #[serde(rename = "GWCnt")]
    pub gw_cnt: Option<usize>,
    #[serde(rename = "GWInfo")]
    pub gw_info: Vec<GWInfoElement>,
}

impl Default for ULMetaData {
    fn default() -> Self {
        ULMetaData {
            dev_eui: Vec::new(),
            dev_addr: Vec::new(),
            f_port: None,
            f_cnt_down: None,
            f_cnt_up: None,
            confirmed: None,
            data_rate: None,
            ul_freq: None,
            margin: None,
            battery: None,
            f_ns_ul_token: Vec::new(),
            recv_time: Utc::now(),
            rf_region: "".to_string(),
            gw_cnt: None,
            gw_info: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct GWInfoElement {
    #[serde(
        default,
        rename = "ID",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub id: Vec<u8>,
    #[serde(rename = "FineRecvTime")]
    pub fine_recv_time: Option<usize>,
    #[serde(
        default,
        rename = "RFRegion",
        with = "rf_region_encode",
        skip_serializing_if = "String::is_empty"
    )]
    pub rf_region: String,
    #[serde(rename = "RSSI")]
    pub rssi: Option<isize>,
    #[serde(rename = "SNR")]
    pub snr: Option<f32>,
    #[serde(rename = "Lat")]
    pub lat: Option<f64>,
    #[serde(rename = "Lon")]
    pub lon: Option<f64>,
    #[serde(
        default,
        rename = "ULToken",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ul_token: Vec<u8>,
    #[serde(rename = "DLAllowed")]
    pub dl_allowed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct DLMetaData {
    #[serde(
        default,
        rename = "DevEUI",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dev_eui: Vec<u8>,
    #[serde(rename = "FPort")]
    pub f_port: Option<u8>,
    #[serde(rename = "FCntDown")]
    pub f_cnt_down: Option<u32>,
    #[serde(default, rename = "Confirmed")]
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
    pub data_rate_1: Option<u8>,
    #[serde(rename = "DataRate2")]
    pub data_rate_2: Option<u8>,
    #[serde(
        default,
        rename = "FNSULToken",
        with = "hex_encode",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub f_ns_ul_token: Vec<u8>,
    #[serde(rename = "GWInfo")]
    pub gw_info: Vec<GWInfoElement>,
    #[serde(default, rename = "HiPriorityFlag")]
    pub hi_priority_flag: bool,
}

mod rf_region_encode {
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(s: &str, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&s.replace('_', "-"))
    }

    pub fn deserialize<'a, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s: &str = serde::de::Deserialize::deserialize(deserializer)?;

        // Some implementations use lowercase.
        Ok(s.to_uppercase())
    }
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

        // HEX encoded values may start with 0x prefix, we must strip this.
        let s = s.trim_start_matches("0x");

        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use httpmock::prelude::*;
    use tokio::sync::oneshot;

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

    #[tokio::test]
    async fn test_async_request() {
        let server = MockServer::start();

        let c = Client::new(ClientConfig {
            sender_id: "010203".into(),
            receiver_id: "0102030405060708".into(),
            server: server.url("/"),
            async_timeout: Duration::from_secs(1),
            ..Default::default()
        })
        .unwrap();

        let mut req = HomeNSReqPayload {
            base: BasePayload {
                sender_id: "010203".into(),
                receiver_id: "0102030405060708".into(),
                message_type: MessageType::HomeNSReq,
                transaction_id: 1234,
                ..Default::default()
            },
            dev_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        };

        let ans = HomeNSAnsPayload {
            base: BasePayloadResult {
                base: BasePayload {
                    sender_id: "0102030405060708".into(),
                    receiver_id: "010203".into(),
                    message_type: MessageType::HomeNSAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: ResultPayload {
                    result_code: ResultCode::Success,
                    description: "".into(),
                },
            },
            h_net_id: vec![3, 2, 1],
        };

        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .body(serde_json::to_string(&req).unwrap());
            then.status(200);
        });

        // OK
        let (tx, rx) = oneshot::channel();
        tx.send(serde_json::to_vec(&ans).unwrap()).unwrap();
        let resp = c.home_ns_req(&mut req, Some(rx)).await.unwrap();
        mock.assert();
        mock.delete();
        assert_eq!(resp, ans);

        // Timeout
        let (_tx, rx) = oneshot::channel();
        let resp = c.home_ns_req(&mut req, Some(rx)).await;
        assert!(resp.is_err());
    }

    #[tokio::test]
    async fn test_sync_request() {
        let server = MockServer::start();

        let c = Client::new(ClientConfig {
            sender_id: "010203".into(),
            receiver_id: "0102030405060708".into(),
            server: server.url("/"),
            ..Default::default()
        })
        .unwrap();

        let mut req = HomeNSReqPayload {
            base: BasePayload {
                sender_id: "010203".into(),
                receiver_id: "0102030405060708".into(),
                message_type: MessageType::HomeNSReq,
                transaction_id: 1234,
                ..Default::default()
            },
            dev_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        };

        let ans = HomeNSAnsPayload {
            base: BasePayloadResult {
                base: BasePayload {
                    sender_id: "0102030405060708".into(),
                    receiver_id: "010203".into(),
                    message_type: MessageType::HomeNSAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: ResultPayload {
                    result_code: ResultCode::Success,
                    description: "".into(),
                },
            },
            h_net_id: vec![3, 2, 1],
        };

        // OK
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .body(serde_json::to_string(&req).unwrap());
            then.body(serde_json::to_vec(&ans).unwrap()).status(200);
        });
        let resp = c.home_ns_req(&mut req, None).await.unwrap();
        mock.assert();
        mock.delete();
        assert_eq!(resp, ans);

        // Error status
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .body(serde_json::to_string(&req).unwrap());
            then.status(500);
        });
        let resp = c.home_ns_req(&mut req, None).await;
        mock.assert();
        mock.delete();
        assert!(resp.is_err());
    }
}
