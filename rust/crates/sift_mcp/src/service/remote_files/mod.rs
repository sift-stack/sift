use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::header::USER_AGENT;
use sift_rs::{
    SiftChannel,
    remote_files::v1::{
        GetRemoteFileDownloadUrlRequest, ListRemoteFilesRequest, ListRemoteFilesResponse,
        RemoteFile, remote_file_service_client::RemoteFileServiceClient,
    },
};

use crate::ClientName;
use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;

const UPLOAD_PATH: &str = "/api/v0/remote-files/upload";
/// Client-side cap on one uploaded file. The server allows more, but an
/// artifact version larger than this is almost certainly a mistake (raw data
/// belongs in ingestion, not artifacts).
pub const MAX_UPLOAD_BYTES: u64 = 1 << 30;
/// Matches the server's file-name length cap on remote files.
const MAX_FILE_NAME_BYTES: usize = 255;
/// Sent when the file extension maps to no known mime type. The server stores
/// whatever the part declares, and the UI needs a non-empty type to decide
/// how to present the file.
const FALLBACK_MIME: &str = "application/octet-stream";
/// Uploads stream from disk and can be large, so the request timeout is far
/// looser than the interactive-call default.
const UPLOAD_TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// The entity types users attach files to. Remote files on other entities
/// (artifact versions, chat uploads, canvas code) are not attachments and stay
/// out of every listing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachmentEntityType {
    Run,
    Asset,
    Annotation,
    /// A comment on an annotation.
    AnnotationLog,
    TestReport,
    TestStep,
}

impl AttachmentEntityType {
    pub const ALL: [Self; 6] = [
        Self::Run,
        Self::Asset,
        Self::Annotation,
        Self::AnnotationLog,
        Self::TestReport,
        Self::TestStep,
    ];

    /// The singular name callers use.
    pub fn name(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Asset => "asset",
            Self::Annotation => "annotation",
            Self::AnnotationLog => "annotation_log",
            Self::TestReport => "test_report",
            Self::TestStep => "test_step",
        }
    }

    /// The value the `ListRemoteFiles` filter compares `entity_type` against:
    /// the backend's table name, not the `ENTITY_TYPE_*` name listed rows carry.
    pub fn filter_value(self) -> &'static str {
        match self {
            Self::Run => "runs",
            Self::Asset => "assets",
            Self::Annotation => "annotations",
            Self::AnnotationLog => "annotation_logs",
            Self::TestReport => "test_reports",
            Self::TestStep => "test_steps",
        }
    }
}

/// ANDs the scope onto the caller's CEL `filter`. With no `entity_type`, the
/// listing is held to every [`AttachmentEntityType`].
fn attachment_filter(
    entity_ids: &[String],
    entity_type: Option<AttachmentEntityType>,
    filter: &str,
) -> String {
    let mut clauses = Vec::new();
    if !entity_ids.is_empty() {
        let ids = entity_ids
            .iter()
            .map(|id| format!("\"{}\"", common::cel_escape(id.trim())))
            .collect::<Vec<_>>()
            .join(", ");
        clauses.push(format!("entity_id in [{ids}]"));
    }
    match entity_type {
        Some(entity_type) => {
            clauses.push(format!("entity_type == \"{}\"", entity_type.filter_value()))
        }
        // The backend fails `entity_type in [...]` with an internal error, so
        // the type set is spelled as `==` alternatives.
        None => clauses.push(format!(
            "({})",
            AttachmentEntityType::ALL
                .iter()
                .map(|entity_type| format!("entity_type == \"{}\"", entity_type.filter_value()))
                .collect::<Vec<_>>()
                .join(" || ")
        )),
    }
    let filter = filter.trim();
    if !filter.is_empty() {
        clauses.push(format!("({filter})"));
    }
    clauses.join(" && ")
}

/// Reads remote files over gRPC: the files attached to runs, assets, and other
/// entities, and the bytes behind artifact versions.
#[derive(Clone)]
pub struct RemoteFileService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl RemoteFileService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    /// Lists the attachments matching the scope and the caller's CEL `filter`,
    /// held to [`AttachmentEntityType`] even when no type is given.
    ///
    /// Makes one request only. The backend's page tokens do not advance past
    /// the first page, so following one returns the same rows again. A page can
    /// also come back short, or empty, with a token when the caller can't view
    /// some of its files, so `has_more` comes from the token, not the row count.
    pub async fn list_attachments(
        &self,
        entity_ids: Vec<String>,
        entity_type: Option<AttachmentEntityType>,
        filter: String,
        limit: Option<u32>,
    ) -> Result<common::Page<RemoteFile>> {
        let (page_size, record_limit) = common::paging(limit);
        let filter = attachment_filter(&entity_ids, entity_type, &filter);

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let filter = filter.clone();
            async move {
                let mut client = RemoteFileServiceClient::new(channel);
                client
                    .list_remote_files(ListRemoteFilesRequest {
                        page_size,
                        filter,
                        ..Default::default()
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to query file attachments")?;

        let ListRemoteFilesResponse {
            mut remote_files,
            next_page_token,
        } = resp;
        let has_more = remote_files.len() > record_limit || !next_page_token.is_empty();
        remote_files.truncate(record_limit);
        Ok(common::Page {
            items: remote_files,
            has_more,
        })
    }

    /// Mints a short-lived signed URL for one remote file's bytes.
    pub async fn download_url(&self, remote_file_id: String) -> Result<String> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let remote_file_id = remote_file_id.clone();
            async move {
                let mut client = RemoteFileServiceClient::new(channel);
                client
                    .get_remote_file_download_url(GetRemoteFileDownloadUrlRequest {
                        remote_file_id,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to get remote file download url")?;
        if resp.download_url.is_empty() {
            bail!("download url response was empty");
        }
        Ok(resp.download_url)
    }
}

/// REST endpoint and credential for calls the gRPC surface does not offer.
/// Today that is one thing: the multipart remote-file upload that gives an
/// artifact version its bytes.
#[derive(Clone)]
pub struct RestConfig {
    pub rest_uri: String,
    pub api_key: String,
    pub client_name: ClientName,
}

impl RestConfig {
    pub fn new(rest_uri: String, api_key: String) -> Self {
        Self {
            rest_uri,
            api_key,
            client_name: ClientName::default(),
        }
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
            user_agent: format!("{}/{cli_version}", config.client_name.as_str()),
        }
    }

    /// Streams one local file into remote_files as the bytes of an artifact
    /// version. The part declares a mime type derived from the file extension
    /// (the server's own extension table is small and misses `.md`, `.py`,
    /// `.parquet`, ...), and the server binds the row to the version.
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

        let mime_type = mime_type_for(&file_name);
        let part = reqwest::multipart::Part::stream_with_length(reqwest::Body::from(file), size)
            .file_name(file_name)
            .mime_str(&mime_type)
            .with_context(|| format!("invalid mime type `{mime_type}`"))?;
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

/// Mime type to declare for a file name, from its extension. Unknown or
/// missing extensions get [`FALLBACK_MIME`] rather than an empty type.
pub fn mime_type_for(file_name: &str) -> String {
    mime_guess::from_path(file_name)
        .first_raw()
        .unwrap_or(FALLBACK_MIME)
        .to_owned()
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
