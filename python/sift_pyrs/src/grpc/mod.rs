use tonic::{
    service::interceptor::InterceptedService,
    transport::{
        channel::{Channel, Endpoint},
        Error,
    },
};
use tower::ServiceBuilder;

/// gRPC channel interceptor.
mod interceptor;
use interceptor::AuthInterceptor;

pub type SiftChannel = InterceptedService<Channel, AuthInterceptor>;

/// Constructs a lazy channel that only establishes a connection when called for the first time.
/// Should be used inside of an async context.
pub fn use_channel(uri: &str, apikey: &str) -> Result<SiftChannel, Error> {
    let channel = Endpoint::from_shared(uri.to_string())?.connect_lazy();

    let intercepted_channel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(AuthInterceptor {
            apikey: apikey.to_string(),
        }))
        .service(channel);

    Ok(intercepted_channel)
}
