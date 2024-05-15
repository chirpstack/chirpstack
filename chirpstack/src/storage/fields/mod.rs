mod big_decimal;
mod key_value;
mod measurements;
mod multicast_group_scheduling_type;

pub use big_decimal::BigDecimal;
pub use key_value::KeyValue;
pub use measurements::*;
pub use multicast_group_scheduling_type::MulticastGroupSchedulingType;

pub mod sql_types {
    // TimestampTz is represented differently in Diesel
    // but it can essentially convert from/to chrono::*DateTime*
    #[cfg(feature = "postgres")]
    pub type Timestamptz = diesel::sql_types::Timestamptz;
    #[cfg(feature = "sqlite")]
    pub type Timestamptz = diesel::sql_types::TimestamptzSqlite;

    // Sqlite has no native json type so use text
    #[cfg(feature = "postgres")]
    pub type JsonT = diesel::sql_types::Jsonb;
    // TODO: sqlite is adding "jsonb" support, different from postgres
    // So we may switch the column to blob?
    // see https://sqlite.org/draft/jsonb.html
    #[cfg(feature = "sqlite")]
    pub type JsonT = diesel::sql_types::Text;

    // Sqlite has no native json type so use text
    #[cfg(feature = "postgres")]
    pub type Uuid = diesel::sql_types::Uuid;
    #[cfg(feature = "sqlite")]
    pub type Uuid = diesel::sql_types::Text;
}
