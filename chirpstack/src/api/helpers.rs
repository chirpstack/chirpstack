use chrono::{DateTime, Utc};

use crate::codec::Codec;
use crate::storage::fields::{MeasurementKind, MulticastGroupSchedulingType};
use crate::storage::{device::DeviceClass, metrics::Aggregation};
use chirpstack_api::{api, common};
use lrwn::region::{CommonName, MacVersion, Revision};

pub trait FromProto<T> {
    #[allow(clippy::wrong_self_convention)]
    fn from_proto(self) -> T;
}

pub trait ToProto<T> {
    fn to_proto(self) -> T;
}

impl FromProto<CommonName> for common::Region {
    fn from_proto(self) -> CommonName {
        match self {
            common::Region::Eu868 => CommonName::EU868,
            common::Region::Us915 => CommonName::US915,
            common::Region::Cn779 => CommonName::CN779,
            common::Region::Eu433 => CommonName::EU433,
            common::Region::Au915 => CommonName::AU915,
            common::Region::Cn470 => CommonName::CN470,
            common::Region::As923 => CommonName::AS923,
            common::Region::As9232 => CommonName::AS923_2,
            common::Region::As9233 => CommonName::AS923_3,
            common::Region::As9234 => CommonName::AS923_4,
            common::Region::Kr920 => CommonName::KR920,
            common::Region::In865 => CommonName::IN865,
            common::Region::Ru864 => CommonName::RU864,
            common::Region::Ism2400 => CommonName::ISM2400,
        }
    }
}

impl ToProto<common::Region> for CommonName {
    fn to_proto(self) -> common::Region {
        match self {
            CommonName::EU868 => common::Region::Eu868,
            CommonName::US915 => common::Region::Us915,
            CommonName::CN779 => common::Region::Cn779,
            CommonName::EU433 => common::Region::Eu433,
            CommonName::AU915 => common::Region::Au915,
            CommonName::CN470 => common::Region::Cn470,
            CommonName::AS923 => common::Region::As923,
            CommonName::AS923_2 => common::Region::As9232,
            CommonName::AS923_3 => common::Region::As9233,
            CommonName::AS923_4 => common::Region::As9234,
            CommonName::KR920 => common::Region::Kr920,
            CommonName::IN865 => common::Region::In865,
            CommonName::RU864 => common::Region::Ru864,
            CommonName::ISM2400 => common::Region::Ism2400,
        }
    }
}

impl FromProto<Revision> for common::RegParamsRevision {
    fn from_proto(self) -> Revision {
        match self {
            common::RegParamsRevision::A => Revision::A,
            common::RegParamsRevision::B => Revision::B,
            common::RegParamsRevision::Rp002100 => Revision::RP002_1_0_0,
            common::RegParamsRevision::Rp002101 => Revision::RP002_1_0_1,
            common::RegParamsRevision::Rp002102 => Revision::RP002_1_0_2,
            common::RegParamsRevision::Rp002103 => Revision::RP002_1_0_3,
        }
    }
}

impl ToProto<common::RegParamsRevision> for Revision {
    fn to_proto(self) -> common::RegParamsRevision {
        match self {
            Revision::A => common::RegParamsRevision::A,
            Revision::B => common::RegParamsRevision::B,
            Revision::RP002_1_0_0 => common::RegParamsRevision::Rp002100,
            Revision::RP002_1_0_1 => common::RegParamsRevision::Rp002101,
            Revision::RP002_1_0_2 => common::RegParamsRevision::Rp002102,
            Revision::RP002_1_0_3 | Revision::Latest => common::RegParamsRevision::Rp002103,
        }
    }
}

impl FromProto<MacVersion> for common::MacVersion {
    fn from_proto(self) -> MacVersion {
        match self {
            common::MacVersion::Lorawan100 => MacVersion::LORAWAN_1_0_0,
            common::MacVersion::Lorawan101 => MacVersion::LORAWAN_1_0_1,
            common::MacVersion::Lorawan102 => MacVersion::LORAWAN_1_0_2,
            common::MacVersion::Lorawan103 => MacVersion::LORAWAN_1_0_3,
            common::MacVersion::Lorawan104 => MacVersion::LORAWAN_1_0_4,
            common::MacVersion::Lorawan110 => MacVersion::LORAWAN_1_1_0,
        }
    }
}

impl ToProto<common::MacVersion> for MacVersion {
    fn to_proto(self) -> common::MacVersion {
        match self {
            MacVersion::LORAWAN_1_0_0 => common::MacVersion::Lorawan100,
            MacVersion::LORAWAN_1_0_1 => common::MacVersion::Lorawan101,
            MacVersion::LORAWAN_1_0_2 => common::MacVersion::Lorawan102,
            MacVersion::LORAWAN_1_0_3 => common::MacVersion::Lorawan103,
            MacVersion::LORAWAN_1_0_4 => common::MacVersion::Lorawan104,
            MacVersion::LORAWAN_1_1_0 | MacVersion::Latest => common::MacVersion::Lorawan110,
        }
    }
}

impl FromProto<lrwn::MACVersion> for common::MacVersion {
    fn from_proto(self) -> lrwn::MACVersion {
        match self {
            common::MacVersion::Lorawan100 => lrwn::MACVersion::LoRaWAN1_0,
            common::MacVersion::Lorawan101 => lrwn::MACVersion::LoRaWAN1_0,
            common::MacVersion::Lorawan102 => lrwn::MACVersion::LoRaWAN1_0,
            common::MacVersion::Lorawan103 => lrwn::MACVersion::LoRaWAN1_0,
            common::MacVersion::Lorawan104 => lrwn::MACVersion::LoRaWAN1_0,
            common::MacVersion::Lorawan110 => lrwn::MACVersion::LoRaWAN1_1,
        }
    }
}

impl ToProto<api::CodecRuntime> for Codec {
    fn to_proto(self) -> api::CodecRuntime {
        match self {
            Codec::NONE => api::CodecRuntime::None,
            Codec::CAYENNE_LPP => api::CodecRuntime::CayenneLpp,
            Codec::JS => api::CodecRuntime::Js,
        }
    }
}

impl FromProto<Codec> for api::CodecRuntime {
    fn from_proto(self) -> Codec {
        match self {
            api::CodecRuntime::None => Codec::NONE,
            api::CodecRuntime::CayenneLpp => Codec::CAYENNE_LPP,
            api::CodecRuntime::Js => Codec::JS,
        }
    }
}

impl ToProto<api::MeasurementKind> for MeasurementKind {
    fn to_proto(self) -> api::MeasurementKind {
        match self {
            MeasurementKind::UNKNOWN => api::MeasurementKind::Unknown,
            MeasurementKind::COUNTER => api::MeasurementKind::Counter,
            MeasurementKind::ABSOLUTE => api::MeasurementKind::Absolute,
            MeasurementKind::GAUGE => api::MeasurementKind::Gauge,
            MeasurementKind::STRING => api::MeasurementKind::String,
        }
    }
}

impl FromProto<MeasurementKind> for api::MeasurementKind {
    fn from_proto(self) -> MeasurementKind {
        match self {
            api::MeasurementKind::Unknown => MeasurementKind::UNKNOWN,
            api::MeasurementKind::Counter => MeasurementKind::COUNTER,
            api::MeasurementKind::Absolute => MeasurementKind::ABSOLUTE,
            api::MeasurementKind::Gauge => MeasurementKind::GAUGE,
            api::MeasurementKind::String => MeasurementKind::STRING,
        }
    }
}

impl ToProto<common::Aggregation> for Aggregation {
    fn to_proto(self) -> common::Aggregation {
        match self {
            Aggregation::HOUR => common::Aggregation::Hour,
            Aggregation::DAY => common::Aggregation::Day,
            Aggregation::MONTH => common::Aggregation::Month,
        }
    }
}

impl FromProto<Aggregation> for common::Aggregation {
    fn from_proto(self) -> Aggregation {
        match self {
            common::Aggregation::Hour => Aggregation::HOUR,
            common::Aggregation::Day => Aggregation::DAY,
            common::Aggregation::Month => Aggregation::MONTH,
        }
    }
}

impl ToProto<common::MType> for lrwn::MType {
    fn to_proto(self) -> common::MType {
        match self {
            lrwn::MType::JoinRequest => common::MType::JoinRequest,
            lrwn::MType::JoinAccept => common::MType::JoinAccept,
            lrwn::MType::UnconfirmedDataUp => common::MType::UnconfirmedDataUp,
            lrwn::MType::UnconfirmedDataDown => common::MType::UnconfirmedDataDown,
            lrwn::MType::ConfirmedDataUp => common::MType::ConfirmedDataUp,
            lrwn::MType::ConfirmedDataDown => common::MType::ConfirmedDataDown,
            lrwn::MType::RejoinRequest => common::MType::RejoinRequest,
            lrwn::MType::Proprietary => common::MType::Proprietary,
        }
    }
}

impl ToProto<api::MulticastGroupSchedulingType> for MulticastGroupSchedulingType {
    fn to_proto(self) -> api::MulticastGroupSchedulingType {
        match self {
            MulticastGroupSchedulingType::DELAY => api::MulticastGroupSchedulingType::Delay,
            MulticastGroupSchedulingType::GPS_TIME => api::MulticastGroupSchedulingType::GpsTime,
        }
    }
}

impl FromProto<MulticastGroupSchedulingType> for api::MulticastGroupSchedulingType {
    fn from_proto(self) -> MulticastGroupSchedulingType {
        match self {
            api::MulticastGroupSchedulingType::Delay => MulticastGroupSchedulingType::DELAY,
            api::MulticastGroupSchedulingType::GpsTime => MulticastGroupSchedulingType::GPS_TIME,
        }
    }
}

impl ToProto<api::RelayModeActivation> for lrwn::RelayModeActivation {
    fn to_proto(self) -> api::RelayModeActivation {
        match self {
            lrwn::RelayModeActivation::DisableRelayMode => {
                api::RelayModeActivation::DisableRelayMode
            }
            lrwn::RelayModeActivation::EnableRelayMode => api::RelayModeActivation::EnableRelayMode,
            lrwn::RelayModeActivation::Dynamic => api::RelayModeActivation::Dynamic,
            lrwn::RelayModeActivation::EndDeviceControlled => {
                api::RelayModeActivation::EndDeviceControlled
            }
        }
    }
}

impl FromProto<lrwn::RelayModeActivation> for api::RelayModeActivation {
    fn from_proto(self) -> lrwn::RelayModeActivation {
        match self {
            api::RelayModeActivation::DisableRelayMode => {
                lrwn::RelayModeActivation::DisableRelayMode
            }
            api::RelayModeActivation::EnableRelayMode => lrwn::RelayModeActivation::EnableRelayMode,
            api::RelayModeActivation::Dynamic => lrwn::RelayModeActivation::Dynamic,
            api::RelayModeActivation::EndDeviceControlled => {
                lrwn::RelayModeActivation::EndDeviceControlled
            }
        }
    }
}

impl ToProto<common::DeviceClass> for DeviceClass {
    fn to_proto(self) -> common::DeviceClass {
        match self {
            DeviceClass::A => common::DeviceClass::ClassA,
            DeviceClass::B => common::DeviceClass::ClassB,
            DeviceClass::C => common::DeviceClass::ClassC,
        }
    }
}

pub fn datetime_to_prost_timestamp(dt: &DateTime<Utc>) -> prost_types::Timestamp {
    let ts = dt.timestamp_nanos_opt().unwrap_or_default();

    prost_types::Timestamp {
        seconds: ts / 1_000_000_000,
        nanos: (ts % 1_000_000_000) as i32,
    }
}
