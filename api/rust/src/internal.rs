include!(concat!(env!("OUT_DIR"), "/internal/internal.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/internal/internal.serde.rs"));
