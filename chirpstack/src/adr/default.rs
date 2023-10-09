use anyhow::{Context, Result};
use async_trait::async_trait;

use super::{Handler, Request, Response};
use crate::region;

pub struct Algorithm {}

impl Algorithm {
    pub fn new() -> Self {
        Algorithm {}
    }

    fn get_ideal_tx_power_index_and_dr(
        nb_step: isize,
        tx_power_index: u8,
        dr: u8,
        max_tx_power_index: u8,
        max_dr: u8,
    ) -> (u8, u8) {
        if nb_step == 0 {
            return (tx_power_index, dr);
        }

        let mut nb_step = nb_step;
        let mut dr = dr;
        let mut tx_power_index = tx_power_index;

        if nb_step > 0 {
            if dr < max_dr {
                // Increase the DR.
                dr += 1;
            } else if tx_power_index < max_tx_power_index {
                // Decrease the tx-power.
                // (note that an increase in index decreases the tx-power)
                tx_power_index += 1;
            }
            nb_step -= 1;
        } else {
            // Increase the tx-power.
            // (note that a decrease in index increases the tx-power)
            // Subtract only if > 0
            tx_power_index = tx_power_index.saturating_sub(1);
            nb_step += 1;
        }

        Self::get_ideal_tx_power_index_and_dr(
            nb_step,
            tx_power_index,
            dr,
            max_tx_power_index,
            max_dr,
        )
    }

    fn required_history_count(&self) -> usize {
        20
    }

    // Returns the history count with equal TxPowerIndex.
    fn get_history_count(&self, req: &Request) -> usize {
        req.uplink_history
            .iter()
            .filter(|x| x.tx_power_index == req.tx_power_index as u32)
            .count()
    }

    fn get_max_snr(&self, req: &Request) -> f32 {
        let mut max_snr: f32 = -999.0;

        for uh in &req.uplink_history {
            if uh.max_snr > max_snr {
                max_snr = uh.max_snr;
            }
        }

        max_snr
    }

    fn get_nb_trans(&self, current_nb_trans: u8, pkt_loss_rate: f32) -> u8 {
        let pkt_loss_table: [[u8; 3]; 4] = [[1, 1, 2], [1, 2, 3], [2, 3, 3], [3, 3, 3]];

        let mut current_nb_trans = current_nb_trans;
        if current_nb_trans < 1 {
            current_nb_trans = 1;
        }

        if current_nb_trans > 3 {
            current_nb_trans = 3;
        }

        let nb_trans_index = current_nb_trans as usize - 1;
        if pkt_loss_rate < 5.0 {
            return pkt_loss_table[0][nb_trans_index];
        } else if pkt_loss_rate < 10.0 {
            return pkt_loss_table[1][nb_trans_index];
        } else if pkt_loss_rate < 30.0 {
            return pkt_loss_table[2][nb_trans_index];
        }

        pkt_loss_table[3][nb_trans_index]
    }

    fn get_packet_loss_percentage(&self, req: &Request) -> f32 {
        if req.uplink_history.len() < self.required_history_count() {
            return 0.0;
        }

        let mut lost_packets: u32 = 0;
        let mut previous_f_cnt: u32 = 0;

        for (i, h) in req.uplink_history.iter().enumerate() {
            if i == 0 {
                previous_f_cnt = h.f_cnt;
                continue;
            }

            lost_packets += h.f_cnt - previous_f_cnt - 1; // there is always an expected difference of 1
            previous_f_cnt = h.f_cnt;
        }

        (lost_packets as f32) / (req.uplink_history.len() as f32) * 100.0
    }
}

#[async_trait]
impl Handler for Algorithm {
    fn get_name(&self) -> String {
        "Default ADR algorithm (LoRa only)".to_string()
    }

    fn get_id(&self) -> String {
        "default".to_string()
    }

    async fn handle(&self, req: &Request) -> Result<Response> {
        let mut resp = Response {
            dr: req.dr,
            tx_power_index: req.tx_power_index,
            nb_trans: req.nb_trans,
        };

        // If ADR is disabled, return with current values.
        if !req.adr {
            return Ok(resp);
        }

        // The max DR might be configured to a non LoRa (125kHz) data-rate.
        // As this algorithm works on LoRa (125kHz) data-rates only, we need to
        // find the max LoRa (125 kHz) data-rate.
        let region_conf =
            region::get(&req.region_config_id).context("Get region config for region")?;
        let mut max_dr = req.max_dr;
        let max_lora_dr = region_conf
            .get_enabled_uplink_data_rates()
            .into_iter()
            .filter(|dr| {
                let dr = region_conf.get_data_rate(*dr).unwrap();
                if let lrwn::region::DataRateModulation::Lora(l) = dr {
                    l.bandwidth == 125000
                } else {
                    false
                }
            })
            .max()
            .unwrap_or(0);

        // Reduce to max LoRa DR.
        if max_dr > max_lora_dr {
            max_dr = max_lora_dr;
        }

        // Lower the DR only if it exceeds the max. allowed DR.
        if req.dr > max_dr {
            resp.dr = max_dr;
        }

        // Set the new nb_trans;
        resp.nb_trans = self.get_nb_trans(req.nb_trans, self.get_packet_loss_percentage(req));

        // Calculate the number of steps.
        let snr_max = self.get_max_snr(req);
        let snr_margin = snr_max - req.required_snr_for_dr - req.installation_margin;
        let n_step = (snr_margin / 3.0) as isize;

        // In case of negative steps the ADR algorithm will increase the TxPower
        // if possible. To avoid up / down / up / down TxPower changes, wait until
        // we have at least the required number of uplink history elements.
        if n_step < 0 && self.get_history_count(req) != self.required_history_count() {
            return Ok(resp);
        }

        let (desired_tx_power_index, desired_dr) = Self::get_ideal_tx_power_index_and_dr(
            n_step,
            resp.tx_power_index,
            resp.dr,
            req.max_tx_power_index,
            max_dr,
        );

        resp.dr = desired_dr;
        resp.tx_power_index = desired_tx_power_index;

        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test;
    use chirpstack_api::internal;
    use std::str::FromStr;

    #[test]
    fn test_id() {
        let a = Algorithm::new();
        assert_eq!("default", a.get_id());
    }

    #[test]
    fn test_get_packet_loss_percentage() {
        let a = Algorithm::new();
        let mut req = Request {
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

        for i in 0..20 {
            if i < 5 {
                req.uplink_history.push(internal::UplinkAdrHistory {
                    f_cnt: i,
                    ..Default::default()
                });
                continue;
            }

            if i < 10 {
                req.uplink_history.push(internal::UplinkAdrHistory {
                    f_cnt: i + 1,
                    ..Default::default()
                });
                continue;
            }

            req.uplink_history.push(internal::UplinkAdrHistory {
                f_cnt: i + 2,

                ..Default::default()
            });
        }

        assert_eq!(10.0, a.get_packet_loss_percentage(&req));
    }

    #[test]
    fn test_get_nb_trans() {
        let a = Algorithm::new();

        struct Test {
            pkt_loss_rate: f32,
            current_nb_trans: u8,
            expected_nb_trans: u8,
        }

        let tests = vec![
            Test {
                pkt_loss_rate: 4.99,
                current_nb_trans: 3,
                expected_nb_trans: 2,
            },
            Test {
                pkt_loss_rate: 9.99,
                current_nb_trans: 2,
                expected_nb_trans: 2,
            },
            Test {
                pkt_loss_rate: 30.0,
                current_nb_trans: 3,
                expected_nb_trans: 3,
            },
        ];

        for tst in &tests {
            assert_eq!(
                tst.expected_nb_trans,
                a.get_nb_trans(tst.current_nb_trans, tst.pkt_loss_rate)
            );
        }
    }

    #[test]
    fn test_get_ideal_tx_power_index_and_dr() {
        struct Test {
            name: String,
            n_step: isize,
            tx_power_index: u8,
            dr: u8,
            max_tx_power_index: u8,
            max_dr: u8,
            expected_tx_power_index: u8,
            expected_dr: u8,
        }

        let tests = vec![
            Test {
                name: "nothing to do".into(),
                n_step: 0,
                tx_power_index: 1,
                dr: 3,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_tx_power_index: 1,
                expected_dr: 3,
            },
            Test {
                name: "one step: one step data-rate increase".into(),
                n_step: 1,
                tx_power_index: 1,
                dr: 4,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 1,
            },
            Test {
                name: "one step: one step tx-power decrease".into(),
                n_step: 1,
                tx_power_index: 1,
                dr: 5,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 2,
            },
            Test {
                name: "two steps: two steps data-rate increase".into(),
                n_step: 2,
                tx_power_index: 1,
                dr: 3,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 1,
            },
            Test {
                name: "two steps: one step data-rate increase, one step tx-power decrease".into(),
                n_step: 2,
                tx_power_index: 1,
                dr: 4,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 2,
            },
            Test {
                name: "two step tx-power decrease".into(),
                n_step: 2,
                tx_power_index: 1,
                dr: 5,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 3,
            },
            Test {
                name: "two steps: one step tx-power decrease".into(),
                n_step: 2,
                tx_power_index: 5,
                dr: 5,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 5,
            },
            Test {
                name: "one negative step: one step power increase".into(),
                n_step: -1,
                tx_power_index: 1,
                dr: 5,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 5,
                expected_tx_power_index: 0,
            },
            Test {
                name: "one negative step: nothing to do (adr engine will not decrease dr)".into(),
                n_step: -1,
                tx_power_index: 0,
                dr: 4,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 4,
                expected_tx_power_index: 0,
            },
            Test {
                name: "10 negative steps, should not adjust anything (as we already reached the min tx-power index)".into(),
                n_step: -10,
                tx_power_index: 0,
                dr: 4,
                max_tx_power_index: 5,
                max_dr: 5,
                expected_dr: 4,
                expected_tx_power_index: 0,
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);
            let (tx_power_index, dr) = Algorithm::get_ideal_tx_power_index_and_dr(
                tst.n_step,
                tst.tx_power_index,
                tst.dr,
                tst.max_tx_power_index,
                tst.max_dr,
            );

            assert_eq!(tst.expected_dr, dr);
            assert_eq!(tst.expected_tx_power_index, tx_power_index);
        }
    }

    #[test]
    fn test_required_history_count() {
        let a = Algorithm::new();
        assert_eq!(20, a.required_history_count());
    }

    #[test]
    fn get_max_snr() {
        let a = Algorithm::new();

        let mut req = Request {
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
        req.uplink_history.push(internal::UplinkAdrHistory {
            max_snr: 3.0,
            ..Default::default()
        });
        req.uplink_history.push(internal::UplinkAdrHistory {
            max_snr: 4.0,
            ..Default::default()
        });
        req.uplink_history.push(internal::UplinkAdrHistory {
            max_snr: 2.0,
            ..Default::default()
        });

        assert_eq!(4.0, a.get_max_snr(&req));
    }

    #[tokio::test]
    async fn test_handle() {
        let a = Algorithm::new();
        let _guard = test::prepare().await;

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
                name: "max dr exceeded, adr disabled".into(),
                request: Request {
                    region_config_id: "eu868".into(),
                    adr: false,
                    dr: 5,
                    tx_power_index: 0,
                    nb_trans: 1,
                    max_dr: 4,
                    max_tx_power_index: 5,
                    ..req_template.clone()
                },
                response: Response {
                    dr: 5,
                    tx_power_index: 0,
                    nb_trans: 1,
                },
            },
            Test {
                name: "max dr exceeded, decrease dr".into(),
                request: Request {
                    region_config_id: "eu868".into(),
                    adr: true,
                    dr: 5,
                    tx_power_index: 0,
                    nb_trans: 1,
                    max_dr: 4,
                    max_tx_power_index: 5,
                    uplink_history: vec![internal::UplinkAdrHistory {
                        max_snr: 0.0,
                        ..Default::default()
                    }],
                    ..req_template.clone()
                },
                response: Response {
                    dr: 4,
                    tx_power_index: 0,
                    nb_trans: 1,
                },
            },
            Test {
                name: "increase dr".into(),
                request: Request {
                    region_config_id: "eu868".into(),
                    adr: true,
                    dr: 0,
                    tx_power_index: 0,
                    nb_trans: 1,
                    max_dr: 5,
                    max_tx_power_index: 5,
                    required_snr_for_dr: -20.0,
                    uplink_history: vec![internal::UplinkAdrHistory {
                        max_snr: -15.0,
                        ..Default::default()
                    }],
                    ..req_template.clone()
                },
                response: Response {
                    dr: 1,
                    tx_power_index: 0,
                    nb_trans: 1,
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
