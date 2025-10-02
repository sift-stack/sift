pub mod disk;
pub use disk::DiskBackupPolicy;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
enum Message<T> {
    /// Data to be backed up.
    Data(T),
    /// Graceful termination; cleans up the backup file.
    Complete,
    /// Force the backup task to flush its contents to the target data container.
    Flush,
}

/// Sanitize a name by replacing illegal characters with underscores.
pub(crate) fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            ':' | '/' | '\\' | '*' | '?' | '"' | '<' | '>' | '|' | '.' => '_',
            _ => {
                if c.is_whitespace() {
                    '_'
                } else {
                    c
                }
            }
        })
        .collect()
}
