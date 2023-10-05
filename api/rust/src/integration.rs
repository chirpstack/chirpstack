include!(concat!(env!("OUT_DIR"), "/integration/integration.rs"));
#[cfg(feature = "json")]
include!(concat!(
    env!("OUT_DIR"),
    "/integration/integration.serde.rs"
));

#[allow(clippy::from_over_into)]
impl Into<String> for LogLevel {
    fn into(self) -> String {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
        .to_string()
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for LogCode {
    fn into(self) -> String {
        match self {
            LogCode::Unknown => "UNKNOWN",
            LogCode::DownlinkPayloadSize => "DOWNLINK_PAYLOAD_SIZE",
            LogCode::UplinkCodec => "UPLINK_CODEC",
            LogCode::DownlinkCodec => "DOWNLINK_CODEC",
            LogCode::Otaa => "OTAA",
            LogCode::UplinkFCntReset => "UPLINK_F_CNT_RESET",
            LogCode::UplinkMic => "UPLINK_MIC",
            LogCode::UplinkFCntRetransmission => "UPLINK_F_CNT_RETRANSMISSION",
            LogCode::DownlinkGateway => "DOWNLINK_GATEWAY",
            LogCode::RelayNewEndDevice => "RELAY_NEW_END_DEVICE",
            LogCode::FCntDown => "F_CNT_DOWN",
        }
        .to_string()
    }
}
