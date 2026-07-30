use std::{
    ffi::OsStr,
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use anyhow::{Context, Result, anyhow};
use serde_json::{Map, Value, json};

use super::{AccessMode, Environment, Harness, Profile, Registration, files};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum State {
    Missing,
    Current(Registration),
    ManagedDrift(Registration),
    Conflict(String),
    Unavailable(String),
}

pub(super) fn inspect(harness: Harness, environment: &Environment) -> Result<State> {
    match harness {
        Harness::Claude => inspect_claude(environment),
        Harness::Codex => inspect_codex(environment),
        Harness::Cursor => inspect_json(harness, environment),
        Harness::OpenCode => inspect_json(harness, environment),
    }
}

pub(super) fn install(
    harness: Harness,
    environment: &Environment,
    registration: &Registration,
) -> Result<()> {
    match harness {
        Harness::Claude => install_claude(environment, registration),
        Harness::Codex => install_codex(environment, registration),
        Harness::Cursor | Harness::OpenCode => install_json(harness, environment, registration),
    }
}

pub(super) fn uninstall(harness: Harness, environment: &Environment) -> Result<bool> {
    match inspect(harness, environment)? {
        State::Missing => Ok(false),
        State::Conflict(_) | State::Unavailable(_) => Ok(false),
        State::Current(_) | State::ManagedDrift(_) => match harness {
            Harness::Claude => {
                run_checked(
                    Command::new("claude").args(["mcp", "remove", "sift", "--scope", "user"]),
                    "remove the Claude Code MCP registration",
                )?;
                Ok(true)
            }
            Harness::Codex => {
                run_checked(
                    Command::new("codex").args(["mcp", "remove", "sift"]),
                    "remove the Codex MCP registration",
                )?;
                Ok(true)
            }
            Harness::Cursor | Harness::OpenCode => {
                remove_json(harness, environment)?;
                Ok(true)
            }
        },
    }
}

fn inspect_claude(environment: &Environment) -> Result<State> {
    if !environment.command_available("claude") {
        return Ok(State::Unavailable(
            "`claude` is not available on PATH".to_string(),
        ));
    }

    let output = Command::new("claude")
        .args(["mcp", "get", "sift"])
        .output()
        .context("failed to inspect Claude Code MCP configuration")?;
    if !output.status.success() {
        return Ok(missing_or_unavailable(output));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let scope = field(&stdout, "Scope:");
    if scope
        .as_deref()
        .is_some_and(|scope| !scope.to_ascii_lowercase().starts_with("user"))
    {
        return Ok(State::Conflict(format!(
            "a non-user `sift` MCP entry ({}) shadows the user integration",
            scope.unwrap_or_default()
        )));
    }

    if environment_block_has_values(&stdout) {
        return Ok(State::Conflict(
            "the existing `sift` MCP entry has custom environment variables".to_string(),
        ));
    }

    let Some(command) = field(&stdout, "Command:") else {
        return Ok(State::Conflict(
            "could not read the command for the existing `sift` MCP entry".to_string(),
        ));
    };
    let args = field(&stdout, "Args:")
        .map(|args| {
            args.split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(classify_command(&command, &args, environment))
}

fn inspect_codex(environment: &Environment) -> Result<State> {
    if !environment.command_available("codex") {
        return Ok(State::Unavailable(
            "`codex` is not available on PATH".to_string(),
        ));
    }

    let output = Command::new("codex")
        .args(["mcp", "get", "sift", "--json"])
        .output()
        .context("failed to inspect Codex MCP configuration")?;
    if !output.status.success() {
        return Ok(missing_or_unavailable(output));
    }

    let payload: Value = serde_json::from_slice(&output.stdout)
        .context("Codex returned an invalid JSON MCP configuration")?;
    let transport = payload.get("transport").unwrap_or(&payload);
    let Some(command) = transport.get("command").and_then(Value::as_str) else {
        return Ok(State::Conflict(
            "could not read the command for the existing `sift` MCP entry".to_string(),
        ));
    };
    let args = transport
        .get("args")
        .and_then(Value::as_array)
        .map(|args| {
            args.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let has_custom_env = transport
        .get("env")
        .is_some_and(|env| !env.is_null() && env.as_object().is_none_or(|env| !env.is_empty()));
    if has_custom_env {
        return Ok(State::Conflict(
            "the existing `sift` MCP entry has custom environment variables".to_string(),
        ));
    }
    Ok(classify_command(command, &args, environment))
}

fn install_claude(environment: &Environment, registration: &Registration) -> Result<()> {
    let _ = Command::new("claude")
        .args(["mcp", "remove", "sift", "--scope", "user"])
        .output();
    let mut command = Command::new("claude");
    command
        .args(["mcp", "add", "--scope", "user", "sift", "--"])
        .arg(&environment.current_exe)
        .args(mcp_args(registration));
    run_checked(
        &mut command,
        "register the Sift MCP server with Claude Code",
    )
}

fn install_codex(environment: &Environment, registration: &Registration) -> Result<()> {
    let _ = Command::new("codex")
        .args(["mcp", "remove", "sift"])
        .output();
    let mut command = Command::new("codex");
    command
        .args(["mcp", "add", "sift", "--"])
        .arg(&environment.current_exe)
        .args(mcp_args(registration));
    run_checked(&mut command, "register the Sift MCP server with Codex")
}

fn mcp_args(registration: &Registration) -> Vec<String> {
    let mut args = vec!["mcp".to_string()];
    if let Profile::Named(profile) = &registration.profile {
        args.push("--profile".to_string());
        args.push(profile.clone());
    }
    if registration.access == AccessMode::Destructive {
        args.push("--allow-destructive".to_string());
    }
    args
}

fn inspect_json(harness: Harness, environment: &Environment) -> Result<State> {
    let path = json_path(harness, environment);
    if harness == Harness::OpenCode && !path.exists() {
        let jsonc = path.with_extension("jsonc");
        if jsonc.exists() {
            return Ok(State::Unavailable(format!(
                "{} uses JSON with comments; add Sift to it manually or rename it to opencode.json",
                jsonc.display()
            )));
        }
    }

    let root = match load_json(&path) {
        Ok(Some(root)) => root,
        Ok(None) => return Ok(State::Missing),
        Err(error) => return Ok(State::Unavailable(error.to_string())),
    };
    let container_key = container_key(harness);
    let Some(servers) = root.get(container_key) else {
        return Ok(State::Missing);
    };
    let Some(servers) = servers.as_object() else {
        return Ok(State::Conflict(format!(
            "`{container_key}` in {} is not a JSON object",
            path.display()
        )));
    };
    let Some(entry) = servers.get("sift") else {
        return Ok(State::Missing);
    };
    Ok(classify_json_entry(harness, entry, environment))
}

fn install_json(
    harness: Harness,
    environment: &Environment,
    registration: &Registration,
) -> Result<()> {
    let path = json_path(harness, environment);
    let mut root = load_json(&path)?.unwrap_or_default();
    let container_key = container_key(harness);
    let servers = object_entry(&mut root, container_key)?;
    let args = mcp_args(registration);
    servers.insert(
        "sift".to_string(),
        match harness {
            Harness::Cursor => json!({
                "command": environment.current_exe,
                "args": args
            }),
            Harness::OpenCode => json!({
                "type": "local",
                "command": std::iter::once(environment.current_exe.to_string_lossy().to_string())
                    .chain(args.iter().cloned())
                    .collect::<Vec<_>>(),
                "enabled": true
            }),
            _ => unreachable!("only JSON-backed harnesses reach install_json"),
        },
    );
    write_json(&path, &root)
}

fn remove_json(harness: Harness, environment: &Environment) -> Result<()> {
    let path = json_path(harness, environment);
    let Some(mut root) = load_json(&path)? else {
        return Ok(());
    };
    if let Some(servers) = root
        .get_mut(container_key(harness))
        .and_then(Value::as_object_mut)
    {
        servers.remove("sift");
    }
    write_json(&path, &root)
}

fn classify_json_entry(harness: Harness, entry: &Value, environment: &Environment) -> State {
    let Some(entry) = entry.as_object() else {
        return State::Conflict("the existing `sift` MCP entry is not an object".to_string());
    };

    let (command, args, metadata_is_managed) = match harness {
        Harness::Cursor => {
            let allowed = ["command", "args"];
            let managed = entry.keys().all(|key| allowed.contains(&key.as_str()));
            (
                entry.get("command").and_then(Value::as_str),
                string_array(entry.get("args")),
                managed,
            )
        }
        Harness::OpenCode => {
            let allowed = ["type", "command", "enabled"];
            let managed = entry.keys().all(|key| allowed.contains(&key.as_str()))
                && entry.get("type").and_then(Value::as_str) == Some("local");
            let command = entry
                .get("command")
                .and_then(Value::as_array)
                .and_then(|command| command.first())
                .and_then(Value::as_str);
            let args = entry
                .get("command")
                .and_then(Value::as_array)
                .map(|command| {
                    command
                        .iter()
                        .skip(1)
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            let enabled = entry.get("enabled").and_then(Value::as_bool) != Some(false);
            (command, args, managed && enabled)
        }
        _ => unreachable!("only JSON-backed harnesses reach classify_json_entry"),
    };

    let Some(command) = command else {
        return State::Conflict(
            "could not read the command for the existing `sift` MCP entry".to_string(),
        );
    };
    let classified = classify_command(command, &args, environment);
    if !metadata_is_managed {
        return match classified {
            State::Current(_) | State::ManagedDrift(_) => State::Conflict(
                "the existing `sift` MCP entry contains custom settings".to_string(),
            ),
            other => other,
        };
    }
    classified
}

fn classify_command(command: &str, args: &[String], environment: &Environment) -> State {
    let current = environment.current_exe.to_string_lossy();
    let current_command = command == "sift-cli" || command == current;
    let registration = registration_from_args(args);
    if current_command && args == mcp_args(&registration) {
        return State::Current(registration);
    }
    if is_sift_cli(command) && has_mcp_command(args) {
        return State::ManagedDrift(registration);
    }
    State::Conflict(format!(
        "the existing `sift` MCP entry runs a custom command: `{command} {}`",
        args.join(" ")
    ))
}

fn registration_from_args(args: &[String]) -> Registration {
    let access = if args.iter().any(|arg| arg == "--allow-destructive") {
        AccessMode::Destructive
    } else {
        AccessMode::ReadOnly
    };
    let profile = args
        .iter()
        .enumerate()
        .find_map(|(index, arg)| {
            if arg == "--profile" {
                args.get(index + 1).cloned()
            } else {
                arg.strip_prefix("--profile=").map(str::to_string)
            }
        })
        .map_or(Profile::Default, Profile::Named);
    Registration::new(access, profile)
}

fn has_mcp_command(args: &[String]) -> bool {
    let mut skip_profile_value = false;
    for arg in args {
        if skip_profile_value {
            skip_profile_value = false;
            continue;
        }
        if arg == "--profile" {
            skip_profile_value = true;
        } else if arg == "mcp" {
            return true;
        }
    }
    false
}

fn is_sift_cli(command: &str) -> bool {
    Path::new(command)
        .file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|name| {
            matches!(
                name.to_ascii_lowercase().as_str(),
                "sift-cli" | "sift-cli.exe"
            )
        })
}

fn json_path(harness: Harness, environment: &Environment) -> PathBuf {
    match harness {
        Harness::Cursor => environment.home.join(".cursor").join("mcp.json"),
        Harness::OpenCode => environment
            .home
            .join(".config")
            .join("opencode")
            .join("opencode.json"),
        _ => unreachable!("only JSON-backed harnesses have JSON paths"),
    }
}

fn container_key(harness: Harness) -> &'static str {
    match harness {
        Harness::Cursor => "mcpServers",
        Harness::OpenCode => "mcp",
        _ => unreachable!("only JSON-backed harnesses have container keys"),
    }
}

fn load_json(path: &Path) -> Result<Option<Map<String, Value>>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(anyhow!(
                "{} is a symbolic link; refusing to replace it",
                path.display()
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to inspect {}", path.display()));
        }
    }

    let contents = match fs::read(path) {
        Ok(contents) => contents,
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", path.display()));
        }
    };
    let value: Value = serde_json::from_slice(&contents)
        .with_context(|| format!("{} is not valid JSON", path.display()))?;
    value
        .as_object()
        .cloned()
        .map(Some)
        .ok_or_else(|| anyhow!("{} must contain a JSON object", path.display()))
}

fn write_json(path: &Path, root: &Map<String, Value>) -> Result<()> {
    let mut contents = serde_json::to_vec_pretty(root)?;
    contents.push(b'\n');
    files::write_atomic(path, &contents)
}

fn object_entry<'a>(
    root: &'a mut Map<String, Value>,
    key: &str,
) -> Result<&'a mut Map<String, Value>> {
    let value = root
        .entry(key.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    value
        .as_object_mut()
        .ok_or_else(|| anyhow!("`{key}` must be a JSON object"))
}

fn string_array(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn field(contents: &str, prefix: &str) -> Option<String> {
    contents
        .lines()
        .find_map(|line| line.trim().strip_prefix(prefix))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn environment_block_has_values(contents: &str) -> bool {
    let mut environment = false;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed == "Environment:" {
            environment = true;
            continue;
        }
        if environment {
            if trimmed.is_empty() || trimmed.starts_with("To remove this server") {
                return false;
            }
            return true;
        }
    }
    false
}

fn missing_or_unavailable(output: Output) -> State {
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if text.to_ascii_lowercase().contains("no mcp server")
        || text.to_ascii_lowercase().contains("not found")
    {
        State::Missing
    } else {
        State::Unavailable(one_line(&text))
    }
}

fn run_checked(command: &mut Command, action: &str) -> Result<()> {
    let output = command
        .output()
        .with_context(|| format!("failed to {action}"))?;
    if output.status.success() {
        return Ok(());
    }
    let detail = one_line(&format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ));
    Err(anyhow!("failed to {action}: {detail}"))
}

fn one_line(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("command failed without an error message")
        .to_string()
}
