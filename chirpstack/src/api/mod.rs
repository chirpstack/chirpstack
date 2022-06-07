use std::convert::Infallible;
use std::time::Duration;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use futures::future::{self, Either, TryFutureExt};
use hyper::{service::make_service_fn, Server};
use rust_embed::RustEmbed;
use tonic::transport::Server as TonicServer;
use tonic_reflection::server::Builder as TonicReflectionBuilder;
use tower::{Service, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{event, Level};
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Filter, Rejection, Reply};

use chirpstack_api::api::application_service_server::ApplicationServiceServer;
use chirpstack_api::api::device_profile_service_server::DeviceProfileServiceServer;
use chirpstack_api::api::device_profile_template_service_server::DeviceProfileTemplateServiceServer;
use chirpstack_api::api::device_service_server::DeviceServiceServer;
use chirpstack_api::api::gateway_service_server::GatewayServiceServer;
use chirpstack_api::api::internal_service_server::InternalServiceServer;
use chirpstack_api::api::multicast_group_service_server::MulticastGroupServiceServer;
use chirpstack_api::api::tenant_service_server::TenantServiceServer;
use chirpstack_api::api::user_service_server::UserServiceServer;

use super::config;
use crate::api::auth::validator;

pub mod application;
pub mod auth;
pub mod device;
pub mod device_profile;
pub mod device_profile_template;
pub mod error;
pub mod gateway;
pub mod helpers;
pub mod internal;
pub mod multicast;
pub mod oidc;
pub mod tenant;
pub mod user;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(RustEmbed)]
#[folder = "../ui/build"]
struct Asset;

pub async fn setup() -> Result<()> {
    let conf = config::get();
    let addr = conf.api.bind.parse()?;

    event!(
        Level::INFO,
        bind = conf.api.bind.as_str(),
        "Setting up API interface"
    );

    // Taken from the tonic hyper_warp_multiplex example:
    // https://github.com/hyperium/tonic/blob/master/examples/src/hyper_warp_multiplex/server.rs#L101
    let service = make_service_fn(move |_| {
        // tonic gRPC service
        let tonic_service = TonicServer::builder()
            .accept_http1(true)
            .add_service(
                TonicReflectionBuilder::configure()
                    .register_encoded_file_descriptor_set(chirpstack_api::api::DESCRIPTOR)
                    .build()
                    .unwrap(),
            )
            .add_service(tonic_web::enable(InternalServiceServer::with_interceptor(
                internal::Internal::new(
                    validator::RequestValidator::new(),
                    conf.api.secret.clone(),
                ),
                auth::auth_interceptor,
            )))
            .add_service(tonic_web::enable(
                ApplicationServiceServer::with_interceptor(
                    application::Application::new(validator::RequestValidator::new()),
                    auth::auth_interceptor,
                ),
            ))
            .add_service(tonic_web::enable(
                DeviceProfileServiceServer::with_interceptor(
                    device_profile::DeviceProfile::new(validator::RequestValidator::new()),
                    auth::auth_interceptor,
                ),
            ))
            .add_service(tonic_web::enable(
                DeviceProfileTemplateServiceServer::with_interceptor(
                    device_profile_template::DeviceProfileTemplate::new(
                        validator::RequestValidator::new(),
                    ),
                    auth::auth_interceptor,
                ),
            ))
            .add_service(tonic_web::enable(TenantServiceServer::with_interceptor(
                tenant::Tenant::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            )))
            .add_service(tonic_web::enable(DeviceServiceServer::with_interceptor(
                device::Device::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            )))
            .add_service(tonic_web::enable(UserServiceServer::with_interceptor(
                user::User::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            )))
            .add_service(tonic_web::enable(GatewayServiceServer::with_interceptor(
                gateway::Gateway::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            )))
            .add_service(tonic_web::enable(
                MulticastGroupServiceServer::with_interceptor(
                    multicast::MulticastGroup::new(validator::RequestValidator::new()),
                    auth::auth_interceptor,
                ),
            ))
            .into_service();
        let mut tonic_service = ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_grpc()
                    .make_span_with(|req: &http::Request<hyper::Body>| {
                        tracing::info_span!(
                        "gRPC",
                        uri = %req.uri().path(),
                        )
                    })
                    .on_request(OnRequest {})
                    .on_response(OnResponse {}),
            )
            .service(tonic_service);

        // HTTP service
        let warp_service = warp::service(
            warp::path!("auth" / "oidc" / "login")
                .and_then(oidc::login_handler)
                .or(warp::path!("auth" / "oidc" / "callback")
                    .and(warp::query::<oidc::CallbackArgs>())
                    .and_then(oidc::callback_handler))
                .or(warp::path::tail().and_then(http_serve)),
        );
        let mut warp_service = ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|req: &http::Request<hyper::Body>| {
                        tracing::info_span!(
                        "http",
                        method = req.method().as_str(),
                        uri = %req.uri().path(),
                        version = ?req.version(),
                        )
                    })
                    .on_request(OnRequest {})
                    .on_response(OnResponse {}),
            )
            .service(warp_service);

        future::ok::<_, Infallible>(tower::service_fn(
            move |req: hyper::Request<hyper::Body>| match req.method() {
                &hyper::Method::GET => Either::Left(
                    warp_service
                        .call(req)
                        .map_ok(|res| res.map(EitherBody::Right))
                        .map_err(Error::from),
                ),
                _ => Either::Right(
                    tonic_service
                        .call(req)
                        .map_ok(|res| res.map(EitherBody::Left))
                        .map_err(Error::from),
                ),
            },
        ))
    });

    Server::bind(&addr).serve(service).await?;

    Ok(())
}

enum EitherBody<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> http_body::Body for EitherBody<A, B>
where
    A: http_body::Body + Send + Unpin,
    B: http_body::Body<Data = A::Data> + Send + Unpin,
    A::Error: Into<Error>,
    B::Error: Into<Error>,
{
    type Data = A::Data;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn is_end_stream(&self) -> bool {
        match self {
            EitherBody::Left(b) => b.is_end_stream(),
            EitherBody::Right(b) => b.is_end_stream(),
        }
    }

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_data(cx).map(map_option_err),
            EitherBody::Right(b) => Pin::new(b).poll_data(cx).map(map_option_err),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
            EitherBody::Right(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
        }
    }
}

fn map_option_err<T, U: Into<Error>>(err: Option<Result<T, U>>) -> Option<Result<T, Error>> {
    err.map(|e| e.map_err(Into::into))
}

async fn http_serve(path: Tail) -> Result<impl Reply, Rejection> {
    let mut path = path.as_str();
    if path.is_empty() {
        path = "index.html";
    }

    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res = Response::new(asset.into());
    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}

#[derive(Debug, Clone)]
struct OnRequest {}

impl<B> tower_http::trace::OnRequest<B> for OnRequest {
    fn on_request(&mut self, _: &http::Request<B>, _: &tracing::Span) {
        tracing::debug!("Started processing request");
    }
}

#[derive(Debug, Clone)]
struct OnResponse {}

impl<B> tower_http::trace::OnResponse<B> for OnResponse {
    fn on_response(self, resp: &http::Response<B>, latency: Duration, _: &tracing::Span) {
        tracing::info!(status = resp.status().as_str(), latency = ?latency, "Finished processing request");
    }
}
