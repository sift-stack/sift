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
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
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
                out.tip(format!("`{}`", install_command(&latest)).cyan().to_string());
            } else if current_str.contains("alpha") {
                out.line("You are on an alpha release.".to_string());
                out.tip(format!("`{}`", install_command(&latest)).cyan().to_string());
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

pub(crate) async fn fetch_latest() -> Result<Option<Version>> {
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

    Ok(latest_stable(releases))
}

fn latest_stable(releases: Vec<GithubRelease>) -> Option<Version> {
    releases
        .into_iter()
        .filter(|release| !release.draft && !release.prerelease)
        .filter_map(|r| r.tag_name.strip_prefix(TAG_PREFIX).map(str::to_string))
        .filter_map(|v| Version::parse(&v).ok())
        .filter(|version| version.pre.is_empty())
        .max()
}

pub(crate) fn install_command(latest: &Version) -> String {
    let (asset, cmd_tmpl) = if cfg!(windows) {
        (
            "sift_cli-installer.ps1",
            "powershell -ExecutionPolicy ByPass -c \"irm {url} | iex\"",
        )
    } else {
        (
            "sift_cli-installer.sh",
            "curl --proto '=https' --tlsv1.2 -LsSf {url} | sh",
        )
    };

    let url = format!(
        "https://github.com/sift-stack/sift/releases/download/{TAG_PREFIX}{latest}/{asset}"
    );
    cmd_tmpl.replace("{url}", &url)
}

#[cfg(test)]
mod tests {
    use super::{GithubRelease, latest_stable};
    use semver::Version;

    #[test]
    fn latest_release_excludes_drafts_and_prereleases() {
        let releases = vec![
            GithubRelease {
                tag_name: "sift_cli-v0.4.0-alpha.1".to_string(),
                draft: false,
                prerelease: true,
            },
            GithubRelease {
                tag_name: "sift_cli-v0.3.1".to_string(),
                draft: true,
                prerelease: false,
            },
            GithubRelease {
                tag_name: "sift_cli-v0.3.0".to_string(),
                draft: false,
                prerelease: false,
            },
        ];

        assert_eq!(
            latest_stable(releases),
            Some(Version::parse("0.3.0").unwrap())
        );
    }
}
