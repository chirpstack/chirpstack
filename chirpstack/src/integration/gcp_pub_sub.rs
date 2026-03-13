use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use gcp_auth::{CustomServiceAccount, TokenProvider};
use prost::Message;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::GcpPubSubConfiguration;
use chirpstack_api::api::Encoding;
use chirpstack_api::integration;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> Client {
    CLIENT
        .get_or_init(|| {
            Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap()
        })
        .clone()
}

#[derive(Deserialize)]
struct CredentialFile {
    #[serde(rename = "type")]
    credential_type: String,
}

#[derive(Deserialize)]
struct ExternalAccountConfig {
    audience: String,
    subject_token_type: String,
    token_url: String,
    credential_source: CredentialSource,
    service_account_impersonation_url: Option<String>,
}

#[derive(Deserialize)]
struct CredentialSource {
    file: String,
}

#[derive(Deserialize)]
struct StsTokenResponse {
    access_token: String,
}

#[derive(Serialize)]
struct GenerateAccessTokenRequest {
    scope: Vec<String>,
    lifetime: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerateAccessTokenResponse {
    access_token: String,
}

enum GcpAuthProvider {
    ServiceAccount(CustomServiceAccount),
    ExternalAccount(ExternalAccountProvider),
}

struct ExternalAccountProvider {
    audience: String,
    subject_token_type: String,
    token_url: String,
    credential_source_file: String,
    service_account_impersonation_url: Option<String>,
}

impl ExternalAccountProvider {
    fn from_json(json: &str) -> Result<Self> {
        let config: ExternalAccountConfig =
            serde_json::from_str(json).context("Parse external account credentials")?;

        if !config.token_url.starts_with("https://sts.googleapis.com/") {
            return Err(anyhow!("Invalid token_url: must be an STS endpoint"));
        }
        if let Some(ref url) = config.service_account_impersonation_url {
            if !url.starts_with("https://iamcredentials.googleapis.com/") {
                return Err(anyhow!(
                    "Invalid service_account_impersonation_url: must be an IAM credentials endpoint"
                ));
            }
        }

        Ok(Self {
            audience: config.audience,
            subject_token_type: config.subject_token_type,
            token_url: config.token_url,
            credential_source_file: config.credential_source.file,
            service_account_impersonation_url: config.service_account_impersonation_url,
        })
    }

    async fn token(&self, scopes: &[&str]) -> Result<String> {
        let subject_token = tokio::fs::read_to_string(&self.credential_source_file)
            .await
            .context("Read subject token file")?;

        // When using service account impersonation, the STS token needs
        // cloud-platform scope to be able to call generateAccessToken.
        // The target scopes are then requested in the impersonation call.
        let sts_scope = if self.service_account_impersonation_url.is_some() {
            "https://www.googleapis.com/auth/cloud-platform".to_string()
        } else {
            scopes.join(" ")
        };

        let sts_resp: StsTokenResponse = get_client()
            .post(&self.token_url)
            .form(&[
                (
                    "grant_type",
                    "urn:ietf:params:oauth:grant-type:token-exchange",
                ),
                ("audience", &self.audience),
                ("subject_token_type", &self.subject_token_type),
                (
                    "requested_token_type",
                    "urn:ietf:params:oauth:token-type:access_token",
                ),
                ("subject_token", subject_token.trim()),
                ("scope", &sts_scope),
            ])
            .send()
            .await
            .context("STS token exchange request")?
            .error_for_status()
            .context("STS token exchange")?
            .json()
            .await
            .context("Parse STS token response")?;

        if let Some(ref impersonation_url) = self.service_account_impersonation_url {
            let resp: GenerateAccessTokenResponse = get_client()
                .post(impersonation_url)
                .header(AUTHORIZATION, format!("Bearer {}", sts_resp.access_token))
                .json(&GenerateAccessTokenRequest {
                    scope: scopes.iter().map(|s| s.to_string()).collect(),
                    lifetime: "3600s".to_string(),
                })
                .send()
                .await
                .context("Service account impersonation request")?
                .error_for_status()
                .context("Service account impersonation")?
                .json()
                .await
                .context("Parse impersonation response")?;

            return Ok(resp.access_token);
        }

        Ok(sts_resp.access_token)
    }
}

impl GcpAuthProvider {
    async fn token(&self, scopes: &[&str]) -> Result<String> {
        match self {
            GcpAuthProvider::ServiceAccount(sa) => {
                let token = sa.token(scopes).await.context("Get GCP bearer token")?;
                Ok(token.as_str().to_string())
            }
            GcpAuthProvider::ExternalAccount(ea) => ea.token(scopes).await,
        }
    }
}

pub struct Integration {
    json: bool,
    project_id: String,
    topic_name: String,
    auth_provider: GcpAuthProvider,
}

#[derive(Serialize)]
struct PublishRequest {
    pub messages: Vec<PubSubMessage>,
}

#[derive(Serialize)]
struct PubSubMessage {
    pub data: String,
    pub attributes: PubSubMessageAttributes,
}

#[derive(Serialize)]
struct PubSubMessageAttributes {
    pub event: String,
    pub dev_eui: String,
    pub application_id: String,
}

impl Integration {
    pub async fn new(conf: &GcpPubSubConfiguration) -> Result<Integration> {
        trace!("Initializing GCP Pub-Sub integration");

        let cred_file: CredentialFile = serde_json::from_str(&conf.credentials_file)
            .context("Parse credentials file")?;

        let auth_provider = match cred_file.credential_type.as_str() {
            "service_account" => match CustomServiceAccount::from_json(&conf.credentials_file) {
                Ok(sa) => GcpAuthProvider::ServiceAccount(sa),
                Err(service_err) => {
                    // Some deployments provide external-account credentials with an incorrect
                    // `type` field. Try external-account parsing before returning an error.
                    if let Ok(ea) = ExternalAccountProvider::from_json(&conf.credentials_file) {
                        info!(
                            "Detected external-account credential format while type=service_account; using external-account provider"
                        );
                        GcpAuthProvider::ExternalAccount(ea)
                    } else {
                        return Err(anyhow!(
                            "Invalid service_account credentials: {}. If you are using Workload Identity Federation, set `type` to `external_account`",
                            service_err
                        ));
                    }
                }
            },
            "external_account" => {
                let ea = ExternalAccountProvider::from_json(&conf.credentials_file)?;
                GcpAuthProvider::ExternalAccount(ea)
            }
            other => return Err(anyhow!("Unsupported credential type: {}", other)),
        };

        Ok(Integration {
            json: match Encoding::try_from(conf.encoding)
                .map_err(|_| anyhow!("Invalid encoding"))?
            {
                Encoding::Json => true,
                Encoding::Protobuf => false,
            },
            project_id: conf.project_id.clone(),
            topic_name: conf.topic_name.clone(),
            auth_provider,
        })
    }

    async fn publish(
        &self,
        event: &str,
        application_id: &str,
        dev_eui: &str,
        pl: &[u8],
    ) -> Result<()> {
        info!(
            event = %event, dev_eui = %dev_eui,
            "Publishing event"
        );

        let topic = format!("projects/{}/topics/{}", self.project_id, self.topic_name);

        let pl = PublishRequest {
            messages: vec![PubSubMessage {
                data: general_purpose::STANDARD.encode(pl),
                attributes: PubSubMessageAttributes {
                    event: event.to_string(),
                    dev_eui: dev_eui.to_string(),
                    application_id: application_id.to_string(),
                },
            }],
        };
        let pl = serde_json::to_string(&pl)?;

        let token = self
            .auth_provider
            .token(&["https://www.googleapis.com/auth/pubsub"])
            .await?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        let res = get_client()
            .post(format!(
                "https://pubsub.googleapis.com/v1/{}:publish",
                topic
            ))
            .body(pl)
            .headers(headers)
            .send()
            .await?;
        res.error_for_status()?;

        Ok(())
    }
}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("up", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("join", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("ack", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("txack", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("log", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("status", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("location", &di.application_id, &di.dev_eui, &pl)
            .await
    }
}
