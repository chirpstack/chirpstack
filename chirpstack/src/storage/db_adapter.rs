// TimestampTz is represented differently in Diesel
// but it can essentially convert from/to chrono::*DateTime*
#[cfg(feature = "postgres")]
pub type DbTimestamptz = diesel::sql_types::Timestamptz;
#[cfg(feature = "sqlite")]
pub type DbTimestamptz = diesel::sql_types::TimestamptzSqlite;

// Sqlite has no native json type so use text
#[cfg(feature = "postgres")]
pub type DbJsonT = diesel::sql_types::Jsonb;
#[cfg(feature = "sqlite")]
pub type DbJsonT = diesel::sql_types::Text;

// Sqlite has no native json type so use text
#[cfg(feature = "postgres")]
pub type DbUuid = diesel::sql_types::Uuid;
#[cfg(feature = "sqlite")]
pub type DbUuid = diesel::sql_types::Text;
