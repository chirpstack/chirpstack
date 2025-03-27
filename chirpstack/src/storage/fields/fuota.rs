use std::fmt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum FuotaJob {
    CreateMcGroup,
    AddDevsToMcGroup,
    AddGwsToMcGroup,
    McGroupSetup,
    McSession,
    FragSessionSetup,
    Enqueue,
    FragStatus,
    DeleteMcGroup,
    Complete,
}

impl fmt::Display for FuotaJob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&FuotaJob> for String {
    fn from(value: &FuotaJob) -> Self {
        match value {
            FuotaJob::CreateMcGroup => "CREATE_MC_GROUP",
            FuotaJob::AddDevsToMcGroup => "ADD_DEVS_TO_MC_GROUP",
            FuotaJob::AddGwsToMcGroup => "ADD_GWS_TO_MC_GROUP",
            FuotaJob::McGroupSetup => "MC_GROUP_SETUP",
            FuotaJob::McSession => "MC_SESSION",
            FuotaJob::FragSessionSetup => "FRAG_SESSION_SETUP",
            FuotaJob::Enqueue => "ENQUEUE",
            FuotaJob::FragStatus => "FRAG_STATUS",
            FuotaJob::DeleteMcGroup => "DELETE_MC_GROUP",
            FuotaJob::Complete => "COMPLETE",
        }
        .to_string()
    }
}

impl TryFrom<&str> for FuotaJob {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "CREATE_MC_GROUP" => Self::CreateMcGroup,
            "ADD_DEVS_TO_MC_GROUP" => Self::AddDevsToMcGroup,
            "ADD_GWS_TO_MC_GROUP" => Self::AddGwsToMcGroup,
            "MC_GROUP_SETUP" => Self::McGroupSetup,
            "MC_SESSION" => Self::McSession,
            "FRAG_SESSION_SETUP" => Self::FragSessionSetup,
            "ENQUEUE" => Self::Enqueue,
            "FRAG_STATUS" => Self::FragStatus,
            "DELETE_MC_GROUP" => Self::DeleteMcGroup,
            "COMPLETE" => Self::Complete,
            _ => return Err(anyhow!("Invalid FuotaJob value: {}", value)),
        })
    }
}

impl<DB> deserialize::FromSql<Text, DB> for FuotaJob
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
impl serialize::ToSql<Text, diesel::pg::Pg> for FuotaJob
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
impl serialize::ToSql<Text, Sqlite> for FuotaJob {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Sqlite>) -> serialize::Result {
        out.set_value(String::from(self));
        Ok(serialize::IsNull::No)
    }
}
