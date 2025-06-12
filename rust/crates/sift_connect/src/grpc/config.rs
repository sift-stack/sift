use sift_error::prelude::*;
use toml::{Table, Value};

/// The expected name of the config file.
pub const SIFT_CONFIG_NAME: &str = "sift.toml";

/// Specifies source of credentials. If `Profile` is used, then the provided string will be used to
/// query the corresponding table from [`SIFT_CONFIG_NAME`] located at
/// [these locations](https://docs.rs/dirs/6.0.0/dirs/fn.config_local_dir.html)
/// depending on your operating system. If `None` is provided, then the top-level table is used.
#[derive(Debug, Clone)]
pub enum Credentials {
    Profile(Option<String>),
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
