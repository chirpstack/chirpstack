use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::Duration;
use openidconnect::core::{
    CoreClient, CoreGenderClaim, CoreIdTokenClaims, CoreIdTokenVerifier, CoreProviderMetadata,
    CoreResponseType,
};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, Scope,
};
use openidconnect::{EmptyAdditionalClaims, UserInfoClaims};
use serde::{Deserialize, Serialize};
use tokio::task;
use tracing::{error, trace};
use warp::{Rejection, Reply};

use crate::config;
use crate::storage::{get_redis_conn, redis_key};

pub type User = UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim>;

#[derive(Serialize, Deserialize)]
pub struct CallbackArgs {
    pub code: String,
    pub state: String,
}

pub async fn login_handler() -> Result<impl Reply, Rejection> {
    let client = match get_client().await {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e, "Get OIDC client error");
            return Ok(warp::reply::with_status(
                "Internal error",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response());
        }
    };

    let (auth_url, csrf_state, nonce) = client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    if let Err(e) = store_nonce(&csrf_state, &nonce).await {
        error!(error = %e, "Store nonce error");
        return Ok(warp::reply::with_status(
            "Internal error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    Ok(
        warp::redirect::found(warp::http::Uri::from_str(auth_url.as_str()).unwrap())
            .into_response(),
    )
}

pub async fn callback_handler(args: CallbackArgs) -> Result<impl Reply, Rejection> {
    // warp::redirect does not work with '#'.
    Ok(warp::reply::with_header(
        warp::http::StatusCode::PERMANENT_REDIRECT,
        warp::http::header::LOCATION,
        format!("/#/login?code={}&state={}", args.code, args.state),
    ))
}

pub async fn get_user(code: &str, state: &str) -> Result<User> {
    let state = CsrfToken::new(state.to_string());
    let nonce = get_nonce(&state).await?;
    let client = get_client().await?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
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
        .request_async(async_http_client)
        .await
        .context("Failed requesting user info")?;

    Ok(userinfo_claims)
}

async fn get_client() -> Result<CoreClient> {
    let conf = config::get();

    if !conf.user_authentication.openid_connect.enabled {
        return Err(anyhow!("OIDC is disabled"));
    }

    let client_id = ClientId::new(conf.user_authentication.openid_connect.client_id.clone());
    let client_secret = ClientSecret::new(
        conf.user_authentication
            .openid_connect
            .client_secret
            .clone(),
    );
    let provider_url =
        IssuerUrl::new(conf.user_authentication.openid_connect.provider_url.clone())?;
    let redirect_url =
        RedirectUrl::new(conf.user_authentication.openid_connect.redirect_url.clone())?;

    let provider_metadata =
        CoreProviderMetadata::discover_async(provider_url, async_http_client).await?;
    let client =
        CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
            .set_redirect_uri(redirect_url);

    Ok(client)
}

async fn store_nonce(state: &CsrfToken, nonce: &Nonce) -> Result<()> {
    task::spawn_blocking({
        let state = state.clone();
        let nonce = nonce.clone();
        move || -> Result<()> {
            trace!("Storing nonce");
            let key = redis_key(format!("auth:oidc:{}", state.secret()));
            let mut c = get_redis_conn()?;

            redis::cmd("PSETEX")
                .arg(key)
                .arg(Duration::minutes(5).num_milliseconds())
                .arg(nonce.secret())
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await?
}

async fn get_nonce(state: &CsrfToken) -> Result<Nonce> {
    task::spawn_blocking({
        let state = state.clone();
        move || -> Result<Nonce> {
            trace!("Getting nonce");
            let key = redis_key(format!("auth:oidc:{}", state.secret()));
            let mut c = get_redis_conn()?;

            let v: String = redis::cmd("GET")
                .arg(&key)
                .query(&mut *c)
                .context("Get nonce")?;

            Ok(Nonce::new(v))
        }
    })
    .await?
}
