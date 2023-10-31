include!(concat!(env!("OUT_DIR"), "/streams/meta.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/streams/meta.serde.rs"));
