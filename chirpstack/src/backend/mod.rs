use anyhow::Result;

pub mod joinserver;
pub mod keywrap;
pub mod roaming;

pub fn setup() -> Result<()> {
    joinserver::setup()?;
    roaming::setup()?;

    Ok(())
}
