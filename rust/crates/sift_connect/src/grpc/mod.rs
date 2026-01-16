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
///
/// This is a type alias for a gRPC channel that has been configured with authentication
/// via [`AuthInterceptor`]. The channel is lazy and won't actually connect until the
/// first RPC call is made.
///
/// # Example
///
/// ```no_run
/// use sift_connect::{Credentials, SiftChannelBuilder};
/// use std::env;
///
/// let credentials = Credentials::Config {
///     uri: env::var("SIFT_URI").unwrap(),
///     apikey: env::var("SIFT_API_KEY").unwrap(),
/// };
///
/// let channel: sift_connect::SiftChannel = SiftChannelBuilder::new(credentials)
///     .build()
///     .unwrap();
/// ```
pub type SiftChannel = InterceptedService<Channel, AuthInterceptor>;

/// Used to build an instance of [SiftChannel].
///
/// This builder provides a fluent API for configuring a gRPC channel connection
/// to Sift's API. It supports custom credentials, TLS configuration, and HTTP/2
/// keep-alive settings.
///
/// # Example
///
/// ```no_run
/// use sift_connect::{Credentials, SiftChannelBuilder};
/// use std::env;
/// use std::time::Duration;
///
/// let credentials = Credentials::Config {
///     uri: env::var("SIFT_URI").unwrap(),
///     apikey: env::var("SIFT_API_KEY").unwrap(),
/// };
///
/// let channel = SiftChannelBuilder::new(credentials)
///     .use_tls(true)
///     .keep_alive_timeout(Duration::from_secs(30))
///     .build()
///     .unwrap();
/// ```
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
    ///
    /// Default settings:
    /// - TLS enabled
    /// - Keep-alive while idle enabled
    /// - Keep-alive timeout: 20 seconds
    /// - Keep-alive interval: 20 seconds
    /// - User agent: crate name and version
    ///
    /// # Arguments
    ///
    /// * `credentials` - The credentials to use for authentication
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    ///
    /// let credentials = Credentials::Config {
    ///     uri: "https://api.siftstack.com".to_string(),
    ///     apikey: "your-api-key".to_string(),
    /// };
    ///
    /// let builder = SiftChannelBuilder::new(credentials);
    /// ```
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

    /// Consume [SiftChannelBuilder] and return a [SiftChannel].
    ///
    /// The [SiftChannel] is lazy and won't actually connect to Sift until the first
    /// RPC is made. This allows you to create the channel early without incurring
    /// connection overhead until it's needed.
    ///
    /// # Returns
    ///
    /// A configured [SiftChannel] ready for use with gRPC clients.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The URI is invalid
    /// - Credentials cannot be loaded (for profile-based credentials)
    /// - TLS configuration fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    ///
    /// let credentials = Credentials::Config {
    ///     uri: "https://api.siftstack.com".to_string(),
    ///     apikey: "your-api-key".to_string(),
    /// };
    ///
    /// let channel = SiftChannelBuilder::new(credentials)
    ///     .build()
    ///     .expect("failed to create channel");
    /// ```
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
                        .tls_config(ClientTlsConfig::new().with_enabled_roots())
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

    /// Override the default user-agent which is the name of the crate.
    ///
    /// The default user-agent is set to `{crate_name}/{crate_version}`. This method
    /// allows you to customize it.
    ///
    /// # Note
    ///
    /// The application firewall is sensitive to certain user-agents. If you experience
    /// any issues connecting to Sift, please notify the team to ascertain if it's related
    /// to a bad user-agent.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - The custom user-agent string to use
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    ///
    /// let credentials = Credentials::Config {
    ///     uri: "https://api.siftstack.com".to_string(),
    ///     apikey: "your-api-key".to_string(),
    /// };
    ///
    /// let builder = SiftChannelBuilder::new(credentials)
    ///     .user_agent("MyApp/1.0");
    /// ```
    pub fn user_agent<S: AsRef<str>>(mut self, user_agent: S) -> Self {
        self.user_agent = user_agent.as_ref().to_string();
        self
    }

    /// Enables or disables TLS.
    ///
    /// # Warning
    ///
    /// In production, TLS should only ever be enabled. For mocking/testing
    /// purposes, TLS may be disabled.
    ///
    /// # Arguments
    ///
    /// * `use_tls` - Whether to enable TLS encryption
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    ///
    /// let credentials = Credentials::Config {
    ///     uri: "https://api.siftstack.com".to_string(),
    ///     apikey: "your-api-key".to_string(),
    /// };
    ///
    /// // Production use - TLS enabled (default)
    /// let builder = SiftChannelBuilder::new(credentials.clone())
    ///     .use_tls(true);
    ///
    /// // Testing use - TLS disabled
    /// let test_builder = SiftChannelBuilder::new(credentials)
    ///     .use_tls(false);
    /// ```
    pub fn use_tls(mut self, use_tls: bool) -> Self {
        self.use_tls = use_tls;
        self
    }

    /// Configures whether to send keep-alive pings while the connection is idle.
    ///
    /// See [`hyper documentation`] for detailed information.
    ///
    /// [`hyper documentation`]: https://docs.rs/hyper/latest/hyper/client/conn/http2/struct.Builder.html#method.keep_alive_while_idle
    ///
    /// # Arguments
    ///
    /// * `keep_alive_while_idle` - Whether to send keep-alive pings while idle
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    ///
    /// # let credentials = Credentials::Config {
    /// #     uri: "https://api.siftstack.com".to_string(),
    /// #     apikey: "your-api-key".to_string(),
    /// # };
    /// let builder = SiftChannelBuilder::new(credentials)
    ///     .keep_alive_while_idle(true);
    /// ```
    pub fn keep_alive_while_idle(mut self, keep_alive_while_idle: bool) -> Self {
        self.keep_alive_while_idle = keep_alive_while_idle;
        self
    }

    /// Configures the timeout for keep-alive pings.
    ///
    /// See [`hyper documentation`] for detailed information.
    ///
    /// [`hyper documentation`]: https://docs.rs/hyper/latest/hyper/client/conn/http2/struct.Builder.html#method.keep_alive_timeout
    ///
    /// # Arguments
    ///
    /// * `keep_alive_timeout` - The timeout duration for keep-alive pings
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    /// use std::time::Duration;
    ///
    /// # let credentials = Credentials::Config {
    /// #     uri: "https://api.siftstack.com".to_string(),
    /// #     apikey: "your-api-key".to_string(),
    /// # };
    /// let builder = SiftChannelBuilder::new(credentials)
    ///     .keep_alive_timeout(Duration::from_secs(30));
    /// ```
    pub fn keep_alive_timeout(mut self, keep_alive_timeout: Duration) -> Self {
        self.keep_alive_timeout = keep_alive_timeout;
        self
    }

    /// Configures the interval between keep-alive pings.
    ///
    /// See [`hyper documentation`] for detailed information.
    ///
    /// [`hyper documentation`]: https://docs.rs/hyper/latest/hyper/client/conn/http2/struct.Builder.html#method.keep_alive_interval
    ///
    /// # Arguments
    ///
    /// * `keep_alive_interval` - The interval duration between keep-alive pings
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_connect::{Credentials, SiftChannelBuilder};
    /// use std::time::Duration;
    ///
    /// # let credentials = Credentials::Config {
    /// #     uri: "https://api.siftstack.com".to_string(),
    /// #     apikey: "your-api-key".to_string(),
    /// # };
    /// let builder = SiftChannelBuilder::new(credentials)
    ///     .keep_alive_interval(Duration::from_secs(30));
    /// ```
    pub fn keep_alive_interval(mut self, keep_alive_interval: Duration) -> Self {
        self.keep_alive_interval = keep_alive_interval;
        self
    }
}
