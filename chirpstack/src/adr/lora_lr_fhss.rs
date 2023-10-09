use anyhow::{Context, Result};
use async_trait::async_trait;

use super::{default, lr_fhss};
use super::{Handler, Request, Response};
use crate::region;

pub struct Algorithm {}

impl Algorithm {
    pub fn new() -> Self {
        Algorithm {}
    }
}

#[async_trait]
impl Handler for Algorithm {
    fn get_name(&self) -> String {
        "LoRa & LR-FHSS ADR algorithm".to_string()
    }

    fn get_id(&self) -> String {
        "lora_lr_fhss".to_string()
    }

    async fn handle(&self, req: &Request) -> Result<Response> {
        let region_conf =
            region::get(&req.region_config_id).context("Get region config for region")?;
        let default_alg = default::Algorithm::new();
        let lr_fhss_alg = lr_fhss::Algorithm::new();

        let default_resp = default_alg.handle(req).await?;
        let lr_fhss_resp = lr_fhss_alg.handle(req).await?;

        // For SF < 10, LoRa is a better option, for SF >= 10 use LR-FHSS.
        let lora_dr = region_conf
            .get_data_rate(default_resp.dr)
            .context("Get data-rate")?;
        if let lrwn::region::DataRateModulation::Lora(dr) = lora_dr {
            if dr.spreading_factor < 10 {
                return Ok(default_resp);
            }
        }

        Ok(lr_fhss_resp)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{config, test};
    use chirpstack_api::internal;
    use std::str::FromStr;

    #[test]
    fn test_id() {
        let a = Algorithm::new();
        assert_eq!("lora_lr_fhss", a.get_id());
    }

    #[tokio::test]
    async fn test_handle() {
        let a = Algorithm::new();
        let _guard = test::prepare().await;

        let mut conf = (*config::get()).clone();
        conf.regions[0]
            .network
            .extra_channels
            .push(config::ExtraChannel {
                frequency: 867300000,
                min_dr: 10,
                max_dr: 11,
            });
        config::set(conf);
        region::setup().unwrap();

        let req_template = Request {
            region_config_id: "eu868".into(),
            region_common_name: lrwn::region::CommonName::EU868,
            dev_eui: lrwn::EUI64::from_str("0102030405060708").unwrap(),
            mac_version: lrwn::region::MacVersion::LORAWAN_1_0_4,
            reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
            adr: true,
            dr: 0,
            tx_power_index: 0,
            nb_trans: 1,
            max_tx_power_index: 0,
            required_snr_for_dr: 0.0,
            installation_margin: 0.0,
            min_dr: 0,
            max_dr: 0,
            uplink_history: vec![],
            skip_f_cnt_check: false,
            device_variables: Default::default(),
        };

        struct Test {
            name: String,
            request: Request,
            response: Response,
        }

        let tests = vec![
            Test {
                name: "switch to DR 3 (LoRa)".into(),
                request: Request {
                    region_config_id: "eu868".into(),
                    adr: true,
                    dr: 0,
                    nb_trans: 1,
                    max_dr: 11,
                    required_snr_for_dr: -20.0,
                    uplink_history: vec![internal::UplinkAdrHistory {
                        max_snr: -10.0,
                        ..Default::default()
                    }],
                    ..req_template.clone()
                },
                response: Response {
                    dr: 3,
                    nb_trans: 1,
                    tx_power_index: 0,
                },
            },
            Test {
                name: "switch to DR 10 (LR-FHSS)".into(),
                request: Request {
                    region_config_id: "eu868".into(),
                    adr: true,
                    dr: 0,
                    nb_trans: 3,
                    max_dr: 11,
                    required_snr_for_dr: -20.0,
                    uplink_history: vec![internal::UplinkAdrHistory {
                        max_snr: -12.0,
                        ..Default::default()
                    }],
                    ..req_template.clone()
                },
                response: Response {
                    dr: 10,
                    nb_trans: 1,
                    tx_power_index: 0,
                },
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);
            let resp = a.handle(&tst.request).await.unwrap();
            assert_eq!(tst.response, resp);
        }
    }
}
