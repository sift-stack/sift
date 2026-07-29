use std::{
    collections::BTreeMap,
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use super::{Environment, Harness, files};

pub(super) const CONTENT: &str = include_str!("../../../plugins/sift/skills/sift/SKILL.md");
const MANAGED_MARKER: &str = "Managed by sift-cli.";
const LEGACY_MARKER: &str = "LOCKSTEP:";
const SIFT_HEADING: &str = "# Sift toolbox";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum State {
    Missing,
    Current,
    ManagedOutdated,
    Conflict,
}

#[derive(Debug)]
pub(super) struct Target {
    pub path: PathBuf,
    pub harnesses: Vec<Harness>,
}

pub(super) fn targets(environment: &Environment) -> Vec<Target> {
    let mut by_path = BTreeMap::<PathBuf, Vec<Harness>>::new();

    for harness in &environment.harnesses {
        let path = match harness {
            Harness::Claude => environment
                .home
                .join(".claude")
                .join("skills")
                .join("sift")
                .join("SKILL.md"),
            Harness::Codex | Harness::Cursor | Harness::OpenCode | Harness::Pi => environment
                .home
                .join(".agents")
                .join("skills")
                .join("sift")
                .join("SKILL.md"),
        };
        by_path.entry(path).or_default().push(*harness);
    }

    by_path
        .into_iter()
        .map(|(path, harnesses)| Target { path, harnesses })
        .collect()
}

pub(super) fn inspect(path: &Path) -> Result<State> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => return Ok(State::Conflict),
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(State::Missing),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to inspect {}", path.display()));
        }
    }

    let installed = match fs::read_to_string(path) {
        Ok(installed) => installed,
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", path.display()));
        }
    };

    if installed == CONTENT {
        Ok(State::Current)
    } else if is_managed(&installed) {
        Ok(State::ManagedOutdated)
    } else {
        Ok(State::Conflict)
    }
}

pub(super) fn install(path: &Path) -> Result<()> {
    files::write_atomic(path, CONTENT.as_bytes())
}

pub(super) fn uninstall(path: &Path) -> Result<bool> {
    match inspect(path)? {
        State::Missing => Ok(false),
        State::Current | State::ManagedOutdated => {
            fs::remove_file(path)
                .with_context(|| format!("failed to remove {}", path.display()))?;
            files::remove_empty_parent(path);
            Ok(true)
        }
        State::Conflict => Ok(false),
    }
}

fn is_managed(contents: &str) -> bool {
    contents.contains(SIFT_HEADING)
        && (contents.contains(MANAGED_MARKER) || contents.contains(LEGACY_MARKER))
}
