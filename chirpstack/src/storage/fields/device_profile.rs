use diesel::backend::Backend;
use diesel::{deserialize, serialize};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, sql_types::Jsonb};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Text, sqlite::Sqlite};
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize, AsExpression, FromSqlRow,
)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct AbpParams {
    pub rx1_delay: u8,
    pub rx1_dr_offset: u8,
    pub rx2_dr: u8,
    pub rx2_freq: u32,
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for AbpParams {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for AbpParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for AbpParams
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        Ok(serde_json::from_str(unsafe { &*s })?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for AbpParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self)?);
        Ok(serialize::IsNull::No)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize, AsExpression, FromSqlRow,
)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct ClassBParams {
    pub timeout: u16,
    #[serde(alias = "ping_slot_nb_k")]
    pub ping_slot_periodicity: u8,
    pub ping_slot_dr: u8,
    pub ping_slot_freq: u32,
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for ClassBParams {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for ClassBParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for ClassBParams
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        Ok(serde_json::from_str(unsafe { &*s })?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for ClassBParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self)?);
        Ok(serialize::IsNull::No)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize, AsExpression, FromSqlRow,
)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct ClassCParams {
    pub timeout: u16,
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for ClassCParams {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for ClassCParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for ClassCParams
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        Ok(serde_json::from_str(unsafe { &*s })?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for ClassCParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self)?);
        Ok(serialize::IsNull::No)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize, AsExpression, FromSqlRow,
)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct RelayParams {
    pub is_relay: bool,
    pub is_relay_ed: bool,
    pub ed_relay_only: bool,
    pub relay_enabled: bool,
    pub relay_cad_periodicity: u8,
    pub default_channel_index: u8,
    pub second_channel_freq: u32,
    pub second_channel_dr: u8,
    pub second_channel_ack_offset: u8,
    #[serde(with = "ed_activation_mode")]
    pub ed_activation_mode: lrwn::RelayModeActivation,
    pub ed_smart_enable_level: u8,
    pub ed_back_off: u8,
    pub ed_uplink_limit_bucket_size: u8,
    pub ed_uplink_limit_reload_rate: u8,
    pub relay_join_req_limit_reload_rate: u8,
    pub relay_notify_limit_reload_rate: u8,
    pub relay_global_uplink_limit_reload_rate: u8,
    pub relay_overall_limit_reload_rate: u8,
    pub relay_join_req_limit_bucket_size: u8,
    pub relay_notify_limit_bucket_size: u8,
    pub relay_global_uplink_limit_bucket_size: u8,
    pub relay_overall_limit_bucket_size: u8,
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for RelayParams {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for RelayParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for RelayParams
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        Ok(serde_json::from_str(unsafe { &*s })?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for RelayParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self)?);
        Ok(serialize::IsNull::No)
    }
}

mod ed_activation_mode {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(v: &lrwn::RelayModeActivation, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(v.to_u8())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<lrwn::RelayModeActivation, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u8::deserialize(deserializer)?;
        let v = lrwn::RelayModeActivation::from_u8(v)
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, AsExpression, FromSqlRow)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
#[serde(default)]
pub struct AppLayerParams {
    pub ts003_version: Option<Ts003Version>,
    pub ts004_version: Option<Ts004Version>,
    pub ts005_version: Option<Ts005Version>,
    pub ts003_f_port: u8,
    pub ts004_f_port: u8,
    pub ts005_f_port: u8,
}

impl Default for AppLayerParams {
    fn default() -> Self {
        Self {
            ts003_version: None,
            ts004_version: None,
            ts005_version: None,
            ts003_f_port: 202,
            ts004_f_port: 201,
            ts005_f_port: 200,
        }
    }
}

impl AppLayerParams {
    pub fn is_app_layer_f_port(&self, f_port: u8) -> bool {
        (self.ts003_version.is_some() && self.ts003_f_port == f_port)
            || (self.ts004_version.is_some() && self.ts004_f_port == f_port)
            || (self.ts005_version.is_some() && self.ts005_f_port == f_port)
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for AppLayerParams {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for AppLayerParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for AppLayerParams
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        Ok(serde_json::from_str(unsafe { &*s })?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for AppLayerParams {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self)?);
        Ok(serialize::IsNull::No)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Ts003Version {
    #[default]
    V100,
    V200,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Ts004Version {
    #[default]
    V100,
    V200,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Ts005Version {
    #[default]
    V100,
    V200,
}
