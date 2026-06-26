use crate::policy::{RetryPolicy, with_retry};
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    docs::v1::{
        ReadDocRequest, ReadDocResponse, SearchDocsRequest, SearchDocsResponse,
        docs_service_client::DocsServiceClient,
    },
};

#[cfg(test)]
mod test;

/// Read-only client for Sift's documentation service (`sift.docs.v1.DocsService`).
/// Auth rides the shared gRPC channel, so this needs nothing beyond the channel.
#[derive(Clone)]
pub struct DocsService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl DocsService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn search_docs(
        &self,
        query: String,
        max_results: Option<u32>,
    ) -> Result<SearchDocsResponse> {
        // Forward to the proto's int32. 0 lets the service apply its documented
        // default of 10 hits and hard cap of 25.
        let max_results = max_results.unwrap_or(0) as i32;

        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let query = query.clone();
            async move {
                let mut client = DocsServiceClient::new(channel);
                client
                    .search_docs(SearchDocsRequest { query, max_results })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to search docs")
    }

    pub async fn read_doc(
        &self,
        path: String,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ReadDocResponse> {
        // Forward to the proto's int32. 0 lets the service apply its defaults
        // (offset 1, all remaining lines).
        let offset = offset.unwrap_or(0) as i32;
        let limit = limit.unwrap_or(0) as i32;

        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let path = path.clone();
            async move {
                let mut client = DocsServiceClient::new(channel);
                client
                    .read_doc(ReadDocRequest {
                        path,
                        offset,
                        limit,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to read doc")
    }
}
