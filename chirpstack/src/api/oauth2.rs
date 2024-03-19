use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::Duration;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use tracing::{error, trace};
use warp::{Rejection, Reply};

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{get_async_redis_conn, redis_key};

#[derive(Deserialize)]
struct ClerkUserinfo {
    pub email: String,
    pub email_verified: bool,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CallbackArgs {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Debug)]
pub struct User {
    pub email: String,
    pub email_verified: bool,
    pub external_id: String,
}

pub async fn login_handler() -> Result<impl Reply, Rejection> {
    let client = match get_client() {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e.full(), "Get OAuth2 client error");
            return Ok(warp::reply::with_status(
                "Internal error",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response());
        }
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    if let Err(e) = store_verifier(&csrf_token, &pkce_verifier).await {
        error!(error = %e.full(), "Store verifier error");
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

fn get_client() -> Result<BasicClient> {
    let conf = config::get();

    if conf.user_authentication.enabled != "oauth2" {
        return Err(anyhow!("OAuth2 is not enabled"));
    }

    let client = BasicClient::new(
        ClientId::new(conf.user_authentication.oauth2.client_id.clone()),
        Some(ClientSecret::new(
            conf.user_authentication.oauth2.client_secret.clone(),
        )),
        AuthUrl::new(conf.user_authentication.oauth2.auth_url.clone())?,
        Some(TokenUrl::new(
            conf.user_authentication.oauth2.token_url.clone(),
        )?),
    )
    .set_redirect_uri(RedirectUrl::new(
        conf.user_authentication.oauth2.redirect_url.clone(),
    )?)
    .set_auth_type(match conf.user_authentication.oauth2.provider.as_ref() {
        "clerk" => AuthType::RequestBody, // clerk does not support BasicAuth
        _ => AuthType::BasicAuth,         //  default oauth2 crate value
    });

    Ok(client)
}

pub async fn get_user(code: &str, state: &str) -> Result<User> {
    let state = oauth2::CsrfToken::new(state.to_string());
    let verifier = get_verifier(&state).await?;
    let client = get_client()?;

    let token = match client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .set_pkce_verifier(verifier)
        .request_async(async_http_client)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(anyhow!(e.to_string()));
        }
    };
    let access_token = token.access_token().secret();

    let conf = config::get();
    let provider = conf.user_authentication.oauth2.provider.clone();
    let userinfo_url = conf.user_authentication.oauth2.userinfo_url.clone();

    match provider.as_ref() {
        "clerk" => get_clerk_user(access_token, &userinfo_url).await,
        _ => Err(anyhow!("Unsupported OAuth2 provider: {}", provider)),
    }
}

async fn get_clerk_user(token: &str, url: &str) -> Result<User> {
    let client = reqwest::Client::new();
    let auth_header = format!("Bearer {}", token);

    let resp: ClerkUserinfo = client
        .get(url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .await?
        .json()
        .await?;

    Ok(User {
        email: resp.email,
        email_verified: resp.email_verified,
        external_id: resp.user_id,
    })
}

async fn store_verifier(
    token: &oauth2::CsrfToken,
    verifier: &oauth2::PkceCodeVerifier,
) -> Result<()> {
    trace!("Storing verifier");

    let key = redis_key(format!("auth:oauth2:{}", token.secret()));
    redis::cmd("PSETEX")
        .arg(key)
        .arg(Duration::try_minutes(5).unwrap().num_milliseconds())
        .arg(verifier.secret())
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    Ok(())
}

async fn get_verifier(token: &oauth2::CsrfToken) -> Result<oauth2::PkceCodeVerifier> {
    trace!("Getting verifier");
    let key = redis_key(format!("auth:oauth2:{}", token.secret()));
    let v: String = redis::cmd("GET")
        .arg(&key)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("Get verifier")?;

    Ok(oauth2::PkceCodeVerifier::new(v))
}
