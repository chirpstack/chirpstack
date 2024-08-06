use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use diesel::backend::Backend;
use diesel::{deserialize, serialize};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, sql_types::Jsonb};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Text, sqlite::Sqlite};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, AsExpression, FromSqlRow, PartialEq, Eq)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
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

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for Measurements {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        let kv: HashMap<String, Measurement> = serde_json::from_value(value)?;
        Ok(Measurements::new(kv))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for Measurements {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(&self.0)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for Measurements
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        let kv: HashMap<String, Measurement> = serde_json::from_str(unsafe { &*s })?;
        Ok(Measurements::new(kv))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for Measurements {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Sqlite>) -> serialize::Result {
        let value = serde_json::to_string(&self.0)?;
        out.set_value(value);
        Ok(serialize::IsNull::No)
    }
}
