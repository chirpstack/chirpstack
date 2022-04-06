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
        let mut c = Configuration {
            base: RegionBaseConfig {
                supports_user_channels: false,
                cf_list_min_dr: 0,
                cf_list_max_dr: 0,
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
                                bandwidth: 500000,
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
                            MacVersion::LORAWAN_1_0_1,
                            [(
                                Revision::Latest,
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
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
                            MacVersion::LORAWAN_1_0_2,
                            [(
                                Revision::Latest, // A & B
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
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
                            MacVersion::LORAWAN_1_1_0,
                            [(
                                Revision::Latest, // A & B
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
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
                            MacVersion::Latest,
                            [
                                (
                                    Revision::RP002_1_0_0,
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 59, n: 51 }),
                                        (3, MaxPayloadSize { m: 123, n: 115 }),
                                        (4, MaxPayloadSize { m: 230, n: 222 }),
                                        (5, MaxPayloadSize { m: 230, n: 222 }),
                                    ]
                                    .iter()
                                    .cloned()
                                    .collect(),
                                ),
                                (
                                    Revision::RP002_1_0_1,
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 31, n: 23 }),
                                        (2, MaxPayloadSize { m: 94, n: 86 }),
                                        (3, MaxPayloadSize { m: 172, n: 164 }),
                                        (4, MaxPayloadSize { m: 230, n: 222 }),
                                        (5, MaxPayloadSize { m: 230, n: 222 }),
                                        (6, MaxPayloadSize { m: 230, n: 222 }),
                                        (7, MaxPayloadSize { m: 230, n: 222 }),
                                    ]
                                    .iter()
                                    .cloned()
                                    .collect(),
                                ),
                                (
                                    Revision::Latest, // RP002-1.0.2, PR002-1.0.3
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 31, n: 23 }),
                                        (2, MaxPayloadSize { m: 94, n: 86 }),
                                        (3, MaxPayloadSize { m: 192, n: 184 }),
                                        (4, MaxPayloadSize { m: 230, n: 222 }),
                                        (5, MaxPayloadSize { m: 230, n: 222 }),
                                        (6, MaxPayloadSize { m: 230, n: 222 }),
                                        (7, MaxPayloadSize { m: 230, n: 222 }),
                                    ]
                                    .iter()
                                    .cloned()
                                    .collect(),
                                ),
                            ]
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
                            MacVersion::LORAWAN_1_0_1,
                            [(
                                Revision::Latest,
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 230, n: 222 }),
                                    (5, MaxPayloadSize { m: 230, n: 222 }),
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
                            MacVersion::LORAWAN_1_0_2,
                            [(
                                Revision::Latest, // A & B
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 250, n: 242 }),
                                    (5, MaxPayloadSize { m: 250, n: 242 }),
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
                            MacVersion::LORAWAN_1_1_0,
                            [(
                                Revision::Latest, // A & B
                                [
                                    (0, MaxPayloadSize { m: 59, n: 51 }),
                                    (1, MaxPayloadSize { m: 59, n: 51 }),
                                    (2, MaxPayloadSize { m: 59, n: 51 }),
                                    (3, MaxPayloadSize { m: 123, n: 115 }),
                                    (4, MaxPayloadSize { m: 250, n: 242 }),
                                    (5, MaxPayloadSize { m: 250, n: 242 }),
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
                            MacVersion::Latest,
                            [
                                (
                                    Revision::RP002_1_0_0,
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 59, n: 51 }),
                                        (3, MaxPayloadSize { m: 123, n: 115 }),
                                        (4, MaxPayloadSize { m: 250, n: 242 }),
                                        (5, MaxPayloadSize { m: 250, n: 242 }),
                                    ]
                                    .iter()
                                    .cloned()
                                    .collect(),
                                ),
                                (
                                    Revision::Latest, // PR002-1.0.1, RP002-1.0.2, PR002-1.0.3
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 31, n: 23 }),
                                        (2, MaxPayloadSize { m: 94, n: 86 }),
                                        (3, MaxPayloadSize { m: 192, n: 184 }),
                                        (4, MaxPayloadSize { m: 250, n: 242 }),
                                        (5, MaxPayloadSize { m: 250, n: 242 }),
                                        (6, MaxPayloadSize { m: 250, n: 242 }),
                                        (7, MaxPayloadSize { m: 250, n: 242 }),
                                    ]
                                    .iter()
                                    .cloned()
                                    .collect(),
                                ),
                            ]
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
                uplink_channels: vec![],
                downlink_channels: vec![],
            },
        };

        // initialize uplink channels
        for i in 0..96 {
            c.base.uplink_channels.push(Channel {
                frequency: 470300000 + (i * 200000),
                min_dr: 0,
                max_dr: 5,
                enabled: true,
                user_defined: false,
            });
        }

        // initialize downlink channels
        for i in 0..48 {
            c.base.downlink_channels.push(Channel {
                frequency: 500300000 + (i * 200000),
                min_dr: 0,
                max_dr: 5,
                enabled: true,
                user_defined: false,
            });
        }

        c
    }
}

impl Region for Configuration {
    fn get_name(&self) -> CommonName {
        CommonName::CN470
    }

    fn get_rx1_channel_index_for_uplink_channel_index(
        &self,
        uplink_channel: usize,
    ) -> Result<usize> {
        Ok(uplink_channel % 48)
    }

    fn get_rx1_frequency_for_uplink_frequency(&self, uplink_freq: u32) -> Result<u32> {
        let up_chan = self.get_uplink_channel_index(uplink_freq, false)?;
        let rx1_chan = self.get_rx1_channel_index_for_uplink_channel_index(up_chan)?;
        Ok(self.base.downlink_channels[rx1_chan].frequency)
    }

    fn get_ping_slot_frequency(&self, dev_addr: DevAddr, beacon_time: Duration) -> Result<u32> {
        let down_channel = (u32::from_be_bytes(dev_addr.to_be_bytes()) as usize
            + (beacon_time.as_secs() / 128) as usize)
            % 8;
        Ok(self.base.downlink_channels[down_channel].frequency)
    }

    fn get_downlink_tx_power(&self, _freq: u32) -> isize {
        14
    }

    fn get_defaults(&self) -> Defaults {
        Defaults {
            rx2_frequency: 505300000,
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
mod tests {
    use super::*;
    use crate::*;

    fn config_full() -> Configuration {
        let c = Configuration::new(false);
        c
    }

    #[test]
    fn test_get_ping_slot_freq() {
        let c = config_full();
        let dev_addr = DevAddr::from_be_bytes([3, 20, 207, 54]);
        let beacon_time = Duration::from_secs((334382 * 60 * 60) + (52 * 60) + 44);

        let freq = c.get_ping_slot_frequency(dev_addr, beacon_time).unwrap();
        assert_eq!(501100000, freq);
    }

    #[test]
    fn test_uplink_channels() {
        let c = config_full();

        struct Test {
            channel: usize,
            freq: u32,
            min_dr: u8,
            max_dr: u8,
        }

        let tests = vec![
            Test {
                channel: 0,
                freq: 470300000,
                min_dr: 0,
                max_dr: 5,
            },
            Test {
                channel: 95,
                freq: 489300000,
                min_dr: 0,
                max_dr: 5,
            },
        ];

        for tst in &tests {
            let chan = c.get_uplink_channel(tst.channel).unwrap();
            assert_eq!(tst.freq, chan.frequency);
            assert_eq!(tst.min_dr, chan.min_dr);
            assert_eq!(tst.max_dr, chan.max_dr);
        }
    }

    #[test]
    fn test_downlink_channels() {
        let c = config_full();

        struct Test {
            freq: u32,
            channel: usize,
            exp_freq: u32,
        }

        let tests = vec![
            Test {
                freq: 470300000,
                channel: 0,
                exp_freq: 500300000,
            },
            Test {
                freq: 489300000,
                channel: 95,
                exp_freq: 509700000,
            },
        ];

        for tst in &tests {
            let uplink_channel_index = c.get_uplink_channel_index(tst.freq, false).unwrap();
            assert_eq!(tst.channel, uplink_channel_index);

            let freq = c.get_rx1_frequency_for_uplink_frequency(tst.freq).unwrap();
            assert_eq!(tst.exp_freq, freq);
        }
    }

    #[test]
    fn test_get_link_adr_req_payloads() {
        let c = config_full();

        struct Test {
            device_channels: Vec<usize>,
            disabled_channels: Vec<usize>,
            expected_uplink_channels: Vec<usize>,
            expected_link_adr_req_payloads: Vec<LinkADRReqPayload>,
        }

        let tests = vec![
            // all channels active
            Test {
                device_channels: c.get_uplink_channel_indices(),
                disabled_channels: vec![],
                expected_uplink_channels: c.get_uplink_channel_indices(),
                expected_link_adr_req_payloads: vec![],
            },
            // channel 6, 38 and 48 disabled
            Test {
                device_channels: c.get_uplink_channel_indices(),
                disabled_channels: vec![6, 38, 45],
                expected_uplink_channels: {
                    let mut out: Vec<usize> = Vec::new();
                    for i in 0..96 {
                        if i == 6 || i == 38 || i == 45 {
                            continue;
                        }
                        out.push(i);
                    }
                    out
                },
                expected_link_adr_req_payloads: vec![
                    LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        redundancy: Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                        ch_mask: ChMask::new([
                            true, true, true, true, true, true, false, true, true, true, true,
                            true, true, true, true, true,
                        ]),
                    },
                    LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        redundancy: Redundancy {
                            ch_mask_cntl: 2,
                            nb_rep: 0,
                        },
                        ch_mask: ChMask::new([
                            true, true, true, true, true, true, false, true, true, true, true,
                            true, true, false, true, true,
                        ]),
                    },
                ],
            },
        ];

        for tst in &tests {
            let mut c = config_full();
            for i in &tst.disabled_channels {
                c.disable_uplink_channel_index(*i).unwrap();
            }

            let pls = c
                .get_link_adr_req_payloads_for_enabled_uplink_channel_indices(&tst.device_channels);
            assert_eq!(tst.expected_link_adr_req_payloads, pls);

            let channels = c
                .get_enabled_uplink_channel_indices_for_link_adr_payloads(
                    &tst.device_channels,
                    &pls,
                )
                .unwrap();
            assert_eq!(tst.expected_uplink_channels, channels);
        }
    }
}
