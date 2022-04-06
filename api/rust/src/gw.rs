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
