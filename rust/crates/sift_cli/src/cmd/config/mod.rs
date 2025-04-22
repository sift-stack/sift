use anyhow::{Context, Result, format_err};
use crossterm::style::{StyledContent, Stylize};
use sift_connect::grpc::config::sift_config;
use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::Path,
};

pub mod create;
pub mod show;

#[cfg(test)]
mod show_test;

#[cfg(test)]
mod create_test;

/// Create a config file if it doesn't exist and return `None` for the first tuple item; otherwise
/// reads an existing config to a string and returns the text content as the first tuple item.
fn maybe_create_then_read_config_to_string<P: AsRef<Path>>(
    config: Option<P>,
) -> Result<(Option<String>, StyledContent<String>)> {
    let sift_config_file = match config {
        Some(p) => p.as_ref().to_path_buf(),
        None => sift_config()
            .ok_or_else(|| format_err!("failed to find user's local data directory"))?,
    };
    let config_display = sift_config_file.display().to_string().bold().cyan();

    let content = match fs::read_to_string(&sift_config_file) {
        Err(err) if err.kind() == ErrorKind::NotFound => {
            File::create(&sift_config_file).with_context(|| "failed to create {config_display}")?;

            return Ok((None, config_display));
        }
        Err(err) => return Err(err).context("something went wrong loading config"),
        Ok(data) => Some(data),
    };

    Ok((content, config_display))
}

/// Read contents of the Sift config file to a string.
fn read_config_to_string<P: AsRef<Path>>(
    config: Option<P>,
) -> Result<(String, StyledContent<String>)> {
    let sift_config_file = match config {
        Some(p) => p.as_ref().to_path_buf(),
        None => sift_config()
            .ok_or_else(|| format_err!("failed to find user's local data directory"))?,
    };
    let config_display = sift_config_file.display().to_string().bold().cyan();

    let content = fs::read_to_string(&sift_config_file)
        .with_context(|| format!("failed to open {}", config_display))?;

    Ok((content, config_display))
}

/// Create or truncate-and-update the contents of the Sift config file.
fn write_to_config<P: AsRef<Path>>(config: Option<P>, content: &[u8]) -> Result<()> {
    let sift_config_file = match config {
        Some(p) => p.as_ref().to_path_buf(),
        None => sift_config()
            .ok_or_else(|| format_err!("failed to find user's local data directory"))?,
    };
    let config_display = sift_config_file.display().to_string().bold().cyan();

    let mut config = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&sift_config_file)
        .with_context(|| format!("failed to open {}", config_display))?;

    let _ = config
        .write(content)
        .context("failed to update config file")?;

    Ok(())
}
