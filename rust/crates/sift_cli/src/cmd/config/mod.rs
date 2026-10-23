#[cfg(test)]
mod tests;

use crate::BIN_NAME;
use anyhow::{Context, Result, anyhow};
use crossterm::style::Stylize;
use std::{
    fs::{create_dir_all, metadata, read_to_string, write},
    path::{Path, PathBuf},
    process::ExitCode,
};
use toml::{Table, Value};

use crate::{
    cli::ConfigUpdateArgs,
    cmd::{API_KEY_KEY, API_KEY_KEY_LEGACY, profile_api_key},
    util::{
        app_uri::{infer_app_uri, normalize_app_uri},
        tty::{Output, is_interactive, read_hidden, read_line},
    },
};

pub const CONFIG_FILE_NAME: &str = "sift.toml";

/// The name shown for the top-level profile.
const DEFAULT_PROFILE: &str = "default";

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

/// The connection fields of one profile, as stored in the config file.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(super) struct ProfileFields {
    grpc_uri: Option<String>,
    rest_uri: Option<String>,
    api_key: Option<String>,
    app_uri: Option<String>,
}

impl ProfileFields {
    /// The fields of `profile`. All unset when the profile does not exist.
    fn of(config: &Table, profile: Option<&str>) -> Self {
        let Ok(table) = profile_table(config, profile) else {
            return Self::default();
        };
        let field = |key: &str| table.get(key).and_then(Value::as_str).map(str::to_string);
        Self {
            grpc_uri: field("grpc_uri"),
            rest_uri: field("rest_uri"),
            api_key: profile_api_key(table).and_then(|v| v.as_str().map(str::to_string)),
            app_uri: field("app_uri"),
        }
    }

    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

pub fn create(profile: Option<String>) -> Result<ExitCode> {
    let path = get_config_file_path()?;
    let p = path.display().to_string();
    let exists = metadata(&path).is_ok();

    if !is_interactive() {
        if exists {
            Output::new()
                .line(format!("A config file already exists at '{}'.", p.yellow()))
                .tip(format!(
                    "Use '{}' to set profile values.",
                    format!("{BIN_NAME} config update").green()
                ))
                .print();
            return Ok(ExitCode::SUCCESS);
        }
        write_config(&path, &Table::new())?;
        Output::new()
            .line(format!(
                "An empty config file has been created at '{}'.",
                p.yellow()
            ))
            .tip(format!(
                "Use '{}' with --grpc-uri, --rest-uri, --api-key, and --app-uri to configure it.",
                format!("{BIN_NAME} config update").green()
            ))
            .print();
        return Ok(ExitCode::SUCCESS);
    }

    let config = if exists {
        read_config(&path)?
    } else {
        Table::new()
    };
    let explicit_default = profile.as_deref() == Some(DEFAULT_PROFILE);

    let profile = match resolve_profile(&config, profile) {
        None if ProfileFields::of(&config, None).is_empty() => None,
        None if explicit_default => {
            Output::new()
                .line(format!(
                    "Profile '{}' already exists.",
                    DEFAULT_PROFILE.yellow()
                ))
                .tip(format!(
                    "Use '{}' to edit it.",
                    format!("{BIN_NAME} config update").green()
                ))
                .print();
            return Ok(ExitCode::SUCCESS);
        }
        Some(name) => match config.get(&name) {
            Some(Value::Table(_)) => {
                Output::new()
                    .line(format!(
                        "Profile '{}' already exists.",
                        name.as_str().yellow()
                    ))
                    .tip(format!(
                        "Use '{}' to edit it.",
                        format!("{BIN_NAME} --profile {name} config update").green()
                    ))
                    .print();
                return Ok(ExitCode::SUCCESS);
            }
            Some(_) => {
                Output::new()
                    .line(format!(
                        "'{name}' is a config field, not a profile. Choose another name."
                    ))
                    .eprint();
                return Ok(ExitCode::FAILURE);
            }
            None => Some(name),
        },
        None => match prompt_new_profile_name(&config)? {
            Some(name) => Some(name),
            None => return aborted(),
        },
    };

    if !exists {
        println!("Creating a config file at '{}'.\n", p.yellow());
    }
    edit_profile(&path, config, profile)
}

pub fn update(profile: Option<String>, args: ConfigUpdateArgs) -> Result<ExitCode> {
    let path = get_config_file_path()?;
    if metadata(&path).is_err() {
        Output::new()
            .line(format!(
                "'{}' not found.",
                path.display().to_string().yellow()
            ))
            .tip(format!(
                "Run '{}' to set up your first profile.",
                format!("{BIN_NAME} config create").green()
            ))
            .eprint();
        return Ok(ExitCode::FAILURE);
    }
    let config = read_config(&path)?;
    let profile = resolve_profile(&config, profile);

    let non_empty = |value: Option<String>| value.filter(|v| !v.is_empty());
    let flags = ProfileFields {
        grpc_uri: non_empty(args.grpc_uri),
        rest_uri: non_empty(args.rest_uri),
        api_key: non_empty(args.api_key),
        app_uri: non_empty(args.app_uri),
    };
    if !args.interactive && !flags.is_empty() {
        let Some(updated) = updated_config(&config, profile.as_deref(), flags)? else {
            return Ok(ExitCode::SUCCESS);
        };
        save(&path, &updated, profile.as_deref())?;
        return Ok(ExitCode::SUCCESS);
    }

    if !is_interactive() {
        Output::new()
            .line("No fields to update, and no terminal to prompt in.")
            .tip(format!(
                "Pass one or more of --grpc-uri, --rest-uri, --api-key, or --app-uri. \
                 Or run '{}' in a terminal to edit a profile interactively.",
                format!("{BIN_NAME} config update").green()
            ))
            .eprint();
        return Ok(ExitCode::from(2));
    }

    let profile = match profile {
        Some(name) if profile_table(&config, Some(&name)).is_err() => {
            Output::new()
                .line(format!("Profile '{}' not found.", name.as_str().yellow()))
                .tip(format!(
                    "Run '{}' to create it.",
                    format!("{BIN_NAME} --profile {name} config create").green()
                ))
                .eprint();
            return Ok(ExitCode::FAILURE);
        }
        Some(name) => Some(name),
        None => select_profile(&config)?,
    };
    edit_profile(&path, config, profile)
}

/// Prompt for each field of `profile`, show the changes, and save them after
/// the user confirms. A blank answer keeps the current value.
fn edit_profile(path: &Path, config: Table, profile: Option<String>) -> Result<ExitCode> {
    let name = profile_name(profile.as_deref());
    let current = ProfileFields::of(&config, profile.as_deref());

    println!(
        "Editing profile '{}'. Press Enter to keep the value in brackets.\n",
        name.yellow()
    );
    let grpc_uri = read_line(&field_prompt("gRPC URL", current.grpc_uri.as_deref()))?;
    let rest_uri = read_line(&field_prompt("REST URL", current.rest_uri.as_deref()))?;
    let api_key = read_hidden(&field_prompt(
        "API key",
        current.api_key.as_deref().map(mask_key).as_deref(),
    ))?;

    let current_app_uri = current.app_uri.as_deref().and_then(normalize_app_uri);
    let suggested_app_uri = current_app_uri.or_else(|| {
        rest_uri
            .as_deref()
            .or(current.rest_uri.as_deref())
            .and_then(infer_app_uri)
    });
    if suggested_app_uri.is_none() {
        println!(
            "  Open your Sift web app and copy its URL origin. Keep the scheme and host \
             (for example, https://sift.example.net)."
        );
    }
    let app_uri = read_line(&field_prompt("App URL", suggested_app_uri))?.or_else(|| {
        suggested_app_uri
            .filter(|_| current_app_uri.is_none())
            .map(str::to_string)
    });

    println!();
    let edits = ProfileFields {
        grpc_uri,
        rest_uri,
        api_key,
        app_uri,
    };
    let Some(updated) = updated_config(&config, profile.as_deref(), edits)? else {
        return Ok(ExitCode::SUCCESS);
    };

    let confirmation = read_line("Apply? [Y/n]: ")?;
    if confirmation.is_some_and(|c| !c.eq_ignore_ascii_case("y") && !c.eq_ignore_ascii_case("yes"))
    {
        return aborted();
    }

    save(path, &updated, profile.as_deref())?;
    println!(
        "Run '{}' to check the connection.",
        format!("{BIN_NAME} {}ping", profile_flag(profile.as_deref())).green()
    );
    Ok(ExitCode::SUCCESS)
}

/// Ask which existing profile to edit. None selects the default profile.
fn select_profile(config: &Table) -> Result<Option<String>> {
    let named = named_profiles(config);
    if named.is_empty() {
        return Ok(None);
    }

    println!("Select a profile to edit:");
    println!("  1) {DEFAULT_PROFILE}");
    for (i, name) in named.iter().enumerate() {
        println!("  {}) {name}", i + 2);
    }
    loop {
        let Some(answer) = read_line("Profile [1]: ")? else {
            return Ok(None);
        };
        if answer == "1" || answer == DEFAULT_PROFILE {
            return Ok(None);
        }
        let by_number = answer
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(2))
            .and_then(|i| named.get(i));
        if let Some(name) = by_number.or_else(|| named.iter().find(|n| **n == answer)) {
            println!();
            return Ok(Some(name.clone()));
        }
        println!("  '{answer}' is not in the list. Enter a number or a profile name.");
    }
}

/// Ask for the name of a profile that does not exist yet. None if the user
/// leaves it blank.
fn prompt_new_profile_name(config: &Table) -> Result<Option<String>> {
    let existing = std::iter::once(DEFAULT_PROFILE.to_string())
        .chain(named_profiles(config))
        .collect::<Vec<_>>();
    println!("Existing profiles: {}.", existing.join(", "));
    loop {
        let Some(name) = read_line("Name for the new profile: ")? else {
            return Ok(None);
        };
        if name == DEFAULT_PROFILE || config.contains_key(&name) {
            println!(
                "  '{name}' is already in use. Enter another name, or run '{}' to edit it.",
                format!("{BIN_NAME} --profile {name} config update").green()
            );
            continue;
        }
        println!();
        return Ok(Some(name));
    }
}

fn field_prompt(label: &str, current: Option<&str>) -> String {
    match current {
        Some(value) => format!("  {label} [{value}]: "),
        None => format!("  {label}: "),
    }
}

/// Show only the last four characters of an API key.
pub(super) fn mask_key(key: &str) -> String {
    let len = key.chars().count();
    if len <= 8 {
        return "****".to_string();
    }
    let tail = key.chars().skip(len - 4).collect::<String>();
    format!("****{tail}")
}

/// The names of the named profiles, sorted.
pub(super) fn named_profiles(config: &Table) -> Vec<String> {
    config
        .iter()
        .filter(|(_, value)| value.is_table())
        .map(|(name, _)| name.clone())
        .collect()
}

/// One changed field, as display strings. An API key is masked.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct Change {
    field: &'static str,
    old: String,
    new: String,
}

/// Apply `edits` to a copy of `config` and print the changed fields of
/// `profile`. None, after a "No changes" notice, when no field changed.
fn updated_config(
    config: &Table,
    profile: Option<&str>,
    edits: ProfileFields,
) -> Result<Option<Table>> {
    let mut updated = config.clone();
    apply_profile_updates(
        &mut updated,
        profile.map(str::to_string),
        edits.grpc_uri,
        edits.rest_uri,
        edits.api_key,
        edits.app_uri,
    )?;
    let changes = profile_changes(config, &updated, profile);
    if changes.is_empty() {
        println!(
            "No changes to '{}'. Nothing was written.",
            profile_name(profile).yellow()
        );
        return Ok(None);
    }

    println!("Changes to '{}':", profile_name(profile).yellow());
    let old_width = changes.iter().map(|c| c.old.len()).max().unwrap_or(0);
    for Change { field, old, new } in &changes {
        println!(
            "  {field:<8}  {old:<old_width$}  ->  {}",
            new.as_str().green()
        );
    }
    Ok(Some(updated))
}

/// The fields of `profile` that differ between `before` and `after`.
pub(super) fn profile_changes(before: &Table, after: &Table, profile: Option<&str>) -> Vec<Change> {
    let (before, after) = (
        ProfileFields::of(before, profile),
        ProfileFields::of(after, profile),
    );
    let show = |value: &Option<String>, secret: bool| match value {
        None => "(unset)".to_string(),
        Some(v) if secret => mask_key(v),
        Some(v) => v.clone(),
    };
    [
        ("grpc_uri", &before.grpc_uri, &after.grpc_uri, false),
        ("rest_uri", &before.rest_uri, &after.rest_uri, false),
        (API_KEY_KEY, &before.api_key, &after.api_key, true),
        ("app_uri", &before.app_uri, &after.app_uri, false),
    ]
    .into_iter()
    .filter(|(_, old, new, _)| old != new)
    .map(|(field, old, new, secret)| Change {
        field,
        old: show(old, secret),
        new: show(new, secret),
    })
    .collect()
}

fn save(path: &Path, config: &Table, profile: Option<&str>) -> Result<()> {
    let app_uri_state = app_uri_state(config, profile)?;
    write_config(path, config)?;
    println!("Saved profile '{}'.", profile_name(profile).yellow());
    print_app_uri_guidance(profile, &app_uri_state);
    Ok(())
}

/// Resolve `--profile default` to the top-level profile, unless the file has a
/// `[default]` table.
pub(super) fn resolve_profile(config: &Table, profile: Option<String>) -> Option<String> {
    profile.filter(|name| name != DEFAULT_PROFILE || config.contains_key(name))
}

fn profile_name(profile: Option<&str>) -> &str {
    profile.unwrap_or(DEFAULT_PROFILE)
}

/// The `--profile <name> ` argument for a suggested command. Empty for the
/// default profile.
pub(super) fn profile_flag(profile: Option<&str>) -> String {
    profile.map_or_else(String::new, |profile| format!("--profile {profile} "))
}

fn aborted() -> Result<ExitCode> {
    Output::new().line("Operation aborted.").print();
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

fn read_config(path: &Path) -> Result<Table> {
    read_to_string(path)
        .context("failed to read config file")?
        .parse::<Table>()
        .context("config file is invalid TOML")
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
        // Write the canonical spelling and remove the older one, so a profile
        // never holds both keys and no reader has to choose between them.
        target.remove(API_KEY_KEY_LEGACY);
        target.insert(String::from(API_KEY_KEY), Value::String(token));
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

pub(super) fn inspect_app_uri(profile: Option<&str>) -> Result<AppUriState> {
    app_uri_state(&read_config(&get_config_file_path()?)?, profile)
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
    let profile_flag = profile_flag(profile);
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

fn write_config(path: &Path, config: &Table) -> Result<()> {
    if let Some(parent_dir) = path.parent() {
        create_dir_all(parent_dir).context("failed to create parent directories")?;
    }
    write(path, config.to_string()).context("failed to write config file")
}
