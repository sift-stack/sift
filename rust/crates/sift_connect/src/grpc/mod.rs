use sift_error::prelude::*;
use std::time::Duration;
use tonic::{
    service::interceptor::InterceptedService,
    transport::{
        ClientTlsConfig,
        channel::{Channel, Endpoint},
    },
};
use tower::ServiceBuilder;

/// Concerned with Sift credentials.
pub mod config;
pub use config::Credentials;

/// Interceptors for [SiftChannel].
pub mod interceptor;
pub use interceptor::AuthInterceptor;

/// A pre-configured gRPC channel to conveniently establish a connection to Sift's gRPC API.
pub type SiftChannel = InterceptedService<Channel, AuthInterceptor>;

/// Used to build an instance of [SiftChannel].
pub struct SiftChannelBuilder {
    credentials: Credentials,
    use_tls: bool,
    keep_alive_while_idle: bool,
    keep_alive_timeout: Duration,
    keep_alive_interval: Duration,
    user_agent: String,
}

impl SiftChannelBuilder {
    /// Initializes a new [SiftChannelBuilder] with sane defaults.
    pub fn new(credentials: Credentials) -> Self {
        let crate_name = env!("CARGO_PKG_NAME");
        let crate_version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("{crate_name}/{crate_version}");

        Self {
            credentials,
            user_agent,
            use_tls: true,
            keep_alive_while_idle: true,
            keep_alive_timeout: Duration::from_secs(20),
            keep_alive_interval: Duration::from_secs(20),
        }
    }

    /// Consume [SiftChannelBuilder] and return a [SiftChannel]. The [SiftChannel] is lazy and
    /// won't actually connect to Sift until the first RPC is made.
    pub fn build(self) -> Result<SiftChannel> {
        let config::SiftChannelConfig { uri, apikey } =
            config::SiftChannelConfig::try_from(self.credentials)?;

        let channel = Endpoint::from_shared(uri)
            .map(|conn| {
                let mut chan = conn
                    .keep_alive_while_idle(self.keep_alive_while_idle)
                    .keep_alive_timeout(self.keep_alive_timeout)
                    .http2_keep_alive_interval(self.keep_alive_interval)
                    .user_agent(self.user_agent)
                    .expect("failed to construct user agent");

                if self.use_tls {
                    chan = chan
                        .tls_config(ClientTlsConfig::new().with_native_roots())
                        .expect("failed to configure TLS");
                }
                chan
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

    /// Override the default user-agent which is the name of the crate. Do note that the
    /// application firewall is sensitive to certain user-agents so if you experience any issues
    /// connecting to Sift, please notify the team to ascertain if it's related to a bad
    /// user-agent.
    pub fn user_agent<S: AsRef<str>>(mut self, user_agent: S) -> Self {
        self.user_agent = user_agent.as_ref().to_string();
        self
    }

    /// Enables/disables TLS. In production, TLS should only ever be enabled. For mocking/testing
    /// purposes, TLS may be disabled.
    pub fn use_tls(mut self, use_tls: bool) -> Self {
        self.use_tls = use_tls;
        self
    }

    /// See [`hyper documentation`].
    ///
    /// [`hyper documentation`]: https://docs.rs/hyper/latest/hyper/client/conn/http2/struct.Builder.html#method.keep_alive_while_idle
    pub fn keep_alive_while_idle(mut self, keep_alive_while_idle: bool) -> Self {
        self.keep_alive_while_idle = keep_alive_while_idle;
        self
    }

    /// See [`hyper documentation`].
    ///
    /// [`hyper documentation`]: https://docs.rs/hyper/latest/hyper/client/conn/http2/struct.Builder.html#method.keep_alive_timeout
    pub fn keep_alive_timeout(mut self, keep_alive_timeout: Duration) -> Self {
        self.keep_alive_timeout = keep_alive_timeout;
        self
    }

    /// See [`hyper documentation`].
    ///
    /// [`hyper documentation`]: https://docs.rs/hyper/latest/hyper/client/conn/http2/struct.Builder.html#method.keep_alive_interval
    pub fn keep_alive_interval(mut self, keep_alive_interval: Duration) -> Self {
        self.keep_alive_interval = keep_alive_interval;
        self
    }
}
