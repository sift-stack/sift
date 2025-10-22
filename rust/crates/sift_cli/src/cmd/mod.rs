use std::{fs::read_to_string, io::ErrorKind};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use crossterm::style::Stylize;
use toml::{Table, Value};

pub mod completions;
pub mod config;
pub mod export;
pub mod import;

pub struct Context {
    pub grpc_uri: String,
    pub api_key: String,
    pub disable_tls: bool,

    #[allow(dead_code)]
    pub rest_uri: String,
}

impl Context {
    pub fn new(profile: Option<String>, disable_tls: bool) -> Result<Self> {
        let config_path = config::get_config_file_path()?;
        let p = config_path.display().to_string();

        let config_txt = match read_to_string(config_path) {
            Ok(txt) => txt,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    return Err(anyhow!("expected to find '{}'.", p.yellow())).context(format!(
                        "Create a config using '{}'.",
                        "sift_cli config create".green()
                    ));
                }
                _ => return Err(anyhow!("failed to read config file")),
            },
        };

        let config_toml = config_txt
            .parse::<Table>()
            .context("failed to parse config file")?;

        let target_profile = match profile {
            Some(prof) => {
                let Some(Value::Table(target)) = config_toml.get(&prof) else {
                    return Err(anyhow!(
                        "Profile '{}' not found or not a TOML table.",
                        prof.yellow()
                    ));
                };
                target
            }
            None => &config_toml,
        };

        let Some(Value::String(grpc_uri)) = target_profile.get("grpc_uri").cloned() else {
            return Err(anyhow!(
                "Expected value of '{}' to be a string",
                "grpc_uri".yellow()
            ));
        };
        if grpc_uri.is_empty() {
            return Err(anyhow!(
                "Expected value of '{}' to be present",
                "grpc_uri".yellow()
            ));
        }

        let Some(Value::String(rest_uri)) = target_profile.get("rest_uri").cloned() else {
            return Err(anyhow!(
                "Expected value of '{}' to be a string",
                "rest_uri".yellow()
            ));
        };
        if rest_uri.is_empty() {
            return Err(anyhow!(
                "Expected value of '{}' to be present",
                "rest_uri".yellow()
            ));
        }

        let Some(Value::String(api_key)) = target_profile.get("apikey").cloned() else {
            return Err(anyhow!(
                "Expected value of '{}' to be a string",
                "apikey".yellow()
            ));
        };
        if api_key.is_empty() {
            return Err(anyhow!(
                "Expected value of '{}' to be present",
                "apikey".yellow()
            ));
        }

        Ok(Self {
            grpc_uri,
            rest_uri,
            api_key,
            disable_tls,
        })
    }
}
