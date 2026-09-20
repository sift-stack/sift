use anyhow::{Context, Result, anyhow, bail};
use sift_rs::{
    SiftChannel,
    artifacts::v1::{
        ArchiveArtifactRequest, ArtifactCreatedVia, ArtifactDetails, ArtifactLinkInput,
        ArtifactStorageClass, ArtifactVersion, CreateArtifactRequest, GetArtifactRequest,
        ListArtifactVersionsRequest, ListArtifactVersionsResponse, ListArtifactsRequest,
        ListArtifactsResponse, UnarchiveArtifactRequest, UpdateArtifactRequest,
        artifact_service_client::ArtifactServiceClient,
    },
    metadata::v1::MetadataValue,
    remote_files::v1::{
        GetRemoteFileDownloadUrlRequest, remote_file_service_client::RemoteFileServiceClient,
    },
};

use std::path::Path;

use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use crate::service::remote_files::RemoteFileUploader;

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub struct ArtifactView {
    pub details: ArtifactDetails,
    pub download_url: Option<String>,
}

impl ArtifactView {
    pub fn version(&self) -> ArtifactVersion {
        self.details.artifact_version.clone().unwrap_or_default()
    }

    pub fn artifact_id(&self) -> String {
        self.details
            .artifact
            .as_ref()
            .map(|artifact| artifact.artifact_id.clone())
            .unwrap_or_default()
    }

    pub fn storage_class(&self) -> i32 {
        self.details
            .artifact
            .as_ref()
            .map(|artifact| artifact.storage_class)
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CreateArtifactInput {
    pub(crate) title: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) conversation_id: Option<String>,
    pub(crate) storage_class: Option<ArtifactStorageClass>,
    pub(crate) created_via: Option<ArtifactCreatedVia>,
    pub(crate) payload: Option<pbjson_types::Struct>,
    pub(crate) metadata: Vec<MetadataValue>,
    pub(crate) links: Vec<ArtifactLinkInput>,
}

#[derive(Clone, Debug)]
pub(crate) struct UpdateArtifactInput {
    pub(crate) artifact_id: String,
    pub(crate) version: ArtifactVersion,
    pub(crate) links: Vec<ArtifactLinkInput>,
    pub(crate) update_mask: Vec<UpdatePath>,
}

/// A path the server publishes on `UpdateArtifactRequest.update_mask`. Spelling
/// one wrong would drop the edit without an error, so callers name a variant and
/// never a string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UpdatePath {
    Title,
    Summary,
    Payload,
    Metadata,
    /// Declares that new bytes follow, so the server leaves the previous
    /// version's files behind instead of carrying them onto this one.
    ReplaceFile,
    Links,
}

impl UpdatePath {
    fn as_str(self) -> &'static str {
        match self {
            Self::Title => "artifact_version.title",
            Self::Summary => "artifact_version.summary",
            Self::Payload => "artifact_version.payload",
            Self::Metadata => "artifact_version.metadata",
            Self::ReplaceFile => "artifact_version.remote_file_id",
            Self::Links => "links",
        }
    }
}

#[derive(Clone)]
pub struct ArtifactService {
    channel: SiftChannel,
    policy: RetryPolicy,
    // Absent only when the server runs without a REST endpoint (some tests);
    // creating an artifact with a file requires it.
    uploader: Option<RemoteFileUploader>,
}

impl ArtifactService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self {
            channel,
            policy,
            uploader: None,
        }
    }

    pub fn with_uploader(mut self, uploader: RemoteFileUploader) -> Self {
        self.uploader = Some(uploader);
        self
    }

    pub async fn list_artifacts(
        &self,
        conversation_id: Option<String>,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<common::Page<ArtifactDetails>> {
        let (page_size, record_limit) = common::paging(limit);
        let mut page_token = String::new();
        let mut results = Vec::new();
        let mut has_more = false;

        loop {
            let channel = self.channel.clone();
            let conversation_id = conversation_id.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let conversation_id = conversation_id.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = ArtifactServiceClient::new(channel);
                    client
                        .list_artifacts(ListArtifactsRequest {
                            conversation_id,
                            page_size,
                            page_token: token,
                            filter,
                            order_by: order_by.unwrap_or_default(),
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query artifacts")?;

            let ListArtifactsResponse {
                artifacts,
                next_page_token,
            } = resp;
            if artifacts.is_empty() {
                break;
            }
            results.extend(artifacts);
            if results.len() >= record_limit {
                has_more = results.len() > record_limit || !next_page_token.is_empty();
                break;
            }
            if next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        Ok(common::Page {
            items: results,
            has_more,
        })
    }

    pub async fn list_artifact_versions(
        &self,
        artifact_id: String,
        limit: Option<u32>,
    ) -> Result<common::Page<ArtifactVersion>> {
        let (page_size, record_limit) = common::paging(limit);
        let mut page_token = String::new();
        let mut results = Vec::new();
        let mut has_more = false;

        loop {
            let channel = self.channel.clone();
            let artifact_id = artifact_id.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let artifact_id = artifact_id.clone();
                let token = token.clone();
                async move {
                    let mut client = ArtifactServiceClient::new(channel);
                    client
                        .list_artifact_versions(ListArtifactVersionsRequest {
                            artifact_id,
                            page_size,
                            page_token: token,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query artifact versions")?;

            let ListArtifactVersionsResponse {
                versions,
                next_page_token,
            } = resp;
            if versions.is_empty() {
                break;
            }
            results.extend(versions);
            if results.len() >= record_limit {
                has_more = results.len() > record_limit || !next_page_token.is_empty();
                break;
            }
            if next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        Ok(common::Page {
            items: results,
            has_more,
        })
    }

    pub async fn download_artifact(
        &self,
        artifact_id: String,
        artifact_version_id: Option<String>,
    ) -> Result<ArtifactView> {
        let details = self.get_artifact(artifact_id, artifact_version_id).await?;
        let remote_file_id = details
            .artifact_version
            .as_ref()
            .and_then(|version| version.remote_file_id.clone());
        let download_url = match remote_file_id {
            Some(remote_file_id) => Some(self.download_url(remote_file_id).await?),
            None => None,
        };
        Ok(ArtifactView {
            details,
            download_url,
        })
    }

    pub async fn archive_artifact(&self, artifact_id: String) -> Result<()> {
        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let artifact_id = artifact_id.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .archive_artifact(ArchiveArtifactRequest { artifact_id })
                    .await
                    .map(|response| response.into_inner())
            }
        })
        .await
        .context("failed to archive artifact")?;
        Ok(())
    }

    pub async fn unarchive_artifact(&self, artifact_id: String) -> Result<()> {
        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let artifact_id = artifact_id.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .unarchive_artifact(UnarchiveArtifactRequest { artifact_id })
                    .await
                    .map(|response| response.into_inner())
            }
        })
        .await
        .context("failed to unarchive artifact")?;
        Ok(())
    }

    pub async fn create_artifact(
        &self,
        input: CreateArtifactInput,
        file_path: Option<&Path>,
    ) -> Result<ArtifactView> {
        // Refuse before creating any rows, so a misconfigured server does not
        // leave a byteless version behind.
        let uploader = self.uploader_for(file_path)?;

        let channel = self.channel.clone();
        let created = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let input = input.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .create_artifact(CreateArtifactRequest {
                        conversation_id: input.conversation_id,
                        title: input.title,
                        summary: input.summary,
                        storage_class: input.storage_class.map(|value| value as i32),
                        created_via: input.created_via.map(|value| value as i32),
                        payload: input.payload,
                        metadata: input.metadata,
                        links: input.links,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create artifact")?
        .artifact
        .ok_or_else(|| anyhow!("create artifact response missing artifact"))?;

        self.attach_bytes(
            created,
            uploader,
            file_path,
            "created",
            "create the artifact again",
        )
        .await
    }

    pub async fn update_artifact(
        &self,
        mut input: UpdateArtifactInput,
        file_path: Option<&Path>,
    ) -> Result<ArtifactView> {
        let uploader = self.uploader_for(file_path)?;
        if file_path.is_some() {
            input.update_mask.push(UpdatePath::ReplaceFile);
        }

        let channel = self.channel.clone();
        let updated = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let input = input.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .update_artifact(UpdateArtifactRequest {
                        artifact_id: input.artifact_id,
                        artifact: Some(ArtifactDetails {
                            artifact: None,
                            artifact_version: Some(input.version),
                        }),
                        links: input.links,
                        update_mask: Some(pbjson_types::FieldMask {
                            paths: input
                                .update_mask
                                .iter()
                                .map(|path| path.as_str().to_string())
                                .collect(),
                        }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update artifact")?
        .artifact
        .ok_or_else(|| anyhow!("update artifact response missing artifact"))?;

        self.attach_bytes(
            updated,
            uploader,
            file_path,
            "written",
            "call update_artifact again",
        )
        .await
    }

    fn uploader_for(&self, file_path: Option<&Path>) -> Result<Option<&RemoteFileUploader>> {
        match file_path {
            Some(_) => Ok(Some(self.uploader.as_ref().context(
                "this server was started without a REST endpoint, so `file_path` is not supported",
            )?)),
            None => Ok(None),
        }
    }

    async fn attach_bytes(
        &self,
        written: ArtifactDetails,
        uploader: Option<&RemoteFileUploader>,
        file_path: Option<&Path>,
        verb: &str,
        retry_warning: &str,
    ) -> Result<ArtifactView> {
        let (Some(uploader), Some(path)) = (uploader, file_path) else {
            return Ok(ArtifactView {
                details: written,
                download_url: None,
            });
        };
        let artifact = written
            .artifact
            .clone()
            .ok_or_else(|| anyhow!("artifact response missing container"))?;
        let version = written
            .artifact_version
            .clone()
            .ok_or_else(|| anyhow!("artifact response missing version"))?;

        let upload_context = format!(
            "artifact {} version {} was {verb}, but uploading `{}` failed; do NOT {retry_warning}",
            artifact.artifact_id,
            version.version,
            path.display()
        );
        uploader
            .upload_artifact_version_file(
                &artifact.organization_id,
                &version.artifact_version_id,
                path,
            )
            .await
            .context(upload_context)?;

        let refreshed = self
            .get_artifact(
                artifact.artifact_id.clone(),
                Some(version.artifact_version_id.clone()),
            )
            .await
            .unwrap_or(written);
        let remote_file_id = refreshed
            .artifact_version
            .as_ref()
            .and_then(|version| version.remote_file_id.clone());
        let download_url = match remote_file_id {
            Some(remote_file_id) => self.download_url(remote_file_id).await.ok(),
            None => None,
        };
        Ok(ArtifactView {
            details: refreshed,
            download_url,
        })
    }

    async fn get_artifact(
        &self,
        artifact_id: String,
        artifact_version_id: Option<String>,
    ) -> Result<ArtifactDetails> {
        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let artifact_id = artifact_id.clone();
            let artifact_version_id = artifact_version_id.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .get_artifact(GetArtifactRequest {
                        artifact_id,
                        artifact_version_id,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to get artifact")?
        .artifact
        .ok_or_else(|| anyhow!("get artifact response missing artifact"))
    }

    async fn download_url(&self, remote_file_id: String) -> Result<String> {
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
        .context("failed to get artifact download url")?;
        if resp.download_url.is_empty() {
            bail!("download url response was empty");
        }
        Ok(resp.download_url)
    }
}
