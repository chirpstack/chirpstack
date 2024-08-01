use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::ready;
use http::{header::CONTENT_TYPE, Request, Response};
use http_body::Body;
use pin_project::pin_project;
use tower::{Layer, Service};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[pin_project(project = GrpcMultiplexFutureEnumProj)]
enum GrpcMultiplexFutureEnum<FS, FO> {
    Grpc {
        #[pin]
        future: FS,
    },
    Other {
        #[pin]
        future: FO,
    },
}

#[pin_project]
pub struct GrpcMultiplexFuture<FS, FO> {
    #[pin]
    future: GrpcMultiplexFutureEnum<FS, FO>,
}

impl<ResBody, FS, FO, ES, EO> Future for GrpcMultiplexFuture<FS, FO>
where
    ResBody: Body,
    FS: Future<Output = Result<Response<ResBody>, ES>>,
    FO: Future<Output = Result<Response<ResBody>, EO>>,
    ES: Into<BoxError> + Send,
    EO: Into<BoxError> + Send,
{
    type Output = Result<Response<ResBody>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.project() {
            GrpcMultiplexFutureEnumProj::Grpc { future } => future.poll(cx).map_err(Into::into),
            GrpcMultiplexFutureEnumProj::Other { future } => future.poll(cx).map_err(Into::into),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GrpcMultiplexService<S, O> {
    grpc: S,
    other: O,
    grpc_ready: bool,
    other_ready: bool,
}

impl<ReqBody, ResBody, S, O> Service<Request<ReqBody>> for GrpcMultiplexService<S, O>
where
    ResBody: Body,
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    O: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Error: Into<BoxError> + Send,
    O::Error: Into<BoxError> + Send,
{
    type Response = S::Response;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    type Future = GrpcMultiplexFuture<S::Future, O::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {
            match (self.grpc_ready, self.other_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(self.grpc.poll_ready(cx)).map_err(Into::into)?;
                    self.grpc_ready = true;
                }
                (_, false) => {
                    ready!(self.other.poll_ready(cx)).map_err(Into::into)?;
                    self.other_ready = true;
                }
            }
        }
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        assert!(self.grpc_ready);
        assert!(self.other_ready);

        if is_grpc_request(&request) {
            GrpcMultiplexFuture {
                future: GrpcMultiplexFutureEnum::Grpc {
                    future: self.grpc.call(request),
                },
            }
        } else {
            GrpcMultiplexFuture {
                future: GrpcMultiplexFutureEnum::Other {
                    future: self.other.call(request),
                },
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GrpcMultiplexLayer<O> {
    other: O,
}

impl<O> GrpcMultiplexLayer<O> {
    pub fn new(other: O) -> Self {
        Self { other }
    }
}

impl<S, O> Layer<S> for GrpcMultiplexLayer<O>
where
    O: Clone,
{
    type Service = GrpcMultiplexService<S, O>;

    fn layer(&self, grpc: S) -> Self::Service {
        GrpcMultiplexService {
            grpc,
            other: self.other.clone(),
            grpc_ready: false,
            other_ready: false,
        }
    }
}

fn is_grpc_request<B>(req: &Request<B>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .map(|content_type| content_type.as_bytes())
        .filter(|content_type| content_type.starts_with(b"application/grpc"))
        .is_some()
}
