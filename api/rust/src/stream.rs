include!(concat!(env!("OUT_DIR"), "/stream/stream.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/stream/stream.serde.rs"));
