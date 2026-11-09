use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{Context, Result, anyhow, bail};
use reqwest::{StatusCode, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use tokio::{sync::OnceCell, time::sleep};

use crate::service::remote_files::RestConfig;

const VALIDATE_PATH: &str = "/api/v1/declarative:validate";
const MIGRATE_PATH: &str = "/api/v1/declarative:migrate";
const SHARE_LINK_PATH: &str = "/api/v1/declarative:sharelink";
const SCHEMA_VERSIONS_PATH: &str = "/api/packages/v1/schemas/declarative-chart/versions";
const SCHEMA_PATH: &str = "/download/schemas/declarative-chart/0/sift.declarative.schema.v0.json";
/// A cached schema file older than this is downloaded again at startup.
const SCHEMA_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const SCHEMA_LOCK_FILENAME: &str = ".lock";
const SCHEMA_FILENAME: &str = "sift.declarative.schema.v0.json";
const SCHEMA_UNAVAILABLE: &str = "This deployment does not serve the declarative chart schema.";
/// Longest slice of an error body echoed back in a failure or invalid-spec
/// message.
const MAX_ERROR_BODY_CHARS: usize = 500;
const INVALID_SPEC_FALLBACK: &str = "declarative spec failed validation";
const DEFAULT_MAX_ATTEMPTS: u32 = 3;
const DEFAULT_RETRY_DELAY: Duration = Duration::from_millis(250);
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// One validation or migration finding, located by `path` within the spec.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct Issue {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationResult {
    pub valid: bool,
    pub spec: String,
    pub errors: Vec<Issue>,
    pub warnings: Vec<Issue>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MigrationResult {
    pub valid: bool,
    pub original_spec: String,
    pub migrated_spec: String,
    pub source_version: String,
    pub target_version: String,
    pub errors: Vec<Issue>,
    pub warnings: Vec<Issue>,
}

/// Outcome of a share-link request. A spec the backend rejects is `Invalid`,
/// not an error; only transport, HTTP, and malformed-response failures are
/// `Err`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShareLinkResult {
    Ok { short_link: String },
    Invalid { message: String },
}

/// Retry and timeout behavior shared by every declarative REST call. A call
/// is retried on transport errors, timeouts, and HTTP 408, 429, and >=500,
/// waiting `retry_delay * 2^(attempt - 1)` between attempts. The crate's
/// `with_retry` only understands gRPC codes, so REST has its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RestRetry {
    pub max_attempts: u32,
    pub retry_delay: Duration,
    pub request_timeout: Duration,
}

impl Default for RestRetry {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            retry_delay: DEFAULT_RETRY_DELAY,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
        }
    }
}

#[derive(Serialize)]
struct ValidateRequest<'a> {
    spec: &'a str,
    strict: bool,
}

#[derive(Serialize)]
struct MigrateRequest<'a> {
    spec: &'a str,
}

#[derive(Serialize)]
struct ShareLinkRequest<'a> {
    spec: &'a str,
}

// Proto3 JSON omits fields at their default value, so every field defaults.
#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ValidateResponse {
    valid: bool,
    spec: String,
    errors: Vec<Issue>,
    warnings: Vec<Issue>,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct MigrateResponse {
    original_spec: String,
    migrated_spec: String,
    source_version: String,
    target_version: String,
    warnings: Vec<Issue>,
    errors: Vec<Issue>,
    valid: bool,
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ShareLinkResponse {
    short_link: String,
}

#[derive(Deserialize)]
struct InvalidSpecBody {
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChartSchema {
    #[serde(rename = "$id")]
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "$defs")]
    pub definitions: Map<String, Value>,
    /// All remaining root-level JSON Schema keywords (`type`, `properties`,
    /// `required`, `if`, `then`, `additionalProperties`, etc.). Does not
    /// include `$id`, `title`, `description`, or `$defs`.
    #[serde(flatten)]
    pub root: Map<String, Value>,
}

#[derive(Deserialize)]
struct SchemaVersions {
    versions: Vec<SchemaVersion>,
}

#[derive(Deserialize)]
struct SchemaVersion {
    version: String,
}

/// The schema parsed from the machine-local cache file at `path`. Each call to
/// `chart_schema` reads the file again, so `schema` and `path` always agree
/// even when another MCP process refreshes the shared file.
#[derive(Debug)]
pub(crate) struct CachedChartSchema {
    pub schema: ChartSchema,
    pub path: PathBuf,
}

async fn blocking<T: Send + 'static>(
    path: &Path,
    work: impl FnOnce(&Path) -> T + Send + 'static,
) -> Result<T> {
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || work(&path))
        .await
        .context("chart schema cache task failed")
}

/// True for a regular file modified less than 24 hours ago (not in the future)
/// that parses as a chart schema.
fn is_fresh_schema(path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    let fresh = metadata.is_file()
        && metadata
            .modified()
            .ok()
            .and_then(|modified| SystemTime::now().duration_since(modified).ok())
            .is_some_and(|age| age < SCHEMA_MAX_AGE);
    fresh
        && std::fs::read(path)
            .ok()
            .is_some_and(|bytes| serde_json::from_slice::<ChartSchema>(&bytes).is_ok())
}

fn private_dir(dir: &Path) -> Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
    builder
        .create(dir)
        .with_context(|| format!("failed to create {}", dir.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
            .with_context(|| format!("failed to make {} private", dir.display()))?;
    }
    Ok(())
}

fn private_file_options() -> std::fs::OpenOptions {
    let mut options = std::fs::OpenOptions::new();
    options.write(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    options
}

/// Waits for the exclusive cross-process lock. Dropping the file releases it.
fn lock_schema_dir(dir: &Path) -> Result<std::fs::File> {
    private_dir(dir)?;
    let file = private_file_options()
        .create(true)
        .truncate(false)
        .open(dir.join(SCHEMA_LOCK_FILENAME))
        .context("failed to open the chart schema cache lock")?;
    file.lock()
        .context("failed to lock the chart schema cache")?;
    Ok(file)
}

/// Writes a private temp file, flushes it to disk, then renames it over the
/// cache file so readers see the old or the new file but never a partial one.
fn publish_schema(dir: &Path, path: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    let temp = dir.join(format!(
        ".{SCHEMA_FILENAME}.{}.{:016x}.tmp",
        std::process::id(),
        rand::random::<u64>()
    ));
    let written = (|| -> std::io::Result<()> {
        let mut file = private_file_options().create_new(true).open(&temp)?;
        file.write_all(bytes)?;
        file.flush()?;
        file.sync_all()?;
        std::fs::rename(&temp, path)
    })();
    if let Err(error) = written {
        let _ = std::fs::remove_file(&temp);
        return Err(error).context("failed to save the chart schema cache");
    }
    Ok(())
}

/// Validates and migrates declarative specs through the backend's REST
/// gateway. The declarative protos are not part of `sift_rs`, so there is no
/// gRPC client. The backend is the authority on the schema: specs are passed
/// through verbatim and never parsed here.
#[derive(Clone)]
pub struct DeclarativeService {
    // Absent only when the server runs without a REST endpoint (some tests).
    client: Option<RestClient>,
    retry: RestRetry,
    /// Set once the cache file is known to be fresh and valid. Failed
    /// initializations are retried by the next call.
    chart_schema_path: Arc<OnceCell<PathBuf>>,
    /// Replaces the OS user cache directory. Tests always set it.
    cache_root: Option<PathBuf>,
    /// Keeps the test cache root alive for every service sharing it.
    #[cfg(test)]
    _test_cache: Option<Arc<tempdir::TempDir>>,
}

#[derive(Clone)]
struct RestClient {
    http: reqwest::Client,
    base_uri: String,
    api_key: String,
    user_agent: String,
}

impl DeclarativeService {
    pub fn new(rest_config: Option<RestConfig>, cli_version: &str) -> Self {
        let client = rest_config.map(|config| RestClient {
            http: reqwest::Client::new(),
            base_uri: config.rest_uri.trim_end_matches('/').to_owned(),
            api_key: config.api_key,
            user_agent: format!("{}/{cli_version}", config.client_name.as_str()),
        });
        Self {
            client,
            retry: RestRetry::default(),
            chart_schema_path: Arc::default(),
            cache_root: None,
            #[cfg(test)]
            _test_cache: None,
        }
        .with_private_test_cache()
    }

    #[cfg(test)]
    fn with_private_test_cache(mut self) -> Self {
        let dir = tempdir::TempDir::new("sift-schema-cache").expect("test cache root");
        self.cache_root = Some(dir.path().to_path_buf());
        self._test_cache = Some(Arc::new(dir));
        self
    }

    #[cfg(not(test))]
    fn with_private_test_cache(self) -> Self {
        self
    }

    /// Makes a test service use `other`'s cache root, like a second MCP
    /// process on the same machine. Process state stays separate.
    #[cfg(test)]
    pub(crate) fn with_shared_cache_root(mut self, other: &Self) -> Self {
        self.cache_root = other.cache_root.clone();
        self._test_cache = other._test_cache.clone();
        self
    }

    /// Overrides the attempt count, backoff base, and per-request timeout.
    #[allow(dead_code)] // Only tests override the defaults today.
    pub fn with_retry(mut self, retry: RestRetry) -> Self {
        self.retry = retry;
        self
    }

    /// Explicit CLI downloads are always fresh and never use the schema cache.
    pub async fn chart_schema_bytes(&self) -> Result<Vec<u8>> {
        self.fetch_chart_schema()
            .await?
            .bytes()
            .await
            .context("failed to read the declarative chart schema response")
            .map(|bytes| bytes.to_vec())
    }

    /// Returns the schema from the machine-local cache file. The first
    /// successful call (the startup preloader) checks the file and downloads
    /// only if it is missing, corrupt, from the future, or at least 24 hours
    /// old. After that, calls only read the local file. A failed refresh
    /// leaves the old file in place and is reported to the caller.
    pub(crate) async fn chart_schema(&self) -> Result<Arc<CachedChartSchema>> {
        let path = self
            .chart_schema_path
            .get_or_try_init(|| self.ensure_schema_file())
            .await?
            .clone();
        let bytes = tokio::fs::read(&path).await.with_context(|| {
            format!("failed to read the cached chart schema {}", path.display())
        })?;
        let schema = serde_json::from_slice(&bytes).with_context(|| {
            format!(
                "the cached chart schema {} is invalid; restart the MCP server to download it again",
                path.display()
            )
        })?;
        Ok(Arc::new(CachedChartSchema { schema, path }))
    }

    /// Cache directory for this deployment. The key is the REST base URI
    /// only, so credentials never reach the path.
    fn schema_cache_dir(&self) -> Result<PathBuf> {
        let client = self.client.as_ref().context(
            "this server was started without a REST endpoint, so the chart schema cannot be downloaded",
        )?;
        let root = match &self.cache_root {
            Some(root) => root.clone(),
            None => dirs::cache_dir().context("could not find a user cache directory")?,
        };
        let digest = Sha256::digest(client.base_uri.as_bytes());
        let key: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        let dir = root.join("sift-cli").join("schemas").join(key);
        std::path::absolute(&dir).context("failed to resolve the chart schema cache directory")
    }

    async fn ensure_schema_file(&self) -> Result<PathBuf> {
        let dir = self.schema_cache_dir()?;
        let path = dir.join(SCHEMA_FILENAME);
        if blocking(&path, |path| is_fresh_schema(path)).await? {
            return Ok(path);
        }

        // Cross-process lock so concurrent starts download once. The wait runs
        // on a blocking thread, and the OS releases the lock if we crash.
        let lock = blocking(&dir, |dir| lock_schema_dir(dir)).await??;
        if blocking(&path, |path| is_fresh_schema(path)).await? {
            return Ok(path);
        }
        let bytes = self.chart_schema_bytes().await?;
        serde_json::from_slice::<ChartSchema>(&bytes)
            .context("failed to decode the declarative chart schema response")?;
        let target = path.clone();
        // The lock moves into the blocking closure: if this future is canceled,
        // a publication already running keeps the lock until it finishes.
        blocking(&dir, move |dir| {
            let _lock = lock;
            publish_schema(dir, &target, &bytes)
        })
        .await??;
        Ok(path)
    }

    async fn fetch_chart_schema(&self) -> Result<reqwest::Response> {
        let unavailable_statuses = [StatusCode::BAD_REQUEST, StatusCode::NOT_FOUND];
        let response = self
            .send(
                SCHEMA_VERSIONS_PATH,
                |http, url| http.get(url),
                "chart schema versions",
                &unavailable_statuses,
            )
            .await?;
        if unavailable_statuses.contains(&response.status()) {
            bail!(SCHEMA_UNAVAILABLE);
        }
        let versions: SchemaVersions = decode(response, "chart schema versions").await?;
        if !versions.versions.iter().any(|entry| entry.version == "0") {
            bail!(SCHEMA_UNAVAILABLE);
        }

        self.send(SCHEMA_PATH, |http, url| http.get(url), "chart schema", &[])
            .await
    }

    /// Validates `spec` in strict mode. A spec that fails validation is an
    /// `Ok` result with `valid == false` and its `errors`, including an
    /// HTTP 400 (an empty spec), which yields one whole-spec error. Only
    /// transport and other HTTP failures are `Err`.
    pub async fn validate(&self, spec: &str) -> Result<ValidationResult> {
        let resp: ValidateResponse = match self
            .post(
                VALIDATE_PATH,
                &ValidateRequest { spec, strict: true },
                "validate",
            )
            .await?
        {
            Ok(resp) => resp,
            Err(message) => {
                return Ok(ValidationResult {
                    errors: vec![whole_spec_issue(message)],
                    ..Default::default()
                });
            }
        };
        Ok(ValidationResult {
            valid: resp.valid,
            spec: resp.spec,
            errors: resp.errors,
            warnings: resp.warnings,
        })
    }

    /// Migrates `spec` to the latest schema version. As with `validate`, an
    /// HTTP 400 is an `Ok` result with `valid == false` and one whole-spec
    /// error.
    pub async fn migrate(&self, spec: &str) -> Result<MigrationResult> {
        let resp: MigrateResponse = match self
            .post(MIGRATE_PATH, &MigrateRequest { spec }, "migrate")
            .await?
        {
            Ok(resp) => resp,
            Err(message) => {
                return Ok(MigrationResult {
                    errors: vec![whole_spec_issue(message)],
                    ..Default::default()
                });
            }
        };
        Ok(MigrationResult {
            valid: resp.valid,
            original_spec: resp.original_spec,
            migrated_spec: resp.migrated_spec,
            source_version: resp.source_version,
            target_version: resp.target_version,
            errors: resp.errors,
            warnings: resp.warnings,
        })
    }

    /// Creates a share link for `spec`. HTTP 400 means the backend rejected
    /// the spec and is returned as `ShareLinkResult::Invalid`.
    pub async fn create_share_link(&self, spec: &str) -> Result<ShareLinkResult> {
        let action = "share link";
        let response = self
            .send(
                SHARE_LINK_PATH,
                |http, url| http.post(url).json(&ShareLinkRequest { spec }),
                action,
                &[StatusCode::BAD_REQUEST],
            )
            .await?;

        if response.status() == StatusCode::BAD_REQUEST {
            let body = truncate(response.text().await.unwrap_or_default());
            return Ok(ShareLinkResult::Invalid {
                message: invalid_spec_message(&body),
            });
        }

        let resp: ShareLinkResponse = decode(response, action).await?;
        if resp.short_link.is_empty() {
            bail!("declarative share link response did not include shortLink");
        }
        Ok(ShareLinkResult::Ok {
            short_link: resp.short_link,
        })
    }

    /// POSTs `body` and decodes the response. HTTP 400 is the backend
    /// rejecting an empty or malformed request, so it comes back as
    /// `Err(message)` for the caller to report as an invalid spec.
    async fn post<B, R>(
        &self,
        path: &str,
        body: &B,
        action: &str,
    ) -> Result<std::result::Result<R, String>>
    where
        B: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let response = self
            .send(
                path,
                |http, url| http.post(url).json(body),
                action,
                &[StatusCode::BAD_REQUEST],
            )
            .await?;
        if response.status() == StatusCode::BAD_REQUEST {
            let body = truncate(response.text().await.unwrap_or_default());
            return Ok(Err(invalid_spec_message(&body)));
        }
        decode(response, action).await.map(Ok)
    }

    /// Sends a request and returns `Ok` for accepted statuses. Retries only transient statuses.
    async fn send(
        &self,
        path: &str,
        build_request: impl Fn(&reqwest::Client, &str) -> reqwest::RequestBuilder,
        action: &str,
        accepted_statuses: &[StatusCode],
    ) -> Result<reqwest::Response> {
        let client = self.client.as_ref().context(
            "this server was started without a REST endpoint, so declarative specs cannot be checked",
        )?;
        let url = format!("{}{path}", client.base_uri);
        let max_attempts = self.retry.max_attempts.max(1);

        let mut attempt = 1;
        loop {
            let result = build_request(&client.http, &url)
                .timeout(self.retry.request_timeout)
                .bearer_auth(&client.api_key)
                .header(USER_AGENT, &client.user_agent)
                .send()
                .await;

            let error = match result {
                Err(error) if error.is_timeout() => anyhow!(error).context(format!(
                    "declarative {action} request timed out after {}ms",
                    self.retry.request_timeout.as_millis()
                )),
                Err(error) => anyhow!(error)
                    .context(format!("failed to reach the declarative {action} endpoint")),
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() || accepted_statuses.contains(&status) {
                        return Ok(response);
                    }
                    let detail = truncate(response.text().await.unwrap_or_default());
                    let error = anyhow!("declarative {action} returned HTTP {status}: {detail}");
                    if !is_retryable_status(status) {
                        return Err(error);
                    }
                    error
                }
            };

            if attempt >= max_attempts {
                return Err(error.context(format!(
                    "declarative {action} failed after {max_attempts} attempts"
                )));
            }
            sleep(backoff(self.retry.retry_delay, attempt)).await;
            attempt += 1;
        }
    }
}

async fn decode<R>(response: reqwest::Response, action: &str) -> Result<R>
where
    R: for<'de> Deserialize<'de>,
{
    response
        .json()
        .await
        .with_context(|| format!("failed to decode the declarative {action} response"))
}

/// An issue that applies to the spec as a whole (empty path).
fn whole_spec_issue(message: String) -> Issue {
    Issue {
        path: String::new(),
        message,
    }
}

fn is_retryable_status(status: StatusCode) -> bool {
    status == StatusCode::REQUEST_TIMEOUT
        || status == StatusCode::TOO_MANY_REQUESTS
        || status.as_u16() >= 500
}

/// `base * 2^(attempt - 1)`, where `attempt` is the one that just failed.
fn backoff(base: Duration, attempt: u32) -> Duration {
    let factor = 1u32.checked_shl(attempt - 1).unwrap_or(u32::MAX);
    base.saturating_mul(factor)
}

fn truncate(body: String) -> String {
    match body.char_indices().nth(MAX_ERROR_BODY_CHARS) {
        Some((end, _)) => body[..end].to_owned(),
        None => body,
    }
}

/// Prefers the backend's JSON `message`, then the raw (already truncated)
/// body, then a generic fallback.
fn invalid_spec_message(body: &str) -> String {
    if body.is_empty() {
        return INVALID_SPEC_FALLBACK.to_owned();
    }
    match serde_json::from_str::<InvalidSpecBody>(body) {
        Ok(InvalidSpecBody {
            message: Some(message),
        }) if !message.is_empty() => message,
        _ => body.to_owned(),
    }
}

#[cfg(test)]
mod schema_test;
#[cfg(test)]
pub(crate) mod test;
