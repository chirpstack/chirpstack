use std::io::Cursor;
use std::ops::{Deref, DerefMut};

use diesel::backend::Backend;
#[cfg(feature = "postgres")]
use diesel::pg::Pg;
use diesel::sql_types::Binary;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
use diesel::{deserialize, serialize};
use prost::Message;

use chirpstack_api::internal;

#[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Binary)]
pub struct DeviceSession(internal::DeviceSession);

impl DeviceSession {
    pub fn new(m: internal::DeviceSession) -> Self {
        DeviceSession(m)
    }
}

impl std::convert::From<internal::DeviceSession> for DeviceSession {
    fn from(u: internal::DeviceSession) -> Self {
        Self(u)
    }
}

impl std::convert::From<&internal::DeviceSession> for DeviceSession {
    fn from(u: &internal::DeviceSession) -> Self {
        Self::from(u.clone())
    }
}

impl std::convert::From<DeviceSession> for internal::DeviceSession {
    fn from(val: DeviceSession) -> Self {
        val.0
    }
}

impl Deref for DeviceSession {
    type Target = internal::DeviceSession;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DeviceSession {
    fn deref_mut(&mut self) -> &mut internal::DeviceSession {
        &mut self.0
    }
}

impl<DB> deserialize::FromSql<Binary, DB> for DeviceSession
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<Binary, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bindata = <*const [u8] as deserialize::FromSql<Binary, DB>>::from_sql(value)?;
        let ds = internal::DeviceSession::decode(&mut Cursor::new(unsafe { &*bindata }))?;
        Ok(DeviceSession(ds))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Binary, Pg> for DeviceSession {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let encoded = self.encode_to_vec();
        <Vec<u8> as serialize::ToSql<Binary, Pg>>::to_sql(&encoded, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Binary, Sqlite> for DeviceSession {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.encode_to_vec());
        Ok(serialize::IsNull::No)
    }
}
