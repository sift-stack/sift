use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use include_dir::{Dir, include_dir};

use super::{Environment, Harness, files};

#[cfg(test)]
pub(super) const CONTENT: &str = include_str!("../../../assets/skills/sift/SKILL.md");
#[cfg(test)]
pub(super) const REFERENCE_DIR: &str = "references";
static BUNDLE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/skills/sift");
const MANAGED_MARKER: &str = "Managed by sift-cli.";
const LEGACY_MARKER: &str = "LOCKSTEP:";
const SIFT_HEADING: &str = "# Sift toolbox";
/// Backup names carry the PID, so a live run only collides with its own targets.
/// Extra indices are orphans a killed run never got to delete, so bound the probe
/// and fail loudly instead of scanning a littered directory forever.
const MAX_BACKUP_CANDIDATES: u32 = 100;

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

#[derive(Debug)]
pub(super) struct Replacement {
    path: PathBuf,
    backup: Option<PathBuf>,
}

impl Replacement {
    pub(super) fn commit(self) -> Result<()> {
        if let Some(backup) = self.backup {
            fs::remove_dir_all(&backup)
                .with_context(|| format!("failed to remove {}", backup.display()))?;
        }
        Ok(())
    }

    pub(super) fn rollback(self) -> Result<()> {
        remove_path(&self.path)?;
        if let Some(backup) = self.backup {
            fs::rename(&backup, &self.path).with_context(|| {
                format!(
                    "failed to restore {} from {}",
                    self.path.display(),
                    backup.display()
                )
            })?;
        }
        Ok(())
    }
}

/// Reference file names carried in the bundle, without the `references/` prefix.
#[cfg(test)]
pub(super) fn bundled_references() -> Vec<String> {
    BUNDLE
        .get_dir(REFERENCE_DIR)
        .into_iter()
        .flat_map(Dir::files)
        .filter_map(|file| file.path().file_name()?.to_str().map(str::to_string))
        .collect()
}

pub(super) fn targets(environment: &Environment) -> Vec<Target> {
    let mut by_path = BTreeMap::<PathBuf, Vec<Harness>>::new();

    for harness in &environment.harnesses {
        let path = match harness {
            Harness::Claude => environment.home.join(".claude").join("skills").join("sift"),
            Harness::Codex | Harness::Cursor | Harness::OpenCode => {
                environment.home.join(".agents").join("skills").join("sift")
            }
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
        Ok(metadata) if !metadata.is_dir() => return Ok(State::Conflict),
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(State::Missing),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to inspect {}", path.display()));
        }
    }

    if directory_matches(&BUNDLE, path)? {
        return Ok(State::Current);
    }

    let skill_path = path.join("SKILL.md");
    let installed = match fs::read_to_string(&skill_path) {
        Ok(installed) => installed,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(State::Conflict),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", skill_path.display()));
        }
    };

    if is_managed(&installed) {
        Ok(State::ManagedOutdated)
    } else {
        Ok(State::Conflict)
    }
}

#[cfg(test)]
pub(super) fn install(path: &Path) -> Result<()> {
    begin_install(path)?.commit()
}

pub(super) fn begin_install(path: &Path) -> Result<Replacement> {
    let backup = match inspect(path)? {
        State::Conflict => {
            return Err(anyhow!(
                "{} is not a managed skill directory",
                path.display()
            ));
        }
        State::Missing => None,
        State::Current | State::ManagedOutdated => {
            let backup = unused_backup_path(path)?;
            fs::rename(path, &backup).with_context(|| {
                format!("failed to move {} to {}", path.display(), backup.display())
            })?;
            Some(backup)
        }
    };

    if let Err(error) = copy_bundle(&BUNDLE, path) {
        let replacement = Replacement {
            path: path.to_path_buf(),
            backup,
        };
        return match replacement.rollback() {
            Ok(()) => Err(error.context("the previous skill directory was restored")),
            Err(rollback_error) => Err(anyhow!(
                "{error:#}; restoring the previous skill directory also failed: {rollback_error:#}"
            )),
        };
    }
    Ok(Replacement {
        path: path.to_path_buf(),
        backup,
    })
}

pub(super) fn uninstall(path: &Path) -> Result<bool> {
    match inspect(path)? {
        State::Missing => Ok(false),
        State::Current | State::ManagedOutdated => {
            fs::remove_dir_all(path)
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

fn directory_matches(bundle: &Dir<'_>, installed: &Path) -> Result<bool> {
    let actual_entries = fs::read_dir(installed)
        .with_context(|| format!("failed to read {}", installed.display()))?
        .map(|entry| entry.map(|entry| entry.file_name()))
        .collect::<std::io::Result<BTreeSet<OsString>>>()
        .with_context(|| format!("failed to read {}", installed.display()))?;
    let expected_entries = bundle
        .files()
        .filter_map(|file| file.path().file_name().map(OsString::from))
        .chain(
            bundle
                .dirs()
                .filter_map(|directory| directory.path().file_name().map(OsString::from)),
        )
        .collect::<BTreeSet<_>>();
    if actual_entries != expected_entries {
        return Ok(false);
    }

    for file in bundle.files() {
        let name = file
            .path()
            .file_name()
            .ok_or_else(|| anyhow!("embedded skill file has no name"))?;
        let target = installed.join(name);
        let metadata = fs::symlink_metadata(&target)
            .with_context(|| format!("failed to inspect {}", target.display()))?;
        if metadata.file_type().is_symlink()
            || !metadata.is_file()
            || fs::read(&target).with_context(|| format!("failed to read {}", target.display()))?
                != file.contents()
        {
            return Ok(false);
        }
    }

    for directory in bundle.dirs() {
        let name = directory
            .path()
            .file_name()
            .ok_or_else(|| anyhow!("embedded skill directory has no name"))?;
        let target = installed.join(name);
        let metadata = fs::symlink_metadata(&target)
            .with_context(|| format!("failed to inspect {}", target.display()))?;
        if metadata.file_type().is_symlink()
            || !metadata.is_dir()
            || !directory_matches(directory, &target)?
        {
            return Ok(false);
        }
    }

    Ok(true)
}

fn copy_bundle(bundle: &Dir<'_>, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create {}", destination.display()))?;

    for directory in bundle.dirs() {
        let name = directory
            .path()
            .file_name()
            .ok_or_else(|| anyhow!("embedded skill directory has no name"))?;
        copy_bundle(directory, &destination.join(name))?;
    }
    for file in bundle.files() {
        let name = file
            .path()
            .file_name()
            .ok_or_else(|| anyhow!("embedded skill file has no name"))?;
        let target = destination.join(name);
        fs::write(&target, file.contents())
            .with_context(|| format!("failed to write {}", target.display()))?;
    }

    Ok(())
}

fn unused_backup_path(path: &Path) -> Result<PathBuf> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| anyhow!("{} has no parent directory", path.display()))?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("sift");
    for index in 0..MAX_BACKUP_CANDIDATES {
        let candidate = parent.join(format!(
            ".{name}.sift-cli-backup-{}-{index}",
            std::process::id()
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(anyhow!(
        "failed to find an unused backup path beside {}",
        path.display()
    ))
}

fn remove_path(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
            fs::remove_dir_all(path).with_context(|| format!("failed to remove {}", path.display()))
        }
        Ok(_) => {
            fs::remove_file(path).with_context(|| format!("failed to remove {}", path.display()))
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("failed to inspect {}", path.display())),
    }
}
