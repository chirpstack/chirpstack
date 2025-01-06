use anyhow::Result;

pub mod clocksync;

pub trait PayloadCodec<Struct = Self> {
    fn decode(b: &[u8]) -> Result<Struct>;
    fn encode(&self) -> Result<Vec<u8>>;
}
