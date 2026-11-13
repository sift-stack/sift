use std::{
    fmt,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{Context, Result, bail};
use reqwest::{Method, Response, StatusCode, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

use crate::{ClientName, service::remote_files::RestConfig};

const SHARE_LINK_PATH: &str = "/api/v1/declarative:sharelink";
const VALIDATE_PATH: &str = "/api/v1/declarative:validate";
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

impl HttpError {
    /// A gateway envelope with gRPC InvalidArgument: Sift rejected the request content.
    pub(crate) fn is_invalid_argument(&self) -> bool {
        self.status == StatusCode::BAD_REQUEST
            && self.grpc_code == Some(tonic::Code::InvalidArgument as i64)
    }
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

/// A part of Sift that draws declarative charts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChartSurface {
    Explore,
    Canvas,
    Edge,
    /// Agent chat in Sift Agents.
    Agents,
}

impl ChartSurface {
    pub(crate) const ALL: [Self; 4] = [Self::Explore, Self::Canvas, Self::Edge, Self::Agents];

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Explore => "explore",
            Self::Canvas => "canvas",
            Self::Edge => "edge",
            Self::Agents => "agents",
        }
    }

    /// Charts made in a Sift chat pod appear in agent chat.
    fn default_for(client_name: ClientName) -> Self {
        match client_name {
            ClientName::Chat => Self::Agents,
            ClientName::SiftMcp => Self::Explore,
        }
    }

    /// Explore and agent chat open a saved share link. The others load the spec itself.
    pub(crate) fn shares_link(self) -> bool {
        match self {
            Self::Explore | Self::Agents => true,
            Self::Canvas | Self::Edge => false,
        }
    }

    fn proto_name(self) -> &'static str {
        match self {
            Self::Explore => "DECLARATIVE_SURFACE_EXPLORE",
            Self::Canvas => "DECLARATIVE_SURFACE_CANVAS",
            Self::Edge => "DECLARATIVE_SURFACE_EDGE",
            Self::Agents => "DECLARATIVE_SURFACE_AGENTS",
        }
    }
}

/// One validation error or warning. `path` is a JSON Pointer into the spec, empty for the root.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct SpecIssue {
    pub path: String,
    pub message: String,
}

impl SpecIssue {
    /// An issue at the document root.
    pub(crate) fn root(message: impl Into<String>) -> Self {
        Self {
            path: String::new(),
            message: message.into(),
        }
    }
}

/// The validate response. Proto3 JSON omits default values, so every field defaults.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ValidateResponse {
    valid: bool,
    errors: Vec<SpecIssue>,
    warnings: Vec<ValidateWarning>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ValidateWarning {
    #[serde(flatten)]
    issue: SpecIssue,
    /// Empty for a contract warning, which applies to every surface.
    surfaces: Vec<String>,
}

impl ValidateWarning {
    fn applies_to(&self, surface: ChartSurface) -> bool {
        self.surfaces.is_empty()
            || self
                .surfaces
                .iter()
                .any(|name| name == surface.proto_name())
    }
}

/// The result of checking a spec for one surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SpecCheck {
    pub valid: bool,
    pub errors: Vec<SpecIssue>,
    /// Contract warnings, plus what the surface cannot draw when `surface_gaps_checked`.
    pub warnings: Vec<SpecIssue>,
    /// False when the deployment ignored the surface, so only the contract was checked.
    pub surface_gaps_checked: bool,
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
    /// The surface when the caller names none, which depends on where the server runs.
    default_surface: ChartSurface,
    /// The last check, so sending the same spec again, such as to save it, repeats no request.
    last_check: LastResult<(ChartSurface, SpecDigest), SpecCheck>,
    /// The last share link. Links are content-addressed, so the same spec gets the same link.
    last_share_link: LastResult<SpecDigest, String>,
}

type SpecDigest = [u8; 32];

fn spec_digest(spec: &str) -> SpecDigest {
    Sha256::digest(spec.as_bytes()).into()
}

/// The most recent result and its key. Never held across an await.
struct LastResult<K, V>(Arc<std::sync::Mutex<Option<(K, V)>>>);

impl<K, V> Clone for LastResult<K, V> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<K, V> Default for LastResult<K, V> {
    fn default() -> Self {
        Self(Arc::default())
    }
}

impl<K: PartialEq, V: Clone> LastResult<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        let last = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        last.as_ref()
            .filter(|(last_key, _)| last_key == key)
            .map(|(_, value)| value.clone())
    }

    fn set(&self, key: K, value: V) {
        *self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some((key, value));
    }
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
        let default_surface = rest_config
            .as_ref()
            .map_or(ChartSurface::Explore, |config| {
                ChartSurface::default_for(config.client_name)
            });
        Self {
            default_surface,
            client: rest_config.map(|config| RestClient {
                http: reqwest::Client::new(),
                base_uri: config.rest_uri.trim_end_matches('/').to_owned(),
                api_key: config.api_key,
                user_agent: format!("{}/{cli_version}", config.client_name.as_str()),
            }),
            cache_root: if cfg!(test) { None } else { dirs::cache_dir() },
            refresh: Arc::default(),
            last_check: LastResult::default(),
            last_share_link: LastResult::default(),
        }
    }

    pub(crate) fn default_surface(&self) -> ChartSurface {
        self.default_surface
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

    /// Strictly validates `spec` and reports what `surface` cannot draw. The same spec and surface
    /// as the last check get its result again without a request.
    pub(crate) async fn check_spec(&self, spec: &str, surface: ChartSurface) -> Result<SpecCheck> {
        let key = (surface, spec_digest(spec));
        if let Some(check) = self.last_check.get(&key) {
            return Ok(check);
        }

        let body = json!({ "spec": spec, "strict": true, "surface": surface.proto_name() });
        let response: ValidateResponse = self
            .send(Method::POST, VALIDATE_PATH, Some(&body))
            .await?
            .json()
            .await
            .context("failed to decode the declarative validate response")?;
        let check = spec_check(response, surface);
        self.last_check.set(key, check.clone());
        Ok(check)
    }

    /// Creates a share link for `spec`, sent exactly as given. The backend validates the spec;
    /// a rejected one is an `HttpError` with status 400 and gRPC InvalidArgument code. The same
    /// spec as the last link gets that link again without a request.
    pub(crate) async fn create_share_link(&self, spec: &str) -> Result<String> {
        let digest = spec_digest(spec);
        if let Some(short_link) = self.last_share_link.get(&digest) {
            return Ok(short_link);
        }

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
        self.last_share_link
            .set(digest, response.short_link.clone());
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

/// Keeps the warnings that apply to `surface`. The gateway drops a surface name the deployment
/// does not know, and the deployment then reports the gaps of every surface. A warning tagged only
/// with other surfaces shows that happened, so `surface_gaps_checked` is false.
fn spec_check(response: ValidateResponse, surface: ChartSurface) -> SpecCheck {
    let surface_gaps_checked = response
        .warnings
        .iter()
        .all(|warning| warning.applies_to(surface));
    SpecCheck {
        valid: response.valid,
        errors: response.errors,
        warnings: response
            .warnings
            .into_iter()
            .filter(|warning| warning.applies_to(surface))
            .map(|warning| warning.issue)
            .collect(),
        surface_gaps_checked,
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
