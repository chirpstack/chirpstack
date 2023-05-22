use std::convert::Infallible;
use std::time::{Duration, Instant};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use futures::future::{self, Either, TryFutureExt};
use hyper::{service::make_service_fn, Server};
use pin_project::pin_project;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use rust_embed::RustEmbed;
use tokio::{task, try_join};
use tonic::transport::Server as TonicServer;
use tonic::Code;
use tonic_reflection::server::Builder as TonicReflectionBuilder;
use tonic_web::GrpcWebLayer;
use tower::{Service, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Filter, Rejection, Reply};

use chirpstack_api::api;
use chirpstack_api::api::application_service_server::ApplicationServiceServer;
use chirpstack_api::api::device_profile_service_server::DeviceProfileServiceServer;
use chirpstack_api::api::device_profile_template_service_server::DeviceProfileTemplateServiceServer;
use chirpstack_api::api::device_service_server::DeviceServiceServer;
use chirpstack_api::api::gateway_service_server::GatewayServiceServer;
use chirpstack_api::api::internal_service_server::InternalServiceServer;
use chirpstack_api::api::multicast_group_service_server::MulticastGroupServiceServer;
use chirpstack_api::api::relay_service_server::RelayServiceServer;
use chirpstack_api::api::tenant_service_server::TenantServiceServer;
use chirpstack_api::api::user_service_server::UserServiceServer;

use super::config;
use crate::api::auth::validator;
use crate::monitoring::prometheus;
use crate::requestlog;

pub mod application;
pub mod auth;
pub mod backend;
pub mod device;
pub mod device_profile;
pub mod device_profile_template;
pub mod error;
pub mod gateway;
pub mod helpers;
pub mod internal;
pub mod monitoring;
pub mod multicast;
pub mod oidc;
pub mod relay;
pub mod tenant;
pub mod user;

lazy_static! {
    static ref GRPC_COUNTER: Family<GrpcLabels, Counter> = {
        let counter = Family::<GrpcLabels, Counter>::default();
        prometheus::register(
            "api_requests_handled",
            "Number of API requests handled by service, method and status code",
            counter.clone(),
        );
        counter
    };
    static ref GRPC_HISTOGRAM: Family<GrpcLabels, Histogram> = {
        let histogram = Family::<GrpcLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(
                [
                    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                ]
                .into_iter(),
            )
        });
        prometheus::register(
            "api_requests_handled_seconds",
            "Duration of API requests handled by service, method and status code",
            histogram.clone(),
        );
        histogram
    };
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(RustEmbed)]
#[folder = "../ui/build"]
struct Asset;

pub async fn setup() -> Result<()> {
    let conf = config::get();
    let addr = conf.api.bind.parse()?;

    info!(bind = %conf.api.bind, "Setting up API interface");

    // Taken from the tonic hyper_warp_multiplex example:
    // https://github.com/hyperium/tonic/blob/master/examples/src/hyper_warp_multiplex/server.rs#L101
    let service = make_service_fn(move |_| {
        // tonic gRPC service
        let tonic_service = TonicServer::builder()
            .accept_http1(true)
            .layer(GrpcWebLayer::new())
            .add_service(
                TonicReflectionBuilder::configure()
                    .register_encoded_file_descriptor_set(chirpstack_api::api::DESCRIPTOR)
                    .build()
                    .unwrap(),
            )
            .add_service(InternalServiceServer::with_interceptor(
                internal::Internal::new(
                    validator::RequestValidator::new(),
                    conf.api.secret.clone(),
                ),
                auth::auth_interceptor,
            ))
            .add_service(ApplicationServiceServer::with_interceptor(
                application::Application::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(DeviceProfileServiceServer::with_interceptor(
                device_profile::DeviceProfile::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(DeviceProfileTemplateServiceServer::with_interceptor(
                device_profile_template::DeviceProfileTemplate::new(
                    validator::RequestValidator::new(),
                ),
                auth::auth_interceptor,
            ))
            .add_service(TenantServiceServer::with_interceptor(
                tenant::Tenant::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(DeviceServiceServer::with_interceptor(
                device::Device::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(UserServiceServer::with_interceptor(
                user::User::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(GatewayServiceServer::with_interceptor(
                gateway::Gateway::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(MulticastGroupServiceServer::with_interceptor(
                multicast::MulticastGroup::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
            ))
            .add_service(RelayServiceServer::with_interceptor(
                relay::Relay::new(validator::RequestValidator::new()),
                auth::auth_interceptor,
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
            .layer(ApiLogger {})
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

    let backend_handle = tokio::spawn(backend::setup());
    let monitoring_handle = tokio::spawn(monitoring::setup());
    let api_handle = tokio::spawn(Server::bind(&addr).serve(service));

    let _ = try_join!(api_handle, backend_handle, monitoring_handle)?;

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

    let mut res = Response::new(asset.data.into());
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

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
struct GrpcLabels {
    service: String,
    method: String,
    status_code: String,
}

struct ApiLogger {}

impl<S> tower::Layer<S> for ApiLogger {
    type Service = ApiLoggerService<S>;

    fn layer(&self, service: S) -> Self::Service {
        ApiLoggerService { inner: service }
    }
}

#[derive(Debug, Clone)]
struct ApiLoggerService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for ApiLoggerService<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>>,
    ReqBody: http_body::Body,
    ResBody: http_body::Body,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ApiLoggerResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<ReqBody>) -> Self::Future {
        let uri = request.uri().path().to_string();
        let uri_parts: Vec<&str> = uri.split('/').collect();
        let response_future = self.inner.call(request);
        let start = Instant::now();
        ApiLoggerResponseFuture {
            response_future,
            start,
            service: uri_parts.get(1).map(|v| v.to_string()).unwrap_or_default(),
            method: uri_parts.get(2).map(|v| v.to_string()).unwrap_or_default(),
        }
    }
}

#[pin_project]
struct ApiLoggerResponseFuture<F> {
    #[pin]
    response_future: F,
    start: Instant,
    service: String,
    method: String,
}

impl<F, ResBody, Error> Future for ApiLoggerResponseFuture<F>
where
    F: Future<Output = Result<http::Response<ResBody>, Error>>,
    ResBody: http_body::Body,
{
    type Output = Result<http::Response<ResBody>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                if let Ok(response) = &result {
                    let status_code: i32 = match response.headers().get("grpc-status") {
                        None => 0,
                        Some(v) => match v.to_str() {
                            Ok(s) => s.parse().unwrap_or_default(),
                            Err(_) => 2,
                        },
                    };
                    let status_code = Code::from_i32(status_code);

                    // Log to Prometheus
                    let labels = GrpcLabels {
                        service: this.service.clone(),
                        method: this.method.clone(),
                        status_code: format!("{:?}", status_code),
                    };
                    GRPC_COUNTER.get_or_create(&labels).inc();
                    GRPC_HISTOGRAM
                        .get_or_create(&labels)
                        .observe(this.start.elapsed().as_secs_f64());

                    // Log API request to Redis
                    let req_log = api::RequestLog {
                        service: this.service.to_string(),
                        method: this.method.to_string(),
                        metadata: response
                            .headers()
                            .iter()
                            .filter(|(k, _)| k.as_str().starts_with("x-log-"))
                            .map(|(k, v)| {
                                (
                                    k.as_str()
                                        .strip_prefix("x-log-")
                                        .unwrap_or_default()
                                        .to_string(),
                                    v.to_str().unwrap().to_string(),
                                )
                            })
                            .collect(),
                    };

                    task::spawn(async move {
                        if let Err(err) = requestlog::log_request(&req_log).await {
                            error!("Log request error, error: {}", err);
                        }
                    });
                }
                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
