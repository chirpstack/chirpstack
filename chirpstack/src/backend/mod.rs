use anyhow::Result;

pub mod joinserver;
pub mod keywrap;
pub mod roaming;

pub async fn setup() -> Result<()> {
    joinserver::setup().await?;
    roaming::setup().await?;

    Ok(())
}
