use diesel::backend::Backend;
#[cfg(feature = "postgres")]
use diesel::pg::Pg;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
use diesel::{deserialize, serialize};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[serde(transparent)]
#[cfg_attr(feature = "postgres", diesel(sql_type = diesel::sql_types::Uuid))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = diesel::sql_types::Text))]
pub struct Uuid(uuid::Uuid);

impl std::convert::From<uuid::Uuid> for Uuid {
    fn from(u: uuid::Uuid) -> Self {
        Self(u)
    }
}

impl std::convert::From<&uuid::Uuid> for Uuid {
    fn from(u: &uuid::Uuid) -> Self {
        Self::from(u.clone())
    }
}

impl std::convert::Into<uuid::Uuid> for Uuid {
    fn into(self) -> uuid::Uuid {
        self.0
    }
}

impl std::ops::Deref for Uuid {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<diesel::sql_types::Uuid, Pg> for Uuid {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let u = <uuid::Uuid>::from_sql(value)?;
        Ok(Uuid(u))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<diesel::sql_types::Uuid, Pg> for Uuid {
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <uuid::Uuid as serialize::ToSql<diesel::sql_types::Uuid, Pg>>::to_sql(
            &self.0,
            &mut out.reborrow(),
        )
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<diesel::sql_types::Text, Sqlite> for Uuid {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        let u = uuid::Uuid::try_parse(unsafe { &*s })?;
        Ok(Uuid(u))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<diesel::sql_types::Text, Sqlite> for Uuid {
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(serialize::IsNull::No)
    }
}
