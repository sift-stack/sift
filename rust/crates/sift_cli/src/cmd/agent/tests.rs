use std::{fs, path::PathBuf};

use clap::Parser;
use serde_json::Value;
use tempdir::TempDir;

use super::{
    AccessInference, AccessMode, Environment, Harness, Profile, ProfileInference, Registration,
    config, skill,
};

fn environment(harnesses: Vec<Harness>) -> (TempDir, Environment) {
    let directory = TempDir::new("sift-cli-agent-tests").unwrap();
    let current_exe = directory.path().join("bin").join("sift-cli");
    (
        directory,
        Environment::for_test(PathBuf::new(), current_exe, harnesses),
    )
}

fn default_registration(access: AccessMode) -> Registration {
    Registration::new(access, Profile::Default)
}

fn named_registration(access: AccessMode, profile: &str) -> Registration {
    Registration::new(access, Profile::Named(profile.to_string()))
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
        directory.path().join(".agents/skills/sift/SKILL.md")
    );
}

#[test]
fn skill_install_is_fresh_and_stateless() {
    let directory = TempDir::new("sift-cli-agent-skill").unwrap();
    let path = directory.path().join(".agents/skills/sift/SKILL.md");

    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Missing);
    skill::install(&path).unwrap();
    assert_eq!(skill::inspect(&path).unwrap(), skill::State::Current);

    fs::write(
        &path,
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
fn unmanaged_skill_is_never_removed() {
    let directory = TempDir::new("sift-cli-agent-skill-conflict").unwrap();
    let path = directory.path().join(".agents/skills/sift/SKILL.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "---\nname: sift\n---\nCustom instructions.\n").unwrap();

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

    config::install(
        Harness::Cursor,
        &environment,
        &default_registration(AccessMode::ReadOnly),
    )
    .unwrap();

    let value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(value["setting"], true);
    assert_eq!(value["mcpServers"]["other"]["command"], "other-server");
    assert_eq!(
        value["mcpServers"]["sift"]["command"],
        current_exe.to_string_lossy().as_ref()
    );
    assert_eq!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Current(default_registration(AccessMode::ReadOnly))
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

    config::install(
        Harness::OpenCode,
        &environment,
        &default_registration(AccessMode::ReadOnly),
    )
    .unwrap();

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
fn named_profile_is_installed_for_every_json_backed_client() {
    let directory = TempDir::new("sift-cli-agent-profile-install").unwrap();
    let current_exe = directory.path().join("bin/sift-cli");
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        current_exe,
        vec![Harness::Cursor, Harness::OpenCode],
    );
    let registration = named_registration(AccessMode::ReadOnly, "localdev");

    config::install(Harness::Cursor, &environment, &registration).unwrap();
    config::install(Harness::OpenCode, &environment, &registration).unwrap();

    let cursor: Value =
        serde_json::from_slice(&fs::read(directory.path().join(".cursor/mcp.json")).unwrap())
            .unwrap();
    assert_eq!(
        cursor["mcpServers"]["sift"]["args"],
        serde_json::json!(["mcp", "--profile", "localdev"])
    );

    let opencode: Value = serde_json::from_slice(
        &fs::read(directory.path().join(".config/opencode/opencode.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        opencode["mcp"]["sift"]["command"],
        serde_json::json!([
            directory
                .path()
                .join("bin/sift-cli")
                .to_string_lossy()
                .as_ref(),
            "mcp",
            "--profile",
            "localdev"
        ])
    );
    assert_eq!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Current(registration.clone())
    );
    assert_eq!(
        config::inspect(Harness::OpenCode, &environment).unwrap(),
        config::State::Current(registration)
    );
}

#[test]
fn profile_global_flag_is_accepted_after_agent_install() {
    let args =
        crate::cli::Args::try_parse_from(["sift-cli", "agent", "install", "--profile", "localdev"])
            .unwrap();

    assert_eq!(args.profile.as_deref(), Some("localdev"));
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

    super::install_environment(
        &environment,
        "Installed",
        &default_registration(AccessMode::ReadOnly),
    )
    .unwrap();

    assert!(
        !directory
            .path()
            .join(".agents/skills/sift/SKILL.md")
            .exists()
    );
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
    let source = directory.path().join("source.md");
    let path = directory.path().join(".agents/skills/sift/SKILL.md");
    fs::write(&source, skill::CONTENT).unwrap();
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
        config::State::Current(default_registration(AccessMode::Destructive))
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

    let registration = default_registration(AccessMode::Destructive);
    config::install(Harness::Cursor, &environment, &registration).unwrap();
    config::install(Harness::OpenCode, &environment, &registration).unwrap();

    assert_eq!(
        config::inspect(Harness::Cursor, &environment).unwrap(),
        config::State::Current(registration.clone())
    );
    assert_eq!(
        config::inspect(Harness::OpenCode, &environment).unwrap(),
        config::State::Current(registration)
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
fn update_profile_inference_preserves_one_profile_and_rejects_mixed_profiles() {
    assert_eq!(
        super::infer_profiles(&[]),
        ProfileInference::Resolved(Profile::Default)
    );
    assert_eq!(
        super::infer_profiles(&[
            Profile::Named("localdev".to_string()),
            Profile::Named("localdev".to_string()),
        ]),
        ProfileInference::Resolved(Profile::Named("localdev".to_string()))
    );
    assert_eq!(
        super::infer_profiles(&[Profile::Default, Profile::Named("localdev".to_string()),]),
        ProfileInference::Mixed
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
fn update_named_and_default_profile_flags_are_mutually_exclusive() {
    assert!(
        crate::cli::Args::try_parse_from([
            "sift-cli",
            "agent",
            "update",
            "--profile",
            "localdev",
            "--default-profile",
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
fn every_plugin_manifest_matches_the_cli_release() {
    let manifests = [
        include_str!("../../../plugins/sift/.codex-plugin/plugin.json"),
        include_str!("../../../plugins/sift/.claude-plugin/plugin.json"),
        include_str!("../../../plugins/sift/.cursor-plugin/plugin.json"),
    ];

    for manifest in manifests {
        let value: Value = serde_json::from_str(manifest).unwrap();
        assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));
    }
}

#[test]
fn codex_claude_and_cursor_package_the_same_mcp_command() {
    let codex_and_claude: Value =
        serde_json::from_str(include_str!("../../../plugins/sift/.mcp.json")).unwrap();
    let cursor: Value =
        serde_json::from_str(include_str!("../../../plugins/sift/mcp.json")).unwrap();

    assert_eq!(
        codex_and_claude["mcpServers"]["sift"],
        cursor["mcpServers"]["sift"]
    );
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

    super::install_environment(
        &environment,
        "Installed",
        &default_registration(AccessMode::ReadOnly),
    )
    .unwrap();

    assert!(
        !directory
            .path()
            .join(".agents/skills/sift/SKILL.md")
            .exists()
    );
}

#[test]
fn uninstall_preflight_keeps_every_target_unchanged_on_conflict() {
    let directory = TempDir::new("sift-cli-agent-uninstall-preflight").unwrap();
    let environment = Environment::for_test(
        directory.path().to_path_buf(),
        directory.path().join("bin/sift-cli"),
        vec![Harness::Cursor],
    );
    let skill_path = directory.path().join(".agents/skills/sift/SKILL.md");
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
