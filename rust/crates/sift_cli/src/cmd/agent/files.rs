use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
};

use anyhow::{Context, Result};

pub(super) fn write_atomic(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| anyhow::anyhow!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("sift-agent-config");
    let temporary = parent.join(format!(".{file_name}.sift-cli-{}.tmp", std::process::id()));

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)
        .with_context(|| format!("failed to create {}", temporary.display()))?;
    if let Ok(metadata) = fs::metadata(path)
        && let Err(error) = fs::set_permissions(&temporary, metadata.permissions())
    {
        let _ = fs::remove_file(&temporary);
        return Err(error)
            .with_context(|| format!("failed to preserve permissions for {}", path.display()));
    }
    if let Err(error) = file.write_all(contents).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(&temporary);
        return Err(error)
            .with_context(|| format!("failed to write temporary file {}", temporary.display()));
    }

    if let Err(error) = fs::rename(&temporary, path) {
        if error.kind() == ErrorKind::AlreadyExists || (cfg!(windows) && path.exists()) {
            fs::remove_file(path)
                .with_context(|| format!("failed to replace {}", path.display()))?;
            fs::rename(&temporary, path)
                .with_context(|| format!("failed to replace {}", path.display()))?;
        } else {
            let _ = fs::remove_file(&temporary);
            return Err(error).with_context(|| {
                format!(
                    "failed to move {} into place at {}",
                    temporary.display(),
                    path.display()
                )
            });
        }
    }

    Ok(())
}

pub(super) fn remove_empty_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir(parent);
    }
}
