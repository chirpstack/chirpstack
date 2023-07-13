use anyhow::Result;

use crate::api::auth::claims;
use crate::config;
use crate::storage::api_key;

pub async fn run(name: &str) -> Result<()> {
    let conf = config::get();

    crate::storage::setup().await?;

    let key = api_key::create(api_key::ApiKey {
        name: name.to_string(),
        is_admin: true,
        ..Default::default()
    })
    .await?;

    let token = claims::AuthClaim::new_for_api_key(&key.id).encode(conf.api.secret.as_ref())?;

    println!("id: {}", key.id);
    println!("token: {}", token);

    Ok(())
}
