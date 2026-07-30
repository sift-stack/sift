use std::{
    ffi::{OsStr, OsString},
    io,
    path::Path,
    process::Command,
};

use anyhow::{Context, Result, anyhow};
use serde_json::Value;

use super::{AccessMode, Environment, Harness};

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
    previous: Option<NativeEntry>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum State {
    Missing,
    Current(AccessMode),
    ManagedDrift(AccessMode),
    Conflict(String),
    Unavailable(String),
}

pub(super) fn inspect(harness: Harness, environment: &Environment) -> Result<State> {
    inspect_with(harness, environment, &SystemRunner)
}

pub(super) fn snapshot(harness: Harness, environment: &Environment) -> Result<Snapshot> {
    let inspection = inspect_native(harness, environment, &SystemRunner)?;
    let previous = match inspection.state {
        State::Missing => None,
        State::Current(_) | State::ManagedDrift(_) => inspection.entry,
        State::Conflict(detail) | State::Unavailable(detail) => {
            return Err(anyhow!(
                "{} MCP registration cannot be snapshotted: {detail}",
                harness.label()
            ));
        }
    };
    Ok(Snapshot { harness, previous })
}

pub(super) fn restore(snapshot: &Snapshot, environment: &Environment) -> Result<()> {
    restore_native(
        snapshot.harness,
        snapshot.previous.as_ref(),
        environment,
        &SystemRunner,
    )
}

fn inspect_with(harness: Harness, environment: &Environment, runner: &dyn Runner) -> Result<State> {
    Ok(inspect_native(harness, environment, runner)?.state)
}

pub(super) fn install(
    harness: Harness,
    environment: &Environment,
    access: AccessMode,
) -> Result<()> {
    install_with(harness, environment, access, &SystemRunner)
}

fn install_with(
    harness: Harness,
    environment: &Environment,
    access: AccessMode,
    runner: &dyn Runner,
) -> Result<()> {
    install_native(harness, environment, access, runner)
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
        State::Conflict(_) | State::Unavailable(_) => Ok(false),
        State::Current(_) | State::ManagedDrift(_) => {
            remove_native(harness, runner)?;
            Ok(true)
        }
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
    }
}

fn inspect_claude(environment: &Environment, runner: &dyn Runner) -> Result<NativeInspection> {
    if !environment.command_available("claude") {
        return Ok(NativeInspection {
            state: State::Unavailable("`claude` is not available on PATH".to_string()),
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
            state: State::Unavailable("`codex` is not available on PATH".to_string()),
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
    access: AccessMode,
    runner: &dyn Runner,
) -> Result<()> {
    let inspection = inspect_native(harness, environment, runner)?;
    let previous = match inspection.state {
        State::Missing => None,
        State::Current(_) | State::ManagedDrift(_) => inspection.entry,
        State::Conflict(detail) | State::Unavailable(detail) => {
            return Err(anyhow!(
                "{} MCP registration cannot be replaced: {detail}",
                harness.label()
            ));
        }
    };
    let desired = NativeEntry {
        command: environment.current_exe.to_string_lossy().to_string(),
        args: mcp_args(access),
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
        State::Conflict(detail) | State::Unavailable(detail) => {
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

fn mcp_args(access: AccessMode) -> Vec<String> {
    let mut args = vec!["mcp".to_string()];
    if access == AccessMode::Destructive {
        args.push("--allow-destructive".to_string());
    }
    args
}

fn classify_command(command: &str, args: &[String], environment: &Environment) -> State {
    let current = environment.current_exe.to_string_lossy();
    let current_command = command == "sift-cli" || command == current;
    let Some(access) = access_from_args(args) else {
        return State::Conflict(format!(
            "the existing `sift` MCP entry runs a custom command: `{command} {}`",
            args.join(" ")
        ));
    };
    if current_command {
        return State::Current(access);
    }
    if is_sift_cli(command) {
        return State::ManagedDrift(access);
    }
    State::Conflict(format!(
        "the existing `sift` MCP entry runs a custom command: `{command} {}`",
        args.join(" ")
    ))
}

fn access_from_args(args: &[String]) -> Option<AccessMode> {
    match args {
        [mcp] if mcp == "mcp" => Some(AccessMode::ReadOnly),
        [mcp, destructive] if mcp == "mcp" && destructive == "--allow-destructive" => {
            Some(AccessMode::Destructive)
        }
        _ => None,
    }
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
        assert_eq!(
            access_from_args(&args(&["mcp"])),
            Some(AccessMode::ReadOnly)
        );
        assert_eq!(
            access_from_args(&args(&["mcp", "--allow-destructive"])),
            Some(AccessMode::Destructive)
        );

        for custom in [
            args(&["mcp", "--custom"]),
            args(&["mcp", "--allow-destructive", "--extra"]),
            args(&["--allow-destructive"]),
            args(&["mcp", "mcp"]),
            args(&[]),
        ] {
            assert_eq!(access_from_args(&custom), None);
        }
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

        let error =
            install_with(Harness::Codex, &environment, AccessMode::ReadOnly, &runner).unwrap_err();

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
