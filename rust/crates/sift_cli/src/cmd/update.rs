use std::{
    io::{self, Write},
    process::ExitCode,
};

use anyhow::{Context as _, Result};
use axoupdater::{AxoUpdater, UpdateRequest, Version};
use crossterm::style::Stylize;

use crate::cli::UpdateArgs;
use crate::cmd::config::get_config_file_path;
use crate::util::tty::Output;

pub async fn run(args: UpdateArgs) -> Result<ExitCode> {
    let UpdateArgs {
        version,
        pre,
        check,
        yes,
    } = args;

    let mut updater = AxoUpdater::new_for("sift_cli");

    if updater.load_receipt().is_err() {
        Output::new()
            .line("sift-cli wasn't installed by the official installer.".to_string())
            .tip(
                "re-run the curl installer to update: \
                 https://github.com/sift-stack/sift/releases/latest",
            )
            .eprint();
        return Ok(ExitCode::FAILURE);
    }

    updater.disable_installer_output();

    let current = Version::parse(env!("CARGO_PKG_VERSION"))
        .context("invalid CARGO_PKG_VERSION baked into binary")?;
    let current_is_prerelease = !current.pre.is_empty();

    let specifier = match version.as_deref() {
        Some(v) => {
            v.parse::<Version>()
                .context("--version must be a valid semver string like 0.3.0")?;
            UpdateRequest::SpecificVersion(v.to_string())
        }
        None if pre || current_is_prerelease => UpdateRequest::LatestMaybePrerelease,
        None => UpdateRequest::Latest,
    };
    updater.configure_version_specifier(specifier);

    let target = updater
        .query_new_version()
        .await
        .context("failed to query latest release from GitHub")?
        .cloned();

    let Some(target) = target else {
        Output::new()
            .line(format!(
                "sift-cli {} is already the latest release.",
                current.to_string().bold(),
            ))
            .print();
        return Ok(ExitCode::SUCCESS);
    };

    if target == current {
        Output::new()
            .line(format!(
                "sift-cli {} is already at the requested version.",
                current.to_string().bold(),
            ))
            .print();
        return Ok(ExitCode::SUCCESS);
    }

    if target < current && version.is_none() {
        Output::new()
            .line(format!(
                "Refusing to downgrade: latest release ({}) is older than installed ({}).",
                target.to_string().yellow(),
                current.to_string().bold(),
            ))
            .tip("pass `--version <X>` to pin a specific version, or `--pre` to include prereleases")
            .eprint();
        return Ok(ExitCode::FAILURE);
    }

    let arrow = if target < current { "↓" } else { "→" };
    println!(
        "Update: {} {arrow} {}",
        current.to_string().yellow(),
        target.to_string().green(),
    );

    if check {
        println!("({} passed — no changes applied)", "--check".bold());
        return Ok(ExitCode::SUCCESS);
    }

    if !yes && !confirm()? {
        println!("Update aborted.");
        return Ok(ExitCode::SUCCESS);
    }

    let result = updater
        .run()
        .await
        .context("update failed")?;

    let mut out = Output::new();
    match result {
        Some(outcome) => {
            out.line(format!(
                "Updated sift-cli to {}",
                outcome.new_version.to_string().green(),
            ));
        }
        None => {
            out.line("Update completed.".to_string());
        }
    }
    if let Ok(path) = get_config_file_path() {
        out.line(format!("Your config at {} is unchanged.", path.display()));
    }
    out.print();

    Ok(ExitCode::SUCCESS)
}

fn confirm() -> Result<bool> {
    print!("Proceed? [y/N] ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(matches!(
        buf.trim().to_lowercase().as_str(),
        "y" | "yes"
    ))
}
