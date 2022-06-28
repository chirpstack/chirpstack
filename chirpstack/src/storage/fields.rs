use std::collections::HashMap;
use std::io::Write;
use std::ops::{Deref, DerefMut};

use diesel::pg::types::sql_types::Jsonb;
use diesel::pg::Pg;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow)]
#[sql_type = "Jsonb"]
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
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        let kv: HashMap<String, String> = serde_json::from_value(value)?;
        Ok(KeyValue(kv))
    }
}

impl serialize::ToSql<Jsonb, Pg> for KeyValue {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        let value = serde_json::to_value(&self.0)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}

#[derive(Debug, Clone, AsExpression, FromSqlRow, PartialEq)]
#[sql_type = "Jsonb"]
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
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        let kv: HashMap<String, Measurement> = serde_json::from_value(value)?;
        Ok(Measurements::new(kv))
    }
}

impl serialize::ToSql<Jsonb, Pg> for Measurements {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        let value = serde_json::to_value(&self.0)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Measurement {
    pub name: String,
    pub kind: MeasurementKind,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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
