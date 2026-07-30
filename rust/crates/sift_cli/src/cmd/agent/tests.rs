use std::{fs, path::PathBuf};

use clap::Parser;
use tempdir::TempDir;

use super::{AccessInference, AccessMode, Environment, Harness, skill};

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
fn claude_and_codex_install_the_skill_to_their_own_conventions() {
    let (directory, mut environment) = environment(vec![Harness::Claude, Harness::Codex]);
    environment.home = directory.path().to_path_buf();

    let targets = skill::targets(&environment);

    assert_eq!(targets.len(), 2);
    let paths = targets
        .iter()
        .map(|target| target.path.clone())
        .collect::<Vec<_>>();
    assert!(paths.contains(&directory.path().join(".claude/skills/sift")));
    assert!(paths.contains(&directory.path().join(".agents/skills/sift")));
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
