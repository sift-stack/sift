use anyhow::{Context, Result};
use std::{io::Write, path::Path};

/// Print the path and contents of the Sift config file or error. The `config` should be `None`
/// for the release version - it mainly exists for testing purposes.
pub fn run<W, P>(mut out: W, config: Option<P>) -> Result<()>
where
    W: Write,
    P: AsRef<Path>,
{
    let (content, config_display) = super::read_config_to_string(config)?;

    writeln!(out, "{content}\n{config_display}")
        .context("failed to write output for config::show")?;

    Ok(())
}
