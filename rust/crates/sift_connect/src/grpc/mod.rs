use sift_error::prelude::*;
use std::time::Duration;
use tonic::{
    service::interceptor::InterceptedService,
    transport::{
        channel::{Channel, Endpoint},
        ClientTlsConfig,
    },
};
use tower::ServiceBuilder;

mod config;
pub use config::{Credentials, SiftChannelConfig};

pub mod interceptor;
pub use interceptor::AuthInterceptor;

/// A pre-configured gRPC channel to conveniently establish a connection to Sift's gRPC API.
pub type SiftChannel = InterceptedService<Channel, AuthInterceptor>;

pub struct SiftChannelBuilder {
    credentials: Credentials,
    use_tls: bool,
    keep_alive_while_idle: bool,
    keep_alive_timeout: Duration,
    keep_alive_interval: Duration,
    user_agent: String,
}

impl SiftChannelBuilder {
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

    pub fn build(self) -> Result<SiftChannel> {
        let SiftChannelConfig { uri, apikey } = SiftChannelConfig::try_from(self.credentials)?;

        let channel = Endpoint::from_shared(uri)
            .map(|conn| {
                let mut chan = conn
                    .keep_alive_while_idle(true)
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

    pub fn user_agent<S: AsRef<str>>(mut self, user_agent: S) -> Self {
        self.user_agent = user_agent.as_ref().to_string();
        self
    }

    pub fn use_tls(mut self, use_tls: bool) -> Self {
        self.use_tls = use_tls;
        self
    }

    pub fn keep_alive_while_idle(mut self, keep_alive_while_idle: bool) -> Self {
        self.keep_alive_while_idle = keep_alive_while_idle;
        self
    }

    pub fn keep_alive_timeout(mut self, keep_alive_timeout: Duration) -> Self {
        self.keep_alive_timeout = keep_alive_timeout;
        self
    }

    pub fn keep_alive_interval(mut self, keep_alive_interval: Duration) -> Self {
        self.keep_alive_interval = keep_alive_interval;
        self
    }
}
