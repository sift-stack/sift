use anyhow::{Context, Result, anyhow};
use crossterm::style::Stylize;
use std::{
    fs::{File, OpenOptions, metadata, read_to_string},
    io::Write,
    path::PathBuf,
    process::ExitCode,
};
use toml::{Table, Value};

use crate::{
    cli::ConfigUpdateArgs,
    util::tty::{Output, PromptUser},
};

pub const CONFIG_FILE_NAME: &str = "sift.toml";

pub fn show() -> Result<ExitCode> {
    let p = get_config_file_path()?;
    let contents = read_to_string(p).context("failed to read config file")?;
    Output::new().line(contents).print();
    Ok(ExitCode::SUCCESS)
}

pub fn create() -> Result<ExitCode> {
    let (_, path) = create_config_file()?;
    let p = path.display().to_string();

    Output::new()
        .line(format!(
            "An empty config file has been created at '{}'.",
            p.yellow()
        ))
        .tip(format!(
            "Use '{}' to configure it.",
            "sift_cli config update".green()
        ))
        .print();

    Ok(ExitCode::SUCCESS)
}

pub fn update(profile: Option<String>, args: ConfigUpdateArgs) -> Result<ExitCode> {
    let prof = profile.clone();
    let mut target = prof.unwrap_or_else(|| String::from("default"));

    let updated_config = {
        if !args.interactive {
            if is_update_empty(&args) {
                Output::new().line("Nothing to update.").print();
                return Ok(ExitCode::SUCCESS);
            }
            get_updated_config(profile, args.grpc_uri, args.rest_uri, args.api_key)?
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

            if let Some(p) = prof.as_ref() {
                target = p.clone();
            }
            let updated = get_updated_config(prof, grpc, rest, key)?;
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

    update_config_file(updated_config)?;

    Output::new()
        .line(format!(
            "Successfully configured the '{}' profile.",
            target.yellow()
        ))
        .print();

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
                "sift_cli config create".green()
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

    let config_file = File::create_new(&path).context("failed to create config file")?;

    Ok((config_file, path))
}

fn get_updated_config(
    profile: Option<String>,
    grpc_uri: Option<String>,
    rest_uri: Option<String>,
    api_key: Option<String>,
) -> Result<String> {
    let path = get_config_file_path()?;

    let contents = read_to_string(path).context("failed to read config file")?;

    let mut config_toml = contents
        .parse::<Table>()
        .context("config file is invalid TOML")?;

    let target = match profile {
        Some(prof) => match config_toml.get_mut(&prof) {
            Some(Value::Table(profile_config)) => profile_config,
            _ => {
                config_toml.insert(prof.clone(), Value::Table(Table::new()));
                config_toml[&prof].as_table_mut().unwrap()
            }
        },
        None => &mut config_toml,
    };

    if let Some(uri) = grpc_uri {
        target.insert(String::from("grpc_uri"), Value::String(uri));
    }
    if let Some(uri) = rest_uri {
        target.insert(String::from("rest_uri"), Value::String(uri));
    }
    if let Some(token) = api_key {
        target.insert(String::from("apikey"), Value::String(token));
    }

    Ok(config_toml.to_string())
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
        ..
    } = args;
    grpc_uri.as_ref().is_none_or(|s| s.is_empty())
        || rest_uri.as_ref().is_none_or(|s| s.is_empty())
        || api_key.as_ref().is_none_or(|s| s.is_empty())
}
