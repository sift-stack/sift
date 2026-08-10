use std::{
    io::{self, IsTerminal},
    process::ExitCode,
};

use anyhow::{Error, Result};
use semver::Version;
use sift_rs::Credentials;
use tokio::sync::watch;
use tracing_subscriber::EnvFilter;

use crate::cli::McpArgs;
use crate::cmd::{Context, version};
use crate::util::tty::Output;

pub async fn report_startup_error(error: Error) -> Result<ExitCode> {
    let message = format!("{error:#}");
    Output::new().line(&message).eprint();
    if io::stdin().is_terminal() {
        return Ok(ExitCode::FAILURE);
    }
    sift_mcp::report_startup_error(message).await?;
    Ok(ExitCode::FAILURE)
}

pub async fn run(ctx: Context, args: McpArgs, app_uri: String) -> Result<ExitCode> {
    // stdout carries the MCP protocol, so every diagnostic goes to stderr,
    // which MCP clients capture in their logs. Without a subscriber the
    // server runs silent and an unexpected death leaves no evidence behind.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        uri = %ctx.grpc_uri,
        "starting Sift MCP server"
    );

    let update_check = select_update_check(args.disable_update_check, start_update_check);

    let credentials = Credentials::Config {
        uri: ctx.grpc_uri,
        apikey: ctx.api_key,
    };
    match sift_mcp::run_with_update_check(
        credentials,
        !ctx.disable_tls,
        app_uri,
        args.allow_create,
        args.allow_destructive,
        env!("CARGO_PKG_VERSION").to_string(),
        update_check,
    )
    .await
    {
        Ok(_) => {
            tracing::info!("Sift MCP server exited cleanly");
            Ok(ExitCode::SUCCESS)
        }
        Err(err) => {
            tracing::error!(error = ?err, "Sift MCP server terminated");
            Err(err)
        }
    }
}

fn select_update_check<F>(
    disable_update_check: bool,
    start: F,
) -> Option<sift_mcp::UpdateCheckReceiver>
where
    F: FnOnce() -> sift_mcp::UpdateCheckReceiver,
{
    (!disable_update_check).then(start)
}

fn start_update_check() -> sift_mcp::UpdateCheckReceiver {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    let current = match Version::parse(&current_version) {
        Ok(current) => current,
        Err(error) => {
            tracing::debug!(%error, "could not parse the installed sift-cli version");
            return ready_update_check(sift_mcp::UpdateCheck::Unavailable {
                current_version,
                message: "Could not parse the installed sift-cli version. Run `sift-cli --version` for details."
                    .to_string(),
            });
        }
    };

    if let Some(cached) = version::read_release_cache(&current) {
        let update_check = if cached.is_outdated {
            update_check_from_versions(&current, &cached.latest_version)
        } else {
            sift_mcp::UpdateCheck::Current {
                current_version,
                latest_version: cached.latest_version.to_string(),
            }
        };
        log_update_notice(&update_check);
        return ready_update_check(update_check);
    }

    let (sender, receiver) = watch::channel(sift_mcp::UpdateCheck::Checking {
        current_version: current_version.clone(),
    });
    tokio::spawn(async move {
        let update_check = match version::fetch_latest().await {
            Ok(Some(latest)) => update_check_from_versions(&current, &latest),
            Ok(None) => sift_mcp::UpdateCheck::Unavailable {
                current_version,
                message:
                    "No stable sift-cli release was found. Run `sift-cli --version` for details."
                        .to_string(),
            },
            Err(error) => {
                tracing::debug!(%error, "could not check for a newer sift-cli release");
                sift_mcp::UpdateCheck::Unavailable {
                    current_version,
                    message: "Could not check for a newer sift-cli release. Run `sift-cli --version` for details."
                        .to_string(),
                }
            }
        };
        log_update_notice(&update_check);
        if sender.send(update_check).is_err() {
            tracing::debug!("the MCP server closed before the update check completed");
        }
    });
    receiver
}

fn update_check_from_versions(current: &Version, latest: &Version) -> sift_mcp::UpdateCheck {
    match version::outdated_warning(current, latest) {
        Some(message) => sift_mcp::UpdateCheck::UpdateAvailable {
            current_version: current.to_string(),
            latest_version: latest.to_string(),
            install_command: version::install_command(latest),
            message,
        },
        None => sift_mcp::UpdateCheck::Current {
            current_version: current.to_string(),
            latest_version: latest.to_string(),
        },
    }
}

fn log_update_notice(update_check: &sift_mcp::UpdateCheck) {
    if let sift_mcp::UpdateCheck::UpdateAvailable { message, .. } = update_check {
        tracing::warn!("{message}");
    }
}

fn ready_update_check(update_check: sift_mcp::UpdateCheck) -> sift_mcp::UpdateCheckReceiver {
    watch::channel(update_check).1
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use clap::Parser;
    use semver::Version;
    use tokio::sync::watch;

    use super::{select_update_check, update_check_from_versions};

    #[test]
    fn disable_update_check_flag_is_accepted() {
        let args = crate::cli::Args::try_parse_from(["sift-cli", "mcp", "--disable-update-check"])
            .unwrap();
        let Some(crate::cli::Cmd::Mcp(args)) = args.cmd else {
            panic!("expected the MCP command");
        };

        assert!(args.disable_update_check);
    }

    #[test]
    fn disabled_update_check_does_not_start_the_check() {
        let called = Cell::new(false);

        let update_check = select_update_check(true, || {
            called.set(true);
            watch::channel(sift_mcp::UpdateCheck::Checking {
                current_version: "0.4.0".to_string(),
            })
            .1
        });

        assert!(update_check.is_none());
        assert!(!called.get());
    }

    #[test]
    fn update_check_contains_the_exact_install_command() {
        let current = Version::parse("0.3.0").unwrap();
        let latest = Version::parse("0.4.0").unwrap();

        let update_check = update_check_from_versions(&current, &latest);

        match update_check {
            sift_mcp::UpdateCheck::UpdateAvailable {
                current_version,
                latest_version,
                install_command,
                message,
            } => {
                assert_eq!(current_version, "0.3.0");
                assert_eq!(latest_version, "0.4.0");
                assert!(message.ends_with(&format!("  {install_command}")));
                assert!(install_command.contains("sift_cli-v0.4.0/sift_cli-installer"));
            }
            update_check => panic!("expected an available update, got {update_check:?}"),
        }
    }

    #[test]
    fn update_check_reports_a_current_binary() {
        let current = Version::parse("0.4.0").unwrap();
        let latest = Version::parse("0.4.0").unwrap();

        let update_check = update_check_from_versions(&current, &latest);

        assert!(matches!(
            update_check,
            sift_mcp::UpdateCheck::Current { .. }
        ));
    }
}
