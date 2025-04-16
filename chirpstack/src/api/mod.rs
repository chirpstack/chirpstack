use std::sync::LazyLock;
use std::time::{Duration, Instant};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::{Context as AnyhowContext, Result};
use axum::{response::IntoResponse, routing::get, Router};
use http::{
    header::{self, HeaderMap, HeaderValue},
    Request, StatusCode, Uri,
};
use pin_project::pin_project;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use rust_embed::RustEmbed;
use tokio::task;
use tokio::try_join;
use tonic::transport::Server as TonicServer;
use tonic::Code;
use tonic_reflection::server::Builder as TonicReflectionBuilder;
use tonic_web::GrpcWebLayer;
use tower::util::ServiceExt;
use tower::Service;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use chirpstack_api::api::application_service_server::ApplicationServiceServer;
use chirpstack_api::api::device_profile_service_server::DeviceProfileServiceServer;
use chirpstack_api::api::device_profile_template_service_server::DeviceProfileTemplateServiceServer;
use chirpstack_api::api::device_service_server::DeviceServiceServer;
use chirpstack_api::api::fuota_service_server::FuotaServiceServer;
use chirpstack_api::api::gateway_service_server::GatewayServiceServer;
use chirpstack_api::api::internal_service_server::InternalServiceServer;
use chirpstack_api::api::multicast_group_service_server::MulticastGroupServiceServer;
use chirpstack_api::api::relay_service_server::RelayServiceServer;
use chirpstack_api::api::tenant_service_server::TenantServiceServer;
use chirpstack_api::api::user_service_server::UserServiceServer;
use chirpstack_api::stream as stream_pb;

use super::config;
use crate::api::auth::validator;
use crate::helpers::errors::PrintFullError;
use crate::monitoring::prometheus;
use crate::stream;

pub mod application;
pub mod auth;
pub mod backend;
pub mod device;
pub mod device_profile;
pub mod device_profile_template;
pub mod error;
pub mod fuota;
pub mod gateway;
mod grpc_multiplex;
pub mod helpers;
pub mod internal;
pub mod monitoring;
pub mod multicast;
pub mod oauth2;
pub mod oidc;
pub mod relay;
pub mod tenant;
pub mod user;

static GRPC_COUNTER: LazyLock<Family<GrpcLabels, Counter>> = LazyLock::new(|| {
    let counter = Family::<GrpcLabels, Counter>::default();
    prometheus::register(
        "api_requests_handled",
        "Number of API requests handled by service, method and status code",
        counter.clone(),
    );
    counter
});
static GRPC_HISTOGRAM: LazyLock<Family<GrpcLabels, Histogram>> = LazyLock::new(|| {
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
});

#[derive(RustEmbed)]
#[folder = "../ui/build"]
struct Asset;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub async fn setup() -> Result<()> {
    let conf = config::get();
    let bind = conf.api.bind.parse().context("Parse api.bind config")?;

    info!(bind = %bind, "Setting up API interface");

    let web = Router::new()
        .route("/auth/oidc/login", get(oidc::login_handler))
        .route("/auth/oidc/callback", get(oidc::callback_handler))
        .route("/auth/oauth2/login", get(oauth2::login_handler))
        .route("/auth/oauth2/callback", get(oauth2::callback_handler))
        .fallback(service_static_handler)
        .into_service()
        .map_response(|r| r.map(tonic::body::Body::new));

    let grpc = TonicServer::builder()
        .accept_http1(true)
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(|req: &Request<_>| {
                    tracing::info_span!(
                    "gRPC",
                    uri = %req.uri().path(),
                    )
                })
                .on_request(OnRequest {})
                .on_response(OnResponse {}),
        )
        .layer(grpc_multiplex::GrpcMultiplexLayer::new(web))
        .layer(ApiLoggerLayer {})
        .layer(GrpcWebLayer::new())
        .add_service(
            TonicReflectionBuilder::configure()
                .register_encoded_file_descriptor_set(chirpstack_api::api::DESCRIPTOR)
                .build_v1()
                .unwrap(),
        )
        .add_service(InternalServiceServer::with_interceptor(
            internal::Internal::new(validator::RequestValidator::new(), conf.api.secret.clone()),
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
            device_profile_template::DeviceProfileTemplate::new(validator::RequestValidator::new()),
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
        .add_service(FuotaServiceServer::with_interceptor(
            fuota::Fuota::new(validator::RequestValidator::new()),
            auth::auth_interceptor,
        ));

    let backend_handle = tokio::spawn(backend::setup());
    let monitoring_handle = tokio::spawn(monitoring::setup());
    let grpc_handle = tokio::spawn(grpc.serve(bind));

    tokio::spawn(async move {
        if let Err(e) = try_join!(grpc_handle, backend_handle, monitoring_handle) {
            error!(error = %e, "Setup API error");
            std::process::exit(-1);
        }
    });

    Ok(())
}

async fn service_static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = {
        let mut chars = uri.path().chars();
        chars.next();
        chars.as_str()
    };
    if path.is_empty() {
        path = "index.html";
    }

    if let Some(asset) = Asset::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime.as_ref()).unwrap(),
        );
        (StatusCode::OK, headers, asset.data.into())
    } else {
        (StatusCode::NOT_FOUND, HeaderMap::new(), vec![])
    }
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

#[derive(Debug, Clone)]
struct ApiLoggerLayer {}

impl<S> tower::Layer<S> for ApiLoggerLayer {
    type Service = ApiLoggerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiLoggerService { inner }
    }
}

#[derive(Debug, Clone)]
struct ApiLoggerService<S> {
    inner: S,
}

impl<ReqBody, ResBody, S> Service<http::Request<ReqBody>> for ApiLoggerService<S>
where
    ResBody: http_body::Body,
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>>,
    S::Error: Into<BoxError> + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ApiLoggerFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<ReqBody>) -> Self::Future {
        let uri = request.uri().path().to_string();
        let uri_parts: Vec<&str> = uri.split('/').collect();
        let future = self.inner.call(request);
        let start = Instant::now();
        ApiLoggerFuture {
            future,
            start,
            service: uri_parts.get(1).map(|v| v.to_string()).unwrap_or_default(),
            method: uri_parts.get(2).map(|v| v.to_string()).unwrap_or_default(),
        }
    }
}

#[pin_project]
struct ApiLoggerFuture<F> {
    #[pin]
    future: F,
    start: Instant,
    service: String,
    method: String,
}

impl<ResBody, F, E> Future for ApiLoggerFuture<F>
where
    ResBody: http_body::Body,
    F: Future<Output = Result<http::Response<ResBody>, E>>,
    E: Into<BoxError> + Send,
{
    type Output = Result<http::Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.future.poll(cx) {
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
                    let req_log = stream_pb::ApiRequestLog {
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
                        if let Err(e) = stream::api_request::log_request(&req_log).await {
                            error!(error = %e.full(), "Log request error");
                        }
                    });
                }
                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
