use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use prost::Message;
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use reqwest::Client;
use tracing::{error, info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::HttpConfiguration;
use chirpstack_api::integration;

pub struct Integration {
    timeout: Duration,
    endpoints: Vec<String>,
    headers: HashMap<String, String>,
    json: bool,
}

impl Integration {
    pub fn new(conf: &HttpConfiguration) -> Integration {
        trace!("Initializing http integration");

        Integration {
            timeout: Duration::from_secs(5),
            headers: conf.headers.clone(),
            json: conf.json,
            endpoints: conf
                .event_endpoint_url
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }

    async fn post_event(&self, event: &str, b: Vec<u8>) -> Result<()> {
        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();

        for (k, v) in &self.headers {
            headers.insert(HeaderName::try_from(k)?, v.parse()?);
        }

        if self.json {
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        } else {
            headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
        }

        for url in &self.endpoints {
            info!(event = %event, url = %url, "Posting event");
            let res = client
                .post(url)
                .body(b.clone())
                .query(&[("event", event)])
                .headers(headers.clone())
                .send()
                .await;

            match res {
                Ok(res) => match res.error_for_status() {
                    Ok(_) => {}
                    Err(e) => {
                        error!(event = %event, url = %url, error = %e, "Posting event failed");
                    }
                },
                Err(e) => {
                    error!(event = %event, url = %url, error = %e, "Posting event failed");
                }
            }
        }

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
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("up", b).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("join", b).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("ack", b).await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("txack", b).await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("log", b).await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("status", b).await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("location", b).await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.post_event("integration", b).await
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_url_split() {
        let i = Integration::new(&HttpConfiguration {
            headers: HashMap::new(),
            json: true,
            event_endpoint_url: "http://a.com,http://b.com, http://c.com , http://d.com"
                .to_string(),
        });

        assert_eq!(
            vec![
                "http://a.com".to_string(),
                "http://b.com".to_string(),
                "http://c.com".to_string(),
                "http://d.com".to_string(),
            ],
            i.endpoints
        );
    }

    #[tokio::test]
    async fn test_http() {
        let server = MockServer::start();

        let i = Integration {
            timeout: Duration::from_secs(5),
            endpoints: vec![server.url("/")],
            headers: [("Foo".to_string(), "Bar".to_string())]
                .iter()
                .cloned()
                .collect(),
            json: true,
        };

        // uplink event
        let pl: integration::UplinkEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "up")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // join event
        let pl: integration::JoinEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "join")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.join_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // ack event
        let pl: integration::AckEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "ack")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });
        i.ack_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // txack event
        let pl: integration::TxAckEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "txack")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });
        i.txack_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // log event
        let pl: integration::LogEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "log")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });
        i.log_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // status event
        let pl: integration::StatusEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "status")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });
        i.status_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // location event
        let pl: integration::LocationEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "location")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });
        i.location_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // integration event
        let pl: integration::IntegrationEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "integration")
                .header("Foo", "Bar")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });
        i.integration_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();
    }
}
