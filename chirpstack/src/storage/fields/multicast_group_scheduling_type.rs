use std::fmt;
use std::str::FromStr;

use diesel::backend::Backend;
use diesel::sql_types::Text;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

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
        let string = <*const str>::from_sql(value)?;
        Ok(Self::from_str(unsafe { &*string })?)
    }
}

#[cfg(feature = "postgres")]
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

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for MulticastGroupSchedulingType {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.to_string());
        Ok(serialize::IsNull::No)
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
