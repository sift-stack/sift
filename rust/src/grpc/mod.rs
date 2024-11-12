use crate::error::{Error, Result, SiftError};
use tonic::{
    service::interceptor::InterceptedService,
    transport::channel::{Channel, Endpoint},
};
use tower::ServiceBuilder;

mod config;
pub use config::SiftChannelConfig;

mod interceptor;
use interceptor::AuthInterceptor;

/// A pre-configured gRPC channel to conveniently establish a connection to Sift's gRPC API.
pub type SiftChannel = InterceptedService<Channel, AuthInterceptor>;

/// Uses `channel_config` to initialize a lazy channel that will only establish a connection
/// after first-use. This can only be used within a [`Tokio 1.x runtime`].
///
/// [`Tokio 1.x runtime`]: https://docs.rs/tokio/latest/tokio/
pub fn use_sift_channel(channel_config: SiftChannelConfig) -> Result<SiftChannel> {
    let SiftChannelConfig { uri, apikey } = channel_config;

    let channel = Endpoint::from_shared(uri)
        .map_err(Error::new_user_error)
        .context("something went while trying to establish a connection to Sift")
        .help("double check that the URL and the API token are both valid")?
        .connect_lazy();

    let intercepted_channel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(AuthInterceptor { apikey }))
        .service(channel);

    Ok(intercepted_channel)
}
