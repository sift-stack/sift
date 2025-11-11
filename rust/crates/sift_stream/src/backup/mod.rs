pub mod disk;
pub use disk::DiskBackupPolicy;

#[cfg(test)]
mod test;

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
