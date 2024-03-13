include!(concat!(env!("OUT_DIR"), "/internal/internal.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/internal/internal.serde.rs"));

#[cfg(feature = "diesel")]
use diesel::{backend::Backend, deserialize, serialize, sql_types::Binary};
#[cfg(feature = "diesel")]
use prost::Message;
#[cfg(feature = "diesel")]
use std::io::Cursor;

impl DeviceSession {
    pub fn get_a_f_cnt_down(&self) -> u32 {
        if self.mac_version().to_string().starts_with("1.0") {
            // LoRaWAN 1.0
            self.n_f_cnt_down
        } else {
            // LoRaWAN 1.1
            self.a_f_cnt_down
        }
    }

    pub fn set_a_f_cnt_down(&mut self, f_cnt: u32) {
        if self.mac_version().to_string().starts_with("1.0") {
            // LoRaWAN 1.0
            self.n_f_cnt_down = f_cnt;
        } else {
            // LoRaWAN 1.1
            self.a_f_cnt_down = f_cnt;
        }
    }
}

#[cfg(feature = "diesel")]
impl<ST, DB> deserialize::FromSql<ST, DB> for DeviceSession
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<ST, DB>,
{
    fn from_sql(value: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes = <Vec<u8> as deserialize::FromSql<ST, DB>>::from_sql(value)?;
        Ok(DeviceSession::decode(&mut Cursor::new(bytes))?)
    }
}

#[cfg(feature = "diesel")]
impl serialize::ToSql<Binary, diesel::pg::Pg> for DeviceSession
where
    [u8]: serialize::ToSql<Binary, diesel::pg::Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, diesel::pg::Pg>) -> serialize::Result {
        <[u8] as serialize::ToSql<Binary, diesel::pg::Pg>>::to_sql(
            &self.encode_to_vec(),
            &mut out.reborrow(),
        )
    }
}
