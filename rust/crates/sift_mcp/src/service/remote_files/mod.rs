use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::header::USER_AGENT;

const UPLOAD_PATH: &str = "/api/v0/remote-files/upload";
const CLIENT_NAME: &str = "sift_mcp";
/// Client-side cap on one uploaded file. The server allows more, but an
/// artifact version larger than this is almost certainly a mistake (raw data
/// belongs in ingestion, not artifacts).
pub const MAX_UPLOAD_BYTES: u64 = 1 << 30;
/// Matches the server's file-name length cap on remote files.
const MAX_FILE_NAME_BYTES: usize = 255;
/// Uploads stream from disk and can be large, so the request timeout is far
/// looser than the interactive-call default.
const UPLOAD_TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// REST endpoint and credential for calls the gRPC surface does not offer.
/// Today that is one thing: the multipart remote-file upload that gives an
/// artifact version its bytes.
#[derive(Clone)]
pub struct RestConfig {
    pub rest_uri: String,
    pub api_key: String,
}

impl RestConfig {
    pub fn new(rest_uri: String, api_key: String) -> Self {
        Self { rest_uri, api_key }
    }
}

/// Uploads local files to the remote-files store over the REST multipart
/// endpoint, attaching each to one entity (for artifacts: entity type
/// `artifact_versions`, entity id = the version's id).
#[derive(Clone)]
pub struct RemoteFileUploader {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    user_agent: String,
}

impl RemoteFileUploader {
    pub fn new(config: RestConfig, cli_version: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: format!("{}{UPLOAD_PATH}", config.rest_uri.trim_end_matches('/')),
            api_key: config.api_key,
            user_agent: format!("{CLIENT_NAME}/{cli_version}"),
        }
    }

    /// Streams one local file into remote_files as the bytes of an artifact
    /// version. The server derives the mime type from the file extension and
    /// binds the row to the version in the same transaction as the upload.
    pub async fn upload_artifact_version_file(
        &self,
        organization_id: &str,
        artifact_version_id: &str,
        path: &Path,
    ) -> Result<()> {
        let file_name = validate_upload_path(path).await?;

        let file = tokio::fs::File::open(path)
            .await
            .with_context(|| format!("failed to open `{}`", path.display()))?;
        let size = file
            .metadata()
            .await
            .with_context(|| format!("failed to stat `{}`", path.display()))?
            .len();

        let part = reqwest::multipart::Part::stream_with_length(reqwest::Body::from(file), size)
            .file_name(file_name);
        let form = reqwest::multipart::Form::new()
            .text("organizationId", organization_id.to_string())
            .text("entityId", artifact_version_id.to_string())
            .text("entityType", "artifact_versions")
            .part("file", part);

        let response = self
            .client
            .post(&self.endpoint)
            .timeout(UPLOAD_TIMEOUT)
            .bearer_auth(&self.api_key)
            .header(USER_AGENT, &self.user_agent)
            .multipart(form)
            .send()
            .await
            .context("failed to reach the remote-file upload endpoint")?;

        let status = response.status();
        if !status.is_success() {
            let detail = response.text().await.unwrap_or_default();
            let detail = detail.chars().take(512).collect::<String>();
            bail!("remote-file upload returned HTTP {status}: {detail}");
        }
        Ok(())
    }
}

/// Checks the path points at a regular, non-empty-named, size-capped file and
/// returns its file name.
async fn validate_upload_path(path: &Path) -> Result<String> {
    let metadata = tokio::fs::metadata(path)
        .await
        .with_context(|| format!("`{}` does not exist or is not readable", path.display()))?;
    if !metadata.is_file() {
        bail!("`{}` is not a regular file", path.display());
    }
    if metadata.len() == 0 {
        bail!("`{}` is empty; artifacts need content", path.display());
    }
    if metadata.len() > MAX_UPLOAD_BYTES {
        bail!(
            "`{}` is {} bytes, above the {} byte artifact limit",
            path.display(),
            metadata.len(),
            MAX_UPLOAD_BYTES
        );
    }
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .filter(|name| !name.is_empty())
        .with_context(|| format!("`{}` has no usable file name", path.display()))?;
    if file_name.len() > MAX_FILE_NAME_BYTES {
        bail!("file name `{file_name}` exceeds {MAX_FILE_NAME_BYTES} bytes");
    }
    Ok(file_name)
}

#[cfg(test)]
mod test;
