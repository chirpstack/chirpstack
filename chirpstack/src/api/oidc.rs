use std::collections::HashMap;

use anyhow::{Context, Result};
use axum::{
    Form,
    extract::{Query, rejection::FormRejection},
    response::{IntoResponse, Redirect, Response},
};
use chrono::{Duration, Utc};
use http::{StatusCode, header::CACHE_CONTROL};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use openidconnect::core::{
    CoreClient, CoreGenderClaim, CoreIdTokenVerifier, CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreResponseType,
};
use openidconnect::{AdditionalClaims, UserInfoClaims, reqwest};
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, trace, warn};
use uuid::Uuid;

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{error::Error as StorageError, get_async_redis_conn, redis_key, user};

const BACKCHANNEL_LOGOUT_EVENT: &str = "http://schemas.openid.net/event/backchannel-logout";

pub type User = UserInfoClaims<CustomClaims, CoreGenderClaim>;

type IdToken = openidconnect::IdToken<
    CustomClaims,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
>;

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Session {
    pub sid: Option<String>,
    pub iat: i64,
}

#[derive(Deserialize)]
pub struct LogoutForm {
    pub logout_token: String,
}

#[derive(Deserialize)]
struct LogoutClaims {
    iat: i64,
    sub: Option<String>,
    sid: Option<String>,
    events: Value,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct LogoutProviderMetadata {
    issuer: IssuerUrl,
    jwks_uri: reqwest::Url,
    id_token_signing_alg_values_supported: Vec<String>,
}

pub async fn login_handler() -> Response {
    let client = match get_client().await {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e.full(), "Get OIDC client error");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let conf = config::get();
    let mut request = client.authorize_url(
        AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    );
    for scope in &conf.user_authentication.openid_connect.scopes {
        request = request.add_scope(Scope::new(scope.to_string()))
    }

    request = request.set_pkce_challenge(pkce_challenge);

    let (auth_url, csrf_state, nonce) = request.url();

    if let Err(e) = store_nonce(&csrf_state, &nonce).await {
        error!(error = %e.full(), "Store nonce error");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    if let Err(e) = store_verifier(&csrf_state, &pkce_verifier).await {
        error!(error = %e.full(), "Store PKCE verifier error");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    Redirect::temporary(auth_url.as_str()).into_response()
}

pub async fn callback_handler(args: Query<CallbackArgs>) -> Response {
    let args: CallbackArgs = args.0;
    Redirect::permanent(&format!("/#/login?code={}&state={}", args.code, args.state))
        .into_response()
}

pub async fn logout_handler(form: Result<Form<LogoutForm>, FormRejection>) -> Response {
    let conf = config::get();
    if conf.user_authentication.enabled != "openid_connect"
        || !conf
            .user_authentication
            .openid_connect
            .backchannel_logout_enabled
    {
        return StatusCode::NOT_FOUND.into_response();
    }

    let status = match form {
        Ok(Form(form)) => match logout(&form.logout_token).await {
            Ok(_) => StatusCode::OK,
            Err(e) => {
                warn!(error = %e.full(), "OIDC back-channel logout error");
                StatusCode::BAD_REQUEST
            }
        },
        Err(_) => StatusCode::BAD_REQUEST,
    };

    (status, [(CACHE_CONTROL, "no-store")]).into_response()
}

pub async fn get_user(code: &str, state: &str) -> Result<(User, Option<Session>)> {
    let state = CsrfToken::new(state.to_string());
    let nonce = get_nonce(&state).await?;
    let pkce_verifier = get_verifier(&state).await?;
    let client = get_client().await?;

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code.to_string()))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    let id_token_verifier: CoreIdTokenVerifier = client.id_token_verifier();
    let id_token: IdToken = token_response
        .extra_fields()
        .id_token()
        .context("Server did not return an ID token")?
        .to_string()
        .parse()?;
    let id_token_claims = id_token
        .claims(&id_token_verifier, &nonce)
        .context("Failed to verify ID token")?;

    let userinfo_claims: User = client
        .user_info(token_response.access_token().to_owned(), None)
        .context("No user info endpoint")?
        .request_async(&http_client)
        .await
        .context("Failed requesting user info")?;

    if userinfo_claims.subject() != id_token_claims.subject() {
        return Err(anyhow!("UserInfo subject does not match ID token subject"));
    }

    let session = if config::get()
        .user_authentication
        .openid_connect
        .backchannel_logout_enabled
    {
        Some(Session {
            sid: id_token_claims
                .additional_claims()
                .other
                .get("sid")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string()),
            iat: id_token_claims.issue_time().timestamp(),
        })
    } else {
        None
    };

    Ok((userinfo_claims, session))
}

// Both the session and the stored logout iat are set by the provider, so this comparison is not
// affected by clock skew between ChirpStack and the provider.
pub async fn is_logged_out(user_id: &Uuid, session: &Session) -> Result<bool> {
    let mut keys = vec![user_logout_key(user_id)];
    if let Some(sid) = &session.sid {
        keys.push(sid_logout_key(sid));
    }

    let mut conn = get_async_redis_conn().await?;
    for key in keys {
        let logout_iat: Option<i64> = redis::cmd("ZSCORE")
            .arg(&key)
            .arg("iat")
            .query_async(&mut conn)
            .await?;
        if logout_iat.is_some_and(|v| session.iat <= v) {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn logout(token: &str) -> Result<()> {
    // Reject malformed tokens before making any requests to the provider.
    decode_header(token)?;

    let conf = config::get();
    let issuer = IssuerUrl::new(conf.user_authentication.openid_connect.provider_url.clone())?;

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let metadata: LogoutProviderMetadata = http_client
        .get(format!(
            "{}/.well-known/openid-configuration",
            issuer.as_str().trim_end_matches('/')
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    if metadata.issuer != issuer {
        return Err(anyhow!("Discovered issuer does not match provider URL"));
    }

    let jwks: JwkSet = http_client
        .get(metadata.jwks_uri)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let algs: Vec<Algorithm> = metadata
        .id_token_signing_alg_values_supported
        .iter()
        .filter_map(|v| v.parse().ok())
        .collect();

    let claims = verify_logout_token(
        token,
        issuer.as_str(),
        &conf.user_authentication.openid_connect.client_id,
        &jwks,
        &algs,
    )
    .context("Verify logout token")?;

    let key = match (&claims.sid, &claims.sub) {
        (Some(sid), _) => sid_logout_key(sid),
        (None, Some(sub)) => match user::get_by_external_id(sub).await {
            Ok(u) => user_logout_key(&u.id),
            Err(StorageError::NotFound(_)) => return Ok(()),
            Err(e) => return Err(e.into()),
        },
        (None, None) => return Err(anyhow!("Logout token must contain sub or sid")),
    };

    // GT makes sure that out-of-order notifications never lower the stored logout iat. The
    // expiration must outlive the user tokens (valid for one day) issued before this logout.
    () = redis::pipe()
        .atomic()
        .cmd("ZADD")
        .arg(&key)
        .arg("GT")
        .arg(claims.iat)
        .arg("iat")
        .ignore()
        .cmd("PEXPIRE")
        .arg(&key)
        .arg(Duration::try_hours(25).unwrap().num_milliseconds())
        .ignore()
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    Ok(())
}

fn verify_logout_token(
    token: &str,
    issuer: &str,
    client_id: &str,
    jwks: &JwkSet,
    algs: &[Algorithm],
) -> Result<LogoutClaims> {
    let header = decode_header(token)?;
    if !algs.contains(&header.alg) {
        return Err(anyhow!("Unsupported signing algorithm: {:?}", header.alg));
    }

    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[client_id]);
    validation.set_required_spec_claims(&["exp", "iss", "aud"]);

    let claims = jwks
        .keys
        .iter()
        .filter(|k| header.kid.is_none() || k.common.key_id == header.kid)
        .filter_map(|k| DecodingKey::from_jwk(k).ok())
        .find_map(|k| decode::<LogoutClaims>(token, &k, &validation).ok())
        .ok_or_else(|| anyhow!("Invalid logout token"))?
        .claims;

    if !claims
        .events
        .get(BACKCHANNEL_LOGOUT_EVENT)
        .is_some_and(|v| v.is_object())
    {
        return Err(anyhow!("Logout token does not contain the logout event"));
    }
    if claims.iat > Utc::now().timestamp() + validation.leeway as i64 {
        return Err(anyhow!("Logout token iat is in the future"));
    }
    if !claims.other.get("jti").is_some_and(|v| v.is_string()) {
        return Err(anyhow!("Logout token does not contain a jti"));
    }
    if claims.other.contains_key("nonce") {
        return Err(anyhow!("Logout token must not contain a nonce"));
    }

    Ok(claims)
}

fn user_logout_key(user_id: &Uuid) -> String {
    redis_key(format!("auth:oidc:logout:user:{}", user_id))
}

fn sid_logout_key(sid: &str) -> String {
    redis_key(format!("auth:oidc:logout:sid:{}", sid))
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

async fn store_verifier(state: &CsrfToken, verifier: &PkceCodeVerifier) -> Result<()> {
    trace!("Storing PKCE verifier");
    let key = redis_key(format!("auth:oidc:pkce:{}", state.secret()));

    () = redis::cmd("PSETEX")
        .arg(key)
        .arg(Duration::try_minutes(5).unwrap().num_milliseconds())
        .arg(verifier.secret())
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    Ok(())
}

async fn get_verifier(state: &CsrfToken) -> Result<PkceCodeVerifier> {
    trace!("Getting PKCE verifier");
    let key = redis_key(format!("auth:oidc:pkce:{}", state.secret()));

    let v: String = redis::cmd("GET")
        .arg(&key)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("Get PKCE verifier")?;

    Ok(PkceCodeVerifier::new(v))
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

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::{
        auth_interceptor,
        claims::AuthClaim,
        validator::{RequestValidator, ValidateActiveUser},
    };
    use crate::test;
    use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
    use chirpstack_api::tonic::{self, Code};
    use jsonwebtoken::{EncodingKey, Header, encode};
    use rsa::{RsaPrivateKey, pkcs8::EncodePrivateKey, traits::PublicKeyParts};
    use serde_json::json;
    use tower::ServiceExt;

    fn signing_key() -> (EncodingKey, JwkSet) {
        let key = RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 2048).unwrap();
        let pem = key.to_pkcs8_pem(Default::default()).unwrap();
        let jwks = serde_json::from_value(json!({"keys": [{
            "kty": "RSA", "kid": "test", "use": "sig", "alg": "RS256",
            "n": URL_SAFE_NO_PAD.encode(key.n().to_bytes_be()),
            "e": URL_SAFE_NO_PAD.encode(key.e().to_bytes_be()),
        }]}))
        .unwrap();
        (EncodingKey::from_rsa_pem(pem.as_bytes()).unwrap(), jwks)
    }

    fn logout_claims(iss: &str) -> Value {
        let now = Utc::now().timestamp();
        json!({
            "iss": iss, "aud": "chirpstack", "iat": now, "exp": now + 120,
            "jti": Uuid::new_v4().to_string(), "sub": "alice", "sid": "laptop",
            "events": { BACKCHANNEL_LOGOUT_EVENT: {} },
        })
    }

    fn logout_token(key: &EncodingKey, claims: &Value) -> String {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some("test".into());
        encode(&header, claims, key).unwrap()
    }

    async fn post_logout(body: String) -> Response {
        axum::Router::new()
            .route("/auth/oidc/logout", axum::routing::post(logout_handler))
            .oneshot(
                http::Request::builder()
                    .method("POST")
                    .uri("/auth/oidc/logout")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(axum::body::Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    async fn authorize(user_id: &Uuid, session: Session) -> Result<(), tonic::Status> {
        let mut claim = AuthClaim::new_for_user(user_id);
        claim.oidc_session = Some(session);
        let token = claim.encode(config::get().api.secret.as_bytes()).unwrap();

        let mut req = tonic::Request::new(());
        req.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );
        let req = auth_interceptor(req)?;

        RequestValidator::new()
            .validate(req.extensions(), ValidateActiveUser::new())
            .await
    }

    #[test]
    fn test_verify_logout_token() {
        let (key, jwks) = signing_key();
        let issuer = "https://issuer.example";
        let verify = |token: &str, algs: &[Algorithm]| {
            verify_logout_token(token, issuer, "chirpstack", &jwks, algs)
        };
        let valid = logout_claims(issuer);
        assert!(verify(&logout_token(&key, &valid), &[Algorithm::RS256]).is_ok());

        let now = Utc::now().timestamp();
        for (field, value) in [
            ("iss", json!("https://other.example")),
            ("aud", json!("other-client")),
            ("exp", json!(now - 120)),
            ("iat", json!(now + 120)),
            ("jti", Value::Null),
            ("jti", json!(1)),
            ("events", json!({})),
            ("events", json!({ BACKCHANNEL_LOGOUT_EVENT: null })),
            ("nonce", json!("nonce")),
        ] {
            let mut claims = valid.clone();
            claims[field] = value;
            assert!(
                verify(&logout_token(&key, &claims), &[Algorithm::RS256]).is_err(),
                "accepted invalid {}",
                field
            );
        }

        for field in ["iss", "aud", "iat", "exp", "jti", "events"] {
            let mut claims = valid.clone();
            claims.as_object_mut().unwrap().remove(field);
            assert!(
                verify(&logout_token(&key, &claims), &[Algorithm::RS256]).is_err(),
                "accepted missing {}",
                field
            );
        }

        // Signed by an unknown key.
        let (other_key, _) = signing_key();
        assert!(verify(&logout_token(&other_key, &valid), &[Algorithm::RS256]).is_err());

        // Algorithm not advertised by the provider.
        assert!(verify(&logout_token(&key, &valid), &[Algorithm::ES256]).is_err());

        // HMAC signature, even when advertised.
        let hmac = encode(
            &Header::new(Algorithm::HS256),
            &valid,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap();
        assert!(verify(&hmac, &[Algorithm::RS256, Algorithm::HS256]).is_err());
    }

    #[tokio::test]
    async fn test_backchannel_logout() {
        let _guard = test::prepare().await;

        let server = httpmock::MockServer::start_async().await;
        let (key, jwks) = signing_key();
        let issuer = server.base_url();
        server
            .mock_async(|when, then| {
                when.method("GET").path("/.well-known/openid-configuration");
                then.status(200).json_body(json!({
                    "issuer": issuer,
                    "jwks_uri": server.url("/jwks"),
                    "id_token_signing_alg_values_supported": ["RS256"],
                }));
            })
            .await;
        server
            .mock_async(|when, then| {
                when.method("GET").path("/jwks");
                then.status(200)
                    .json_body(serde_json::to_value(&jwks).unwrap());
            })
            .await;

        let mut conf = (*config::get()).clone();
        conf.user_authentication.enabled = "openid_connect".into();
        conf.user_authentication
            .openid_connect
            .backchannel_logout_enabled = true;
        conf.user_authentication.openid_connect.provider_url = issuer.clone();
        conf.user_authentication.openid_connect.client_id = "chirpstack".into();
        config::set(conf);

        let u = user::create(user::User {
            email: "alice@example.com".into(),
            is_active: true,
            external_id: Some("alice".into()),
            ..Default::default()
        })
        .await
        .unwrap();

        let now = Utc::now().timestamp();
        let laptop = Session {
            sid: Some("laptop".into()),
            iat: now - 10,
        };
        let phone = Session {
            sid: Some("phone".into()),
            iat: now - 10,
        };
        let no_sid = Session {
            sid: None,
            iat: now - 10,
        };

        // Invalid requests.
        let resp = post_logout("wrong_field=value".into()).await;
        assert_eq!(StatusCode::BAD_REQUEST, resp.status());
        assert_eq!("no-store", resp.headers()[CACHE_CONTROL]);
        let resp = post_logout("logout_token=not-a-jwt".into()).await;
        assert_eq!(StatusCode::BAD_REQUEST, resp.status());
        let mut claims = logout_claims(&issuer);
        claims.as_object_mut().unwrap().remove("sub");
        claims.as_object_mut().unwrap().remove("sid");
        let resp = post_logout(format!("logout_token={}", logout_token(&key, &claims))).await;
        assert_eq!(StatusCode::BAD_REQUEST, resp.status());

        // Logout by sid only ends that session. Duplicate deliveries succeed.
        authorize(&u.id, laptop.clone()).await.unwrap();
        let claims = logout_claims(&issuer);
        let token = logout_token(&key, &claims);
        for _ in 0..2 {
            let resp = post_logout(format!("logout_token={}", token)).await;
            assert_eq!(StatusCode::OK, resp.status());
            assert_eq!("no-store", resp.headers()[CACHE_CONTROL]);
        }
        assert_eq!(
            Code::Unauthenticated,
            authorize(&u.id, laptop.clone()).await.unwrap_err().code()
        );
        authorize(&u.id, phone.clone()).await.unwrap();
        authorize(&u.id, no_sid.clone()).await.unwrap();

        // Logout by sub ends all sessions of the user issued before the logout.
        let mut claims = logout_claims(&issuer);
        claims.as_object_mut().unwrap().remove("sid");
        let resp = post_logout(format!("logout_token={}", logout_token(&key, &claims))).await;
        assert_eq!(StatusCode::OK, resp.status());
        for session in [&phone, &no_sid] {
            assert!(is_logged_out(&u.id, session).await.unwrap());
        }
        let new_session = Session {
            sid: Some("tablet".into()),
            iat: claims["iat"].as_i64().unwrap() + 1,
        };
        assert!(!is_logged_out(&u.id, &new_session).await.unwrap());

        // Older notifications delivered later never lower the logout iat.
        let latest = Utc::now().timestamp() - 10;
        for iat in [latest, latest - 5] {
            let mut claims = logout_claims(&issuer);
            claims["sid"] = json!("tv");
            claims["iat"] = json!(iat);
            let resp = post_logout(format!("logout_token={}", logout_token(&key, &claims))).await;
            assert_eq!(StatusCode::OK, resp.status());
        }
        for (iat, logged_out) in [(latest, true), (latest + 1, false)] {
            let session = Session {
                sid: Some("tv".into()),
                iat,
            };
            assert_eq!(
                logged_out,
                is_logged_out(&Uuid::new_v4(), &session).await.unwrap()
            );
        }

        // Unknown users have nothing to log out.
        claims["sub"] = json!("bob");
        let resp = post_logout(format!("logout_token={}", logout_token(&key, &claims))).await;
        assert_eq!(StatusCode::OK, resp.status());
    }
}
