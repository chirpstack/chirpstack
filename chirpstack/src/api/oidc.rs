use std::collections::HashMap;

use anyhow::{Context, Result};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use chrono::Duration;
use http::StatusCode;
use openidconnect::core::{
    CoreClient, CoreGenderClaim, CoreIdTokenClaims, CoreIdTokenVerifier, CoreProviderMetadata,
    CoreResponseType,
};
use openidconnect::{reqwest, AdditionalClaims, UserInfoClaims};
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IssuerUrl, Nonce, OAuth2TokenResponse, RedirectUrl, Scope,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, trace};

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{get_async_redis_conn, redis_key};

pub type User = UserInfoClaims<CustomClaims, CoreGenderClaim>;

type Client = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaims {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

impl AdditionalClaims for CustomClaims {}

#[derive(Deserialize)]
pub struct CallbackArgs {
    pub code: String,
    pub state: String,
}

pub async fn login_handler() -> Response {
    let client = match get_client().await {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e.full(), "Get OIDC client error");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };

    let conf = config::get();
    let mut request = client.authorize_url(
        AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    );
    for scope in &conf.user_authentication.openid_connect.scopes {
        request = request.add_scope(Scope::new(scope.to_string()))
    }
    let (auth_url, csrf_state, nonce) = request.url();

    if let Err(e) = store_nonce(&csrf_state, &nonce).await {
        error!(error = %e.full(), "Store nonce error");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    Redirect::temporary(auth_url.as_str()).into_response()
}

pub async fn callback_handler(args: Query<CallbackArgs>) -> Response {
    let args: CallbackArgs = args.0;
    Redirect::permanent(&format!("/#/login?code={}&state={}", args.code, args.state))
        .into_response()
}

pub async fn get_user(code: &str, state: &str) -> Result<User> {
    let state = CsrfToken::new(state.to_string());
    let nonce = get_nonce(&state).await?;
    let client = get_client().await?;

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code.to_string()))?
        .request_async(&http_client)
        .await?;

    let id_token_verifier: CoreIdTokenVerifier = client.id_token_verifier();
    let _id_token_claims: &CoreIdTokenClaims = token_response
        .extra_fields()
        .id_token()
        .context("Server did not return an ID token")?
        .claims(&id_token_verifier, &nonce)
        .context("Failed to verify ID token")?;

    let userinfo_claims: User = client
        .user_info(token_response.access_token().to_owned(), None)
        .context("No user info endpoint")?
        .request_async(&http_client)
        .await
        .context("Failed requesting user info")?;

    Ok(userinfo_claims)
}

async fn store_nonce(state: &CsrfToken, nonce: &Nonce) -> Result<()> {
    trace!("Storing nonce");
    let key = redis_key(format!("auth:oidc:{}", state.secret()));

    () = redis::cmd("PSETEX")
        .arg(key)
        .arg(Duration::try_minutes(5).unwrap().num_milliseconds())
        .arg(nonce.secret())
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    Ok(())
}

async fn get_nonce(state: &CsrfToken) -> Result<Nonce> {
    trace!("Getting nonce");
    let key = redis_key(format!("auth:oidc:{}", state.secret()));

    let v: String = redis::cmd("GET")
        .arg(&key)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("Get nonce")?;

    Ok(Nonce::new(v))
}

async fn get_client() -> Result<Client> {
    let conf = config::get();

    if conf.user_authentication.enabled != "openid_connect" {
        return Err(anyhow!("OIDC is not enabled"));
    }

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(conf.user_authentication.openid_connect.provider_url.clone())?,
        &http_client,
    )
    .await?;

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(conf.user_authentication.openid_connect.client_id.clone()),
        Some(ClientSecret::new(
            conf.user_authentication
                .openid_connect
                .client_secret
                .clone(),
        )),
    )
    .set_redirect_uri(RedirectUrl::new(
        conf.user_authentication.openid_connect.redirect_url.clone(),
    )?);

    Ok(client)
}
