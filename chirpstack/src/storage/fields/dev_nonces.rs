use std::collections::HashMap;

use diesel::backend::Backend;

use diesel::{deserialize, serialize};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, sql_types::Jsonb};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Text, sqlite::Sqlite};

use lrwn::EUI64;

#[derive(Default, Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct DevNonces(HashMap<EUI64, Vec<u16>>);

impl DevNonces {
    pub fn contains(&self, join_eui: EUI64, dev_nonce: u16) -> bool {
        if let Some(v) = self.0.get(&join_eui) {
            v.contains(&dev_nonce)
        } else {
            false
        }
    }

    pub fn insert(&mut self, join_eui: EUI64, dev_nonce: u16) {
        self.0.entry(join_eui).or_default().push(dev_nonce)
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for DevNonces {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        let dev_nonces: HashMap<EUI64, Vec<u16>> = serde_json::from_value(value)?;
        Ok(DevNonces(dev_nonces))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for DevNonces {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(&self.0)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for DevNonces
where
    *const str: deserialize::FromSql<Text, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        let dev_nonces: HashMap<EUI64, Vec<u16>> = serde_json::from_str(unsafe { &*s })?;
        Ok(DevNonces(dev_nonces))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for DevNonces {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self.0)?);
        Ok(serialize::IsNull::No)
    }
}
