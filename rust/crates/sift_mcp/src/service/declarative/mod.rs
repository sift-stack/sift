use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use reqwest::{StatusCode, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::service::remote_files::RestConfig;

const VALIDATE_PATH: &str = "/api/v1/declarative:validate";
const MIGRATE_PATH: &str = "/api/v1/declarative:migrate";
const SHARE_LINK_PATH: &str = "/api/v1/declarative:sharelink";
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

/// Validates and migrates declarative specs through the backend's REST
/// gateway. The declarative protos are not part of `sift_rs`, so there is no
/// gRPC client. The backend is the authority on the schema: specs are passed
/// through verbatim and never parsed here.
#[derive(Clone)]
pub struct DeclarativeService {
    // Absent only when the server runs without a REST endpoint (some tests).
    client: Option<RestClient>,
    retry: RestRetry,
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
        let response = self
            .send(
                SHARE_LINK_PATH,
                &ShareLinkRequest { spec },
                action,
                Some(StatusCode::BAD_REQUEST),
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
            .send(path, body, action, Some(StatusCode::BAD_REQUEST))
            .await?;
        if response.status() == StatusCode::BAD_REQUEST {
            let body = truncate(response.text().await.unwrap_or_default());
            return Ok(Err(invalid_spec_message(&body)));
        }
        decode(response, action).await.map(Ok)
    }

    /// POSTs `body` under the retry policy. Returns the first successful
    /// response, or one whose status is `accepted_status`. Any other status
    /// is an error, retried only when transient.
    async fn send<B: Serialize>(
        &self,
        path: &str,
        body: &B,
        action: &str,
        accepted_status: Option<StatusCode>,
    ) -> Result<reqwest::Response> {
        let client = self.client.as_ref().context(
            "this server was started without a REST endpoint, so declarative specs cannot be checked",
        )?;
        let url = format!("{}{path}", client.base_uri);
        let max_attempts = self.retry.max_attempts.max(1);

        let mut attempt = 1;
        loop {
            let result = client
                .http
                .post(&url)
                .timeout(self.retry.request_timeout)
                .bearer_auth(&client.api_key)
                .header(USER_AGENT, &client.user_agent)
                .json(body)
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
pub(crate) mod test;
