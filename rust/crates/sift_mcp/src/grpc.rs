use std::{error::Error, fmt};
use tonic::{
    body::Body,
    codegen::{Body as HttpBody, Bytes, StdError, http},
};
use tower::{Service, ServiceExt, util::BoxCloneSyncService};

/// The gRPC transport every service in this crate sends requests through.
///
/// The type is erased so this crate does not depend on how the channel is built. The caller owns
/// the endpoint, TLS, authentication and user agent, and hands over the result with
/// [grpc_channel].
pub type GrpcChannel =
    BoxCloneSyncService<http::Request<Body>, http::Response<Body>, GrpcChannelError>;

/// Erases the type of a gRPC channel, such as a `tonic::transport::Channel` or one wrapped in an
/// interceptor, so the server can use it.
pub fn grpc_channel<S, B>(service: S) -> GrpcChannel
where
    S: Service<http::Request<Body>, Response = http::Response<B>> + Clone + Send + Sync + 'static,
    S::Future: Send + 'static,
    S::Error: Into<StdError> + 'static,
    B: HttpBody<Data = Bytes> + Send + 'static,
    B::Error: Into<StdError>,
{
    BoxCloneSyncService::new(
        service
            .map_response(|response| response.map(Body::new))
            .map_err(|error| GrpcChannelError(error.into())),
    )
}

/// The transport error of a [GrpcChannel].
///
/// It is a named type and not a bare boxed error because the futures of the generated clients do
/// not prove `Send` over `Box<dyn Error>`. It prints as the wrapped error and exposes it as its
/// source, so `tonic::Status::from_error` still finds the cause and assigns the same code.
#[derive(Debug)]
pub struct GrpcChannelError(StdError);

impl fmt::Display for GrpcChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for GrpcChannelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.0)
    }
}
