use std::ops::Deref;

use diesel::backend::Backend;

use diesel::{deserialize, serialize};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, pg::sql_types::Array, sql_types::Nullable, sql_types::Text};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Text, sqlite::Sqlite};

#[derive(Default, Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Array<Nullable<Text>>))]
#[cfg_attr(feature = "sqlite", diesel(sql_type = Text))]
pub struct StringVec(Vec<Option<String>>);

impl StringVec {
    pub fn new(v: Vec<Option<String>>) -> Self {
        StringVec(v)
    }
}

impl Deref for StringVec {
    type Target = Vec<Option<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Array<Nullable<Text>>, Pg> for StringVec {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value =
            <Vec<Option<String>> as deserialize::FromSql<Array<Nullable<Text>>, Pg>>::from_sql(
                value,
            )?;
        Ok(StringVec(value))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Array<Nullable<Text>>, Pg> for StringVec {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <Vec<Option<String>> as serialize::ToSql<Array<Nullable<Text>>, Pg>>::to_sql(&self.0, out)
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for StringVec {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as deserialize::FromSql<Text, Sqlite>>::from_sql(value)?;
        let value: Vec<Option<String>> = serde_json::from_str(&value)?;
        Ok(StringVec(value))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for StringVec {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(&self.0)?);
        Ok(serialize::IsNull::No)
    }
}
