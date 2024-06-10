use anyhow::Result;
use std::time::Duration;
use tonic::{
    service::interceptor::InterceptedService,
    transport::channel::{Channel, Endpoint},
};
use tower::ServiceBuilder;

/// Concerned with initializing the initial credentials object.
mod credentials;
use credentials::Credentials;

/// gRPC channel interceptor.
mod interceptor;
use interceptor::{AuthInterceptor, SetTimeoutInterceptor};

/// Max time in seconds to wait to establish a connection.
pub const CONNECT_TIMEOUT_SECS: u64 = 5;

/// Max time to allow a request to complete. Set on client-side and also used to inform
/// server via `grpc-timeout` header.
pub const REQUEST_TIMEOUT_SECS: u64 = 30;

pub type SiftChannel =
    InterceptedService<InterceptedService<Channel, AuthInterceptor>, SetTimeoutInterceptor>;

/// Constructs a lazy channel that only establishes a connection when called for the first time.
/// This channel has interceptors to handle authorization and setting timeout-related metadata.
/// Should be used inside of an async context.
pub fn use_channel() -> Result<SiftChannel> {
    let Credentials { uri, apikey } = Credentials::new()?;

    let channel = Endpoint::from_shared(uri)?
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .connect_lazy();

    let intercepted_channel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(SetTimeoutInterceptor))
        .layer(tonic::service::interceptor(AuthInterceptor { apikey }))
        .service(channel);

    Ok(intercepted_channel)
}
