use anyhow::{Context, Result, anyhow, bail};
use serde::Serialize;
use sift_rs::{
    SiftChannel,
    artifacts::v1::{
        Artifact, ArtifactAuthoringKind, CreateArtifactRequest, GetArtifactRequest,
        ListArtifactsRequest, ListArtifactsResponse,
        artifact_service_client::ArtifactServiceClient,
    },
    remote_files::v1::{
        GetRemoteFileDownloadUrlRequest, remote_file_service_client::RemoteFileServiceClient,
    },
};

use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Serialize)]
pub struct ArtifactView {
    #[serde(flatten)]
    pub inner: Artifact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

#[derive(Clone)]
pub struct ArtifactService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl ArtifactService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_artifacts(
        &self,
        conversation_id: Option<String>,
        include_archived: bool,
        limit: Option<u32>,
    ) -> Result<common::Page<Artifact>> {
        let (page_size, record_limit) = common::paging(limit);
        let mut page_token = String::new();
        let mut results = Vec::new();
        let mut has_more = false;

        loop {
            let channel = self.channel.clone();
            let conversation_id = conversation_id.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let conversation_id = conversation_id.clone();
                let token = token.clone();
                async move {
                    let mut client = ArtifactServiceClient::new(channel);
                    client
                        .list_artifacts(ListArtifactsRequest {
                            conversation_id,
                            page_size,
                            page_token: token,
                            include_archived,
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

    pub async fn get_artifact(
        &self,
        artifact_id: String,
        artifact_version_id: Option<String>,
    ) -> Result<ArtifactView> {
        let artifact = self
            .get_artifact_inner(artifact_id, artifact_version_id)
            .await?;
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
        title: Option<String>,
        summary: Option<String>,
        conversation_id: Option<String>,
        artifact_id: Option<String>,
        authoring_kind: ArtifactAuthoringKind,
    ) -> Result<ArtifactView> {
        let channel = self.channel.clone();
        let created = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let artifact_id = artifact_id.clone();
            let conversation_id = conversation_id.clone();
            let title = title.clone();
            let summary = summary.clone();
            async move {
                let mut client = ArtifactServiceClient::new(channel);
                client
                    .create_artifact(CreateArtifactRequest {
                        artifact_id,
                        conversation_id,
                        title,
                        summary,
                        authoring_kind: Some(authoring_kind as i32),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create artifact")?
        .artifact
        .ok_or_else(|| anyhow!("create artifact response missing artifact"))?;

        Ok(ArtifactView {
            inner: created,
            download_url: None,
        })
    }

    async fn get_artifact_inner(
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
