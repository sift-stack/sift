use std::{fs, path::PathBuf};

use clap::Parser;
use serde_json::Value;
use tempdir::TempDir;

use super::{AccessInference, AccessMode, Environment, Harness, config, skill};

fn environment(harnesses: Vec<Harness>) -> (TempDir, Environment) {
    let directory = TempDir::new("sift-cli-agent-tests").unwrap();
    let current_exe = directory.path().join("bin").join("sift-cli");
    (
        directory,
        Environment::for_test(PathBuf::new(), current_exe, harnesses),
    )
}

#[test]
fn stale_native_config_directories_are_not_detected_as_installed_clients() {
    let directory = TempDir::new("sift-cli-agent-detection").unwrap();
    fs::create_dir_all(directory.path().join(".claude")).unwrap();
    fs::create_dir_all(directory.path().join(".codex")).unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        Vec::new(),
    );

    assert!(environment.detect_harnesses().is_empty());
}

#[test]
fn one_shared_skill_covers_agent_skills_clients() {
    let (directory, mut environment) =
        environment(vec![Harness::Codex, Harness::Cursor, Harness::OpenCode]);
    environment.home = directory.path().to_path_buf();

    let targets = skill::targets(&environment);

    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].harnesses.len(), 3);
    assert_eq!(
        targets[0].path,
        directory.path().join(".agents/skills/sift")
    );
}

#[test]
fn skill_install_is_fresh_and_stateless() {
    let directory = TempDir::new("sift-cli-agent-skill").unwrap();
    let path = directory.path().join(".agents/skills/sift");

    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Missing);
    skill::install(&path).unwrap();
    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Current);

    fs::write(
        path.join("SKILL.md"),
        skill::CONTENT.replace("# Sift toolbox", "# Sift toolbox\n\nOld release."),
    )
    .unwrap();
    assert_eq!(
        skill::inspect(&path).unwrap(),
        skill::State::ManagedOutdated
    );
    skill::install(&path).unwrap();
    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Current);
}

#[test]
fn skill_install_removes_every_stale_managed_file() {
    let directory = TempDir::new("sift-cli-agent-skill-mirror").unwrap();
    let path = directory.path().join(".agents/skills/sift");
    skill::install(&path).unwrap();
    fs::write(path.join("removed.md"), "stale top-level content").unwrap();
    fs::create_dir_all(path.join("old/nested")).unwrap();
    fs::write(path.join("old/nested/removed.md"), "stale nested content").unwrap();

    assert_eq!(
        skill::inspect(&path).unwrap(),
        skill::State::ManagedOutdated
    );
    skill::install(&path).unwrap();

    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Current);
    assert!(!path.join("removed.md").exists());
    assert!(!path.join("old").exists());
    assert_eq!(
        fs::read_to_string(path.join("SKILL.md")).unwrap(),
        skill::CONTENT
    );
}

#[test]
fn successful_reinstall_leaves_no_backup_behind() {
    let directory = TempDir::new("sift-cli-agent-skill-backup").unwrap();
    let path = directory.path().join(".agents/skills/sift");

    skill::install(&path).unwrap();
    skill::install(&path).unwrap();

    let leftovers = fs::read_dir(path.parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .filter(|name| name.contains("sift-cli-backup"))
        .collect::<Vec<_>>();

    assert!(leftovers.is_empty(), "backups left behind: {leftovers:?}");
}

#[test]
fn unmanaged_skill_is_never_removed() {
    let directory = TempDir::new("sift-cli-agent-skill-conflict").unwrap();
    let path = directory.path().join(".agents/skills/sift");
    fs::create_dir_all(&path).unwrap();
    fs::write(
        path.join("SKILL.md"),
        "---\nname: sift\n---\nCustom instructions.\n",
    )
    .unwrap();

    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Conflict);
    assert!(!skill::uninstall(&path).unwrap());
    assert!(path.exists());
}

#[test]
fn cursor_config_merge_preserves_other_servers() {
    let directory = TempDir::new("sift-cli-agent-cursor").unwrap();
    let current_exe = directory.path().join("bin/sift-cli");
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        current_exe.clone(),
        vec![Harness::Cursor],
    );
    let path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        r#"{"mcpServers":{"other":{"command":"other-server"}},"setting":true}"#,
    )
    .unwrap();

    config::install(Harness::Cursor, &environment, AccessMode::ReadOnly).unwrap();

    let value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(value["setting"], true);
    assert_eq!(value["mcpServers"]["other"]["command"], "other-server");
    assert_eq!(
        value["mcpServers"]["sift"]["command"],
        current_exe.to_string_lossy().as_ref()
    );
    assert_eq!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Current(AccessMode::ReadOnly)
    );
}

#[test]
fn opencode_config_uses_local_command_array() {
    let directory = TempDir::new("sift-cli-agent-opencode").unwrap();
    let current_exe = directory.path().join("bin/sift-cli");
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        current_exe.clone(),
        vec![Harness::OpenCode],
    );

    config::install(Harness::OpenCode, &environment, AccessMode::ReadOnly).unwrap();

    let path = directory.path().join(".config/opencode/opencode.json");
    let value: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    assert_eq!(value["mcp"]["sift"]["type"], "local");
    assert_eq!(value["mcp"]["sift"]["enabled"], true);
    assert_eq!(
        value["mcp"]["sift"]["command"][0],
        current_exe.to_string_lossy().as_ref()
    );
    assert_eq!(value["mcp"]["sift"]["command"][1], "mcp");
}

#[test]
fn custom_same_name_server_is_a_conflict() {
    let directory = TempDir::new("sift-cli-agent-conflict").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        r#"{"mcpServers":{"sift":{"command":"my-wrapper","args":["mcp"]}}}"#,
    )
    .unwrap();

    assert!(matches!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Conflict(_)
    ));
}

#[test]
fn malformed_config_container_is_caught_during_preflight() {
    let directory = TempDir::new("sift-cli-agent-malformed-config").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let config_path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, r#"{"mcpServers":[]}"#).unwrap();

    super::install_environment(&environment, "Installed", AccessMode::ReadOnly).unwrap();

    assert!(!directory.path().join(".agents/skills/sift").exists());
    assert_eq!(
        fs::read_to_string(config_path).unwrap(),
        r#"{"mcpServers":[]}"#
    );
}

#[cfg(unix)]
#[test]
fn symlinked_skill_is_never_replaced() {
    use std::os::unix::fs::symlink;

    let directory = TempDir::new("sift-cli-agent-symlink").unwrap();
    let source = directory.path().join("source");
    let path = directory.path().join(".agents/skills/sift");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("SKILL.md"), skill::CONTENT).unwrap();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    symlink(&source, &path).unwrap();

    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Conflict);
    assert!(!skill::uninstall(&path).unwrap());
    assert!(path.is_symlink());
}

#[test]
fn destructive_access_is_a_current_managed_registration() {
    let directory = TempDir::new("sift-cli-agent-destructive-access").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        r#"{"mcpServers":{"sift":{"command":"sift-cli","args":["mcp","--allow-destructive"]}}}"#,
    )
    .unwrap();

    assert_eq!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Current(AccessMode::Destructive)
    );
}

#[test]
fn destructive_install_updates_every_json_backed_client() {
    let directory = TempDir::new("sift-cli-agent-destructive-install").unwrap();
    let current_exe = directory.path().join("bin/sift-cli");
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        current_exe,
        vec![Harness::Cursor, Harness::OpenCode],
    );

    config::install(Harness::Cursor, &environment, AccessMode::Destructive).unwrap();
    config::install(Harness::OpenCode, &environment, AccessMode::Destructive).unwrap();

    assert_eq!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Current(AccessMode::Destructive)
    );
    assert_eq!(
        config::inspect(Harness::OpenCode, &environment).unwrap(),
        config::State::Current(AccessMode::Destructive)
    );
}

#[test]
fn update_access_inference_preserves_one_mode_and_rejects_mixed_modes() {
    assert_eq!(
        super::infer_access_modes(&[]),
        AccessInference::Resolved(AccessMode::ReadOnly)
    );
    assert_eq!(
        super::infer_access_modes(&[AccessMode::Destructive, AccessMode::Destructive]),
        AccessInference::Resolved(AccessMode::Destructive)
    );
    assert_eq!(
        super::infer_access_modes(&[AccessMode::ReadOnly, AccessMode::Destructive]),
        AccessInference::Mixed
    );
}

#[test]
fn update_access_flags_are_mutually_exclusive() {
    assert!(
        crate::cli::Args::try_parse_from([
            "sift-cli",
            "agent",
            "update",
            "--allow-destructive",
            "--read-only",
        ])
        .is_err()
    );
}

#[test]
fn uninstall_removes_only_sift_json_entry() {
    let directory = TempDir::new("sift-cli-agent-uninstall").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        r#"{"mcpServers":{"sift":{"command":"sift-cli","args":["mcp"]},"other":{"command":"other"}}}"#,
    )
    .unwrap();

    assert!(config::uninstall(Harness::Cursor, &environment).unwrap());

    let value: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    assert!(value["mcpServers"].get("sift").is_none());
    assert_eq!(value["mcpServers"]["other"]["command"], "other");
}

#[test]
fn install_preflight_keeps_every_target_unchanged_on_conflict() {
    let directory = TempDir::new("sift-cli-agent-install-preflight").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let config_path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(
        config_path,
        r#"{"mcpServers":{"sift":{"command":"custom-sift","args":["mcp"]}}}"#,
    )
    .unwrap();

    super::install_environment(&environment, "Installed", AccessMode::ReadOnly).unwrap();

    assert!(!directory.path().join(".agents/skills/sift").exists());
}

#[cfg(unix)]
#[test]
fn failed_later_client_install_rolls_back_earlier_clients_and_skills() {
    use std::os::unix::fs::PermissionsExt;

    let directory = TempDir::new("sift-cli-agent-install-rollback").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor, Harness::OpenCode],
    );
    let cursor_path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(cursor_path.parent().unwrap()).unwrap();
    let original_cursor = br#"{"mcpServers":{"other":{"command":"other"}}}"#;
    fs::write(&cursor_path, original_cursor).unwrap();

    let opencode_dir = directory.path().join(".config/opencode");
    fs::create_dir_all(&opencode_dir).unwrap();
    fs::set_permissions(&opencode_dir, fs::Permissions::from_mode(0o555)).unwrap();

    let result = super::install_environment(&environment, "Installed", AccessMode::ReadOnly);

    fs::set_permissions(&opencode_dir, fs::Permissions::from_mode(0o755)).unwrap();
    let error = result.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("all earlier agent integration changes were rolled back")
    );
    assert_eq!(fs::read(&cursor_path).unwrap(), original_cursor);
    assert!(!directory.path().join(".agents/skills/sift").exists());
}

#[test]
fn uninstall_preflight_keeps_every_target_unchanged_on_conflict() {
    let directory = TempDir::new("sift-cli-agent-uninstall-preflight").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let skill_path = directory.path().join(".agents/skills/sift");
    skill::install(&skill_path).unwrap();
    let config_path = directory.path().join(".cursor/mcp.json");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(
        config_path,
        r#"{"mcpServers":{"sift":{"command":"custom-sift","args":["mcp"]}}}"#,
    )
    .unwrap();

    super::uninstall_environment(&environment).unwrap();

    assert!(skill_path.exists());
}
