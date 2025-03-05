use anyhow::{Context, Result};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use chrono::Duration;
use http::StatusCode;
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet,
    EndpointSet, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use tracing::{error, trace};

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{get_async_redis_conn, redis_key};

type Client = BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Deserialize)]
struct ClerkUserinfo {
    pub email: String,
    pub email_verified: bool,
    pub user_id: String,
}

#[derive(Deserialize)]
struct YandexUserinfo {
    pub default_email: String,
    pub id: String,
}

#[derive(Deserialize)]
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

pub async fn login_handler() -> Response {
    let client = match get_client() {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e.full(), "Get OAuth2 client error");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let conf = config::get();

    let mut request = client.authorize_url(CsrfToken::new_random);

    for scope in &conf.user_authentication.oauth2.scopes {
        request = request.add_scope(Scope::new(scope.to_string()))
    }
    let (auth_url, csrf_token) = request.set_pkce_challenge(pkce_challenge).url();

    if let Err(e) = store_verifier(&csrf_token, &pkce_verifier).await {
        error!(error = %e.full(), "Store verifier error");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    Redirect::temporary(auth_url.as_str()).into_response()
}

pub async fn callback_handler(args: Query<CallbackArgs>) -> Response {
    let args: CallbackArgs = args.0;
    Redirect::permanent(&format!("/#/login?code={}&state={}", args.code, args.state))
        .into_response()
}

fn get_client() -> Result<Client> {
    let conf = config::get();

    if conf.user_authentication.enabled != "oauth2" {
        return Err(anyhow!("OAuth2 is not enabled"));
    }

    let client = BasicClient::new(ClientId::new(
        conf.user_authentication.oauth2.client_id.clone(),
    ))
    .set_client_secret(ClientSecret::new(
        conf.user_authentication.oauth2.client_secret.clone(),
    ))
    .set_auth_uri(AuthUrl::new(
        conf.user_authentication.oauth2.auth_url.clone(),
    )?)
    .set_token_uri(TokenUrl::new(
        conf.user_authentication.oauth2.token_url.clone(),
    )?)
    .set_redirect_uri(RedirectUrl::new(
        conf.user_authentication.oauth2.redirect_url.clone(),
    )?)
    .set_auth_type(match conf.user_authentication.oauth2.provider.as_ref() {
        "clerk" => AuthType::RequestBody, // clerk does not support BasicAuth
        _ => AuthType::BasicAuth,         // default oauth2 crate value
    });

    Ok(client)
}

pub async fn get_user(code: &str, state: &str) -> Result<User> {
    let state = oauth2::CsrfToken::new(state.to_string());
    let verifier = get_verifier(&state).await?;
    let client = get_client()?;

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let token = match client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .set_pkce_verifier(verifier)
        .request_async(&http_client)
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
    let assume_email_verified = conf.user_authentication.oauth2.assume_email_verified;

    match provider.as_ref() {
        "clerk" => get_clerk_user(access_token, &userinfo_url).await,
        "yandex" => get_yandex_user(access_token, &userinfo_url, assume_email_verified).await,
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

async fn get_yandex_user(token: &str, url: &str, assume_email_verified: bool) -> Result<User> {
    let client = reqwest::Client::new();
    let auth_header = format!("Bearer {}", token);

    let resp: YandexUserinfo = client
        .get(url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .await?
        .json()
        .await?;

    Ok(User {
        email: resp.default_email,
        email_verified: assume_email_verified,
        external_id: resp.id,
    })
}

async fn store_verifier(
    token: &oauth2::CsrfToken,
    verifier: &oauth2::PkceCodeVerifier,
) -> Result<()> {
    trace!("Storing verifier");

    let key = redis_key(format!("auth:oauth2:{}", token.secret()));
    () = redis::cmd("PSETEX")
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
