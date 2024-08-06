use diesel::backend::Backend;
use diesel::{deserialize, serialize};
#[cfg(feature = "postgres")]
use diesel::{
    pg::Pg,
    sql_types::{Array, Int4, Nullable},
};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Text, sqlite::Sqlite};
use serde::{Deserialize, Serialize};

#[cfg(feature = "postgres")]
type DevNoncesPgType = Array<Nullable<Int4>>;

// Sqlite has no native array type so use text
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[serde(transparent)]
#[cfg_attr(feature = "postgres", diesel(sql_type = DevNoncesPgType))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct DevNonces(DevNoncesInner);

pub type DevNoncesInner = Vec<Option<i32>>;

impl std::default::Default for DevNonces {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl std::convert::AsRef<DevNoncesInner> for DevNonces {
    fn as_ref(&self) -> &DevNoncesInner {
        &self.0
    }
}

impl std::convert::From<DevNoncesInner> for DevNonces {
    fn from(value: DevNoncesInner) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for DevNonces {
    type Target = DevNoncesInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DevNonces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<DevNoncesPgType, Pg> for DevNonces {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let sql_val = <DevNoncesInner>::from_sql(value)?;
        Ok(DevNonces(sql_val))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<DevNoncesPgType, Pg> for DevNonces {
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <DevNoncesInner as serialize::ToSql<DevNoncesPgType, Pg>>::to_sql(
            &self.0,
            &mut out.reborrow(),
        )
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
        let nonces = serde_json::from_str::<DevNonces>(unsafe { &*s })?;
        Ok(nonces)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for DevNonces {
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(self)?);
        Ok(serialize::IsNull::No)
    }
}
