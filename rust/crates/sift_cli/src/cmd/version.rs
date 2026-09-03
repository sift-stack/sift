use std::{
    fs,
    future::Future,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow};
use crossterm::style::Stylize;
use reqwest::ClientBuilder;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::util::tty::Output;

const RELEASES_URL: &str = "https://api.github.com/repos/sift-stack/sift/releases?per_page=100";
const TAG_PREFIX: &str = "sift_cli-v";
const USER_AGENT: &str = concat!("sift-cli/", env!("CARGO_PKG_VERSION"));
const RELEASE_CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const RELEASE_FAILURE_CACHE_TTL: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CachedRelease {
    Latest(Version),
    Unavailable,
}

#[derive(Deserialize, Serialize)]
struct ReleaseCache {
    latest_version: Option<String>,
    checked_at: u64,
}

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

    match latest_with_cache().await {
        Ok(Some(latest)) => {
            out.line(format!("Latest release: {}", latest.to_string().bold()));
            if latest > current {
                out.line(format!(
                    "{}: {} → {}",
                    "Update available".green(),
                    current_str.yellow(),
                    latest.to_string().green(),
                ));
                out.tip(format!("`{}`", install_command()).cyan().to_string());
            } else if current_str.contains("alpha") {
                out.line("You are on an alpha release.".to_string());
                out.tip(format!("`{}`", install_command()).cyan().to_string());
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
    let result = async {
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
    .await;

    let latest = result.as_ref().ok().and_then(|latest| latest.as_ref());
    if let Err(error) = write_release_cache(latest) {
        tracing::debug!(%error, "could not update the sift-cli release cache");
    }

    result
}

pub(crate) async fn latest_with_cache() -> Result<Option<Version>> {
    let path = release_cache_path();
    cached_or_fetch_latest_at(path.as_deref(), SystemTime::now(), fetch_latest).await
}

async fn cached_or_fetch_latest_at<F, Fut>(
    path: Option<&Path>,
    now: SystemTime,
    refresh: F,
) -> Result<Option<Version>>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Option<Version>>>,
{
    match path.and_then(|path| read_release_cache_at(path, now)) {
        Some(CachedRelease::Latest(latest)) => return Ok(Some(latest)),
        Some(CachedRelease::Unavailable) => {
            return Err(anyhow!("a recent release check failed"));
        }
        None => {}
    }
    refresh().await
}

pub(crate) fn read_release_cache() -> Option<CachedRelease> {
    let path = release_cache_path()?;
    read_release_cache_at(&path, SystemTime::now())
}

fn release_cache_path() -> Option<PathBuf> {
    dirs::cache_dir().map(|path| path.join("sift-cli").join("latest-release.json"))
}

fn read_release_cache_at(path: &Path, now: SystemTime) -> Option<CachedRelease> {
    let cache: ReleaseCache = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
    let checked_at = UNIX_EPOCH.checked_add(Duration::from_secs(cache.checked_at))?;
    let age = now.duration_since(checked_at).ok()?;
    let ttl = if cache.latest_version.is_some() {
        RELEASE_CACHE_TTL
    } else {
        RELEASE_FAILURE_CACHE_TTL
    };
    if age > ttl {
        return None;
    }

    match cache.latest_version {
        Some(latest_version) => Version::parse(&latest_version)
            .ok()
            .map(CachedRelease::Latest),
        None => Some(CachedRelease::Unavailable),
    }
}

fn write_release_cache(latest: Option<&Version>) -> Result<()> {
    let path =
        release_cache_path().ok_or_else(|| anyhow!("the platform has no cache directory"))?;
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("the release cache path has no parent"))?;
    fs::create_dir_all(parent).context("failed to create the sift-cli cache directory")?;
    let checked_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("the system clock is before the Unix epoch")?
        .as_secs();
    let cache = ReleaseCache {
        latest_version: latest.map(ToString::to_string),
        checked_at,
    };
    let contents = serde_json::to_vec(&cache).context("failed to serialize the release cache")?;
    fs::write(path, contents).context("failed to write the release cache")
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

pub(crate) fn install_command() -> String {
    if cfg!(windows) {
        r#"powershell -c "irm https://api.siftstack.com/install/sift-cli.ps1 | iex""#.to_string()
    } else {
        "curl -fsSL https://api.siftstack.com/install/sift-cli | sh".to_string()
    }
}

pub(crate) fn outdated_warning(current: &Version, latest: &Version) -> Option<String> {
    (latest > current).then(|| {
        format!(
            "sift-cli {current} is outdated; latest is {latest}\nUpdate with:\n\n  {}",
            install_command()
        )
    })
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::{
        fs,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use semver::Version;
    use serde_json::json;
    use tempdir::TempDir;

    use super::{
        CachedRelease, GithubRelease, RELEASE_CACHE_TTL, RELEASE_FAILURE_CACHE_TTL,
        cached_or_fetch_latest_at, install_command, latest_stable, outdated_warning,
        read_release_cache_at,
    };

    fn write_cache(path: &std::path::Path, latest_version: Option<&str>, checked_at: SystemTime) {
        let checked_at = checked_at.duration_since(UNIX_EPOCH).unwrap().as_secs();
        fs::write(
            path,
            json!({
                "latest_version": latest_version,
                "checked_at": checked_at,
            })
            .to_string(),
        )
        .unwrap();
    }

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

    #[test]
    fn fresh_release_cache_returns_the_latest_release() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(&path, Some("0.4.0"), now - Duration::from_secs(60));

        let cached = read_release_cache_at(&path, now).unwrap();

        assert_eq!(
            cached,
            CachedRelease::Latest(Version::parse("0.4.0").unwrap())
        );
    }

    #[tokio::test]
    async fn fresh_release_cache_skips_the_refresh() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(&path, Some("0.4.0"), now - Duration::from_secs(60));
        let refreshed = Cell::new(false);

        let latest = cached_or_fetch_latest_at(Some(&path), now, || async {
            refreshed.set(true);
            Ok(None)
        })
        .await
        .unwrap();

        assert_eq!(latest, Some(Version::parse("0.4.0").unwrap()));
        assert!(!refreshed.get());
    }

    #[tokio::test]
    async fn expired_release_cache_runs_the_refresh() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(
            &path,
            Some("0.3.0"),
            now - RELEASE_CACHE_TTL - Duration::from_secs(1),
        );
        let refreshed = Cell::new(false);

        let latest = cached_or_fetch_latest_at(Some(&path), now, || async {
            refreshed.set(true);
            Ok(Some(Version::parse("0.4.0").unwrap()))
        })
        .await
        .unwrap();

        assert_eq!(latest, Some(Version::parse("0.4.0").unwrap()));
        assert!(refreshed.get());
    }

    #[test]
    fn expired_release_cache_is_unknown() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(
            &path,
            Some("0.4.0"),
            now - RELEASE_CACHE_TTL - Duration::from_secs(1),
        );

        assert!(read_release_cache_at(&path, now).is_none());
    }

    #[test]
    fn corrupt_release_cache_is_unknown() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        fs::write(&path, "not json").unwrap();

        assert!(read_release_cache_at(&path, SystemTime::now()).is_none());
    }

    #[test]
    fn fresh_release_cache_returns_an_older_release() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(&path, Some("0.3.0"), now);

        let cached = read_release_cache_at(&path, now).unwrap();

        assert_eq!(
            cached,
            CachedRelease::Latest(Version::parse("0.3.0").unwrap())
        );
    }

    #[test]
    fn fresh_failure_cache_is_unavailable() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(&path, None, now - Duration::from_secs(60));

        assert_eq!(
            read_release_cache_at(&path, now),
            Some(CachedRelease::Unavailable)
        );
    }

    #[tokio::test]
    async fn fresh_failure_cache_skips_the_refresh() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(&path, None, now - Duration::from_secs(60));
        let refreshed = Cell::new(false);

        let result = cached_or_fetch_latest_at(Some(&path), now, || async {
            refreshed.set(true);
            Ok(Some(Version::parse("0.4.0").unwrap()))
        })
        .await;

        assert!(result.is_err());
        assert!(!refreshed.get());
    }

    #[tokio::test]
    async fn expired_failure_cache_runs_the_refresh() {
        let dir = TempDir::new("sift-release-cache").unwrap();
        let path = dir.path().join("latest-release.json");
        let now = SystemTime::now();
        write_cache(
            &path,
            None,
            now - RELEASE_FAILURE_CACHE_TTL - Duration::from_secs(1),
        );
        let refreshed = Cell::new(false);

        let latest = cached_or_fetch_latest_at(Some(&path), now, || async {
            refreshed.set(true);
            Ok(Some(Version::parse("0.4.0").unwrap()))
        })
        .await
        .unwrap();

        assert_eq!(latest, Some(Version::parse("0.4.0").unwrap()));
        assert!(refreshed.get());
    }

    #[test]
    fn outdated_warning_includes_the_shared_install_command() {
        let current = Version::parse("0.3.0").unwrap();
        let latest = Version::parse("0.4.0").unwrap();

        let warning = outdated_warning(&current, &latest).unwrap();
        let command = install_command();

        assert!(warning.contains("sift-cli 0.3.0 is outdated; latest is 0.4.0"));
        assert!(warning.ends_with(&format!("  {command}")));
        assert!(warning[..warning.len().min(512)].contains(&command));
    }

    #[test]
    fn install_command_uses_the_stable_endpoint() {
        let expected = if cfg!(windows) {
            r#"powershell -c "irm https://api.siftstack.com/install/sift-cli.ps1 | iex""#
        } else {
            "curl -fsSL https://api.siftstack.com/install/sift-cli | sh"
        };

        assert_eq!(install_command(), expected);
    }

    #[test]
    fn outdated_warning_is_absent_for_the_latest_release() {
        let current = Version::parse("0.4.0").unwrap();
        let latest = Version::parse("0.4.0").unwrap();

        assert_eq!(outdated_warning(&current, &latest), None);
    }
}
