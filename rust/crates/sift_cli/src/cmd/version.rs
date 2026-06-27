use std::{process::ExitCode, time::Duration};

use anyhow::Result;
use crossterm::style::Stylize;
use reqwest::ClientBuilder;
use semver::Version;
use serde::Deserialize;

use crate::util::tty::Output;

const RELEASES_URL: &str = "https://api.github.com/repos/sift-stack/sift/releases?per_page=100";
const TAG_PREFIX: &str = "sift_cli-v";
const USER_AGENT: &str = concat!("sift-cli/", env!("CARGO_PKG_VERSION"));

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
}

pub async fn run() -> Result<ExitCode> {
    let current_str = env!("CARGO_PKG_VERSION");
    let current = Version::parse(current_str)?;

    let mut out = Output::new();
    out.line(format!("sift-cli {}", current_str.bold()));

    match fetch_latest().await {
        Ok(Some(latest)) => {
            out.line(format!("Latest release: {}", latest.to_string().bold()));
            if latest > current {
                out.line(format!(
                    "{}: {} → {}",
                    "Update available".green(),
                    current_str.yellow(),
                    latest.to_string().green(),
                ));
                out.tip("run `sift-cli update` to install the latest release");
            } else {
                out.line("You're on the latest release.".to_string());
            }
        }
        Ok(None) => {
            out.line(format!(
                "{}: no `{TAG_PREFIX}*` releases found on GitHub",
                "warning".yellow(),
            ));
        }
        Err(err) => {
            out.line(format!(
                "{}: unable to check for updates ({err})",
                "warning".yellow(),
            ));
        }
    }

    out.print();
    Ok(ExitCode::SUCCESS)
}

async fn fetch_latest() -> Result<Option<Version>> {
    let client = ClientBuilder::new()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(5))
        .build()?;

    let releases: Vec<GithubRelease> = client
        .get(RELEASES_URL)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(releases
        .into_iter()
        .filter_map(|r| r.tag_name.strip_prefix(TAG_PREFIX).map(str::to_string))
        .filter_map(|v| Version::parse(&v).ok())
        .max())
}
