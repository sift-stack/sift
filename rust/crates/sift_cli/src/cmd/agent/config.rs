use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, anyhow};
use serde_json::{Map, Value, json};

use super::{AccessMode, Environment, Harness, Profile, Registration, files};

#[derive(Debug)]
struct CommandOutput {
    success: bool,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

trait Runner {
    fn output(&self, program: &str, args: &[OsString]) -> io::Result<CommandOutput>;
}

struct SystemRunner;

impl Runner for SystemRunner {
    fn output(&self, program: &str, args: &[OsString]) -> io::Result<CommandOutput> {
        Command::new(program)
            .args(args)
            .output()
            .map(|output| CommandOutput {
                success: output.status.success(),
                stdout: output.stdout,
                stderr: output.stderr,
            })
    }
}

#[derive(Debug, Clone)]
struct NativeEntry {
    command: String,
    args: Vec<String>,
}

#[derive(Debug)]
struct NativeInspection {
    state: State,
    entry: Option<NativeEntry>,
}

#[derive(Debug)]
pub(super) struct Snapshot {
    harness: Harness,
    contents: SnapshotContents,
}

#[derive(Debug)]
enum SnapshotContents {
    Native(Option<NativeEntry>),
    Json {
        path: PathBuf,
        contents: Option<Vec<u8>>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum State {
    Missing,
    Current(Registration),
    ManagedDrift(Registration),
    Conflict(String),
    Unregistrable(String),
    Unavailable(String),
}

pub(super) fn inspect(harness: Harness, environment: &Environment) -> Result<State> {
    inspect_with(harness, environment, &SystemRunner)
}

pub(super) fn snapshot(harness: Harness, environment: &Environment) -> Result<Snapshot> {
    let contents = match harness {
        Harness::Claude | Harness::Codex => {
            let inspection = inspect_native(harness, environment, &SystemRunner)?;
            match inspection.state {
                State::Missing => SnapshotContents::Native(None),
                State::Current(_) | State::ManagedDrift(_) => {
                    SnapshotContents::Native(inspection.entry)
                }
                State::Conflict(detail)
                | State::Unregistrable(detail)
                | State::Unavailable(detail) => {
                    return Err(anyhow!(
                        "{} MCP registration cannot be snapshotted: {detail}",
                        harness.label()
                    ));
                }
            }
        }
        Harness::Cursor | Harness::OpenCode => {
            let path = json_path(harness, environment);
            let contents = match fs::symlink_metadata(&path) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    return Err(anyhow!(
                        "{} is a symbolic link; refusing to snapshot it",
                        path.display()
                    ));
                }
                Ok(_) => Some(
                    fs::read(&path)
                        .with_context(|| format!("failed to read {}", path.display()))?,
                ),
                Err(error) if error.kind() == ErrorKind::NotFound => None,
                Err(error) => {
                    return Err(error)
                        .with_context(|| format!("failed to inspect {}", path.display()));
                }
            };
            SnapshotContents::Json { path, contents }
        }
    };
    Ok(Snapshot { harness, contents })
}

pub(super) fn restore(snapshot: &Snapshot, environment: &Environment) -> Result<()> {
    match &snapshot.contents {
        SnapshotContents::Native(previous) => restore_native(
            snapshot.harness,
            previous.as_ref(),
            environment,
            &SystemRunner,
        ),
        SnapshotContents::Json { path, contents } => match contents {
            Some(contents) => files::write_atomic(path, contents),
            None => match fs::symlink_metadata(path) {
                Ok(metadata) if metadata.file_type().is_symlink() => Err(anyhow!(
                    "{} became a symbolic link; refusing to remove it",
                    path.display()
                )),
                Ok(_) => {
                    fs::remove_file(path)
                        .with_context(|| format!("failed to remove {}", path.display()))?;
                    files::remove_empty_parent(path);
                    Ok(())
                }
                Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
                Err(error) => {
                    Err(error).with_context(|| format!("failed to inspect {}", path.display()))
                }
            },
        },
    }
}

fn inspect_with(harness: Harness, environment: &Environment, runner: &dyn Runner) -> Result<State> {
    match harness {
        Harness::Claude | Harness::Codex => Ok(inspect_native(harness, environment, runner)?.state),
        Harness::Cursor => inspect_json(harness, environment),
        Harness::OpenCode => inspect_json(harness, environment),
    }
}

pub(super) fn install(
    harness: Harness,
    environment: &Environment,
    registration: &Registration,
) -> Result<()> {
    install_with(harness, environment, registration, &SystemRunner)
}

fn install_with(
    harness: Harness,
    environment: &Environment,
    registration: &Registration,
    runner: &dyn Runner,
) -> Result<()> {
    match harness {
        Harness::Claude | Harness::Codex => {
            install_native(harness, environment, registration, runner)
        }
        Harness::Cursor | Harness::OpenCode => install_json(harness, environment, registration),
    }
}

pub(super) fn uninstall(harness: Harness, environment: &Environment) -> Result<bool> {
    uninstall_with(harness, environment, &SystemRunner)
}

fn uninstall_with(
    harness: Harness,
    environment: &Environment,
    runner: &dyn Runner,
) -> Result<bool> {
    match inspect_with(harness, environment, runner)? {
        State::Missing => Ok(false),
        State::Conflict(_) | State::Unregistrable(_) | State::Unavailable(_) => Ok(false),
        State::Current(_) | State::ManagedDrift(_) => match harness {
            Harness::Claude | Harness::Codex => {
                remove_native(harness, runner)?;
                Ok(true)
            }
            Harness::Cursor | Harness::OpenCode => {
                remove_json(harness, environment)?;
                Ok(true)
            }
        },
    }
}

fn inspect_native(
    harness: Harness,
    environment: &Environment,
    runner: &dyn Runner,
) -> Result<NativeInspection> {
    match harness {
        Harness::Claude => inspect_claude(environment, runner),
        Harness::Codex => inspect_codex(environment, runner),
        Harness::Cursor | Harness::OpenCode => {
            unreachable!("JSON-backed harnesses do not have native registrations")
        }
    }
}

fn inspect_claude(environment: &Environment, runner: &dyn Runner) -> Result<NativeInspection> {
    if !environment.command_available("claude") {
        return Ok(NativeInspection {
            state: State::Unregistrable("`claude` is not available on PATH".to_string()),
            entry: None,
        });
    }

    let output = runner
        .output("claude", &os_args(["mcp", "get", "sift"]))
        .context("failed to inspect Claude Code MCP configuration")?;
    if !output.success {
        return Ok(NativeInspection {
            state: missing_or_unavailable(output),
            entry: None,
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let scope = field(&stdout, "Scope:");
    if scope
        .as_deref()
        .is_some_and(|scope| !scope.to_ascii_lowercase().starts_with("user"))
    {
        return Ok(NativeInspection {
            state: State::Conflict(format!(
                "a non-user `sift` MCP entry ({}) shadows the user integration",
                scope.unwrap_or_default()
            )),
            entry: None,
        });
    }

    if environment_block_has_values(&stdout) {
        return Ok(NativeInspection {
            state: State::Conflict(
                "the existing `sift` MCP entry has custom environment variables".to_string(),
            ),
            entry: None,
        });
    }

    let Some(command) = field(&stdout, "Command:") else {
        return Ok(NativeInspection {
            state: State::Conflict(
                "could not read the command for the existing `sift` MCP entry".to_string(),
            ),
            entry: None,
        });
    };
    let args = field(&stdout, "Args:")
        .map(|args| {
            args.split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(NativeInspection {
        state: classify_command(&command, &args, environment),
        entry: Some(NativeEntry { command, args }),
    })
}

fn inspect_codex(environment: &Environment, runner: &dyn Runner) -> Result<NativeInspection> {
    if !environment.command_available("codex") {
        return Ok(NativeInspection {
            state: State::Unregistrable("`codex` is not available on PATH".to_string()),
            entry: None,
        });
    }

    let output = runner
        .output("codex", &os_args(["mcp", "get", "sift", "--json"]))
        .context("failed to inspect Codex MCP configuration")?;
    if !output.success {
        return Ok(NativeInspection {
            state: missing_or_unavailable(output),
            entry: None,
        });
    }

    let payload: Value = serde_json::from_slice(&output.stdout)
        .context("Codex returned an invalid JSON MCP configuration")?;
    let transport = payload.get("transport").unwrap_or(&payload);
    let Some(command) = transport.get("command").and_then(Value::as_str) else {
        return Ok(NativeInspection {
            state: State::Conflict(
                "could not read the command for the existing `sift` MCP entry".to_string(),
            ),
            entry: None,
        });
    };
    let Some(args) = string_array(transport.get("args")) else {
        return Ok(NativeInspection {
            state: State::Conflict(
                "the existing `sift` MCP entry has invalid arguments".to_string(),
            ),
            entry: None,
        });
    };
    if !codex_metadata_is_managed(&payload, transport) {
        return Ok(NativeInspection {
            state: State::Conflict(
                "the existing `sift` MCP entry contains custom settings".to_string(),
            ),
            entry: None,
        });
    }
    Ok(NativeInspection {
        state: classify_command(command, &args, environment),
        entry: Some(NativeEntry {
            command: command.to_string(),
            args,
        }),
    })
}

fn install_native(
    harness: Harness,
    environment: &Environment,
    registration: &Registration,
    runner: &dyn Runner,
) -> Result<()> {
    let inspection = inspect_native(harness, environment, runner)?;
    let previous = match inspection.state {
        State::Missing => None,
        State::Current(_) | State::ManagedDrift(_) => inspection.entry,
        State::Conflict(detail) | State::Unregistrable(detail) | State::Unavailable(detail) => {
            return Err(anyhow!(
                "{} MCP registration cannot be replaced: {detail}",
                harness.label()
            ));
        }
    };
    let desired = NativeEntry {
        command: environment.current_exe.to_string_lossy().to_string(),
        args: mcp_args(registration),
    };

    if previous.is_some() {
        remove_native(harness, runner)?;
    }
    if let Err(error) = add_native(harness, &desired, runner) {
        let Some(previous) = previous else {
            return Err(error);
        };
        return match add_native(harness, &previous, runner) {
            Ok(()) => Err(error.context("the previous MCP registration was restored")),
            Err(rollback_error) => Err(anyhow!(
                "{error:#}; restoring the previous MCP registration also failed: {rollback_error:#}"
            )),
        };
    }
    Ok(())
}

fn restore_native(
    harness: Harness,
    previous: Option<&NativeEntry>,
    environment: &Environment,
    runner: &dyn Runner,
) -> Result<()> {
    let inspection = inspect_native(harness, environment, runner)?;
    let current = match inspection.state {
        State::Missing => None,
        State::Current(_) | State::ManagedDrift(_) => inspection.entry,
        State::Conflict(detail) | State::Unregistrable(detail) | State::Unavailable(detail) => {
            return Err(anyhow!(
                "{} MCP registration cannot be restored: {detail}",
                harness.label()
            ));
        }
    };

    if current.is_some() {
        remove_native(harness, runner)?;
    }
    let Some(previous) = previous else {
        return Ok(());
    };
    if let Err(error) = add_native(harness, previous, runner) {
        let Some(current) = current else {
            return Err(error);
        };
        return match add_native(harness, &current, runner) {
            Ok(()) => Err(error.context("the newer MCP registration was restored")),
            Err(rollback_error) => Err(anyhow!(
                "{error:#}; restoring the newer MCP registration also failed: {rollback_error:#}"
            )),
        };
    }
    Ok(())
}

fn mcp_args(registration: &Registration) -> Vec<String> {
    let mut args = vec!["mcp".to_string()];
    if let Profile::Named(profile) = &registration.profile {
        args.push("--profile".to_string());
        args.push(profile.clone());
    }
    match registration.access {
        AccessMode::ReadOnly => {}
        AccessMode::Create => args.push("--allow-create".to_string()),
        AccessMode::Destructive => args.push("--allow-destructive".to_string()),
    }
    if registration.disable_update_check {
        args.push("--disable-update-check".to_string());
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
            let Some(args) = string_array(entry.get("args")) else {
                return State::Conflict(
                    "the existing `sift` MCP entry has invalid arguments".to_string(),
                );
            };
            (
                entry
                    .get("command")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                args,
                managed,
            )
        }
        Harness::OpenCode => {
            let allowed = ["type", "command", "enabled"];
            let managed = entry.keys().all(|key| allowed.contains(&key.as_str()))
                && entry.get("type").and_then(Value::as_str) == Some("local");
            let Some(command_parts) = string_array(entry.get("command")) else {
                return State::Conflict(
                    "the existing `sift` MCP entry has an invalid command".to_string(),
                );
            };
            let command = command_parts.first().cloned();
            let args = command_parts.iter().skip(1).cloned().collect();
            let enabled = true_or_missing(entry.get("enabled"));
            (command, args, managed && enabled)
        }
        _ => unreachable!("only JSON-backed harnesses reach classify_json_entry"),
    };

    let Some(command) = command else {
        return State::Conflict(
            "could not read the command for the existing `sift` MCP entry".to_string(),
        );
    };
    let classified = classify_command(&command, &args, environment);
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
    let Some(registration) = registration_from_args(args) else {
        return State::Conflict(format!(
            "the existing `sift` MCP entry runs a custom command: `{command} {}`",
            args.join(" ")
        ));
    };
    if current_command {
        return State::Current(registration);
    }
    if is_sift_cli(command) {
        return State::ManagedDrift(registration);
    }
    State::Conflict(format!(
        "the existing `sift` MCP entry runs a custom command: `{command} {}`",
        args.join(" ")
    ))
}

fn registration_from_args(args: &[String]) -> Option<Registration> {
    let (base_args, disable_update_check) = match args.split_last() {
        Some((flag, base_args)) if flag == "--disable-update-check" => (base_args, true),
        _ => (args, false),
    };
    let registration = match base_args {
        [mcp] if mcp == "mcp" => Some(Registration::new(AccessMode::ReadOnly, Profile::Default)),
        [mcp, access] if mcp == "mcp" => {
            access_from_flag(access).map(|a| Registration::new(a, Profile::Default))
        }
        [mcp, profile_flag, profile]
            if mcp == "mcp" && profile_flag == "--profile" && valid_profile(profile) =>
        {
            Some(Registration::new(
                AccessMode::ReadOnly,
                Profile::Named(profile.clone()),
            ))
        }
        [mcp, profile_flag, profile, access]
            if mcp == "mcp" && profile_flag == "--profile" && valid_profile(profile) =>
        {
            access_from_flag(access).map(|a| Registration::new(a, Profile::Named(profile.clone())))
        }
        _ => None,
    }?;
    Some(registration.with_update_check_disabled(disable_update_check))
}

fn access_from_flag(flag: &str) -> Option<AccessMode> {
    match flag {
        "--allow-create" => Some(AccessMode::Create),
        "--allow-destructive" => Some(AccessMode::Destructive),
        _ => None,
    }
}

fn valid_profile(profile: &str) -> bool {
    !profile.is_empty() && !profile.starts_with('-')
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

fn string_array(value: Option<&Value>) -> Option<Vec<String>> {
    let Some(value) = value else {
        return Some(Vec::new());
    };
    value
        .as_array()?
        .iter()
        .map(|value| value.as_str().map(str::to_string))
        .collect()
}

fn codex_metadata_is_managed(payload: &Value, transport: &Value) -> bool {
    let Some(transport) = transport.as_object() else {
        return false;
    };
    let allowed_transport = ["type", "command", "args", "env", "env_vars", "cwd"];
    if !transport
        .keys()
        .all(|key| allowed_transport.contains(&key.as_str()))
        || transport
            .get("type")
            .is_some_and(|value| value.as_str() != Some("stdio"))
        || nonempty_object(transport.get("env"))
        || nonempty_array(transport.get("env_vars"))
        || transport.get("cwd").is_some_and(|value| !value.is_null())
    {
        return false;
    }

    if payload.get("transport").is_none() {
        return true;
    }
    let Some(payload) = payload.as_object() else {
        return false;
    };
    let allowed_payload = [
        "name",
        "enabled",
        "disabled_reason",
        "transport",
        "enabled_tools",
        "disabled_tools",
        "startup_timeout_sec",
        "tool_timeout_sec",
    ];
    payload
        .keys()
        .all(|key| allowed_payload.contains(&key.as_str()))
        && payload.get("name").and_then(Value::as_str) == Some("sift")
        && true_or_missing(payload.get("enabled"))
        && null_or_missing(payload.get("disabled_reason"))
        && empty_or_missing(payload.get("enabled_tools"))
        && empty_or_missing(payload.get("disabled_tools"))
        && null_or_missing(payload.get("startup_timeout_sec"))
        && null_or_missing(payload.get("tool_timeout_sec"))
}

fn nonempty_object(value: Option<&Value>) -> bool {
    value.is_some_and(|value| {
        !value.is_null() && value.as_object().is_none_or(|map| !map.is_empty())
    })
}

fn nonempty_array(value: Option<&Value>) -> bool {
    value.is_some_and(|value| {
        !value.is_null() && value.as_array().is_none_or(|items| !items.is_empty())
    })
}

fn empty_or_missing(value: Option<&Value>) -> bool {
    value.is_none_or(|value| value.is_null() || value.as_array().is_some_and(Vec::is_empty))
}

fn null_or_missing(value: Option<&Value>) -> bool {
    value.is_none_or(Value::is_null)
}

fn true_or_missing(value: Option<&Value>) -> bool {
    value.is_none_or(|value| value.as_bool() == Some(true))
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

fn missing_or_unavailable(output: CommandOutput) -> State {
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

fn add_native(harness: Harness, entry: &NativeEntry, runner: &dyn Runner) -> Result<()> {
    let (program, mut args, action) = match harness {
        Harness::Claude => (
            "claude",
            os_args(["mcp", "add", "--scope", "user", "sift", "--"]),
            "register the Sift MCP server with Claude Code",
        ),
        Harness::Codex => (
            "codex",
            os_args(["mcp", "add", "sift", "--"]),
            "register the Sift MCP server with Codex",
        ),
        Harness::Cursor | Harness::OpenCode => {
            unreachable!("JSON-backed harnesses do not use native registration commands")
        }
    };
    args.push(OsString::from(&entry.command));
    args.extend(entry.args.iter().map(OsString::from));
    run_checked(runner, program, &args, action)
}

fn remove_native(harness: Harness, runner: &dyn Runner) -> Result<()> {
    let (program, args, action) = match harness {
        Harness::Claude => (
            "claude",
            os_args(["mcp", "remove", "sift", "--scope", "user"]),
            "remove the Claude Code MCP registration",
        ),
        Harness::Codex => (
            "codex",
            os_args(["mcp", "remove", "sift"]),
            "remove the Codex MCP registration",
        ),
        Harness::Cursor | Harness::OpenCode => {
            unreachable!("JSON-backed harnesses do not use native registration commands")
        }
    };
    run_checked(runner, program, &args, action)
}

fn run_checked(runner: &dyn Runner, program: &str, args: &[OsString], action: &str) -> Result<()> {
    let output = runner
        .output(program, args)
        .with_context(|| format!("failed to {action}"))?;
    if output.success {
        return Ok(());
    }
    let detail = one_line(&format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ));
    Err(anyhow!("failed to {action}: {detail}"))
}

fn os_args<const N: usize>(values: [&str; N]) -> Vec<OsString> {
    values.into_iter().map(OsString::from).collect()
}

fn one_line(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("command failed without an error message")
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::VecDeque, fs};

    use serde_json::json;
    use tempdir::TempDir;

    use super::*;

    struct FakeRunner {
        outputs: RefCell<VecDeque<CommandOutput>>,
        calls: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl FakeRunner {
        fn new(outputs: impl IntoIterator<Item = CommandOutput>) -> Self {
            Self {
                outputs: RefCell::new(outputs.into_iter().collect()),
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl Runner for FakeRunner {
        fn output(&self, program: &str, args: &[OsString]) -> io::Result<CommandOutput> {
            self.calls.borrow_mut().push((
                program.to_string(),
                args.iter()
                    .map(|arg| arg.to_string_lossy().to_string())
                    .collect(),
            ));
            self.outputs
                .borrow_mut()
                .pop_front()
                .ok_or_else(|| io::Error::other("fake runner has no queued output"))
        }
    }

    fn output(
        success: bool,
        stdout: impl Into<Vec<u8>>,
        stderr: impl Into<Vec<u8>>,
    ) -> CommandOutput {
        CommandOutput {
            success,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn only_exact_sift_mcp_argument_shapes_are_managed() {
        assert!(registration_from_args(&args(&["mcp"])).is_some());
        assert!(registration_from_args(&args(&["mcp", "--allow-create"])).is_some());
        assert!(registration_from_args(&args(&["mcp", "--allow-destructive"])).is_some());
        assert!(registration_from_args(&args(&["mcp", "--profile", "localdev"])).is_some());
        assert!(
            registration_from_args(&args(&["mcp", "--profile", "localdev", "--allow-create",]))
                .is_some()
        );
        assert!(
            registration_from_args(&args(&[
                "mcp",
                "--profile",
                "localdev",
                "--allow-destructive",
            ]))
            .is_some()
        );
        assert!(registration_from_args(&args(&["mcp", "--disable-update-check"])).is_some());
        assert!(
            registration_from_args(&args(&["mcp", "--allow-create", "--disable-update-check",]))
                .is_some()
        );
        assert!(
            registration_from_args(&args(&[
                "mcp",
                "--profile",
                "localdev",
                "--allow-destructive",
                "--disable-update-check",
            ]))
            .is_some()
        );

        for custom in [
            args(&["mcp", "--custom"]),
            args(&["mcp", "--profile"]),
            args(&["mcp", "--profile", "--allow-destructive"]),
            args(&["mcp", "--allow-destructive", "--profile", "localdev"]),
            args(&["mcp", "--allow-create", "--profile", "localdev"]),
            args(&["mcp", "--profile", "localdev", "--extra"]),
        ] {
            assert_eq!(registration_from_args(&custom), None);
        }
    }

    #[test]
    fn disabled_update_check_round_trips_through_managed_args() {
        let registration = Registration::new(AccessMode::Create, Profile::Default)
            .with_update_check_disabled(true);

        let args = mcp_args(&registration);

        assert_eq!(args, ["mcp", "--allow-create", "--disable-update-check"]);
        assert_eq!(registration_from_args(&args), Some(registration));
    }

    #[test]
    fn codex_default_metadata_is_managed_but_custom_settings_are_not() {
        let payload = json!({
            "name": "sift",
            "enabled": true,
            "disabled_reason": null,
            "transport": {
                "type": "stdio",
                "command": "sift-cli",
                "args": ["mcp"],
                "env": null,
                "env_vars": [],
                "cwd": null
            },
            "enabled_tools": null,
            "disabled_tools": null,
            "startup_timeout_sec": null,
            "tool_timeout_sec": null
        });
        assert!(codex_metadata_is_managed(&payload, &payload["transport"]));

        for custom in [
            ("enabled", json!(false)),
            ("enabled", json!("true")),
            ("enabled_tools", json!(["query"])),
            ("disabled_tools", json!(["delete"])),
            ("startup_timeout_sec", json!(30)),
            ("tool_timeout_sec", json!(60)),
        ] {
            let mut changed = payload.clone();
            changed[custom.0] = custom.1;
            assert!(!codex_metadata_is_managed(&changed, &changed["transport"]));
        }

        for custom in [
            ("env", json!({"SIFT_API_KEY": "custom"})),
            ("env_vars", json!(["SIFT_API_KEY"])),
            ("cwd", json!("/tmp/custom")),
        ] {
            let mut changed = payload.clone();
            changed["transport"][custom.0] = custom.1;
            assert!(!codex_metadata_is_managed(&changed, &changed["transport"]));
        }
    }

    #[cfg(unix)]
    #[test]
    fn failed_native_replacement_restores_the_previous_registration() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TempDir::new("sift-cli-agent-native-rollback").unwrap();
        let bin = directory.path().join("bin");
        fs::create_dir_all(&bin).unwrap();
        let codex = bin.join("codex");
        fs::write(&codex, "").unwrap();
        fs::set_permissions(&codex, fs::Permissions::from_mode(0o755)).unwrap();

        let current_exe = directory.path().join("current/sift-cli");
        let mut environment = Environment::for_test(
            directory.path().to_path_buf(),
            current_exe.clone(),
            vec![Harness::Codex],
        );
        environment.path = bin.into_os_string();
        let existing = json!({
            "name": "sift",
            "enabled": true,
            "disabled_reason": null,
            "transport": {
                "type": "stdio",
                "command": "/old/sift-cli",
                "args": ["mcp"],
                "env": null,
                "env_vars": [],
                "cwd": null
            },
            "enabled_tools": null,
            "disabled_tools": null,
            "startup_timeout_sec": null,
            "tool_timeout_sec": null
        });
        let runner = FakeRunner::new([
            output(true, serde_json::to_vec(&existing).unwrap(), Vec::new()),
            output(true, Vec::new(), Vec::new()),
            output(false, Vec::new(), b"desired add failed".to_vec()),
            output(true, Vec::new(), Vec::new()),
        ]);

        let error = install_with(
            Harness::Codex,
            &environment,
            &Registration::new(AccessMode::ReadOnly, Profile::Default),
            &runner,
        )
        .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("the previous MCP registration was restored")
        );
        assert_eq!(
            runner.calls.into_inner(),
            vec![
                ("codex".to_string(), args(&["mcp", "get", "sift", "--json"])),
                ("codex".to_string(), args(&["mcp", "remove", "sift"])),
                (
                    "codex".to_string(),
                    vec![
                        "mcp".to_string(),
                        "add".to_string(),
                        "sift".to_string(),
                        "--".to_string(),
                        current_exe.to_string_lossy().to_string(),
                        "mcp".to_string(),
                    ]
                ),
                (
                    "codex".to_string(),
                    args(&["mcp", "add", "sift", "--", "/old/sift-cli", "mcp"])
                ),
            ]
        );
    }
}
