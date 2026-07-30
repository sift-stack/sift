use std::{fs, path::Path};

pub(super) fn remove_empty_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir(parent);
    }
}
