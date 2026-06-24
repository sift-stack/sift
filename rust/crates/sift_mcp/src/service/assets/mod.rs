use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    assets::v1::{
        Asset, ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient,
    },
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct AssetService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl AssetService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_assets(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Asset>> {
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
                    let mut client = AssetServiceClient::new(channel);
                    client
                        .list_assets(ListAssetsRequest {
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
            .context("failed to query assets")?;

            let ListAssetsResponse {
                assets,
                next_page_token,
            } = resp;
            if assets.is_empty() {
                break;
            }
            results.extend(assets);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
