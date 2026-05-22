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
}

impl AssetService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_assets(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Asset>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = AssetServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_assets(ListAssetsRequest {
                    filter: filter.to_string(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query assets")?;

            let ListAssetsResponse {
                assets,
                next_page_token,
            } = resp.into_inner();
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
