mod big_decimal;
mod dev_nonces;
pub mod device;
pub mod device_profile;
mod device_session;
mod fuota;
mod key_value;
mod measurements;
mod multicast_group_scheduling_type;
mod uuid;

pub use big_decimal::BigDecimal;
pub use dev_nonces::DevNonces;
pub use device_profile::{AbpParams, AppLayerParams, ClassBParams, ClassCParams, RelayParams};
pub use device_session::DeviceSession;
pub use fuota::{FuotaJob, RequestFragmentationSessionStatus};
pub use key_value::KeyValue;
pub use measurements::*;
pub use multicast_group_scheduling_type::MulticastGroupSchedulingType;
pub use uuid::Uuid;

#[cfg(feature = "postgres")]
pub mod sql_types {
    pub type Timestamptz = diesel::sql_types::Timestamptz;

    pub type JsonT = diesel::sql_types::Jsonb;

    pub type Uuid = diesel::sql_types::Uuid;
}

#[cfg(feature = "sqlite")]
pub mod sql_types {
    pub type Timestamptz = diesel::sql_types::TimestamptzSqlite;

    // TODO: sqlite is adding "jsonb" support, different from postgres
    // So we may switch the column to blob?
    // see https://sqlite.org/draft/jsonb.html
    pub type JsonT = diesel::sql_types::Text;

    // Sqlite has no native json type so use text
    pub type Uuid = diesel::sql_types::Text;
}
