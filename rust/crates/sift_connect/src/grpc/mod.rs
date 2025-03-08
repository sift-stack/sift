use sift_error::prelude::*;
use std::time::Duration;
use tonic::{
    service::interceptor::InterceptedService,
    transport::channel::{Channel, Endpoint},
};
use tower::ServiceBuilder;

mod config;
pub use config::{Credentials, SiftChannelConfig};

mod interceptor;
use interceptor::AuthInterceptor;

/// A pre-configured gRPC channel to conveniently establish a connection to Sift's gRPC API.
pub type SiftChannel = InterceptedService<Channel, AuthInterceptor>;

pub fn connect_grpc(credentials: Credentials) -> Result<SiftChannel> {
    let config = SiftChannelConfig::try_from(credentials)?;
    use_sift_channel(config)
}

/// Uses `channel_config` to initialize a lazy channel that will only establish a connection
/// after first-use. This can only be used within a [`Tokio 1.x runtime`]. The returned [`SiftChannel`] is
/// a gRPC Channel that is pre-configured with an authorization interceptor as well as HTTP2
/// keep-alive. If these settings don't suit your needs, prefer to use a custom [`tonic endpoint`].
///
/// [`Tokio 1.x runtime`]: https://docs.rs/tokio/latest/tokio/
/// [`tonic endpoint`]: https://docs.rs/tonic/latest/tonic/transport/channel/struct.Endpoint.html
pub fn use_sift_channel(channel_config: SiftChannelConfig) -> Result<SiftChannel> {
    let SiftChannelConfig { uri, apikey } = channel_config;

    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");
    let user_agent = format!("{crate_name}/{crate_version}");

    let channel = Endpoint::from_shared(uri)
        .map(|conn| {
            conn.keep_alive_while_idle(true)
                .keep_alive_timeout(Duration::from_secs(20))
                .http2_keep_alive_interval(Duration::from_secs(20))
                .user_agent(user_agent)
                .expect("failed to construct user agent") // this shouldn't fail
        })
        .map_err(|e| Error::new(ErrorKind::GrpcConnectError, e))
        .context("something went while trying to establish a connection to Sift")
        .help("double check that the URL and the API token are both valid")?
        .connect_lazy();

    let intercepted_channel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(AuthInterceptor { apikey }))
        .service(channel);

    Ok(intercepted_channel)
}
