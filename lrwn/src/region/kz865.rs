use std::time::Duration;

use anyhow::Result;

use super::{
    Channel, CommonName, DataRate, DataRateModulation, Defaults, FskDataRate, LinkADRReqPayload,
    LoraDataRate, MacVersion, MaxPayloadSize, Region, RegionBaseConfig, Revision,
};
use crate::{CFList, DevAddr};

pub struct Configuration {
    base: RegionBaseConfig,
}

impl Configuration {
    pub fn new(repeater_compatible: bool) -> Configuration {
        Configuration {
            base: RegionBaseConfig {
                supports_user_channels: true,
                cf_list_min_dr: 0,
                cf_list_max_dr: 5,
                data_rates: [
                    (
                        0,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 12,
                                bandwidth: 125000,
                            }),
                        },
                    ),
                    (
                        1,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 11,
                                bandwidth: 125000,
                            }),
                        },
                    ),
                    (
                        2,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 10,
                                bandwidth: 125000,
                            }),
                        },
                    ),
                    (
                        3,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 9,
                                bandwidth: 125000,
                            }),
                        },
                    ),
                    (
                        4,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 8,
                                bandwidth: 125000,
                            }),
                        },
                    ),
                    (
                        5,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 7,
                                bandwidth: 125000,
                            }),
                        },
                    ),
                    (
                        6,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 7,
                                bandwidth: 250000,
                            }),
                        },
                    ),
                    (
                        7,
                        DataRate {
                            uplink: true,
                            downlink: true,
                            modulation: DataRateModulation::Fsk(FskDataRate { bitrate: 50000 }),
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                max_payload_size_per_dr: match repeater_compatible {
                    true => [
                        (
                            MacVersion::LORAWAN_1_0_3,
                            [(
                                Revision::Latest, // A
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
                                    (6, MaxPayloadSize { m: 230, n: 222 }),
                                    (7, MaxPayloadSize { m: 230, n: 222 }),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        ),
                        (
                            MacVersion::LORAWAN_1_1_0, // B
                            [(
                                Revision::Latest,
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
                                    (6, MaxPayloadSize { m: 230, n: 222 }),
                                    (7, MaxPayloadSize { m: 230, n: 222 }),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        ),
                        (
                            MacVersion::Latest, // B
                            [(
                                Revision::Latest, // RP002-1.0.0, RP002-1.0.1, RP002-1.0.2, RP002-1.0.3
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
                                    (6, MaxPayloadSize { m: 230, n: 222 }),
                                    (7, MaxPayloadSize { m: 230, n: 222 }),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        ),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    false => [
                        (
                            MacVersion::LORAWAN_1_0_3,
                            [(
                                Revision::Latest, // A
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 250, n: 242 }),
                                    (5, MaxPayloadSize { m: 250, n: 242 }),
                                    (6, MaxPayloadSize { m: 250, n: 242 }),
                                    (7, MaxPayloadSize { m: 250, n: 242 }),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        ),
                        (
                            MacVersion::LORAWAN_1_1_0, // B
                            [(
                                Revision::Latest,
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 250, n: 242 }),
                                    (5, MaxPayloadSize { m: 250, n: 242 }),
                                    (6, MaxPayloadSize { m: 250, n: 242 }),
                                    (7, MaxPayloadSize { m: 250, n: 242 }),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        ),
                        (
                            MacVersion::Latest, // B
                            [(
                                Revision::Latest, // RP002-1.0.0, RP002-1.0.1, RP002-1.0.2, RP002-1.0.3
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 250, n: 242 }),
                                    (5, MaxPayloadSize { m: 250, n: 242 }),
                                    (6, MaxPayloadSize { m: 250, n: 242 }),
                                    (7, MaxPayloadSize { m: 250, n: 242 }),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            )]
                            .iter()
                            .cloned()
                            .collect(),
                        ),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                },
                rx1_data_rate_table: [
                    (0, vec![0, 0, 0, 0, 0, 0]),
                    (1, vec![1, 0, 0, 0, 0, 0]),
                    (2, vec![2, 1, 0, 0, 0, 0]),
                    (3, vec![3, 2, 1, 0, 0, 0]),
                    (4, vec![4, 3, 2, 1, 0, 0]),
                    (5, vec![5, 4, 3, 2, 1, 0]),
                    (6, vec![6, 5, 4, 3, 2, 1]),
                    (7, vec![7, 6, 5, 4, 3, 2]),
                ]
                .iter()
                .cloned()
                .collect(),
                tx_power_offsets: vec![
                    0,   // 0
                    -2,  // 1
                    -4,  // 2
                    -6,  // 3
                    -8,  // 4
                    -10, // 5
                    -12, // 6
                    -14, // 7
                ],
                uplink_channels: vec![
                    Channel {
                        frequency: 865100000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                    Channel {
                        frequency: 865300000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
					Channel {
                        frequency: 865500000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                    Channel {
                        frequency: 865700000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
					Channel {
                        frequency: 865900000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                ],
                downlink_channels: vec![
                    Channel {
                        frequency: 865100000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                    Channel {
                        frequency: 865300000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
					Channel {
                        frequency: 865500000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                    Channel {
                        frequency: 865700000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
					Channel {
                        frequency: 865900000,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                ],
            },
        }
    }
}

impl Region for Configuration {
    fn get_name(&self) -> CommonName {
        CommonName::KZ865
    }

    fn get_rx1_channel_index_for_uplink_channel_index(
        &self,
        uplink_channel: usize,
    ) -> Result<usize> {
        Ok(uplink_channel)
    }

    fn get_rx1_frequency_for_uplink_frequency(&self, uplink_freq: u32) -> Result<u32> {
        Ok(uplink_freq)
    }

    fn get_ping_slot_frequency(&self, _dev_addr: DevAddr, _beacon_time: Duration) -> Result<u32> {
        Ok(866700000)
    }

    fn get_downlink_tx_power(&self, _freq: u32) -> isize {
        14
    }

    fn get_defaults(&self) -> Defaults {
        Defaults {
            rx2_frequency: 866700000,
            rx2_dr: 0,
            rx1_delay: Duration::from_secs(1),
            rx2_delay: Duration::from_secs(2),
            join_accept_delay1: Duration::from_secs(5),
            join_accept_delay2: Duration::from_secs(6),
        }
    }

    fn implements_tx_param_setup(&self, _mac_version: MacVersion) -> bool {
        false
    }

    fn get_data_rate_index(&self, uplink: bool, modulation: &DataRateModulation) -> Result<u8> {
        self.base.get_data_rate_index(uplink, modulation)
    }

    fn get_data_rate(&self, dr: u8) -> Result<DataRateModulation> {
        self.base.get_data_rate(dr)
    }

    fn get_max_payload_size(
        &self,
        mac_version: MacVersion,
        reg_params_revision: Revision,
        dr: u8,
    ) -> Result<MaxPayloadSize> {
        self.base
            .get_max_payload_size(mac_version, reg_params_revision, dr)
    }

    fn get_rx1_data_rate_index(&self, uplink_dr: u8, rx1_dr_offset: usize) -> Result<u8> {
        self.base.get_rx1_data_rate_index(uplink_dr, rx1_dr_offset)
    }

    fn get_tx_power_offset(&self, tx_power: usize) -> Result<isize> {
        self.base.get_tx_power_offset(tx_power)
    }

    fn add_channel(&mut self, frequency: u32, min_dr: u8, max_dr: u8) -> Result<()> {
        self.base.add_channel(frequency, min_dr, max_dr)
    }

    fn get_uplink_channel(&self, channel: usize) -> Result<Channel> {
        self.base.get_uplink_channel(channel)
    }

    fn get_uplink_channel_index(&self, frequency: u32, user_defined: bool) -> Result<usize> {
        self.base.get_uplink_channel_index(frequency, user_defined)
    }

    fn get_uplink_channel_index_for_freq_dr(&self, frequency: u32, dr: u8) -> Result<usize> {
        self.base
            .get_uplink_channel_index_for_freq_dr(frequency, dr)
    }

    fn get_downlink_channel(&self, channel: usize) -> Result<Channel> {
        self.base.get_downlink_channel(channel)
    }

    fn disable_uplink_channel_index(&mut self, channel: usize) -> Result<()> {
        self.base.disable_uplink_channel_index(channel)
    }

    fn enable_uplink_channel_index(&mut self, channel: usize) -> Result<()> {
        self.base.enable_uplink_channel_index(channel)
    }

    fn get_uplink_channel_indices(&self) -> Vec<usize> {
        self.base.get_uplink_channel_indices()
    }

    fn get_default_uplink_channel_indices(&self) -> Vec<usize> {
        self.base.get_default_uplink_channel_indices()
    }

    fn get_user_defined_uplink_channel_indices(&self) -> Vec<usize> {
        self.base.get_user_defined_uplink_channel_indices()
    }

    fn get_enabled_uplink_channel_indices(&self) -> Vec<usize> {
        self.base.get_enabled_uplink_channel_indices()
    }

    fn get_disabled_uplink_channel_indices(&self) -> Vec<usize> {
        self.base.get_disabled_uplink_channel_indices()
    }

    fn get_enabled_uplink_data_rates(&self) -> Vec<u8> {
        self.base.get_enabled_uplink_data_rates()
    }

    fn get_cf_list(&self, mac_version: MacVersion) -> Option<CFList> {
        self.base.get_cf_list(mac_version)
    }

    fn get_link_adr_req_payloads_for_enabled_uplink_channel_indices(
        &self,
        device_enabled_channels: &[usize],
    ) -> Vec<LinkADRReqPayload> {
        self.base
            .get_link_adr_req_payloads_for_enabled_uplink_channel_indices(device_enabled_channels)
    }

    fn get_enabled_uplink_channel_indices_for_link_adr_payloads(
        &self,
        device_enabled_channels: &[usize],
        pls: &[LinkADRReqPayload],
    ) -> Result<Vec<usize>> {
        self.base
            .get_enabled_uplink_channel_indices_for_link_adr_payloads(device_enabled_channels, pls)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    fn config() -> Configuration {
        Configuration::new(false)
    }

    fn config_user_channels() -> Configuration {
        let mut c = Configuration::new(false);
        c.add_channel(864100000, 0, 5).unwrap();
        c.add_channel(864300000, 0, 5).unwrap();
        c.add_channel(864500000, 0, 5).unwrap();
        c.add_channel(864700000, 0, 5).unwrap();
        c.add_channel(864900000, 0, 5).unwrap();
        c
    }

    #[test]
    fn test_get_ping_slot_freq() {
        let c = config();
        let dev_addr = DevAddr::from_be_bytes([3, 20, 207, 54]);
        let beacon_time = Duration::from_secs((334382 * 60 * 60) + (52 * 60) + 44);

        let freq = c.get_ping_slot_frequency(dev_addr, beacon_time).unwrap();
        assert_eq!(868900000, freq);
    }

    #[test]
    fn get_rx1_channel_index_for_uplink_channel_index() {
        let c = config();
        assert_eq!(
            3,
            c.get_rx1_channel_index_for_uplink_channel_index(3).unwrap()
        );
    }

    #[test]
    fn get_rx1_frequency_for_uplink_frequency() {
        let c = config();
        assert_eq!(
            868900000,
            c.get_rx1_frequency_for_uplink_frequency(868900000).unwrap()
        );
    }

    #[test]
    fn test_get_user_channel_indices() {
        let c = config();
        assert_eq!(0, c.get_user_defined_uplink_channel_indices().len());

        let c = config_user_channels();
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            c.get_user_defined_uplink_channel_indices()
        );
    }

    #[test]
    fn test_get_link_adr_req_payloads() {
        struct Test {
            device_channels: Vec<usize>,
            disabled_channels: Vec<usize>,
            expected_uplink_channels: Vec<usize>,
            expected_link_adr_req_payloads: Vec<LinkADRReqPayload>,
        }

        let tests = vec![
            // No device channels
            // In this case we only activate the base channels
            Test {
                device_channels: vec![],
                disabled_channels: vec![],
                expected_uplink_channels: vec![0, 1],
                expected_link_adr_req_payloads: vec![LinkADRReqPayload {
                    dr: 0,
                    tx_power: 0,
                    redundancy: Redundancy {
                        ch_mask_cntl: 0,
                        nb_rep: 0,
                    },
                    ch_mask: ChMask::from_slice(&[true, true]).unwrap(),
                }],
            },
            // Base channels are active
            // we do not activate the CFList channels as we don't
            // now if the node knows about these frequencies
            Test {
                device_channels: vec![0, 1],
                disabled_channels: vec![],
                expected_uplink_channels: vec![0, 1],
                expected_link_adr_req_payloads: vec![],
            },
            // Base channels + two CFList channels are active
            // we do not activate the CFList channels as we don't
            // now if the node knows about these frequencies
            Test {
                device_channels: vec![0, 1, 2, 3],
                disabled_channels: vec![],
                expected_uplink_channels: vec![0, 1, 2, 3],
                expected_link_adr_req_payloads: vec![],
            },
            // Everything is in sync
            Test {
                device_channels: vec![0, 1, 2, 3, 4, 5, 6],
                disabled_channels: vec![],
                expected_uplink_channels: vec![0, 1, 2, 3, 4, 5, 6],
                expected_link_adr_req_payloads: vec![],
            },
            // Base channels + CFList are active on node,
            // but CFList channels are disabled on the network.
            Test {
                device_channels: vec![0, 1, 2, 3, 4, 5, 6],
                disabled_channels: vec![2, 3, 4, 5, 6],
                expected_uplink_channels: vec![0, 1],
                expected_link_adr_req_payloads: vec![LinkADRReqPayload {
                    dr: 0,
                    tx_power: 0,
                    redundancy: Redundancy {
                        ch_mask_cntl: 0,
                        nb_rep: 0,
                    },
                    ch_mask: ChMask::from_slice(&[true, true]).unwrap(),
                }],
            },
        ];

        for test in &tests {
            let mut c = config_user_channels();
            for i in &test.disabled_channels {
                c.disable_uplink_channel_index(*i).unwrap();
            }

            let pls = c.get_link_adr_req_payloads_for_enabled_uplink_channel_indices(
                &test.device_channels,
            );
            assert_eq!(test.expected_link_adr_req_payloads, pls);

            let channels = c
                .get_enabled_uplink_channel_indices_for_link_adr_payloads(
                    &test.device_channels,
                    &pls,
                )
                .unwrap();
            assert_eq!(test.expected_uplink_channels, channels);
        }
    }

    #[test]
    fn get_cf_list() {
        let c = config_user_channels();
        assert_eq!(
            CFList::Channels(
                CFListChannels::from_slice(&[
                    864100000, 864300000, 864500000, 864700000, 864900000,
                ])
                .unwrap(),
            ),
            c.get_cf_list(MacVersion::LORAWAN_1_0_4).unwrap(),
        );
    }
}