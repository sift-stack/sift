use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    metadata::v1::{
        ListMetadataKeysRequest, ListMetadataKeysResponse, ListMetadataUsageRequest,
        ListMetadataUsageResponse, ListMetadataValuesRequest, ListMetadataValuesResponse,
        MetadataKey, MetadataUsage, MetadataValue, metadata_service_client::MetadataServiceClient,
    },
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct MetadataService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl MetadataService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_metadata_keys(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<MetadataKey>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = MetadataServiceClient::new(channel);
                    client
                        .list_metadata_keys(ListMetadataKeysRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query metadata keys")?;

            let ListMetadataKeysResponse {
                metadata_keys,
                next_page_token,
            } = resp;
            if metadata_keys.is_empty() {
                break;
            }
            results.extend(metadata_keys);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    pub async fn list_metadata_values(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        metadata_key_name: String,
    ) -> Result<Vec<MetadataValue>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let metadata_key_name = metadata_key_name.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let metadata_key_name = metadata_key_name.clone();
                let token = token.clone();
                async move {
                    let mut client = MetadataServiceClient::new(channel);
                    client
                        .list_metadata_values(ListMetadataValuesRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                            metadata_key_name,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query metadata values")?;

            let ListMetadataValuesResponse {
                metadata_values,
                next_page_token,
            } = resp;
            if metadata_values.is_empty() {
                break;
            }
            results.extend(metadata_values);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    pub async fn list_metadata_usage(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<MetadataUsage>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = MetadataServiceClient::new(channel);
                    client
                        .list_metadata_usage(ListMetadataUsageRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query metadata usage")?;

            let ListMetadataUsageResponse {
                metadata_usages,
                next_page_token,
            } = resp;
            if metadata_usages.is_empty() {
                break;
            }
            results.extend(metadata_usages);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
