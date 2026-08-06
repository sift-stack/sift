use crate::BIN_NAME;
use crate::util::app_uri::{infer_app_uri, normalize_app_uri};
use anyhow::{Context as AnyhowContext, Result, anyhow};
use crossterm::style::Stylize;
use std::{fs::read_to_string, io::ErrorKind, path::Path};
use toml::{Table, Value};

pub mod agent;
pub mod config;
pub mod doc;
pub mod export;
pub mod import;
pub mod install;
pub mod mcp;
pub mod ping;
pub mod version;

pub struct Context {
    pub grpc_uri: String,
    pub api_key: String,
    pub disable_tls: bool,
    #[allow(dead_code)]
    pub rest_uri: String,
    pub app_uri: Option<String>,
}

impl Context {
    pub fn new(profile: Option<String>, disable_tls: bool) -> Result<Self> {
        let config_path = config::get_config_file_path()?;
        Self::from_config_path(&config_path, profile, disable_tls)
    }

    fn from_config_path(
        config_path: &Path,
        profile: Option<String>,
        disable_tls: bool,
    ) -> Result<Self> {
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

        let app_uri = target_profile
            .get("app_uri")
            .and_then(Value::as_str)
            .and_then(normalize_app_uri)
            .map(str::to_string);

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

    pub fn require_app_uri(&self, profile: Option<&str>) -> Result<&str> {
        self.app_uri
            .as_deref()
            .ok_or_else(|| anyhow!(app_uri_guidance(profile, &self.rest_uri)))
    }
}

fn app_uri_guidance(profile: Option<&str>, rest_uri: &str) -> String {
    let profile_name = profile.unwrap_or("default");
    let profile_flag = profile.map_or_else(String::new, |profile| format!("--profile {profile} "));
    match infer_app_uri(rest_uri) {
        Some(app_uri) => format!(
            "The Sift MCP server cannot start because profile '{profile_name}' has no usable \
             'app_uri'. Run '{BIN_NAME} {profile_flag}config update --app-uri {app_uri}', then \
             restart the MCP client."
        ),
        None => format!(
            "The Sift MCP server cannot start because profile '{profile_name}' has no usable \
             'app_uri'. Open your Sift web app and copy its URL origin. Run '{BIN_NAME} \
             {profile_flag}config update --app-uri <SIFT_WEB_ORIGIN>', then restart the MCP \
             client."
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use std::fs;
    use tempdir::TempDir;

    const COMPLETE_CONFIG: &str = r#"
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"
apikey = "default-key"

[mission]
grpc_uri = "https://grpc.example.net"
rest_uri = "https://api.example.net"
app_uri = "https://sift.example.net"
apikey = "mission-key"
"#;

    fn context(config: &str, profile: Option<&str>) -> anyhow::Result<Context> {
        let directory = TempDir::new("sift-cli-context").unwrap();
        let path = directory.path().join("sift.toml");
        fs::write(&path, config).unwrap();
        Context::from_config_path(&path, profile.map(str::to_string), false)
    }

    #[test]
    fn loads_complete_default_and_named_profiles() {
        let default = context(COMPLETE_CONFIG, None).unwrap();
        assert_eq!(default.grpc_uri, "https://grpc-api.siftstack.com");
        assert_eq!(default.rest_uri, "https://api.siftstack.com");
        assert_eq!(
            default.app_uri.as_deref(),
            Some("https://app.siftstack.com")
        );
        assert_eq!(default.api_key, "default-key");

        let mission = context(COMPLETE_CONFIG, Some("mission")).unwrap();
        assert_eq!(mission.grpc_uri, "https://grpc.example.net");
        assert_eq!(mission.rest_uri, "https://api.example.net");
        assert_eq!(mission.app_uri.as_deref(), Some("https://sift.example.net"));
        assert_eq!(mission.api_key, "mission-key");
    }

    #[test]
    fn rejects_each_missing_required_connection_field() {
        for (field, config) in [
            (
                "grpc_uri",
                r#"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"
apikey = "key"
"#,
            ),
            (
                "rest_uri",
                r#"
grpc_uri = "https://grpc-api.siftstack.com"
app_uri = "https://app.siftstack.com"
apikey = "key"
"#,
            ),
            (
                "apikey",
                r#"
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"
"#,
            ),
        ] {
            let message = format!("{:#}", context(config, None).err().unwrap());
            assert!(message.contains(field), "field: {field}, error: {message}");
        }
    }

    #[test]
    fn incomplete_app_uri_remains_loadable_for_recovery() {
        for app_uri in [
            "",
            "app_uri = \"\"",
            "app_uri = \"   \"",
            "app_uri = \" / \"",
            "app_uri = 42",
        ] {
            let config = format!(
                r#"
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
apikey = "key"
{app_uri}
"#
            );
            assert_eq!(context(&config, None).unwrap().app_uri, None);
        }
    }

    #[test]
    fn app_uri_is_trimmed_for_consumers() {
        let config = r#"
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri = "  https://app.siftstack.com///  "
apikey = "key"
"#;
        assert_eq!(
            context(config, None).unwrap().app_uri.as_deref(),
            Some("https://app.siftstack.com")
        );
    }

    #[test]
    fn mcp_requires_app_uri_with_profile_guidance() {
        let known = r#"
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
apikey = "key"
"#;
        let known = context(known, None).unwrap();
        let known_message = format!("{:#}", known.require_app_uri(None).unwrap_err());
        assert!(
            known_message.contains("sift-cli config update --app-uri https://app.siftstack.com")
        );

        let custom = r#"
[mission]
grpc_uri = "https://grpc.example.net"
rest_uri = "https://api.example.net"
apikey = "key"
"#;
        let custom = context(custom, Some("mission")).unwrap();
        let custom_message = format!("{:#}", custom.require_app_uri(Some("mission")).unwrap_err());
        assert!(custom_message.contains("Open your Sift web app"));
        assert!(custom_message.contains("sift-cli --profile mission config update --app-uri"));
    }
}
