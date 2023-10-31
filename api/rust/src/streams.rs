include!(concat!(env!("OUT_DIR"), "/streams/streams.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/streams/streams.serde.rs"));
