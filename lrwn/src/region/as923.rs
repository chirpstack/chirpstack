use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;

use super::{
    Channel, CommonName, DataRate, DataRateModulation, Defaults, FskDataRate, LinkADRReqPayload,
    LoraDataRate, MacVersion, MaxPayloadSize, Region, RegionBaseConfig, Revision,
};
use crate::{CFList, DevAddr};

pub struct Configuration {
    cn: CommonName,
    freq_offset: u32,
    dwell_time_400ms: bool,
    base: RegionBaseConfig,
}

impl Configuration {
    pub fn new(cn: CommonName, repeater_compatible: bool, dwell_time_400ms: bool) -> Configuration {
        let freq_offset: u32 = match cn {
            CommonName::AS923_2 => 1800000,
            CommonName::AS923_3 => 6600000,
            CommonName::AS923_4 => 5900000,
            _ => 0,
        };

        Configuration {
            cn,
            freq_offset,
            dwell_time_400ms,
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
                                coding_rate: "4/5".into(),
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
                                coding_rate: "4/5".into(),
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
                                coding_rate: "4/5".into(),
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
                                coding_rate: "4/5".into(),
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
                                coding_rate: "4/5".into(),
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
                                coding_rate: "4/5".into(),
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
                                coding_rate: "4/5".into(),
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
                    true => match dwell_time_400ms {
                        // repeater + dwell time
                        true => [
                            (
                                MacVersion::LORAWAN_1_0_2,
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
                                MacVersion::LORAWAN_1_0_3,
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
                                MacVersion::LORAWAN_1_1_0,
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_0, // RP002-1.0.0
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_1, // RP002-1.0.1
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_2, // RP002-1.0.2
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::Latest, // RP002-1.0.3
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                        // repeater + no dwell time
                        false => [
                            (
                                MacVersion::LORAWAN_1_0_2,
                                [(
                                    Revision::Latest, // B
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
                                MacVersion::LORAWAN_1_1_0,
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_0, // RP002-1.0.0
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_1, // RP002-1.0.1
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 123, n: 115 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_2, // RP002-1.0.2
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 123, n: 115 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::Latest, // RP002-1.0.3
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 123, n: 115 }),
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
                    },
                    false => match dwell_time_400ms {
                        // no repeater + dwell time
                        true => [
                            (
                                MacVersion::LORAWAN_1_0_2,
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
                                MacVersion::LORAWAN_1_0_3,
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
                                MacVersion::LORAWAN_1_1_0,
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_0, // RP002-1.0.0
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_1, // RP002-1.0.1
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_2, // RP002-1.0.2
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::Latest, // RP002-1.0.3
                                    [
                                        (0, MaxPayloadSize { m: 0, n: 0 }),
                                        (1, MaxPayloadSize { m: 0, n: 0 }),
                                        (2, MaxPayloadSize { m: 19, n: 11 }),
                                        (3, MaxPayloadSize { m: 61, n: 53 }),
                                        (4, MaxPayloadSize { m: 133, n: 125 }),
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
                        // no repeater + no dwell time
                        false => [
                            (
                                MacVersion::LORAWAN_1_0_2,
                                [(
                                    Revision::Latest, // B
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
                                MacVersion::LORAWAN_1_1_0,
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_0, // RP002-1.0.0
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_1, // RP002-1.0.1
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 123, n: 115 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::RP002_1_0_2, // RP002-1.0.2
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 123, n: 115 }),
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
                                MacVersion::Latest,
                                [(
                                    Revision::Latest, // RP002-1.0.3
                                    [
                                        (0, MaxPayloadSize { m: 59, n: 51 }),
                                        (1, MaxPayloadSize { m: 59, n: 51 }),
                                        (2, MaxPayloadSize { m: 123, n: 115 }),
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
                },
                rx1_data_rate_table: HashMap::new(), // implemented as function
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
                        frequency: 923200000 - freq_offset,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                    Channel {
                        frequency: 923400000 - freq_offset,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                ],
                downlink_channels: vec![
                    Channel {
                        frequency: 923200000 - freq_offset,
                        min_dr: 0,
                        max_dr: 5,
                        enabled: true,
                        user_defined: false,
                    },
                    Channel {
                        frequency: 923400000 - freq_offset,
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
        self.cn
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
        Ok(923400000 - self.freq_offset)
    }

    fn get_downlink_tx_power(&self, _freq: u32) -> isize {
        14
    }

    fn get_defaults(&self) -> Defaults {
        Defaults {
            rx2_frequency: 923200000 - self.freq_offset,
            rx2_dr: 2,
            rx1_delay: Duration::from_secs(1),
            rx2_delay: Duration::from_secs(2),
            join_accept_delay1: Duration::from_secs(5),
            join_accept_delay2: Duration::from_secs(6),
        }
    }

    fn implements_tx_param_setup(&self, _mac_version: MacVersion) -> bool {
        true
    }

    fn get_rx1_data_rate_index(&self, uplink_dr: u8, rx1_dr_offset: usize) -> Result<u8> {
        if uplink_dr > 7 {
            return Err(anyhow!("Invalid uplink data-rate: {}", uplink_dr));
        }

        let min_dr: u8 = match self.dwell_time_400ms {
            true => 2,
            false => 0,
        };

        let effective_rx1_dr_offset: isize = match rx1_dr_offset {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => -1,
            7 => -2,
            _ => {
                return Err(anyhow!("Invalid RX1 data-rate offset: {}", rx1_dr_offset));
            }
        };

        let dr = uplink_dr as isize - effective_rx1_dr_offset;
        let dr: u8 = if dr < min_dr as isize {
            min_dr
        } else {
            dr as u8
        } as u8;

        Ok(if dr > 5 { 5 } else { dr })
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
pub mod test {
    use super::*;
    use crate::*;

    fn config() -> Configuration {
        let c = Configuration::new(CommonName::AS923, true, true);
        c
    }

    #[test]
    fn test_defaults() {
        assert_eq!(923200000, config().get_defaults().rx2_frequency);
    }

    #[test]
    fn test_get_rx1_channel_index_for_uplink_channel_index() {
        let c = config();
        assert_eq!(
            3,
            c.get_rx1_channel_index_for_uplink_channel_index(3).unwrap()
        );
    }

    #[test]
    fn test_get_rx1_frequency_for_uplink_frequency() {
        let c = config();
        assert_eq!(
            923200000,
            c.get_rx1_frequency_for_uplink_frequency(923200000).unwrap()
        );
    }

    #[test]
    fn test_get_rx1_dr_dwell_time() {
        struct Test {
            uplink_dr: u8,
            rx1_dr_offset: usize,
            expected_dr: u8,
        }

        let tests = vec![
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 0,
                expected_dr: 5,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 1,
                expected_dr: 4,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 2,
                expected_dr: 3,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 3,
                expected_dr: 2,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 4,
                expected_dr: 2,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 5,
                expected_dr: 2,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 6,
                expected_dr: 5,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 7,
                expected_dr: 5,
            },
            Test {
                uplink_dr: 2,
                rx1_dr_offset: 6,
                expected_dr: 3,
            },
            Test {
                uplink_dr: 2,
                rx1_dr_offset: 7,
                expected_dr: 4,
            },
        ];

        let c = Configuration::new(CommonName::AS923, true, true);
        for tst in &tests {
            assert_eq!(
                tst.expected_dr,
                c.get_rx1_data_rate_index(tst.uplink_dr, tst.rx1_dr_offset)
                    .unwrap()
            );
        }
    }

    #[test]
    fn test_get_rx1_dr_no_dwell_time() {
        struct Test {
            uplink_dr: u8,
            rx1_dr_offset: usize,
            expected_dr: u8,
        }

        let tests = vec![
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 0,
                expected_dr: 5,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 1,
                expected_dr: 4,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 2,
                expected_dr: 3,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 3,
                expected_dr: 2,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 4,
                expected_dr: 1,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 5,
                expected_dr: 0,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 6,
                expected_dr: 5,
            },
            Test {
                uplink_dr: 5,
                rx1_dr_offset: 7,
                expected_dr: 5,
            },
            Test {
                uplink_dr: 2,
                rx1_dr_offset: 6,
                expected_dr: 3,
            },
            Test {
                uplink_dr: 2,
                rx1_dr_offset: 7,
                expected_dr: 4,
            },
        ];

        let c = Configuration::new(CommonName::AS923, true, false);
        for tst in &tests {
            assert_eq!(
                tst.expected_dr,
                c.get_rx1_data_rate_index(tst.uplink_dr, tst.rx1_dr_offset)
                    .unwrap()
            );
        }
    }

    #[test]
    fn test_as923_2() {
        let c = Configuration::new(CommonName::AS923_2, true, false);
        assert_eq!(CommonName::AS923_2, c.get_name());

        assert_eq!(923200000 - 1800000, c.get_defaults().rx2_frequency);
        assert_eq!(
            923400000 - 1800000,
            c.get_ping_slot_frequency(DevAddr::default(), Duration::default())
                .unwrap()
        );

        assert_eq!(
            923200000 - 1800000,
            c.get_uplink_channel(0).unwrap().frequency
        );
        assert_eq!(
            923200000 - 1800000,
            c.get_downlink_channel(0).unwrap().frequency
        );
        assert_eq!(
            923400000 - 1800000,
            c.get_uplink_channel(1).unwrap().frequency
        );
        assert_eq!(
            923400000 - 1800000,
            c.get_downlink_channel(1).unwrap().frequency
        );
    }

    #[test]
    fn test_as923_3() {
        let c = Configuration::new(CommonName::AS923_3, true, false);
        assert_eq!(CommonName::AS923_3, c.get_name());

        assert_eq!(923200000 - 6600000, c.get_defaults().rx2_frequency);
        assert_eq!(
            923400000 - 6600000,
            c.get_ping_slot_frequency(DevAddr::default(), Duration::default())
                .unwrap()
        );

        assert_eq!(
            923200000 - 6600000,
            c.get_uplink_channel(0).unwrap().frequency
        );
        assert_eq!(
            923200000 - 6600000,
            c.get_downlink_channel(0).unwrap().frequency
        );
        assert_eq!(
            923400000 - 6600000,
            c.get_uplink_channel(1).unwrap().frequency
        );
        assert_eq!(
            923400000 - 6600000,
            c.get_downlink_channel(1).unwrap().frequency
        );
    }

    #[test]
    fn test_as923_4() {
        let c = Configuration::new(CommonName::AS923_4, true, false);
        assert_eq!(CommonName::AS923_4, c.get_name());

        assert_eq!(923200000 - 5900000, c.get_defaults().rx2_frequency);
        assert_eq!(
            923400000 - 5900000,
            c.get_ping_slot_frequency(DevAddr::default(), Duration::default())
                .unwrap()
        );

        assert_eq!(
            923200000 - 5900000,
            c.get_uplink_channel(0).unwrap().frequency
        );
        assert_eq!(
            923200000 - 5900000,
            c.get_downlink_channel(0).unwrap().frequency
        );
        assert_eq!(
            923400000 - 5900000,
            c.get_uplink_channel(1).unwrap().frequency
        );
        assert_eq!(
            923400000 - 5900000,
            c.get_downlink_channel(1).unwrap().frequency
        );
    }
}
