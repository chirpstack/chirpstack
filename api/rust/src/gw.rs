use rand::Rng;

tonic::include_proto!("gw/gw");
include!(concat!(env!("OUT_DIR"), "/gw/gw.serde.rs"));

#[allow(clippy::from_over_into)]
impl Into<String> for TxAckStatus {
    fn into(self) -> String {
        match self {
            TxAckStatus::Ignored => "IGNORED",
            TxAckStatus::Ok => "OK",
            TxAckStatus::TooLate => "TOO_LATE",
            TxAckStatus::TooEarly => "TOO_EARLY",
            TxAckStatus::CollisionPacket => "COLLISION_PACKET",
            TxAckStatus::CollisionBeacon => "COLLISION_BEACON",
            TxAckStatus::TxFreq => "TX_FREQ",
            TxAckStatus::TxPower => "TX_POWER",
            TxAckStatus::GpsUnlocked => "GPS_UNLOCKED",
            TxAckStatus::QueueFull => "QUEUE_FULL",
            TxAckStatus::InternalError => "INTERNAL_ERROR",
        }
        .to_string()
    }
}

impl UplinkFrame {
    pub fn v4_migrate(&mut self) {
        if let Some(tx_info) = &self.tx_info_legacy {
            if self.tx_info.is_none() {
                self.tx_info = Some(UplinkTxInfo {
                    frequency: tx_info.frequency,
                    modulation: Some(Modulation {
                        parameters: tx_info.modulation_info.as_ref().map(|v| match v {
                            uplink_tx_info_legacy::ModulationInfo::LoraModulationInfo(info) => {
                                modulation::Parameters::Lora(LoraModulationInfo {
                                    bandwidth: info.bandwidth * 1000,
                                    spreading_factor: info.spreading_factor,
                                    code_rate: match info.code_rate_legacy.as_ref() {
                                        "4/5" => CodeRate::Cr45,
                                        "2/3" | "4/6" => CodeRate::Cr46,
                                        "4/7" => CodeRate::Cr47,
                                        "1/2" | "2/4" | "4/8" => CodeRate::Cr48,
                                        _ => CodeRate::CrUndefined,
                                    }
                                    .into(),
                                    code_rate_legacy: "".into(),
                                    polarization_inversion: info.polarization_inversion,
                                })
                            }
                            uplink_tx_info_legacy::ModulationInfo::FskModulationInfo(info) => {
                                modulation::Parameters::Fsk(info.clone())
                            }
                            uplink_tx_info_legacy::ModulationInfo::LrFhssModulationInfo(info) => {
                                modulation::Parameters::LrFhss(info.clone())
                            }
                        }),
                    }),
                });
                self.tx_info_legacy = None;
            }
        }

        if let Some(rx_info) = &self.rx_info_legacy {
            if self.rx_info.is_none() {
                let mut rng = rand::thread_rng();

                self.rx_info = Some(UplinkRxInfo {
                    gateway_id: hex::encode(&rx_info.gateway_id),
                    uplink_id: rng.gen::<u32>(),
                    time: rx_info.time.clone(),
                    time_since_gps_epoch: rx_info.time_since_gps_epoch.clone(),
                    fine_time_since_gps_epoch: None,
                    rssi: rx_info.rssi,
                    snr: rx_info.lora_snr as f32,
                    board: rx_info.board,
                    antenna: rx_info.antenna,
                    location: rx_info.location.clone(),
                    context: rx_info.context.clone(),
                    metadata: Some(pbjson_types::Struct {
                        fields: rx_info
                            .metadata
                            .iter()
                            .map(|(k, v)| {
                                (
                                    k.to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StringValue(
                                            v.to_string(),
                                        )),
                                    },
                                )
                            })
                            .collect(),
                    }),
                });
            }
        }
    }
}

impl UplinkRxInfo {
    pub fn get_metadata_string(&self, k: &str) -> Option<String> {
        if let Some(v) = &self.metadata {
            if let Some(v) = v.fields.get(k) {
                if let Some(pbjson_types::value::Kind::StringValue(v)) = &v.kind {
                    return Some(v.clone());
                }
            }
        }
        None
    }

    pub fn set_metadata_string(&mut self, k: &str, v: &str) {
        if self.metadata.is_none() {
            self.metadata = Some(pbjson_types::Struct {
                ..Default::default()
            });
        }

        if let Some(md) = &mut self.metadata {
            md.fields.insert(
                k.to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StringValue(v.to_string())),
                },
            );
        }
    }
}

impl DownlinkFrame {
    pub fn v4_migrate(&mut self) {
        self.gateway_id_legacy = hex::decode(&self.gateway_id).unwrap();
        self.downlink_id_legacy = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        self.downlink_id_legacy
            .extend_from_slice(&self.downlink_id.to_be_bytes());

        for i in self.items.iter_mut() {
            if i.tx_info_legacy.is_none() {
                if let Some(tx_info) = &i.tx_info {
                    let mut tx_info_legacy = DownlinkTxInfoLegacy {
                        frequency: tx_info.frequency,
                        power: tx_info.power,
                        board: tx_info.board,
                        antenna: tx_info.antenna,
                        context: tx_info.context.clone(),
                        ..Default::default()
                    };

                    if let Some(modulation) = &tx_info.modulation {
                        match &modulation.parameters {
                            Some(modulation::Parameters::Lora(v)) => {
                                tx_info_legacy.modulation = crate::common::Modulation::Lora.into();
                                tx_info_legacy.modulation_info = Some(
                                    downlink_tx_info_legacy::ModulationInfo::LoraModulationInfo(
                                        LoraModulationInfo {
                                            bandwidth: v.bandwidth / 1000,
                                            spreading_factor: v.spreading_factor,
                                            code_rate_legacy: match v.code_rate() {
                                                CodeRate::CrUndefined => "",
                                                CodeRate::Cr45 => "4/5",
                                                CodeRate::Cr46 => "4/6",
                                                CodeRate::Cr47 => "4/7",
                                                CodeRate::Cr48 => "4/8",
                                            }
                                            .into(),
                                            polarization_inversion: v.polarization_inversion,
                                            ..Default::default()
                                        },
                                    ),
                                );
                            }
                            Some(modulation::Parameters::Fsk(v)) => {
                                tx_info_legacy.modulation = crate::common::Modulation::Fsk.into();
                                tx_info_legacy.modulation_info = Some(
                                    downlink_tx_info_legacy::ModulationInfo::FskModulationInfo(
                                        FskModulationInfo {
                                            frequency_deviation: v.frequency_deviation,
                                            datarate: v.datarate,
                                        },
                                    ),
                                );
                            }
                            _ => {}
                        }
                    }

                    if let Some(timing) = &tx_info.timing {
                        match &timing.parameters {
                            Some(timing::Parameters::Immediately(v)) => {
                                tx_info_legacy.timing = DownlinkTiming::Immediately.into();
                                tx_info_legacy.timing_info = Some(
                                    downlink_tx_info_legacy::TimingInfo::ImmediatelyTimingInfo(
                                        v.clone(),
                                    ),
                                );
                            }
                            Some(timing::Parameters::Delay(v)) => {
                                tx_info_legacy.timing = DownlinkTiming::Delay.into();
                                tx_info_legacy.timing_info = Some(
                                    downlink_tx_info_legacy::TimingInfo::DelayTimingInfo(v.clone()),
                                );
                            }
                            Some(timing::Parameters::GpsEpoch(v)) => {
                                tx_info_legacy.timing = DownlinkTiming::GpsEpoch.into();
                                tx_info_legacy.timing_info =
                                    Some(downlink_tx_info_legacy::TimingInfo::GpsEpochTimingInfo(
                                        v.clone(),
                                    ));
                            }
                            _ => {}
                        }
                    }

                    i.tx_info_legacy = Some(tx_info_legacy);
                }
            }
        }
    }
}

impl DownlinkTxAck {
    pub fn v4_migrate(&mut self) {
        if self.gateway_id.is_empty() {
            self.gateway_id = hex::encode(&self.gateway_id_legacy);
        }

        if self.downlink_id == 0 && self.downlink_id_legacy.len() == 16 {
            self.downlink_id = u32::from_be_bytes([
                self.downlink_id_legacy[12],
                self.downlink_id_legacy[13],
                self.downlink_id_legacy[14],
                self.downlink_id_legacy[15],
            ])
        }
    }
}
