use anyhow::{Context, Result, anyhow, bail};
use serde::Serialize;
use sift_rs::{
    SiftChannel,
    artifacts::v1::{
        Artifact, ArtifactAuthoringKind, ArtifactCreatedVia, ArtifactLinkInput,
        ArtifactStorageClass, CreateArtifactRequest, GetArtifactRequest, ListArtifactsRequest,
        ListArtifactsResponse, artifact_service_client::ArtifactServiceClient,
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

#[derive(Clone, Debug, Serialize)]
pub struct ArtifactView {
    #[serde(flatten)]
    pub inner: Artifact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct CreateArtifactInput {
    pub(crate) title: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) conversation_id: Option<String>,
    pub(crate) artifact_id: Option<String>,
    pub(crate) authoring_kind: ArtifactAuthoringKind,
    pub(crate) storage_class: Option<ArtifactStorageClass>,
    pub(crate) created_via: Option<ArtifactCreatedVia>,
    pub(crate) kind: Option<String>,
    pub(crate) payload: Option<pbjson_types::Struct>,
    pub(crate) metadata: Vec<MetadataValue>,
    pub(crate) links: Vec<ArtifactLinkInput>,
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
        include_archived: bool,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<common::Page<Artifact>> {
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
                            include_archived,
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

    pub async fn download_artifact(
        &self,
        artifact_id: String,
        artifact_version_id: Option<String>,
    ) -> Result<ArtifactView> {
        let artifact = self.get_artifact(artifact_id, artifact_version_id).await?;
        let download_url = match artifact.remote_file_id.clone() {
            Some(remote_file_id) => Some(self.download_url(remote_file_id).await?),
            None => None,
        };
        Ok(ArtifactView {
            inner: artifact,
            download_url,
        })
    }

    pub async fn create_artifact(
        &self,
        input: CreateArtifactInput,
        file_path: Option<&Path>,
    ) -> Result<ArtifactView> {
        // Refuse before creating any rows, so a misconfigured server does not
        // leave a byteless version behind.
        let uploader = match file_path {
            Some(_) => Some(self.uploader.as_ref().context(
                "this server was started without a REST endpoint, so `file_path` is not supported",
            )?),
            None => None,
        };

        let channel = self.channel.clone();
        let created = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let input = input.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .create_artifact(CreateArtifactRequest {
                        artifact_id: input.artifact_id,
                        conversation_id: input.conversation_id,
                        title: input.title,
                        summary: input.summary,
                        authoring_kind: Some(input.authoring_kind as i32),
                        storage_class: input.storage_class.map(|value| value as i32),
                        created_via: input.created_via.map(|value| value as i32),
                        kind: input.kind,
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

        let (Some(uploader), Some(path)) = (uploader, file_path) else {
            return Ok(ArtifactView {
                inner: created,
                download_url: None,
            });
        };

        // The version row exists from here on: a failed upload must say so,
        // or the agent will retry the create and mint a duplicate artifact.
        let upload_context = format!(
            "artifact {} version {} was created, but uploading `{}` failed; do NOT create the artifact again",
            created.artifact_id,
            created.version,
            path.display()
        );
        uploader
            .upload_artifact_version_file(
                &created.organization_id,
                &created.artifact_version_id,
                path,
            )
            .await
            .context(upload_context)?;

        // Refresh so the returned artifact carries the uploaded file's name,
        // mime type, and remote_file_id, and mint the download link.
        let refreshed = self
            .get_artifact(
                created.artifact_id.clone(),
                Some(created.artifact_version_id.clone()),
            )
            .await
            .unwrap_or(created);
        let download_url = match refreshed.remote_file_id.clone() {
            Some(remote_file_id) => self.download_url(remote_file_id).await.ok(),
            None => None,
        };
        Ok(ArtifactView {
            inner: refreshed,
            download_url,
        })
    }

    async fn get_artifact(
        &self,
        artifact_id: String,
        artifact_version_id: Option<String>,
    ) -> Result<Artifact> {
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
