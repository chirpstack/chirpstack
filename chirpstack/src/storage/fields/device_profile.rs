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
        let value = serde_json::to_value(&self)?;
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
