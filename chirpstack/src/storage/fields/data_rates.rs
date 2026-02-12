use std::ops::Deref;

use diesel::backend::Backend;

use diesel::{deserialize, serialize};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, pg::sql_types::Array, sql_types::Nullable, sql_types::SmallInt};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Text, sqlite::Sqlite};

#[derive(Default, Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Array<Nullable<SmallInt>>))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct DataRates(Vec<Option<i16>>);

impl DataRates {
    pub fn new(v: Vec<Option<i16>>) -> Self {
        DataRates(v)
    }
}

impl Deref for DataRates {
    type Target = Vec<Option<i16>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Array<Nullable<SmallInt>>, Pg> for DataRates {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value =
            <Vec<Option<i16>> as deserialize::FromSql<Array<Nullable<SmallInt>>, Pg>>::from_sql(
                value,
            )?;

        Ok(DataRates(value))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Array<Nullable<SmallInt>>, Pg> for DataRates {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <Vec<Option<i16>> as serialize::ToSql<Array<Nullable<SmallInt>>, Pg>>::to_sql(&self.0, out)
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for DataRates {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as deserialize::FromSql<Text, Sqlite>>::from_sql(value)?;
        let value: Vec<Option<i16>> = serde_json::from_str(&value)?;
        Ok(DataRates(value))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for DataRates {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self.0)?);
        Ok(serialize::IsNull::No)
    }
}
