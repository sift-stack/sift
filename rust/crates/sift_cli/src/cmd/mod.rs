use crate::BIN_NAME;
use crate::util::app_uri::infer_app_uri;
use anyhow::{Context as AnyhowContext, Result, anyhow};
use crossterm::style::Stylize;
use std::{fs::read_to_string, io::ErrorKind};
use toml::{Table, Value};

#[cfg(feature = "mcp")]
pub mod agent;
pub mod config;
pub mod doc;
pub mod export;
pub mod import;
pub mod install;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod ping;
pub mod version;

pub struct Context {
    pub grpc_uri: String,
    pub api_key: String,
    pub disable_tls: bool,
    #[allow(dead_code)]
    pub rest_uri: String,
    pub app_uri: String,
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
                        format!("{BIN_NAME} config create").green()
                    ));
                }
                _ => return Err(anyhow!("failed to read config file")),
            },
        };

        let config_toml = config_txt
            .parse::<Table>()
            .context("failed to parse config file")?;

        let target_profile = match profile.as_deref() {
            Some(prof) => {
                let Some(Value::Table(target)) = config_toml.get(prof) else {
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

        let app_uri = required_app_uri(target_profile, profile.as_deref(), &rest_uri)?;

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
            app_uri,
        })
    }
}

fn required_app_uri(
    target_profile: &Table,
    profile: Option<&str>,
    rest_uri: &str,
) -> Result<String> {
    let guidance = app_uri_guidance(profile, rest_uri);
    let Some(Value::String(app_uri)) = target_profile.get("app_uri").cloned() else {
        return Err(anyhow!(
            "Expected value of '{}' to be a string",
            "app_uri".yellow()
        ))
        .context(guidance);
    };
    if app_uri.trim().is_empty() {
        return Err(anyhow!(
            "Expected value of '{}' to be present",
            "app_uri".yellow()
        ))
        .context(guidance);
    }
    Ok(app_uri)
}

fn app_uri_guidance(profile: Option<&str>, rest_uri: &str) -> String {
    let profile_flag = profile.map_or_else(String::new, |profile| format!("--profile {profile} "));
    match infer_app_uri(rest_uri) {
        Some(app_uri) => {
            format!("Set it with '{BIN_NAME} {profile_flag}config update --app-uri {app_uri}'.")
        }
        None => format!(
            "Open your Sift web app and copy its URL origin. Then run '{BIN_NAME} {profile_flag}config update --app-uri <SIFT_WEB_ORIGIN>'."
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::required_app_uri;
    use toml::Table;

    #[test]
    fn app_uri_is_required() {
        for input in [
            r#"rest_uri = "https://api.siftstack.com""#,
            r#"app_uri = """#,
            r#"app_uri = "   ""#,
            "app_uri = 42",
        ] {
            let profile = input.parse::<Table>().unwrap();
            assert!(
                required_app_uri(&profile, None, "https://api.siftstack.com")
                    .unwrap_err()
                    .to_string()
                    .contains("Set it with"),
                "input: {input}"
            );
        }

        let configured = r#"app_uri = "https://app.siftstack.com""#.parse::<Table>().unwrap();
        assert_eq!(
            required_app_uri(&configured, None, "https://api.siftstack.com").unwrap(),
            "https://app.siftstack.com"
        );
    }

    #[test]
    fn unknown_app_uri_guidance_uses_the_profile() {
        let missing = Table::new();
        let error =
            required_app_uri(&missing, Some("mission"), "https://api.example.net").unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("Open your Sift web app"));
        assert!(message.contains("sift-cli --profile mission config update"));
    }
}
