use anyhow::{Context, Result, format_err};
use crossterm::style::Stylize;
use std::{io::Write, path::Path};
use toml::{Table, Value, map::Map};

/// Create/update a profile in the config file and creates the file if needed. The `config` should be `None`
/// for the release version - it mainly exists for testing purposes.
pub fn run<W, P>(
    mut out: W,
    config: Option<P>,
    name: Option<String>,
    uri: String,
    apikey: String,
    force: bool,
) -> Result<()>
where
    W: Write,
    P: AsRef<Path>,
{
    let config = config.map(|p| p.as_ref().to_path_buf());
    let (content, config_display) =
        super::maybe_create_then_read_config_to_string(config.as_ref())?;
    let mut toml_config = match content {
        Some(text) => text
            .parse::<Table>()
            .with_context(|| format!("failed to parse {}", config_display))?,
        None => Map::new(),
    };

    let profile = match name {
        Some(name) => {
            let profile = toml_config.get(&name);

            if !force && profile.is_some() {
                return Err(format_err!(
                    "{} profile already exists - provide {} to overwrite",
                    name.bold().green(),
                    "-f, --force".to_string().bold().cyan()
                ));
            }
            Some(name)
        }
        None => {
            let default_uri = toml_config.get("uri");
            let default_api_key = toml_config.get("apikey");

            if !force && (default_uri.is_some() || default_api_key.is_some()) {
                return Err(format_err!(
                    "a default profile already exists - provide {} to overwrite",
                    "-f, --force".to_string().bold().cyan()
                ));
            }
            None
        }
    };

    match profile {
        Some(ref profile) => match toml_config.get_mut(profile) {
            Some(Value::Table(profile_config)) => {
                if let Some(uri_toml) = profile_config.get_mut("uri") {
                    *uri_toml = Value::String(uri);
                } else {
                    profile_config.insert("uri".into(), Value::String(uri));
                }

                if let Some(apikey_toml) = profile_config.get_mut("apikey") {
                    *apikey_toml = Value::String(apikey);
                } else {
                    profile_config.insert("apikey".into(), Value::String(apikey));
                }
            }
            _ => {
                let mut profile_config = Map::new();
                profile_config.insert("uri".into(), Value::String(uri));
                profile_config.insert("apikey".into(), Value::String(apikey));
                toml_config.insert(profile.clone(), Value::Table(profile_config));
            }
        },
        None => {
            if let Some(uri_toml) = toml_config.get_mut("uri") {
                *uri_toml = Value::String(uri);
            } else {
                toml_config.insert("uri".into(), Value::String(uri));
            }

            if let Some(apikey_toml) = toml_config.get_mut("apikey") {
                *apikey_toml = Value::String(apikey);
            } else {
                toml_config.insert("apikey".into(), Value::String(apikey));
            }
        }
    }

    let updated_config = toml_config.to_string();
    super::write_to_config(config, updated_config.as_bytes()).context("failed to update config")?;

    writeln!(
        out,
        "Successfully updated {} profile in {}",
        profile
            .unwrap_or_else(|| "default".to_string())
            .bold()
            .green(),
        config_display,
    )
    .context("failed to write output for config::create")
}
