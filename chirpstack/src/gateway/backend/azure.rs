use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use prost::Message;
use reqwest::header::{HeaderMap, HeaderName, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use sha2::Sha256;
use tracing::{info, trace};
use crate::storage::application::AzureServiceBusConfiguration;
use chirpstack_api::api::Encoding;

pub struct AzureBackend {
    timeout: Duration,
    json: bool,
    uri: String,
    key_name: String,
    key: String,
}

impl AzureBackend {
    pub fn new(conf: &AzureServiceBusConfiguration) -> Result<AzureBackend> {
        trace!("Initializing Azure Service-Bus Backend");

        let kv = parse_connection_string(&conf.connection_string);

        Ok(AzureBackend {
            timeout: Duration::from_secs(5),
            json: match Encoding::from_i32(conf.encoding)
                .ok_or_else(|| anyhow!("Invalid encoding"))?
            {
                Encoding::Json => true,
                Encoding::Protobuf => false,
            },
            uri: format!(
                "https://{}{}",
                kv.get("Endpoint")
                    .cloned()
                    .unwrap_or_default()
                    .replace("sb://", ""),
                conf.publish_name
            ),
            key_name: kv.get("SharedAccessKeyName").cloned().unwrap_or_default(),
            key: kv.get("SharedAccessKey").cloned().unwrap_or_default(),
        })
    }

    
}

type HmacSha256 = Hmac<Sha256>;

fn create_sas_token(
    uri: &str,
    key_name: &str,
    key: &str,
    expiration: &SystemTime,
) -> Result<String> {
    let encoded_url = urlencoding::encode(uri);
    let exp = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let sig = format!("{}\n{}", encoded_url, exp);
    let mut m = HmacSha256::new_from_slice(key.as_bytes())?;
    m.update(sig.as_bytes());
    let result = base64::encode(m.finalize().into_bytes());

    let hash = urlencoding::encode(&result);

    Ok(format!(
        "SharedAccessSignature sig={}&se={}&skn={}&sr={}",
        hash, exp, key_name, encoded_url
    ))
}

fn parse_connection_string(s: &str) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();

    for pair in s.split(';') {
        let kv: Vec<&str> = pair.splitn(2, '=').collect();
        if kv.len() != 2 {
            continue;
        }

        out.insert(kv[0].to_string(), kv[1].to_string());
    }

    out
}
