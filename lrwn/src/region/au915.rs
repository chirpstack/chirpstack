use std::time::Duration;

use anyhow::Result;

use super::{
    Channel, CommonName, DataRate, DataRateModulation, Defaults, LinkADRReqPayload, LoraDataRate,
    LrFhssDataRate, MacVersion, MaxPayloadSize, Region, RegionBaseConfig, Revision,
};
use crate::{CFList, ChMask, DevAddr, Redundancy};

pub struct Configuration {
    base: RegionBaseConfig,
}

impl Configuration {
    pub fn new(repeater_compatible: bool, dwell_time_400ms: bool) -> Self {
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
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 12,
                                bandwidth: 125000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        1,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 11,
                                bandwidth: 125000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        2,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 10,
                                bandwidth: 125000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        3,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 9,
                                bandwidth: 125000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        4,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 8,
                                bandwidth: 125000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        5,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 7,
                                bandwidth: 125000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        6,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 8,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        7,
                        DataRate {
                            uplink: true,
                            downlink: false,
                            modulation: DataRateModulation::LrFhss(LrFhssDataRate {
                                coding_rate: "2/6".to_string(),
                                occupied_channel_width: 1523000,
                            }),
                        },
                    ),
                    (
                        8,
                        DataRate {
                            uplink: false,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 12,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        9,
                        DataRate {
                            uplink: false,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 11,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        10,
                        DataRate {
                            uplink: false,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 10,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        11,
                        DataRate {
                            uplink: false,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 9,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        12,
                        DataRate {
                            uplink: false,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 8,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                    (
                        13,
                        DataRate {
                            uplink: false,
                            downlink: true,
                            modulation: DataRateModulation::Lora(LoraDataRate {
                                spreading_factor: 7,
                                bandwidth: 500000,
                                coding_rate: "4/5".into(),
                            }),
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                max_payload_size_per_dr: match repeater_compatible {
                    true => match dwell_time_400ms {
                        // repeater + dwell time
                        true => [
                            (
                                MacVersion::LORAWAN_1_0_3, // LoRaWAN < 1.0.3 + < LoRaWAN 1.1.0B does not have dwell-time
                                [(
                                    Revision::Latest, // A
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
                                        (5, MaxPayloadSize { m: 250, n: 242 }),
                                        (6, MaxPayloadSize { m: 250, n: 242 }),
                                        // 7
                                        (8, MaxPayloadSize { m: 41, n: 33 }),
                                        (9, MaxPayloadSize { m: 117, n: 109 }),
                                        (10, MaxPayloadSize { m: 230, n: 222 }),
                                        (11, MaxPayloadSize { m: 230, n: 222 }),
                                        (12, MaxPayloadSize { m: 230, n: 222 }),
                                        (13, MaxPayloadSize { m: 230, n: 222 }),
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
                                    Revision::Latest, // B
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
                                        (5, MaxPayloadSize { m: 250, n: 242 }),
                                        (6, MaxPayloadSize { m: 250, n: 242 }),
                                        // 7
                                        (8, MaxPayloadSize { m: 41, n: 33 }),
                                        (9, MaxPayloadSize { m: 117, n: 109 }),
                                        (10, MaxPayloadSize { m: 230, n: 222 }),
                                        (11, MaxPayloadSize { m: 230, n: 222 }),
                                        (12, MaxPayloadSize { m: 230, n: 222 }),
                                        (13, MaxPayloadSize { m: 230, n: 222 }),
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
                                            (0, MaxPayloadSize { m: 0, n: 0 }),
                                            (1, MaxPayloadSize { m: 0, n: 0 }),
                                            (2, MaxPayloadSize { m: 19, n: 11 }),
                                            (3, MaxPayloadSize { m: 61, n: 53 }),
                                            (4, MaxPayloadSize { m: 133, n: 125 }),
                                            (5, MaxPayloadSize { m: 230, n: 222 }),
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 41, n: 33 }),
                                            (9, MaxPayloadSize { m: 117, n: 109 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::RP002_1_0_1,
                                        [
                                            (0, MaxPayloadSize { m: 0, n: 0 }),
                                            (1, MaxPayloadSize { m: 0, n: 0 }),
                                            (2, MaxPayloadSize { m: 19, n: 11 }),
                                            (3, MaxPayloadSize { m: 61, n: 53 }),
                                            (4, MaxPayloadSize { m: 133, n: 125 }),
                                            (5, MaxPayloadSize { m: 230, n: 222 }),
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 41, n: 33 }),
                                            (9, MaxPayloadSize { m: 117, n: 109 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::Latest, // RP002-1.0.2
                                        [
                                            (0, MaxPayloadSize { m: 0, n: 0 }),
                                            (1, MaxPayloadSize { m: 0, n: 0 }),
                                            (2, MaxPayloadSize { m: 19, n: 11 }),
                                            (3, MaxPayloadSize { m: 61, n: 53 }),
                                            (4, MaxPayloadSize { m: 133, n: 125 }),
                                            (5, MaxPayloadSize { m: 230, n: 222 }),
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            (7, MaxPayloadSize { m: 58, n: 50 }),
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
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
                        // repeater + no dwell time
                        false => [
                            (
                                MacVersion::LORAWAN_1_0_1,
                                [(
                                    Revision::Latest,
                                    [
                                        (0, MaxPayloadSize { m: 19, n: 11 }),
                                        (1, MaxPayloadSize { m: 61, n: 53 }),
                                        (2, MaxPayloadSize { m: 134, n: 126 }),
                                        (3, MaxPayloadSize { m: 250, n: 242 }),
                                        (4, MaxPayloadSize { m: 250, n: 242 }),
                                        // 5 - 7
                                        (8, MaxPayloadSize { m: 41, n: 33 }),
                                        (9, MaxPayloadSize { m: 117, n: 109 }),
                                        (10, MaxPayloadSize { m: 230, n: 222 }),
                                        (11, MaxPayloadSize { m: 230, n: 222 }),
                                        (12, MaxPayloadSize { m: 230, n: 222 }),
                                        (13, MaxPayloadSize { m: 230, n: 222 }),
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
                                [
                                    (
                                        Revision::A,
                                        [
                                            (0, MaxPayloadSize { m: 19, n: 11 }),
                                            (1, MaxPayloadSize { m: 61, n: 53 }),
                                            (2, MaxPayloadSize { m: 134, n: 126 }),
                                            (3, MaxPayloadSize { m: 250, n: 242 }),
                                            (4, MaxPayloadSize { m: 250, n: 242 }),
                                            // 5 - 7
                                            (8, MaxPayloadSize { m: 41, n: 33 }),
                                            (9, MaxPayloadSize { m: 117, n: 109 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::Latest, // B
                                        [
                                            (0, MaxPayloadSize { m: 59, n: 51 }),
                                            (1, MaxPayloadSize { m: 59, n: 51 }),
                                            (2, MaxPayloadSize { m: 59, n: 51 }),
                                            (3, MaxPayloadSize { m: 123, n: 115 }),
                                            (4, MaxPayloadSize { m: 230, n: 222 }),
                                            (5, MaxPayloadSize { m: 230, n: 222 }),
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 41, n: 33 }),
                                            (9, MaxPayloadSize { m: 117, n: 109 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
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
                                        // 7
                                        (8, MaxPayloadSize { m: 41, n: 33 }),
                                        (9, MaxPayloadSize { m: 117, n: 109 }),
                                        (10, MaxPayloadSize { m: 230, n: 222 }),
                                        (11, MaxPayloadSize { m: 230, n: 222 }),
                                        (12, MaxPayloadSize { m: 230, n: 222 }),
                                        (13, MaxPayloadSize { m: 230, n: 222 }),
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
                                        (6, MaxPayloadSize { m: 230, n: 222 }),
                                        // 7
                                        (8, MaxPayloadSize { m: 41, n: 33 }),
                                        (9, MaxPayloadSize { m: 117, n: 109 }),
                                        (10, MaxPayloadSize { m: 230, n: 222 }),
                                        (11, MaxPayloadSize { m: 230, n: 222 }),
                                        (12, MaxPayloadSize { m: 230, n: 222 }),
                                        (13, MaxPayloadSize { m: 230, n: 222 }),
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
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 41, n: 33 }),
                                            (9, MaxPayloadSize { m: 117, n: 109 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::RP002_1_0_1,
                                        [
                                            (0, MaxPayloadSize { m: 59, n: 51 }),
                                            (1, MaxPayloadSize { m: 59, n: 51 }),
                                            (2, MaxPayloadSize { m: 59, n: 51 }),
                                            (3, MaxPayloadSize { m: 123, n: 115 }),
                                            (4, MaxPayloadSize { m: 230, n: 222 }),
                                            (5, MaxPayloadSize { m: 230, n: 222 }),
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 41, n: 33 }),
                                            (9, MaxPayloadSize { m: 117, n: 109 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::Latest, // RP002-1.0.2
                                        [
                                            (0, MaxPayloadSize { m: 59, n: 51 }),
                                            (1, MaxPayloadSize { m: 59, n: 51 }),
                                            (2, MaxPayloadSize { m: 59, n: 51 }),
                                            (3, MaxPayloadSize { m: 123, n: 115 }),
                                            (4, MaxPayloadSize { m: 230, n: 222 }),
                                            (5, MaxPayloadSize { m: 230, n: 222 }),
                                            (6, MaxPayloadSize { m: 230, n: 222 }),
                                            (7, MaxPayloadSize { m: 58, n: 50 }),
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 230, n: 222 }),
                                            (11, MaxPayloadSize { m: 230, n: 222 }),
                                            (12, MaxPayloadSize { m: 230, n: 222 }),
                                            (13, MaxPayloadSize { m: 230, n: 222 }),
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
                    false => match dwell_time_400ms {
                        // no repeater + dwell time
                        true => [
                            (
                                MacVersion::LORAWAN_1_0_3, // LoRaWAN < 1.0.3 + < LoRaWAN 1.1.0B does not have dwell-time
                                [(
                                    Revision::Latest, // A
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
                                        (5, MaxPayloadSize { m: 250, n: 242 }),
                                        (6, MaxPayloadSize { m: 250, n: 242 }),
                                        // 7
                                        (8, MaxPayloadSize { m: 61, n: 53 }),
                                        (9, MaxPayloadSize { m: 137, n: 129 }),
                                        (10, MaxPayloadSize { m: 250, n: 242 }),
                                        (11, MaxPayloadSize { m: 250, n: 242 }),
                                        (12, MaxPayloadSize { m: 250, n: 242 }),
                                        (13, MaxPayloadSize { m: 250, n: 242 }),
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
                                    Revision::Latest, // B
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
                                        (5, MaxPayloadSize { m: 250, n: 242 }),
                                        (6, MaxPayloadSize { m: 250, n: 242 }),
                                        // 7
                                        (8, MaxPayloadSize { m: 61, n: 53 }),
                                        (9, MaxPayloadSize { m: 137, n: 129 }),
                                        (10, MaxPayloadSize { m: 250, n: 242 }),
                                        (11, MaxPayloadSize { m: 250, n: 242 }),
                                        (12, MaxPayloadSize { m: 250, n: 242 }),
                                        (13, MaxPayloadSize { m: 250, n: 242 }),
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
                                            (0, MaxPayloadSize { m: 0, n: 0 }),
                                            (1, MaxPayloadSize { m: 0, n: 0 }),
                                            (2, MaxPayloadSize { m: 19, n: 11 }),
                                            (3, MaxPayloadSize { m: 61, n: 53 }),
                                            (4, MaxPayloadSize { m: 133, n: 125 }),
                                            (5, MaxPayloadSize { m: 250, n: 242 }),
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 222 }),
                                            (11, MaxPayloadSize { m: 250, n: 250 }),
                                            (12, MaxPayloadSize { m: 250, n: 250 }),
                                            (13, MaxPayloadSize { m: 250, n: 250 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::RP002_1_0_1,
                                        [
                                            (0, MaxPayloadSize { m: 0, n: 0 }),
                                            (1, MaxPayloadSize { m: 0, n: 0 }),
                                            (2, MaxPayloadSize { m: 19, n: 11 }),
                                            (3, MaxPayloadSize { m: 61, n: 53 }),
                                            (4, MaxPayloadSize { m: 133, n: 125 }),
                                            (5, MaxPayloadSize { m: 250, n: 242 }),
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 242 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::Latest, // RP002-1.0.2
                                        [
                                            (0, MaxPayloadSize { m: 0, n: 0 }),
                                            (1, MaxPayloadSize { m: 0, n: 0 }),
                                            (2, MaxPayloadSize { m: 19, n: 11 }),
                                            (3, MaxPayloadSize { m: 61, n: 53 }),
                                            (4, MaxPayloadSize { m: 133, n: 125 }),
                                            (5, MaxPayloadSize { m: 250, n: 242 }),
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            (7, MaxPayloadSize { m: 58, n: 50 }),
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 242 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
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
                        // no repeater + no dwell time
                        false => [
                            (
                                MacVersion::LORAWAN_1_0_1,
                                [(
                                    Revision::Latest,
                                    [
                                        (0, MaxPayloadSize { m: 19, n: 11 }),
                                        (1, MaxPayloadSize { m: 61, n: 53 }),
                                        (2, MaxPayloadSize { m: 134, n: 126 }),
                                        (3, MaxPayloadSize { m: 250, n: 242 }),
                                        (4, MaxPayloadSize { m: 250, n: 242 }),
                                        // 5 - 7
                                        (8, MaxPayloadSize { m: 61, n: 53 }),
                                        (9, MaxPayloadSize { m: 137, n: 129 }),
                                        (10, MaxPayloadSize { m: 250, n: 242 }),
                                        (11, MaxPayloadSize { m: 250, n: 242 }),
                                        (12, MaxPayloadSize { m: 250, n: 242 }),
                                        (13, MaxPayloadSize { m: 250, n: 242 }),
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
                                [
                                    (
                                        Revision::A,
                                        [
                                            (0, MaxPayloadSize { m: 19, n: 11 }),
                                            (1, MaxPayloadSize { m: 61, n: 53 }),
                                            (2, MaxPayloadSize { m: 134, n: 126 }),
                                            (3, MaxPayloadSize { m: 250, n: 242 }),
                                            (4, MaxPayloadSize { m: 250, n: 242 }),
                                            // 5 - 7
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 242 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::Latest, // B
                                        [
                                            (0, MaxPayloadSize { m: 59, n: 51 }),
                                            (1, MaxPayloadSize { m: 59, n: 51 }),
                                            (2, MaxPayloadSize { m: 59, n: 51 }),
                                            (3, MaxPayloadSize { m: 123, n: 115 }),
                                            (4, MaxPayloadSize { m: 250, n: 242 }),
                                            (5, MaxPayloadSize { m: 250, n: 242 }),
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 242 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
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
                                        // 7
                                        (8, MaxPayloadSize { m: 61, n: 53 }),
                                        (9, MaxPayloadSize { m: 137, n: 129 }),
                                        (10, MaxPayloadSize { m: 250, n: 242 }),
                                        (11, MaxPayloadSize { m: 250, n: 242 }),
                                        (12, MaxPayloadSize { m: 250, n: 242 }),
                                        (13, MaxPayloadSize { m: 250, n: 242 }),
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
                                        (6, MaxPayloadSize { m: 250, n: 242 }),
                                        // 7
                                        (8, MaxPayloadSize { m: 61, n: 53 }),
                                        (9, MaxPayloadSize { m: 137, n: 129 }),
                                        (10, MaxPayloadSize { m: 250, n: 242 }),
                                        (11, MaxPayloadSize { m: 250, n: 242 }),
                                        (12, MaxPayloadSize { m: 250, n: 242 }),
                                        (13, MaxPayloadSize { m: 250, n: 242 }),
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
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 242 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::RP002_1_0_1,
                                        [
                                            (0, MaxPayloadSize { m: 59, n: 51 }),
                                            (1, MaxPayloadSize { m: 59, n: 51 }),
                                            (2, MaxPayloadSize { m: 59, n: 51 }),
                                            (3, MaxPayloadSize { m: 123, n: 115 }),
                                            (4, MaxPayloadSize { m: 250, n: 242 }),
                                            (5, MaxPayloadSize { m: 250, n: 242 }),
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            // 7
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 242 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    ),
                                    (
                                        Revision::Latest, // RP002-1.0.2
                                        [
                                            (0, MaxPayloadSize { m: 59, n: 51 }),
                                            (1, MaxPayloadSize { m: 59, n: 51 }),
                                            (2, MaxPayloadSize { m: 59, n: 51 }),
                                            (3, MaxPayloadSize { m: 123, n: 115 }),
                                            (4, MaxPayloadSize { m: 250, n: 242 }),
                                            (5, MaxPayloadSize { m: 250, n: 242 }),
                                            (6, MaxPayloadSize { m: 250, n: 242 }),
                                            (7, MaxPayloadSize { m: 58, n: 50 }),
                                            (8, MaxPayloadSize { m: 61, n: 53 }),
                                            (9, MaxPayloadSize { m: 137, n: 129 }),
                                            (10, MaxPayloadSize { m: 250, n: 222 }),
                                            (11, MaxPayloadSize { m: 250, n: 242 }),
                                            (12, MaxPayloadSize { m: 250, n: 242 }),
                                            (13, MaxPayloadSize { m: 250, n: 242 }),
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
                },
                rx1_data_rate_table: [
                    (0, vec![8, 8, 8, 8, 8, 8]),
                    (1, vec![9, 8, 8, 8, 8, 8]),
                    (2, vec![10, 9, 8, 8, 8, 8]),
                    (3, vec![11, 10, 9, 8, 8, 8]),
                    (4, vec![12, 11, 10, 9, 8, 8]),
                    (5, vec![13, 12, 11, 10, 9, 8]),
                    (6, vec![13, 13, 12, 11, 10, 9]),
                    (7, vec![9, 8, 8, 8, 8, 8]),
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
                    -16, // 8
                    -18, // 9
                    -20, // 10
                    -22, // 11
                    -24, // 12
                    -26, // 13
                    -28, // 14
                ],
                uplink_channels: vec![],
                downlink_channels: vec![],
            },
        };

        // initialize uplink channel 0 - 63
        for i in 0..64 {
            c.base.uplink_channels.push(Channel {
                frequency: 915200000 + (i * 200000),
                min_dr: 0,
                max_dr: 5,
                enabled: true,
                user_defined: false,
            });
        }

        // initialize uplink channel 64 - 71
        for i in 0..8 {
            c.base.uplink_channels.push(Channel {
                frequency: 915900000 + (i * 1600000),
                min_dr: 6,
                max_dr: 7,
                enabled: true,
                user_defined: false,
            });
        }

        // initialize downlink channel 0 - 7
        for i in 0..8 {
            c.base.downlink_channels.push(Channel {
                frequency: 923300000 + (i * 600000),
                min_dr: 8,
                max_dr: 13,
                enabled: true,
                user_defined: false,
            });
        }

        c
    }
}

impl Region for Configuration {
    fn get_name(&self) -> CommonName {
        CommonName::AU915
    }

    fn get_rx1_channel_index_for_uplink_channel_index(
        &self,
        uplink_channel: usize,
    ) -> Result<usize> {
        Ok(uplink_channel % 8)
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

        // Beaconing is performed on the same channel that normal downstream traffic as defined in the Class A specification.
        Ok(self.base.downlink_channels[down_channel].frequency)
    }

    fn get_downlink_tx_power(&self, _freq: u32) -> isize {
        27
    }

    fn get_defaults(&self) -> Defaults {
        Defaults {
            rx2_frequency: 923300000,
            rx2_dr: 8,
            rx1_delay: Duration::from_secs(1),
            rx2_delay: Duration::from_secs(2),
            join_accept_delay1: Duration::from_secs(5),
            join_accept_delay2: Duration::from_secs(6),
        }
    }

    fn implements_tx_param_setup(&self, mac_version: MacVersion) -> bool {
        !(mac_version == MacVersion::LORAWAN_1_0_1 || mac_version == MacVersion::LORAWAN_1_0_2)
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
        let payloads_a = self
            .base
            .get_link_adr_req_payloads_for_enabled_uplink_channel_indices(device_enabled_channels);

        let enabled_channels = self.get_enabled_uplink_channel_indices();

        let mut out = vec![LinkADRReqPayload {
            dr: 0,
            tx_power: 0,
            redundancy: Redundancy {
                ch_mask_cntl: 7,
                nb_rep: 0,
            },
            ch_mask: ChMask::new([false; 16]),
        }];

        let mut ch_mask_cntl: isize = -1;

        for c in &enabled_channels {
            let c = *c;

            // use the ChMask of the first LinkADRReqPayload, besides
            // turning off all 125 kHz this payload contains the ChMask
            // for the last block of channels.
            if c >= 64 {
                out[0].ch_mask.set(c % 16, true);
                continue;
            }

            if (c / 16) as isize != ch_mask_cntl {
                ch_mask_cntl = (c / 16) as isize;
                let mut pl = LinkADRReqPayload {
                    dr: 0,
                    tx_power: 0,
                    redundancy: Redundancy {
                        ch_mask_cntl: ch_mask_cntl as u8,
                        nb_rep: 0,
                    },
                    ch_mask: ChMask::new([false; 16]),
                };

                // set the channel mask for this block
                for ec in &enabled_channels {
                    let ec = *ec;
                    if (ec as isize) >= (ch_mask_cntl * 16)
                        && (ec as isize) < (ch_mask_cntl + 1) * 16
                    {
                        pl.ch_mask.set(ec % 16, true);
                    }
                }

                out.push(pl);
            }
        }

        if payloads_a.len() < out.len() {
            return payloads_a;
        }

        out
    }

    fn get_enabled_uplink_channel_indices_for_link_adr_payloads(
        &self,
        device_enabled_channels: &[usize],
        pls: &[LinkADRReqPayload],
    ) -> Result<Vec<usize>> {
        let mut ch_mask = vec![false; self.base.uplink_channels.len()];
        for i in device_enabled_channels {
            let i = *i;
            if i < ch_mask.len() {
                ch_mask[i] = true;
            }
        }

        for pl in pls {
            if pl.redundancy.ch_mask_cntl == 6 || pl.redundancy.ch_mask_cntl == 7 {
                for cm in ch_mask.iter_mut().take(64) {
                    if pl.redundancy.ch_mask_cntl == 6 {
                        *cm = true;
                    } else {
                        *cm = false;
                    }
                }

                for (i, cm) in pl.ch_mask.into_iter().enumerate() {
                    if i < 8 {
                        ch_mask[64 + i] = cm;
                    }
                }
            } else {
                for (i, enabled) in pl.ch_mask.into_iter().enumerate() {
                    let chan_num = (pl.redundancy.ch_mask_cntl * 16) as usize + i;

                    if chan_num >= ch_mask.len() {
                        if !enabled {
                            continue;
                        }

                        return Err(anyhow!("Channel does not exist"));
                    }

                    ch_mask[chan_num] = enabled;
                }
            }
        }

        // turn the ch_mask into a slice of enabled channel numbers
        let mut out: Vec<usize> = Vec::new();
        for (i, enabled) in ch_mask.iter().enumerate() {
            if *enabled {
                out.push(i);
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::*;

    fn config_full() -> Configuration {
        let c = Configuration::new(false, false);
        c
    }

    fn config_chan_8_15() -> Configuration {
        let mut c = Configuration::new(false, false);
        for i in 0..72 {
            c.disable_uplink_channel_index(i).unwrap();
        }
        for i in 8..16 {
            c.enable_uplink_channel_index(i).unwrap();
        }
        c
    }

    #[test]
    fn ping_slot_freq() {
        let c = config_full();
        let dev_addr = DevAddr::from_be_bytes([3, 20, 207, 54]);
        let beacon_time = Duration::from_secs((334382 * 60 * 60) + (52 * 60) + 44);

        let freq = c.get_ping_slot_frequency(dev_addr, beacon_time).unwrap();
        assert_eq!(925700000, freq);
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
                freq: 915200000,
                min_dr: 0,
                max_dr: 5,
            },
            Test {
                channel: 63,
                freq: 927800000,
                min_dr: 0,
                max_dr: 5,
            },
            Test {
                channel: 64,
                freq: 915900000,
                min_dr: 6,
                max_dr: 7,
            },
            Test {
                channel: 71,
                freq: 927100000,
                min_dr: 6,
                max_dr: 7,
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
                freq: 915900000,
                channel: 64,
                exp_freq: 923300000,
            },
            Test {
                freq: 915200000,
                channel: 0,
                exp_freq: 923300000,
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
    fn test_get_channel_indices() {
        let c = config_full();
        assert_eq!(72, c.get_enabled_uplink_channel_indices().len());
        assert_eq!(0, c.get_disabled_uplink_channel_indices().len());

        let c = config_chan_8_15();
        assert_eq!(
            vec![8, 9, 10, 11, 12, 13, 14, 15],
            c.get_enabled_uplink_channel_indices()
        );

        assert_eq!(72 - 8, c.get_disabled_uplink_channel_indices().len());
    }

    #[test]
    fn test_cf_list() {
        let c = config_chan_8_15();
        assert_eq!(true, c.get_cf_list(MacVersion::LORAWAN_1_0_2).is_none());

        let lw_11_cf_list = c.get_cf_list(MacVersion::LORAWAN_1_1_0).unwrap();
        assert_eq!(
            CFList::ChannelMask(CFListChannelMasks::new(vec![
                ChMask::new([
                    false, false, false, false, false, false, false, false, true, true, true, true,
                    true, true, true, true,
                ]),
                ChMask::new([false; 16]),
                ChMask::new([false; 16]),
                ChMask::new([false; 16]),
                ChMask::new([false; 16]),
            ],)),
            lw_11_cf_list
        );
    }

    #[test]
    fn test_get_dr_index() {
        let c = config_full();

        struct Test {
            dr_modulation: DataRateModulation,
            uplink: bool,
            expected_dr: u8,
        }

        let tests = vec![
            Test {
                dr_modulation: DataRateModulation::Lora(LoraDataRate {
                    spreading_factor: 12,
                    bandwidth: 125000,
                    coding_rate: "4/5".into(),
                }),
                uplink: true,
                expected_dr: 0,
            },
            Test {
                dr_modulation: DataRateModulation::Lora(LoraDataRate {
                    spreading_factor: 12,
                    bandwidth: 500000,
                    coding_rate: "4/5".into(),
                }),
                uplink: false,
                expected_dr: 8,
            },
            Test {
                dr_modulation: DataRateModulation::Lora(LoraDataRate {
                    spreading_factor: 8,
                    bandwidth: 500000,
                    coding_rate: "4/5".into(),
                }),
                uplink: true,
                expected_dr: 6,
            },
            Test {
                dr_modulation: DataRateModulation::Lora(LoraDataRate {
                    spreading_factor: 8,
                    bandwidth: 500000,
                    coding_rate: "4/5".into(),
                }),
                uplink: false,
                expected_dr: 12,
            },
        ];

        for tst in &tests {
            let dr = c
                .get_data_rate_index(tst.uplink, &tst.dr_modulation)
                .unwrap();
            assert_eq!(tst.expected_dr, dr);
        }
    }

    #[test]
    fn test_get_link_adr_req_payloads() {
        let c = config_full();

        struct Test {
            device_channels: Vec<usize>,
            enabled_channels: Vec<usize>,
            expected_uplink_channels: Vec<usize>,
            expected_link_adr_req_payloads: Vec<LinkADRReqPayload>,
        }

        let tests = vec![
            // All channels active.
            Test {
                device_channels: c.get_uplink_channel_indices(),
                enabled_channels: c.get_uplink_channel_indices(),
                expected_uplink_channels: c.get_uplink_channel_indices(),
                expected_link_adr_req_payloads: vec![],
            },
            // Only activate 0 - 7 + 64.
            Test {
                device_channels: c.get_uplink_channel_indices(),
                enabled_channels: vec![0, 1, 2, 3, 4, 5, 6, 7, 64],
                expected_uplink_channels: vec![0, 1, 2, 3, 4, 5, 6, 7, 64],
                expected_link_adr_req_payloads: vec![
                    LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        redundancy: Redundancy {
                            ch_mask_cntl: 7,
                            nb_rep: 0,
                        },
                        ch_mask: ChMask::new([
                            true, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false,
                        ]),
                    },
                    LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        redundancy: Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                        ch_mask: ChMask::new([
                            true, true, true, true, true, true, true, true, false, false, false,
                            false, false, false, false, false,
                        ]),
                    },
                ],
            },
            // Only activate 0 - 15 + 64 & 65.
            Test {
                device_channels: c.get_uplink_channel_indices(),
                enabled_channels: vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 64, 65,
                ],
                expected_uplink_channels: vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 64, 65,
                ],
                expected_link_adr_req_payloads: vec![
                    LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        redundancy: Redundancy {
                            ch_mask_cntl: 7,
                            nb_rep: 0,
                        },
                        ch_mask: ChMask::new([
                            true, true, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false,
                        ]),
                    },
                    LinkADRReqPayload {
                        dr: 0,
                        tx_power: 0,
                        redundancy: Redundancy {
                            ch_mask_cntl: 0,
                            nb_rep: 0,
                        },
                        ch_mask: ChMask::new([
                            true, true, true, true, true, true, true, true, true, true, true, true,
                            true, true, true, true,
                        ]),
                    },
                ],
            },
        ];

        for tst in &tests {
            let mut c = config_full();
            for i in 0..72 {
                c.disable_uplink_channel_index(i).unwrap();
            }

            for i in &tst.enabled_channels {
                c.enable_uplink_channel_index(*i).unwrap();
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
