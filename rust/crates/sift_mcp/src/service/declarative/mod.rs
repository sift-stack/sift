use std::{
    fmt,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{Context, Result, bail};
use reqwest::{Method, Response, StatusCode, header::USER_AGENT};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

use crate::service::remote_files::RestConfig;

const SHARE_LINK_PATH: &str = "/api/v1/declarative:sharelink";
const SCHEMA_PATH: &str = "/download/schemas/declarative-chart/0/sift.declarative.schema.v0.json";
const SCHEMA_FILENAME: &str = "sift.declarative.schema.v0.json";
/// A cached schema file at least this old is downloaded again.
const SCHEMA_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
/// Longest slice of an error body kept in an error message.
const MAX_ERROR_BODY_CHARS: usize = 500;

/// A non-success HTTP response from Sift. Callers downcast to branch on `status`.
#[derive(Debug)]
pub(crate) struct HttpError {
    pub status: StatusCode,
    /// Present only for a gRPC-gateway error envelope with a nonempty string message.
    pub grpc_code: Option<i64>,
    pub message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sift returned HTTP {}: {}", self.status, self.message)
    }
}

impl std::error::Error for HttpError {}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ShareLinkResponse {
    short_link: String,
}

/// Talks to the declarative chart REST endpoints. The declarative protos are not part of
/// `sift_rs`, so there is no gRPC client. The backend is the authority on the spec: it is passed
/// through verbatim and never parsed here.
#[derive(Clone)]
pub struct DeclarativeService {
    // Absent only when the server runs without a REST endpoint.
    client: Option<RestClient>,
    /// Replaces the OS user cache directory. Always unset in tests until `with_cache_root`.
    cache_root: Option<PathBuf>,
    /// Serializes cache refreshes within this process. Other processes may download too; the
    /// atomic rename keeps every reader on a complete file.
    refresh: Arc<Mutex<()>>,
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
        Self {
            client: rest_config.map(|config| RestClient {
                http: reqwest::Client::new(),
                base_uri: config.rest_uri.trim_end_matches('/').to_owned(),
                api_key: config.api_key,
                user_agent: format!("{}/{cli_version}", config.client_name.as_str()),
            }),
            cache_root: if cfg!(test) { None } else { dirs::cache_dir() },
            refresh: Arc::default(),
        }
    }

    /// Tests pick their own private cache root, so none touches the real user cache.
    #[cfg(test)]
    pub(crate) fn with_cache_root(mut self, root: &Path) -> Self {
        self.cache_root = Some(root.to_path_buf());
        self
    }

    /// Downloads the schema fresh, bypassing the cache, and returns the exact bytes served.
    pub async fn chart_schema_bytes(&self) -> Result<Vec<u8>> {
        let response = self
            .send(Method::GET, SCHEMA_PATH, None)
            .await
            .context("failed to download the declarative chart schema")?;
        let bytes = response
            .bytes()
            .await
            .context("failed to read the declarative chart schema response")?;
        validate_schema(&bytes)?;
        Ok(bytes.to_vec())
    }

    /// Path of the machine-local schema file for this deployment, downloading it first when it is
    /// missing, corrupt, or at least 24 hours old. Checked on every call, so a deleted or damaged
    /// file is repaired by the next one. A failed download leaves the previous file untouched.
    pub(crate) async fn chart_schema_path(&self) -> Result<PathBuf> {
        let path = self.schema_cache_path()?;
        let _refreshing = self.refresh.lock().await;
        if is_fresh(&path).await {
            return Ok(path);
        }
        let bytes = self.chart_schema_bytes().await?;
        let target = path.clone();
        tokio::task::spawn_blocking(move || write_private_atomic(&target, &bytes))
            .await
            .context("chart schema cache task failed")??;
        Ok(path)
    }

    /// Keyed by the REST base URI only, so credentials never reach the path.
    fn schema_cache_path(&self) -> Result<PathBuf> {
        let client = self.client.as_ref().context(
            "this server was started without a REST endpoint, so the chart schema cannot be downloaded",
        )?;
        let root = self
            .cache_root
            .as_ref()
            .context("could not find a user cache directory")?;
        let key: String = Sha256::digest(client.base_uri.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        std::path::absolute(
            root.join("sift-cli")
                .join("schemas")
                .join(key)
                .join(SCHEMA_FILENAME),
        )
        .context("failed to resolve the chart schema cache path")
    }

    /// Creates a share link for `spec`, sent exactly as given. The backend validates the spec;
    /// a rejected one is an `HttpError` with status 400 and gRPC InvalidArgument code.
    pub(crate) async fn create_share_link(&self, spec: &str) -> Result<String> {
        let body = json!({ "spec": spec });
        let response: ShareLinkResponse = self
            .send(Method::POST, SHARE_LINK_PATH, Some(&body))
            .await?
            .json()
            .await
            .context("failed to decode the declarative share link response")?;
        if response.short_link.is_empty() {
            bail!("declarative share link response did not include shortLink");
        }
        Ok(response.short_link)
    }

    /// One authenticated request with a bounded timeout. A non-success status is an `HttpError`.
    async fn send(&self, method: Method, path: &str, body: Option<&Value>) -> Result<Response> {
        let client = self.client.as_ref().context(
            "this server was started without a REST endpoint, so declarative charts are unavailable",
        )?;
        let mut request = client
            .http
            .request(method, format!("{}{path}", client.base_uri))
            .timeout(REQUEST_TIMEOUT)
            .bearer_auth(&client.api_key)
            .header(USER_AGENT, &client.user_agent);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request
            .send()
            .await
            .with_context(|| format!("failed to reach Sift at {path}"))?;
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let body = response.text().await.unwrap_or_default();
        Err(parse_http_error(status, &body).into())
    }
}

/// Requires a JSON object with an object `$defs`. The bytes are kept as served.
fn validate_schema(bytes: &[u8]) -> Result<()> {
    let schema: Value = serde_json::from_slice(bytes)
        .context("failed to decode the declarative chart schema response")?;
    if !schema.get("$defs").is_some_and(Value::is_object) {
        bail!("the declarative chart schema response is not a JSON Schema with `$defs`");
    }
    Ok(())
}

/// True for a valid schema file modified less than 24 hours ago (not in the future).
async fn is_fresh(path: &Path) -> bool {
    let Ok(modified) = tokio::fs::metadata(path)
        .await
        .and_then(|metadata| metadata.modified())
    else {
        return false;
    };
    SystemTime::now()
        .duration_since(modified)
        .is_ok_and(|age| age < SCHEMA_MAX_AGE)
        && tokio::fs::read(path)
            .await
            .is_ok_and(|bytes| validate_schema(&bytes).is_ok())
}

/// Writes a private temp file next to `path`, then renames it over `path` so readers see the old
/// or the new file but never a partial one.
fn write_private_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .context("chart schema cache path has no parent")?;
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
        builder.mode(0o700);
        options.mode(0o600);
        builder
            .create(dir)
            .and_then(|()| std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)))
            .with_context(|| format!("failed to prepare {}", dir.display()))?;
    }
    #[cfg(not(unix))]
    builder
        .create(dir)
        .with_context(|| format!("failed to prepare {}", dir.display()))?;

    let temp = dir.join(format!(
        ".{SCHEMA_FILENAME}.{}.{:016x}.tmp",
        std::process::id(),
        rand::random::<u64>()
    ));
    let written = options.open(&temp).and_then(|mut file| {
        file.write_all(bytes)?;
        file.sync_all()?;
        std::fs::rename(&temp, path)
    });
    if let Err(error) = written {
        let _ = std::fs::remove_file(&temp);
        return Err(error).context("failed to save the chart schema cache");
    }
    Ok(())
}

/// Preserve a recognized gateway status separately from proxy errors such as HTML 400s.
fn parse_http_error(status: StatusCode, body: &str) -> HttpError {
    let payload = serde_json::from_str::<Value>(body).ok();
    let message = payload
        .as_ref()
        .and_then(|value| value.get("message")?.as_str())
        .filter(|message| !message.is_empty());
    let grpc_code = message.and_then(|_| payload.as_ref()?.get("code")?.as_i64());
    HttpError {
        status,
        grpc_code,
        message: message
            .unwrap_or(body)
            .chars()
            .take(MAX_ERROR_BODY_CHARS)
            .collect(),
    }
}

#[cfg(test)]
pub(crate) mod test;
