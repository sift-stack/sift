use crate::Clargs;
use anyhow::{Result, format_err};
use clap::{CommandFactory, crate_name};
use clap_complete::Shell;
use std::io::Write;

/// Generate autocompletions for various shells.
pub fn run<W: Write>(mut out: W, shell: Option<Shell>) -> Result<()> {
    let shell = match shell {
        Some(val) => val,
        None => Shell::from_env()
            .ok_or_else(|| format_err!("failed to read 'SHELL' environment variable"))?,
    };
    clap_complete::generate(shell, &mut Clargs::command(), crate_name!(), &mut out);
    Ok(())
}
