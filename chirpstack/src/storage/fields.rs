use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use diesel::backend::Backend;
use diesel::pg::Pg;
use diesel::sql_types::{Jsonb, Text};
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
pub struct KeyValue(HashMap<String, String>);

impl KeyValue {
    pub fn new(m: HashMap<String, String>) -> Self {
        KeyValue(m)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_hashmap(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}

impl Deref for KeyValue {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeyValue {
    fn deref_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl deserialize::FromSql<Jsonb, Pg> for KeyValue {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        let kv: HashMap<String, String> = serde_json::from_value(value)?;
        Ok(KeyValue(kv))
    }
}

impl serialize::ToSql<Jsonb, Pg> for KeyValue {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(&self.0)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[derive(Debug, Clone, AsExpression, FromSqlRow, PartialEq, Eq)]
#[diesel(sql_type = Jsonb)]
pub struct Measurements(HashMap<String, Measurement>);

impl Measurements {
    pub fn new(m: HashMap<String, Measurement>) -> Self {
        Measurements(m)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_hashmap(&self) -> HashMap<String, Measurement> {
        self.0.clone()
    }
}

impl Deref for Measurements {
    type Target = HashMap<String, Measurement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Measurements {
    fn deref_mut(&mut self) -> &mut HashMap<String, Measurement> {
        &mut self.0
    }
}

impl deserialize::FromSql<Jsonb, Pg> for Measurements {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        let kv: HashMap<String, Measurement> = serde_json::from_value(value)?;
        Ok(Measurements::new(kv))
    }
}

impl serialize::ToSql<Jsonb, Pg> for Measurements {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(&self.0)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Measurement {
    pub name: String,
    pub kind: MeasurementKind,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementKind {
    // Unknown.
    UNKNOWN,
    // Incrementing counters which are not reset on each reporting.
    COUNTER,
    // Counters that do get reset upon reading.
    ABSOLUTE,
    // E.g. a temperature value.
    GAUGE,
    // E.g. a firmware version, true / false value.
    STRING,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, AsExpression, FromSqlRow)]
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum MulticastGroupSchedulingType {
    // Delay.
    DELAY,
    // GPS time.
    GPS_TIME,
}

impl fmt::Display for MulticastGroupSchedulingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for MulticastGroupSchedulingType
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(value)?;
        Ok(MulticastGroupSchedulingType::from_str(&string)?)
    }
}

impl serialize::ToSql<Text, diesel::pg::Pg> for MulticastGroupSchedulingType
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

impl FromStr for MulticastGroupSchedulingType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "DELAY" => MulticastGroupSchedulingType::DELAY,
            "GPS_TIME" => MulticastGroupSchedulingType::GPS_TIME,
            _ => {
                return Err(anyhow!("Unexpected MulticastGroupSchedulingType: {}", s));
            }
        })
    }
}
