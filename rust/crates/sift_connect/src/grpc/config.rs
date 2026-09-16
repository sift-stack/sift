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
/// api_key = "default-api-key"
///
/// [production]
/// uri = "https://api.siftstack.com"
/// api_key = "production-api-key"
/// ```
///
/// The older `apikey` spelling is still valid, so a file from an earlier release
/// still works. If a table holds both keys, the loader uses `api_key`.
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

                let (table, location) = match &profile {
                    Some(p) => {
                        let Some(Value::Table(sub_table)) = config_toml.get(p) else {
                            return Err(Error::new_msg(
                                ErrorKind::ConfigError,
                                format!("expected a '{p}' sub-table in '{}'", config.display()),
                            ));
                        };
                        (sub_table, format!("'{p}'"))
                    }
                    None => (
                        &config_toml,
                        format!("a top-level entry in '{}'", config.display()),
                    ),
                };

                let Some(uri) = lookup(table, &[URI_KEY]) else {
                    return Err(Error::new_msg(
                        ErrorKind::ConfigError,
                        format!("expected {location} to contain '{URI_KEY}'"),
                    ));
                };

                let Some(apikey) = lookup(table, API_KEY_KEYS) else {
                    return Err(Error::new_msg(
                        ErrorKind::ConfigError,
                        format!("expected {location} to contain '{}'", API_KEY_KEYS[0]),
                    ));
                };

                Ok(SiftChannelConfig::new(uri, apikey))
            }
        }
    }
}

/// The accepted TOML keys for the API key, canonical first. `api_key` matches
/// the spelling that the Sift API and `sift-cli` use. `apikey` is the older
/// spelling that earlier releases wrote. A lookup accepts either key. Nothing
/// writes the older one.
pub const API_KEY_KEYS: &[&str] = &["api_key", "apikey"];

/// The TOML key naming the gRPC endpoint.
pub const URI_KEY: &str = "uri";

/// The value of the first key in `keys` that `table` sets, or `None`.
fn lookup<'a>(table: &'a Table, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| match table.get(*key) {
        Some(Value::String(value)) if !value.is_empty() => Some(value.as_str()),
        _ => None,
    })
}
