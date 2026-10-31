use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex, PoisonError},
    time::Duration,
};

use anyhow::{Context, Result, anyhow, bail};
use reqwest::{StatusCode, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use tokio::time::sleep;

use crate::service::remote_files::RestConfig;

const VALIDATE_PATH: &str = "/api/v1/declarative:validate";
const MIGRATE_PATH: &str = "/api/v1/declarative:migrate";
const SHARE_LINK_PATH: &str = "/api/v1/declarative:sharelink";
const SCHEMA_PATH: &str = "/api/v1/declarative:schema";
const DEFS_REF_PREFIX: &str = "#/$defs/";
/// Appears in the backend's not-found message for an unknown schema version.
/// A deployment without the route returns the gateway's plain "Not Found".
const UNKNOWN_VERSION_MESSAGE: &str = "declarative schema version";
const MAX_CLOSE_MATCHES: usize = 5;
/// Shortest shared prefix that makes a definition name a close match.
const MIN_SHARED_PREFIX: usize = 4;
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

/// Why the schema could not be fetched.
#[derive(Debug)]
pub enum SchemaError {
    /// HTTP 404 from the gateway: this deployment has no schema endpoint.
    NotServed,
    /// HTTP 404 from the endpoint: the version is unknown. Holds the
    /// backend's message, which lists the available versions.
    VersionNotFound(String),
    Other(anyhow::Error),
}

impl From<anyhow::Error> for SchemaError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(error)
    }
}

/// A requested `$defs` name the schema does not define.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownDefinition {
    pub name: String,
    pub close_matches: Vec<String>,
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

/// Validates and migrates declarative specs through the backend's REST
/// gateway. The declarative protos are not part of `sift_rs`, so there is no
/// gRPC client. The backend is the authority on the schema: specs are passed
/// through verbatim and never parsed here.
#[derive(Clone)]
pub struct DeclarativeService {
    // Absent only when the server runs without a REST endpoint (some tests).
    client: Option<RestClient>,
    retry: RestRetry,
    // Fetched schemas keyed by the requested version. A deployment serves the
    // schema compiled into its build, so it changes only on deploy.
    schemas: Arc<Mutex<HashMap<Option<u32>, Arc<Value>>>>,
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
            schemas: Arc::default(),
        }
    }

    /// Overrides the attempt count, backoff base, and per-request timeout.
    #[allow(dead_code)] // Only tests override the defaults today.
    pub fn with_retry(mut self, retry: RestRetry) -> Self {
        self.retry = retry;
        self
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
        let body = ShareLinkRequest { spec };
        let response = self
            .send(
                SHARE_LINK_PATH,
                action,
                Some(StatusCode::BAD_REQUEST),
                |http, url| http.post(url).json(&body),
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

    /// Fetches the deployment's declarative chart JSON Schema. `version` is
    /// the schema major; `None` asks for the current one. Successful fetches
    /// are cached for the life of the service.
    pub async fn fetch_schema(&self, version: Option<u32>) -> Result<Arc<Value>, SchemaError> {
        if let Some(schema) = self.cached_schemas().get(&version) {
            return Ok(Arc::clone(schema));
        }

        let action = "schema";
        let response = self
            .send(
                SCHEMA_PATH,
                action,
                Some(StatusCode::NOT_FOUND),
                |http, url| match version {
                    Some(version) => http.get(format!("{url}?version={version}")),
                    None => http.get(url),
                },
            )
            .await?;
        if response.status() == StatusCode::NOT_FOUND {
            let message = body_message(&truncate(response.text().await.unwrap_or_default()));
            return Err(match message {
                Some(message) if message.contains(UNKNOWN_VERSION_MESSAGE) => {
                    SchemaError::VersionNotFound(message)
                }
                _ => SchemaError::NotServed,
            });
        }

        let schema: Value = decode(response, action).await?;
        if !schema.get("$defs").is_some_and(Value::is_object) {
            return Err(anyhow!(
                "unexpected declarative schema response: the body is not a JSON Schema with `$defs`"
            )
            .into());
        }

        let schema = Arc::new(schema);
        self.cached_schemas().insert(version, Arc::clone(&schema));
        Ok(schema)
    }

    fn cached_schemas(&self) -> std::sync::MutexGuard<'_, HashMap<Option<u32>, Arc<Value>>> {
        self.schemas.lock().unwrap_or_else(PoisonError::into_inner)
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
            .send(path, action, Some(StatusCode::BAD_REQUEST), |http, url| {
                http.post(url).json(body)
            })
            .await?;
        if response.status() == StatusCode::BAD_REQUEST {
            let body = truncate(response.text().await.unwrap_or_default());
            return Ok(Err(invalid_spec_message(&body)));
        }
        decode(response, action).await.map(Ok)
    }

    /// Sends the request `build` makes for the endpoint URL, under the retry
    /// policy, adding the timeout, auth, and user agent. Returns the first
    /// successful response, or one whose status is `accepted_status`. Any
    /// other status is an error, retried only when transient.
    async fn send(
        &self,
        path: &str,
        action: &str,
        accepted_status: Option<StatusCode>,
        build: impl Fn(&reqwest::Client, &str) -> reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let client = self.client.as_ref().context(
            "this server was started without a REST endpoint, so the declarative chart service is unavailable",
        )?;
        let url = format!("{}{path}", client.base_uri);
        let max_attempts = self.retry.max_attempts.max(1);

        let mut attempt = 1;
        loop {
            let result = build(&client.http, &url)
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
                    if status.is_success() || Some(status) == accepted_status {
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
    body_message(body).unwrap_or_else(|| INVALID_SPEC_FALLBACK.to_owned())
}

/// The backend's JSON `message`, else the raw body, else `None` when empty.
fn body_message(body: &str) -> Option<String> {
    if body.is_empty() {
        return None;
    }
    match serde_json::from_str::<InvalidSpecBody>(body) {
        Ok(InvalidSpecBody {
            message: Some(message),
        }) if !message.is_empty() => Some(message),
        _ => Some(body.to_owned()),
    }
}

/// A compact view of `schema`: the root without `$defs`, plus each `$defs`
/// name with the first sentence of its description, sorted by name.
pub fn schema_index(schema: &Value) -> Value {
    let root: Map<String, Value> = schema
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(key, _)| *key != "$defs")
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    let mut definitions: Vec<Value> = defs(schema)
        .map(|defs| {
            defs.iter()
                .map(|(name, definition)| {
                    let description = definition
                        .get("description")
                        .and_then(Value::as_str)
                        .unwrap_or_default();
                    json!({ "name": name, "summary": first_sentence(description) })
                })
                .collect()
        })
        .unwrap_or_default();
    definitions.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));

    json!({
        "version": schema_version(schema),
        "id": schema.get("$id").cloned().unwrap_or(Value::Null),
        "root": root,
        "definitions": definitions,
    })
}

/// The requested `$defs` entries, plus `references`: the sorted `$defs`
/// names they reference that were not requested. Any unknown name fails
/// the whole selection, each with its close matches.
pub fn select_definitions(
    schema: &Value,
    names: &[String],
) -> std::result::Result<Value, Vec<UnknownDefinition>> {
    let empty = Map::new();
    let defs = defs(schema).unwrap_or(&empty);
    let names: BTreeSet<&String> = names.iter().collect();

    let unknown: Vec<UnknownDefinition> = names
        .iter()
        .filter(|name| !defs.contains_key(name.as_str()))
        .map(|name| UnknownDefinition {
            name: (*name).clone(),
            close_matches: close_matches(name, defs.keys()),
        })
        .collect();
    if !unknown.is_empty() {
        return Err(unknown);
    }

    let selected: Map<String, Value> = names
        .iter()
        .map(|name| ((*name).clone(), defs[name.as_str()].clone()))
        .collect();
    let mut references = BTreeSet::new();
    for definition in selected.values() {
        collect_references(definition, &mut references);
    }
    references.retain(|name| !selected.contains_key(name));

    Ok(json!({
        "version": schema_version(schema),
        "definitions": selected,
        "references": references,
    }))
}

/// Every `$defs` name, sorted.
pub fn definition_names(schema: &Value) -> Vec<String> {
    let mut names: Vec<String> = defs(schema)
        .map(|defs| defs.keys().cloned().collect())
        .unwrap_or_default();
    names.sort();
    names
}

/// The major version the schema pins with `properties.version.const`.
pub fn schema_version(schema: &Value) -> Value {
    schema
        .pointer("/properties/version/const")
        .cloned()
        .unwrap_or(Value::Null)
}

fn defs(schema: &Value) -> Option<&Map<String, Value>> {
    schema.get("$defs").and_then(Value::as_object)
}

/// Ends at the first ". " that does not close an `e.g.` or `i.e.`.
fn first_sentence(text: &str) -> &str {
    text.match_indices(". ")
        .map(|(end, _)| &text[..=end])
        .find(|sentence| !sentence.ends_with("e.g.") && !sentence.ends_with("i.e."))
        .unwrap_or(text)
}

fn collect_references(value: &Value, references: &mut BTreeSet<String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if key == "$ref"
                    && let Some(name) = child.as_str().and_then(|r| r.strip_prefix(DEFS_REF_PREFIX))
                {
                    references.insert(name.to_owned());
                }
                collect_references(child, references);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_references(item, references);
            }
        }
        _ => {}
    }
}

/// Names that contain or are contained in `name`, ignoring case, closest in
/// length first; else names sharing at least `MIN_SHARED_PREFIX` leading
/// characters with it, longest shared prefix first.
fn close_matches<'a>(name: &str, candidates: impl Iterator<Item = &'a String>) -> Vec<String> {
    let name = name.to_lowercase();
    let candidates: Vec<(String, &String)> = candidates
        .map(|candidate| (candidate.to_lowercase(), candidate))
        .collect();

    let mut matches: Vec<String> = candidates
        .iter()
        .filter(|(lower, _)| lower.contains(&name) || name.contains(lower.as_str()))
        .map(|(_, candidate)| (*candidate).clone())
        .collect();
    if matches.is_empty() {
        let mut by_prefix: Vec<(usize, &String)> = candidates
            .iter()
            .map(|(lower, candidate)| (shared_prefix(&name, lower), *candidate))
            .filter(|(shared, _)| *shared >= MIN_SHARED_PREFIX)
            .collect();
        by_prefix.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        matches = by_prefix
            .into_iter()
            .map(|(_, candidate)| candidate.clone())
            .collect();
    } else {
        matches.sort_by_key(|candidate| (candidate.len().abs_diff(name.len()), candidate.clone()));
    }
    matches.truncate(MAX_CLOSE_MATCHES);
    matches
}

fn shared_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(a, b)| a == b).count()
}

#[cfg(test)]
pub(crate) mod test;
