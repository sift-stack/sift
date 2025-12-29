use sift_error::prelude::*;
use toml::{Table, Value};

/// The expected name of the config file.
pub const SIFT_CONFIG_NAME: &str = "sift.toml";

/// Specifies the source of credentials for connecting to Sift.
///
/// Credentials can be provided either directly via `Config` or loaded from a
/// configuration file using `Profile`.
///
/// # Profile-based Credentials
///
/// If `Profile` is used, the provided string will be used to query the corresponding
/// table from [`SIFT_CONFIG_NAME`] located at [these locations](https://docs.rs/dirs/6.0.0/dirs/fn.config_local_dir.html)
/// depending on your operating system. If `None` is provided, then the top-level
/// table is used.
///
/// Example `sift.toml` file:
///
/// ```toml
/// uri = "https://api.siftstack.com"
/// apikey = "default-api-key"
///
/// [production]
/// uri = "https://api.siftstack.com"
/// apikey = "production-api-key"
/// ```
///
/// # Direct Credentials
///
/// The `Config` variant allows you to provide credentials directly without
/// requiring a configuration file.
///
/// # Example
///
/// ```no_run
/// use sift_connect::Credentials;
///
/// // Direct credentials
/// let creds = Credentials::Config {
///     uri: "https://api.siftstack.com".to_string(),
///     apikey: "your-api-key".to_string(),
/// };
///
/// // Profile-based credentials (default profile)
/// let default_profile = Credentials::Profile(None);
///
/// // Profile-based credentials (named profile)
/// let prod_profile = Credentials::Profile(Some("production".to_string()));
/// ```
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Load credentials from a named profile in the configuration file.
    ///
    /// If `None`, uses the default (top-level) profile.
    Profile(Option<String>),
    /// Provide credentials directly.
    ///
    /// Fields:
    /// - `uri`: The Sift API endpoint URI
    /// - `apikey`: The API key for authentication
    Config { uri: String, apikey: String },
}

#[derive(Default, Clone)]
pub(crate) struct SiftChannelConfig {
    pub uri: String,
    pub apikey: String,
}

impl SiftChannelConfig {
    pub fn new(uri: &str, apikey: &str) -> Self {
        Self {
            uri: uri.to_string(),
            apikey: apikey.to_string(),
        }
    }
}

impl TryFrom<Credentials> for SiftChannelConfig {
    type Error = Error;

    fn try_from(creds: Credentials) -> Result<Self> {
        match creds {
            Credentials::Config { uri, apikey } => Ok(Self::new(&uri, &apikey)),
            Credentials::Profile(profile) => {
                let config = dirs::config_local_dir()
                    .map(|dir| dir.join(SIFT_CONFIG_NAME))
                    .ok_or_else(|| {
                        Error::new_general("failed to find path to user config directory")
                    })?;

                let config_str = std::fs::read_to_string(&config)
                    .map_err(Error::from)
                    .with_context(|| format!("failed to load '{}'", config.display()))
                    .help("ensure that the config file is in the expected location")?;

                let config_toml = config_str
                    .parse::<Table>()
                    .map_err(|e| Error::new(ErrorKind::ConfigError, e))
                    .with_context(|| format!("failed to parse {}", config.display()))
                    .help("ensure that the config file is properly formated")?;

                match profile {
                    Some(p) => {
                        let Some(Value::Table(sub_table)) = config_toml.get(&p) else {
                            return Err(Error::new_msg(
                                ErrorKind::ConfigError,
                                format!("expected a '{p}' sub-table in '{}'", config.display()),
                            ));
                        };

                        let Some(Value::String(uri)) = sub_table.get("uri") else {
                            return Err(Error::new_msg(
                                ErrorKind::ConfigError,
                                format!("expected '{p}' to contain 'uri' entry"),
                            ));
                        };

                        let Some(Value::String(apikey)) = sub_table.get("apikey") else {
                            return Err(Error::new_msg(
                                ErrorKind::ConfigError,
                                format!("expected '{p}' to contain 'apikey' entry"),
                            ));
                        };

                        Ok(SiftChannelConfig::new(uri, apikey))
                    }
                    None => {
                        let Some(Value::String(uri)) = config_toml.get("uri") else {
                            return Err(Error::new_msg(
                                ErrorKind::ConfigError,
                                format!(
                                    "expected '{}' to contain a top-level 'uri' entry",
                                    config.display()
                                ),
                            ));
                        };

                        let Some(Value::String(apikey)) = config_toml.get("apikey") else {
                            return Err(Error::new_msg(
                                ErrorKind::ConfigError,
                                format!(
                                    "expected '{}' to contain a top-level 'apikey' entry",
                                    config.display()
                                ),
                            ));
                        };

                        Ok(SiftChannelConfig::new(uri, apikey))
                    }
                }
            }
        }
    }
}
