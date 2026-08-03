#[cfg(test)]
mod tests;

use crate::BIN_NAME;
use anyhow::{Context, Result, anyhow};
use crossterm::style::Stylize;
use std::{
    fs::{File, OpenOptions, create_dir_all, metadata, read_to_string},
    io::Write,
    path::PathBuf,
    process::ExitCode,
};
use toml::{Table, Value};

use crate::{
    cli::ConfigUpdateArgs,
    util::{
        app_uri::{infer_app_uri, normalize_app_uri},
        tty::{Output, PromptUser},
    },
};

pub const CONFIG_FILE_NAME: &str = "sift.toml";

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum AppUriState {
    Configured(String),
    MissingKnown(String),
    MissingUnknown(Option<String>),
    Invalid,
}

pub fn show() -> Result<ExitCode> {
    let p = get_config_file_path()?;
    let contents = read_to_string(p).context("failed to read config file")?;
    Output::new().line(contents).print();
    Ok(ExitCode::SUCCESS)
}

pub fn create() -> Result<ExitCode> {
    let path = get_config_file_path()?;
    let p = path.display().to_string();

    if metadata(&path).is_ok() {
        Output::new()
            .line(format!("A config file already exists at '{}'.", p.yellow()))
            .tip(format!(
                "Use '{}' to view the contents.",
                format!("{BIN_NAME} config show").green()
            ))
            .print();
        return Ok(ExitCode::SUCCESS);
    }

    create_config_file()?;

    Output::new()
        .line(format!(
            "An empty config file has been created at '{}'.",
            p.yellow()
        ))
        .tip(format!(
            "Use '{}' to configure it.",
            format!("{BIN_NAME} config update").green()
        ))
        .print();

    Ok(ExitCode::SUCCESS)
}

pub fn update(profile: Option<String>, args: ConfigUpdateArgs) -> Result<ExitCode> {
    let mut configured_profile = profile.clone();

    let updated_config = {
        if !args.interactive {
            if is_update_empty(&args) {
                Output::new().line("Nothing to update.").print();
                return Ok(ExitCode::SUCCESS);
            }
            get_updated_config(
                profile.clone(),
                args.grpc_uri,
                args.rest_uri,
                args.api_key,
                args.app_uri,
            )?
        } else {
            let [prof, grpc, rest, key]: [Option<String>; 4] = PromptUser::new()
                .header("Any blank values will be ignored preserving the original.")
                .prompt("  Specify the profile to configure (leave blank for default profile): ")
                .prompt("  Specify the gRPC API base URL: ")
                .prompt("  Specify the REST API base URL: ")
                .prompt("  Provide your Sift API key: ")
                .run()?
                .try_into()
                .unwrap();

            configured_profile = prof.clone();
            let suggested_app_uri = rest.as_deref().and_then(infer_app_uri);
            let app_prompt = suggested_app_uri.map_or_else(
                || {
                    "  Open your Sift web app and copy its URL origin. Keep the scheme and host. \
                     Specify that origin here (for example, https://sift.example.net): "
                        .to_string()
                },
                |uri| format!("  Specify the Sift web app URL [{uri}]: "),
            );
            let [app]: [Option<String>; 1] = PromptUser::new()
                .prompt(app_prompt)
                .run()?
                .try_into()
                .unwrap();
            let app = app.or_else(|| suggested_app_uri.map(str::to_string));

            let updated = get_updated_config(prof, grpc, rest, key, app)?;
            let divider = "-".repeat(40);

            let [confirmation]: [Option<String>; 1] = PromptUser::new()
                .prompt(format!(
                    "\n{divider}\n{updated}\n{divider}\nDoes this look correct? [y/n]: "
                ))
                .run()?
                .try_into()
                .unwrap();

            if confirmation.is_none_or(|c| c != "y") {
                Output::new().line("Operation aborted.").print();
                return Ok(ExitCode::SUCCESS);
            }
            updated
        }
    };

    let config_toml = updated_config
        .parse::<Table>()
        .context("updated config is invalid TOML")?;
    let app_uri_state = app_uri_state(&config_toml, configured_profile.as_deref())?;
    update_config_file(updated_config)?;

    let target = configured_profile
        .as_deref()
        .unwrap_or("default")
        .to_string();
    Output::new()
        .line(format!(
            "Successfully configured the '{}' profile.",
            target.yellow()
        ))
        .print();
    print_app_uri_guidance(configured_profile.as_deref(), &app_uri_state);

    Ok(ExitCode::SUCCESS)
}

pub fn config_where() -> Result<ExitCode> {
    let expected_path = get_config_file_path()?;
    let p = expected_path.display().to_string();

    if metadata(&expected_path).is_err() {
        Output::new()
            .line(format!("'{}' not found.", p.yellow()))
            .tip(format!(
                "try running '{}' first.",
                format!("{BIN_NAME} config create").green()
            ))
            .eprint();
        return Ok(ExitCode::FAILURE);
    }
    Output::new().line(p.to_string()).print();
    Ok(ExitCode::SUCCESS)
}

pub(super) fn get_config_file_path() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join(CONFIG_FILE_NAME))
        .ok_or(anyhow!("user config directory not found"))
}

fn create_config_file() -> Result<(File, PathBuf)> {
    let path = get_config_file_path()?;

    // Create the parent directories if they don't exist.
    if let Some(parent_dir) = path.parent() {
        create_dir_all(parent_dir).context("failed to create parent directories")?;
    }

    let config_file = File::create_new(&path).context("failed to create config file")?;

    Ok((config_file, path))
}

fn get_updated_config(
    profile: Option<String>,
    grpc_uri: Option<String>,
    rest_uri: Option<String>,
    api_key: Option<String>,
    app_uri: Option<String>,
) -> Result<String> {
    let path = get_config_file_path()?;

    let contents = read_to_string(path).context("failed to read config file")?;

    let mut config_toml = contents
        .parse::<Table>()
        .context("config file is invalid TOML")?;

    apply_profile_updates(
        &mut config_toml,
        profile,
        grpc_uri,
        rest_uri,
        api_key,
        app_uri,
    )?;

    Ok(config_toml.to_string())
}

fn apply_profile_updates(
    config_toml: &mut Table,
    profile: Option<String>,
    grpc_uri: Option<String>,
    rest_uri: Option<String>,
    api_key: Option<String>,
    app_uri: Option<String>,
) -> Result<()> {
    let target = match profile {
        Some(prof) => match config_toml.get_mut(&prof) {
            Some(Value::Table(profile_config)) => profile_config,
            _ => {
                config_toml.insert(prof.clone(), Value::Table(Table::new()));
                config_toml[&prof].as_table_mut().unwrap()
            }
        },
        None => config_toml,
    };

    let infer_missing_app_uri =
        rest_uri.is_some() && app_uri.as_deref().and_then(normalize_app_uri).is_none();

    if let Some(uri) = grpc_uri {
        target.insert(String::from("grpc_uri"), Value::String(uri));
    }
    if let Some(uri) = rest_uri {
        target.insert(String::from("rest_uri"), Value::String(uri));
    }
    if let Some(token) = api_key {
        target.insert(String::from("apikey"), Value::String(token));
    }
    if let Some(uri) = app_uri.as_deref().and_then(normalize_app_uri) {
        target.insert(String::from("app_uri"), Value::String(uri.to_string()));
    }
    let app_uri_is_missing = target
        .get("app_uri")
        .and_then(Value::as_str)
        .and_then(normalize_app_uri)
        .is_none();
    if infer_missing_app_uri
        && app_uri_is_missing
        && let Some(rest_uri) = target.get("rest_uri").and_then(Value::as_str)
        && let Some(app_uri) = infer_app_uri(rest_uri)
    {
        target.insert(String::from("app_uri"), Value::String(app_uri.to_string()));
    }

    Ok(())
}

#[cfg(feature = "mcp")]
pub(super) fn inspect_app_uri(profile: Option<&str>) -> Result<AppUriState> {
    let path = get_config_file_path()?;
    let contents = read_to_string(path).context("failed to read config file")?;
    let config_toml = contents
        .parse::<Table>()
        .context("config file is invalid TOML")?;
    app_uri_state(&config_toml, profile)
}

fn app_uri_state(config_toml: &Table, profile: Option<&str>) -> Result<AppUriState> {
    let target = profile_table(config_toml, profile)?;
    match target.get("app_uri") {
        Some(Value::String(uri)) if let Some(uri) = normalize_app_uri(uri) => {
            return Ok(AppUriState::Configured(uri.to_string()));
        }
        Some(Value::String(_)) | None => {}
        Some(_) => return Ok(AppUriState::Invalid),
    }

    let rest_uri = target
        .get("rest_uri")
        .and_then(Value::as_str)
        .map(str::to_string);
    Ok(match rest_uri.as_deref().and_then(infer_app_uri) {
        Some(app_uri) => AppUriState::MissingKnown(app_uri.to_string()),
        None => AppUriState::MissingUnknown(rest_uri),
    })
}

fn profile_table<'a>(config_toml: &'a Table, profile: Option<&str>) -> Result<&'a Table> {
    match profile {
        Some(profile) => config_toml
            .get(profile)
            .and_then(Value::as_table)
            .ok_or_else(|| anyhow!("Profile '{profile}' not found or not a TOML table.")),
        None => Ok(config_toml),
    }
}

fn print_app_uri_guidance(profile: Option<&str>, state: &AppUriState) {
    let profile_flag = profile.map_or_else(String::new, |profile| format!("--profile {profile} "));
    match state {
        AppUriState::Configured(_) => {}
        AppUriState::MissingKnown(app_uri) => println!(
            "{} This profile has no app_uri. Set it with `sift-cli {profile_flag}config \
             update --app-uri {app_uri}`.",
            "[warning]".yellow()
        ),
        AppUriState::MissingUnknown(_) => println!(
            "{} This profile has no app_uri. Open your Sift web app and copy its URL \
             origin. Then run `sift-cli {profile_flag}config update --app-uri \
             <SIFT_WEB_ORIGIN>`.",
            "[warning]".yellow()
        ),
        AppUriState::Invalid => {
            println!(
                "{} This profile has an app_uri value that is not a string.",
                "[warning]".yellow()
            )
        }
    }
}

fn update_config_file(updated: String) -> Result<()> {
    let path = get_config_file_path()?;

    let mut config = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .context("failed to open config file")?;

    write!(config, "{updated}").context("failed to update config file")
}

fn is_update_empty(args: &ConfigUpdateArgs) -> bool {
    let ConfigUpdateArgs {
        grpc_uri,
        rest_uri,
        api_key,
        app_uri,
        ..
    } = args;
    grpc_uri.as_ref().is_none_or(|s| s.is_empty())
        && rest_uri.as_ref().is_none_or(|s| s.is_empty())
        && api_key.as_ref().is_none_or(|s| s.is_empty())
        && app_uri.as_ref().is_none_or(|s| s.is_empty())
}
