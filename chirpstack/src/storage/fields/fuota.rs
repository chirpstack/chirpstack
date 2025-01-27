use anyhow::Error;
use diesel::backend::Backend;
use diesel::sql_types::Text;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum RequestFragmentationSessionStatus {
    NoRequest,
    AfterFragEnqueue,
    AfterSessTimeout,
}

impl From<&RequestFragmentationSessionStatus> for String {
    fn from(value: &RequestFragmentationSessionStatus) -> Self {
        match value {
            RequestFragmentationSessionStatus::NoRequest => "NO_REQUEST",
            RequestFragmentationSessionStatus::AfterFragEnqueue => "AFTER_FRAG_ENQUEUE",
            RequestFragmentationSessionStatus::AfterSessTimeout => "AFTER_SESS_TIMEOUT",
        }
        .to_string()
    }
}

impl TryFrom<&str> for RequestFragmentationSessionStatus {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "NO_REQUEST" => Self::NoRequest,
            "AFTER_FRAG_ENQUEUE" => Self::AfterFragEnqueue,
            "AFTER_SESS_TIMEOUT" => Self::AfterSessTimeout,
            _ => {
                return Err(anyhow!(
                    "Invalid RequestFragmentationSessionStatus value: {}",
                    value
                ))
            }
        })
    }
}

impl<DB> deserialize::FromSql<Text, DB> for RequestFragmentationSessionStatus
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = <*const str>::from_sql(value)?;
        Ok(Self::try_from(unsafe { &*string })?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Text, diesel::pg::Pg> for RequestFragmentationSessionStatus
where
    str: serialize::ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> serialize::Result {
        <str as serialize::ToSql<Text, diesel::pg::Pg>>::to_sql(
            &String::from(self),
            &mut out.reborrow(),
        )
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for RequestFragmentationSessionStatus {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Sqlite>) -> serialize::Result {
        out.set_value(String::from(self));
        Ok(serialize::IsNull::No)
    }
}
