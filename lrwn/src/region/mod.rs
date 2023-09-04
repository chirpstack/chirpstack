use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
#[cfg(feature = "diesel")]
use diesel::{
    backend::Backend,
    sql_types::Text,
    {deserialize, serialize},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    CFList, CFListChannelMasks, CFListChannels, ChMask, DevAddr, LinkADRReqPayload, Redundancy,
};

pub mod as923;
pub mod au915;
pub mod cn470;
pub mod cn779;
pub mod eu433;
pub mod eu868;
pub mod in865;
pub mod ism2400;
pub mod kr920;
pub mod ru864;
pub mod us915;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::Text))]
pub enum CommonName {
    EU868,
    US915,
    CN779,
    EU433,
    AU915,
    CN470,
    AS923,
    AS923_2,
    AS923_3,
    AS923_4,
    KR920,
    IN865,
    RU864,
    ISM2400,
}

impl fmt::Display for CommonName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "diesel")]
impl<DB> deserialize::FromSql<Text, DB> for CommonName
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(value)?;
        Ok(CommonName::from_str(&string)?)
    }
}

#[cfg(feature = "diesel")]
impl serialize::ToSql<Text, diesel::pg::Pg> for CommonName
where
    str: serialize::ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> serialize::Result {
        <str as serialize::ToSql<Text, diesel::pg::Pg>>::to_sql(
            &self.to_string(),
            &mut out.reborrow(),
        )
    }
}

impl FromStr for CommonName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "EU868" => CommonName::EU868,
            "US915" => CommonName::US915,
            "CN779" => CommonName::CN779,
            "EU433" => CommonName::EU433,
            "AU915" => CommonName::AU915,
            "CN470" => CommonName::CN470,
            "AS923" => CommonName::AS923,
            "AS923_2" | "AS923-2" => CommonName::AS923_2,
            "AS923_3" | "AS923-3" => CommonName::AS923_3,
            "AS923_4" | "AS923-4" => CommonName::AS923_4,
            "KR920" => CommonName::KR920,
            "IN865" => CommonName::IN865,
            "RU864" => CommonName::RU864,
            "ISM2400" => CommonName::ISM2400,
            _ => {
                return Err(anyhow!("Unexpected CommonName: {}", s));
            }
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::Text))]
pub enum Revision {
    Latest,
    A,
    B,
    RP002_1_0_0,
    RP002_1_0_1,
    RP002_1_0_2,
    RP002_1_0_3,
}

impl Revision {
    fn _to_string(&self) -> String {
        match self {
            Revision::A => "A".to_string(),
            Revision::B => "B".to_string(),
            Revision::RP002_1_0_0 => "RP002-1.0.0".to_string(),
            Revision::RP002_1_0_1 => "RP002-1.0.1".to_string(),
            Revision::RP002_1_0_2 => "RP002-1.0.2".to_string(),
            Revision::RP002_1_0_3 | Revision::Latest => "RP002-1.0.3".to_string(),
        }
    }
}

impl fmt::Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._to_string(),)
    }
}

impl fmt::Debug for Revision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._to_string(),)
    }
}

impl FromStr for Revision {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "A" => Revision::A,
            "B" => Revision::B,
            "RP002-1.0.0" => Revision::RP002_1_0_0,
            "RP002-1.0.1" => Revision::RP002_1_0_1,
            "RP002-1.0.2" => Revision::RP002_1_0_2,
            "RP002-1.0.3" => Revision::RP002_1_0_3,
            _ => {
                return Err(anyhow!("Unexpected Revision: {}", s));
            }
        })
    }
}

#[cfg(feature = "diesel")]
impl<DB> deserialize::FromSql<Text, DB> for Revision
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(value)?;
        Ok(Revision::from_str(&string)?)
    }
}

#[cfg(feature = "diesel")]
impl serialize::ToSql<Text, diesel::pg::Pg> for Revision
where
    str: serialize::ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> serialize::Result {
        <str as serialize::ToSql<Text, diesel::pg::Pg>>::to_sql(
            &self.to_string(),
            &mut out.reborrow(),
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::Text))]
pub enum MacVersion {
    Latest,
    LORAWAN_1_0_0,
    LORAWAN_1_0_1,
    LORAWAN_1_0_2,
    LORAWAN_1_0_3,
    LORAWAN_1_0_4,
    LORAWAN_1_1_0,
}

impl MacVersion {
    fn _to_string(&self) -> String {
        match self {
            MacVersion::LORAWAN_1_0_0 => "1.0.0".to_string(),
            MacVersion::LORAWAN_1_0_1 => "1.0.1".to_string(),
            MacVersion::LORAWAN_1_0_2 => "1.0.2".to_string(),
            MacVersion::LORAWAN_1_0_3 => "1.0.3".to_string(),
            MacVersion::LORAWAN_1_0_4 => "1.0.4".to_string(),
            MacVersion::LORAWAN_1_1_0 | MacVersion::Latest => "1.1.0".to_string(),
        }
    }
}

impl fmt::Display for MacVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._to_string(),)
    }
}

impl fmt::Debug for MacVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._to_string(),)
    }
}

impl FromStr for MacVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "1.0.0" | "1.0" => MacVersion::LORAWAN_1_0_0,
            "1.0.1" => MacVersion::LORAWAN_1_0_1,
            "1.0.2" => MacVersion::LORAWAN_1_0_2,
            "1.0.3" => MacVersion::LORAWAN_1_0_3,
            "1.0.4" => MacVersion::LORAWAN_1_0_4,
            "1.1.0" | "1.1" => MacVersion::LORAWAN_1_1_0,
            _ => {
                return Err(anyhow!("Unexpected MacVersion: {}", s));
            }
        })
    }
}

#[cfg(feature = "diesel")]
impl<DB> deserialize::FromSql<Text, DB> for MacVersion
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(value)?;
        Ok(MacVersion::from_str(&string)?)
    }
}

#[cfg(feature = "diesel")]
impl serialize::ToSql<Text, diesel::pg::Pg> for MacVersion
where
    str: serialize::ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> serialize::Result {
        <str as serialize::ToSql<Text, diesel::pg::Pg>>::to_sql(
            &self.to_string(),
            &mut out.reborrow(),
        )
    }
}

#[derive(Clone)]
pub struct DataRate {
    pub uplink: bool,
    pub downlink: bool,
    pub modulation: DataRateModulation,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DataRateModulation {
    Lora(LoraDataRate),
    Fsk(FskDataRate),
    LrFhss(LrFhssDataRate),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoraDataRate {
    pub spreading_factor: u8,
    pub bandwidth: u32,
    pub coding_rate: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FskDataRate {
    pub bitrate: u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LrFhssDataRate {
    pub coding_rate: String,
    pub occupied_channel_width: u32,
}

pub struct Defaults {
    pub rx2_frequency: u32,
    pub rx2_dr: u8,
    pub rx1_delay: Duration,
    pub rx2_delay: Duration,
    pub join_accept_delay1: Duration,
    pub join_accept_delay2: Duration,
}

#[derive(Clone)]
pub struct MaxPayloadSize {
    /// The maximum MACPayload size length.
    pub m: usize,
    /// The maximum application payload length in the absence of the optional FOpt control field.
    pub n: usize,
}

#[derive(Clone, Default)]
pub struct Channel {
    pub frequency: u32,
    pub min_dr: u8,
    pub max_dr: u8,
    pub enabled: bool,
    pub user_defined: bool,
}

pub trait Region {
    /// Returns the region name.
    fn get_name(&self) -> CommonName;

    /// Returns the data-rate given the modulation parameters.
    fn get_data_rate_index(&self, uplink: bool, modulation: &DataRateModulation) -> Result<u8>;

    /// Returns the modulation parameters given the data-rate.
    fn get_data_rate(&self, dr: u8) -> Result<DataRateModulation>;

    /// Returns the max-payload size for the given data-rate index, protocol version
    /// and regional-parameters revision.
    /// When the version or revision is unknown, it will return the most recent
    /// implemented revision values.
    fn get_max_payload_size(
        &self,
        mac_version: MacVersion,
        reg_params_revision: Revision,
        dr: u8,
    ) -> Result<MaxPayloadSize>;

    /// Returns the RX1 data-rate given the uplink data-rate and RX1 data-rate offset.
    fn get_rx1_data_rate_index(&self, uplink_dr: u8, rx1_dr_offset: usize) -> Result<u8>;

    /// Returns the TX Power offset for the given offset index.
    fn get_tx_power_offset(&self, tx_power: usize) -> Result<isize>;

    /// Add an extra (user-configured) uplink / downlink channel.
    /// Note: this is not supported by every region.
    fn add_channel(&mut self, frequency: u32, min_dr: u8, max_dr: u8) -> Result<()>;

    /// Returns the uplink channel for the given index.
    fn get_uplink_channel(&self, channel: usize) -> Result<Channel>;

    /// Returns the uplink channel index given a frequency.
    /// As it is possible that the same frequency occurs twice (eg. one time as
    /// a default LoRaWAN channel and one time as an user-defined channel using a 250 kHz
    /// data-rate), a bool must be given indicating this is a default channel or not.
    fn get_uplink_channel_index(&self, frequency: u32, user_defined: bool) -> Result<usize>;

    /// Returns the uplink channel index given a frequency and data-rate.
    fn get_uplink_channel_index_for_freq_dr(&self, frequency: u32, dr: u8) -> Result<usize>;

    /// Returns the downlink channel for the given index.
    fn get_downlink_channel(&self, channel: usize) -> Result<Channel>;

    /// Disables the given uplink channel index.
    fn disable_uplink_channel_index(&mut self, channel: usize) -> Result<()>;

    /// Enables the given uplink channel index.
    fn enable_uplink_channel_index(&mut self, channel: usize) -> Result<()>;

    /// Returns all available uplink channel indices.
    fn get_uplink_channel_indices(&self) -> Vec<usize>;

    /// Returns all default available uplink channel indices.
    fn get_default_uplink_channel_indices(&self) -> Vec<usize>;

    /// Returns all custom uplink channels.
    fn get_user_defined_uplink_channel_indices(&self) -> Vec<usize>;

    /// Returns the enabled uplink channel indices.
    fn get_enabled_uplink_channel_indices(&self) -> Vec<usize>;

    /// Returns the disabled uplink channel indices.
    fn get_disabled_uplink_channel_indices(&self) -> Vec<usize>;

    // Returns the list of enabled uplink data-rates.
    fn get_enabled_uplink_data_rates(&self) -> Vec<u8>;

    /// Returns the channel to use for RX1 given the uplink channel index.
    fn get_rx1_channel_index_for_uplink_channel_index(
        &self,
        uplink_channel: usize,
    ) -> Result<usize>;

    /// Returns the frequency to use for RX1 given the uplink frequency.
    fn get_rx1_frequency_for_uplink_frequency(&self, uplink_freq: u32) -> Result<u32>;

    /// Returns the frequency to use for the Class-B ping-slot.
    fn get_ping_slot_frequency(&self, dev_addr: DevAddr, beacon_time: Duration) -> Result<u32>;

    /// Returns the CFList used for OTAA activation.
    /// The CFList contains the extra channels (e.g. for the EU band) or the
    /// channel-mask for LoRaWAN 1.1+ devices (e.g. for the US band).
    /// In case of extra channels, only the first 5 extra channels with DR 0-5
    /// are returned. Other channels must be set using mac-commands. When there
    /// are no extra channels, this method returns None.
    fn get_cf_list(&self, mac_version: MacVersion) -> Option<CFList>;

    /// Returns the LinkADRReqPayloads to reconfigure the device to the current enabled channels.
    /// Note that in case of activation, user-defined channels (e.g. CFList) will be ignored as it
    /// is unknown if the device is aware of these extra frequencies.
    fn get_link_adr_req_payloads_for_enabled_uplink_channel_indices(
        &self,
        device_enabled_channels: &[usize],
    ) -> Vec<LinkADRReqPayload>;

    /// Returns the enabled uplink channel indices after applying the given LinkADRReqPayloads
    /// to the given enabled device channels.
    fn get_enabled_uplink_channel_indices_for_link_adr_payloads(
        &self,
        device_enabled_channels: &[usize],
        pls: &[LinkADRReqPayload],
    ) -> Result<Vec<usize>>;

    /// Returns the TX power for downlink transmissions using the given frequency.
    /// Depending the band, it could return different values for different frequencies.
    fn get_downlink_tx_power(&self, frequency: u32) -> isize;

    /// Returns the defaults.
    fn get_defaults(&self) -> Defaults;

    /// Returns if the device supports the TxParamSetup mac-command.
    fn implements_tx_param_setup(&self, mac_version: MacVersion) -> bool;
}

struct RegionBaseConfig {
    supports_user_channels: bool,
    cf_list_min_dr: u8,
    cf_list_max_dr: u8,
    data_rates: HashMap<u8, DataRate>,
    max_payload_size_per_dr: HashMap<MacVersion, HashMap<Revision, HashMap<u8, MaxPayloadSize>>>,
    rx1_data_rate_table: HashMap<u8, Vec<u8>>,
    tx_power_offsets: Vec<isize>,
    uplink_channels: Vec<Channel>,
    downlink_channels: Vec<Channel>,
}

impl RegionBaseConfig {
    fn get_data_rate_index(&self, uplink: bool, modulation: &DataRateModulation) -> Result<u8> {
        for (i, dr) in &self.data_rates {
            if uplink != dr.uplink && uplink == dr.downlink {
                continue;
            }

            if modulation == &dr.modulation {
                return Ok(*i);
            }
        }

        Err(anyhow!("Unknown data-rate: {:?}", modulation))
    }

    fn get_data_rate(&self, dr: u8) -> Result<DataRateModulation> {
        Ok(self
            .data_rates
            .get(&dr)
            .ok_or_else(|| anyhow!("Unknown data-rate index"))?
            .modulation
            .clone())
    }

    fn get_max_payload_size(
        &self,
        mac_version: MacVersion,
        reg_params_revision: Revision,
        dr: u8,
    ) -> Result<MaxPayloadSize> {
        let reg_params_map = match self.max_payload_size_per_dr.get(&mac_version) {
            Some(v) => v,
            None => self
                .max_payload_size_per_dr
                .get(&MacVersion::Latest)
                .ok_or_else(|| anyhow!("Unknown mac-version"))?,
        };

        let dr_map = match reg_params_map.get(&reg_params_revision) {
            Some(v) => v,
            None => reg_params_map
                .get(&Revision::Latest)
                .ok_or_else(|| anyhow!("Unknown revision"))?,
        };

        Ok(dr_map
            .get(&dr)
            .ok_or_else(|| anyhow!("Invalid data-rate"))?
            .clone())
    }

    fn get_rx1_data_rate_index(&self, uplink_dr: u8, rx1_dr_offset: usize) -> Result<u8> {
        let offset_vec = self
            .rx1_data_rate_table
            .get(&uplink_dr)
            .ok_or_else(|| anyhow!("Unknown data-rate"))?;

        Ok(*offset_vec
            .get(rx1_dr_offset)
            .ok_or_else(|| anyhow!("Invalid rx1 data-rate offset"))?)
    }

    fn get_tx_power_offset(&self, tx_power: usize) -> Result<isize> {
        Ok(*self
            .tx_power_offsets
            .get(tx_power)
            .ok_or_else(|| anyhow!("Invalid tx-power"))?)
    }

    fn add_channel(&mut self, frequency: u32, min_dr: u8, max_dr: u8) -> Result<()> {
        if !self.supports_user_channels {
            return Err(anyhow!(
                "User defined channels are not supported for this band"
            ));
        }

        let c = Channel {
            frequency,
            min_dr,
            max_dr,
            user_defined: true,
            enabled: frequency != 0,
        };

        self.uplink_channels.push(c.clone());
        self.downlink_channels.push(c);

        Ok(())
    }

    fn get_uplink_channel(&self, channel: usize) -> Result<Channel> {
        Ok(self
            .uplink_channels
            .get(channel)
            .ok_or_else(|| anyhow!("Invalid channel"))?
            .clone())
    }

    fn get_uplink_channel_index(&self, frequency: u32, user_defined: bool) -> Result<usize> {
        for (i, c) in self.uplink_channels.iter().enumerate() {
            if frequency == c.frequency && user_defined == c.user_defined {
                return Ok(i);
            }
        }

        Err(anyhow!("Unknown channel for frequency: {}", frequency))
    }

    fn get_uplink_channel_index_for_freq_dr(&self, frequency: u32, dr: u8) -> Result<usize> {
        for user_defined in &[true, false] {
            let i = match self.get_uplink_channel_index(frequency, *user_defined) {
                Ok(v) => v,
                Err(_) => {
                    continue;
                }
            };

            let c = self.get_uplink_channel(i).context("Get channel error")?;

            // there could be multiple channels using the same frequency, but with different data-rates.
            // eg EU868:
            //  channel 1 (868.3 DR 0-5)
            //  channel x (868.3 DR 6)
            if c.min_dr <= dr && c.max_dr >= dr {
                return Ok(i);
            }
        }

        Err(anyhow!(
            "No channel found for frequency: {}, dr: {}",
            frequency,
            dr
        ))
    }

    fn get_downlink_channel(&self, channel: usize) -> Result<Channel> {
        Ok(self
            .downlink_channels
            .get(channel)
            .ok_or_else(|| anyhow!("Invalid channel"))?
            .clone())
    }

    fn disable_uplink_channel_index(&mut self, channel: usize) -> Result<()> {
        let channel = self
            .uplink_channels
            .get_mut(channel)
            .ok_or_else(|| anyhow!("Invalid channel"))?;
        channel.enabled = false;
        Ok(())
    }

    fn enable_uplink_channel_index(&mut self, channel: usize) -> Result<()> {
        let channel = self
            .uplink_channels
            .get_mut(channel)
            .ok_or_else(|| anyhow!("Invalid channel"))?;
        channel.enabled = true;
        Ok(())
    }

    fn get_uplink_channel_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, _) in self.uplink_channels.iter().enumerate() {
            out.push(i);
        }
        out
    }

    fn get_default_uplink_channel_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, c) in self.uplink_channels.iter().enumerate() {
            if !c.user_defined {
                out.push(i);
            }
        }
        out
    }

    fn get_user_defined_uplink_channel_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, c) in self.uplink_channels.iter().enumerate() {
            if c.user_defined {
                out.push(i);
            }
        }
        out
    }

    fn get_enabled_uplink_channel_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, c) in self.uplink_channels.iter().enumerate() {
            if c.enabled {
                out.push(i);
            }
        }
        out
    }

    fn get_disabled_uplink_channel_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, c) in self.uplink_channels.iter().enumerate() {
            if !c.enabled {
                out.push(i);
            }
        }
        out
    }

    fn get_enabled_uplink_data_rates(&self) -> Vec<u8> {
        let mut out: HashSet<u8> = HashSet::new();
        for uc in &self.uplink_channels {
            // ..=max_dr: inclusive range, we want to include max_dr
            for dr in uc.min_dr..=uc.max_dr {
                out.insert(dr);
            }
        }

        let mut out: Vec<u8> = out.iter().cloned().collect();
        out.sort_unstable();
        out
    }

    fn get_cf_list(&self, mac_version: MacVersion) -> Option<CFList> {
        if self.supports_user_channels {
            return self.get_cf_list_channels();
        }

        // Sending the channel-mask in the CFList is supported since LoRaWAN 1.0.3.
        // For earlier versions, only a CFList with (extra) channel-list is
        // supported.
        if !self.supports_user_channels
            && (mac_version == MacVersion::LORAWAN_1_0_0
                || mac_version == MacVersion::LORAWAN_1_0_1
                || mac_version == MacVersion::LORAWAN_1_0_2)
        {
            return None;
        }

        self.get_cf_list_channel_mask()
    }

    fn get_cf_list_channels(&self) -> Option<CFList> {
        let mut channels: [u32; 5] = [0; 5];
        let mut i = 0;

        for c in &self.uplink_channels {
            if c.user_defined
                && i < channels.len()
                && c.min_dr == self.cf_list_min_dr
                && c.max_dr == self.cf_list_max_dr
            {
                channels[i] = c.frequency;
                i += 1;
            }
        }

        if i == 0 {
            return None;
        }

        Some(CFList::Channels(CFListChannels::new(channels)))
    }

    fn get_cf_list_channel_mask(&self) -> Option<CFList> {
        const SIZE: usize = 16;
        let mut masks: Vec<ChMask> = Vec::new();
        let mut mask: [bool; SIZE] = [false; SIZE];

        for (i, c) in self.uplink_channels.iter().enumerate() {
            if i != 0 && i % SIZE == 0 {
                masks.push(ChMask::new(mask));
                mask = [false; SIZE];
            }
            mask[i % SIZE] = c.enabled;
        }
        masks.push(ChMask::new(mask));

        Some(CFList::ChannelMask(CFListChannelMasks::new(masks)))
    }

    fn get_link_adr_req_payloads_for_enabled_uplink_channel_indices(
        &self,
        device_enabled_channels: &[usize],
    ) -> Vec<LinkADRReqPayload> {
        let enabled_channels = self.get_enabled_uplink_channel_indices();
        let device_set: HashSet<usize> = device_enabled_channels.iter().cloned().collect();
        let enabled_set: HashSet<usize> = enabled_channels.iter().cloned().collect();

        // Get the diff between desided and actual channels.
        // This returns the channels that must be activated and / or de-activated
        // on the device.
        let mut diff: Vec<usize> = enabled_set
            .symmetric_difference(&device_set)
            .cloned()
            .collect();

        let mut filtered_diff: Vec<usize> = Vec::new();
        for i in &diff {
            if device_set.contains(i) || !self.uplink_channels[*i].user_defined {
                filtered_diff.push(*i);
            }
        }

        // Nothing to do.
        if diff.is_empty() || filtered_diff.is_empty() {
            return vec![];
        }

        // Make sure we're dealing with a sorted slice.
        diff.sort_unstable();

        let mut out: Vec<LinkADRReqPayload> = Vec::new();
        let mut ch_mask_cntl = -1;

        // Loop over the channel blocks that contain different channels
        // note that each payload holds 16 channels and that the chMaskCntl
        // defines the block.
        for i in diff {
            if i as isize / 16 != ch_mask_cntl {
                ch_mask_cntl = i as isize / 16;
                let mut chmask: [bool; 16] = [false; 16];

                // Set enabled channels in this block to active
                // note that we don't enable user defined channels (CFList) as
                // we have no knowledge if the nodes has been provisioned with
                // these frequencies.
                for e in &enabled_channels {
                    if (!self.uplink_channels[*e].user_defined || device_set.contains(e))
                        && (*e as isize) >= ch_mask_cntl * 16
                        && (*e as isize) < (ch_mask_cntl + 1) * 16
                    {
                        chmask[e % 16] = true;
                    }
                }

                out.push(LinkADRReqPayload {
                    dr: 0,
                    tx_power: 0,
                    ch_mask: ChMask::new(chmask),
                    redundancy: Redundancy {
                        ch_mask_cntl: ch_mask_cntl as u8,
                        nb_rep: 0,
                    },
                });
            }
        }

        out
    }

    fn get_enabled_uplink_channel_indices_for_link_adr_payloads(
        &self,
        device_enabled_channels: &[usize],
        pls: &[LinkADRReqPayload],
    ) -> Result<Vec<usize>> {
        let mut chmask: Vec<bool> = vec![false; self.uplink_channels.len()];

        for c in device_enabled_channels {
            // Make sure that we don't exceed the chMask length. in case we exceed
            // we ignore the channel as it might have been removed from the network.
            if *c < chmask.len() {
                chmask[*c] = true;
            }
        }

        for pl in pls {
            for (i, enabled) in pl.ch_mask.into_iter().enumerate() {
                let ii = (pl.redundancy.ch_mask_cntl as usize * 16) + i;

                if ii >= chmask.len() && !enabled {
                    continue;
                }

                if ii >= chmask.len() {
                    return Err(anyhow!("Channel does not exist"));
                }

                chmask[ii] = enabled;
            }
        }

        // Turn the chMask into a slice of enabled channel numbers
        let mut out: Vec<usize> = Vec::new();
        for (i, enabled) in chmask.iter().enumerate() {
            if *enabled {
                out.push(i);
            }
        }

        Ok(out)
    }
}

pub fn get(
    common_name: CommonName,
    repeater_compatible: bool,
    dwell_time_400ms: bool,
) -> Box<dyn Region + Sync + Send> {
    match common_name {
        CommonName::AS923 => Box::new(as923::Configuration::new(
            CommonName::AS923,
            repeater_compatible,
            dwell_time_400ms,
        )),
        CommonName::AS923_2 => Box::new(as923::Configuration::new(
            CommonName::AS923_2,
            repeater_compatible,
            dwell_time_400ms,
        )),
        CommonName::AS923_3 => Box::new(as923::Configuration::new(
            CommonName::AS923_3,
            repeater_compatible,
            dwell_time_400ms,
        )),
        CommonName::AS923_4 => Box::new(as923::Configuration::new(
            CommonName::AS923_4,
            repeater_compatible,
            dwell_time_400ms,
        )),
        CommonName::AU915 => Box::new(au915::Configuration::new(
            repeater_compatible,
            dwell_time_400ms,
        )),
        CommonName::CN470 => Box::new(cn470::Configuration::new(repeater_compatible)),
        CommonName::CN779 => Box::new(cn779::Configuration::new(repeater_compatible)),
        CommonName::EU433 => Box::new(eu433::Configuration::new(repeater_compatible)),
        CommonName::EU868 => Box::new(eu868::Configuration::new(repeater_compatible)),
        CommonName::IN865 => Box::new(in865::Configuration::new(repeater_compatible)),
        CommonName::ISM2400 => Box::new(ism2400::Configuration::new(repeater_compatible)),
        CommonName::KR920 => Box::new(kr920::Configuration::new(repeater_compatible)),
        CommonName::RU864 => Box::new(ru864::Configuration::new(repeater_compatible)),
        CommonName::US915 => Box::new(us915::Configuration::new(repeater_compatible)),
    }
}
