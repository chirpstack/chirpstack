include!(concat!(env!("OUT_DIR"), "/meta/meta.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/meta/meta.serde.rs"));
