use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    assets::v1::{
        Asset, ListAssetsRequest, ListAssetsResponse, UpdateAssetRequest,
        asset_service_client::AssetServiceClient,
    },
    metadata::v1::MetadataValue,
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

    /// Update a subset of an existing asset's fields. Per
    /// `protos/sift/assets/v1/assets.proto::UpdateAssetRequest`, the updatable
    /// fields are `tags`, `metadata`, `archived_date`, and `is_archived`. This
    /// service exposes `tags` and `metadata` only; archive flow has its own
    /// dedicated RPC and would be a separate tool.
    ///
    /// Both `tags` and `metadata` use REPLACE semantics — passing `Some(vec![])`
    /// clears the field. The caller is responsible for read-modify-write when
    /// appending.
    pub async fn update_asset(
        &self,
        asset_id: String,
        tags: Option<Vec<String>>,
        metadata: Option<Vec<MetadataValue>>,
    ) -> Result<Asset> {
        let mut asset = Asset {
            asset_id,
            ..Default::default()
        };
        let mut paths = Vec::new();

        if let Some(v) = tags {
            asset.tags = v;
            paths.push("tags".to_string());
        }
        if let Some(v) = metadata {
            asset.metadata = v;
            paths.push("metadata".to_string());
        }

        let grpc_channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let grpc_channel = grpc_channel.clone();
            let asset = asset.clone();
            let paths = paths.clone();
            async move {
                let mut client = AssetServiceClient::new(grpc_channel);
                client
                    .update_asset(UpdateAssetRequest {
                        asset: Some(asset),
                        update_mask: Some(FieldMask { paths }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update asset")?;

        resp.asset
            .ok_or_else(|| anyhow!("update_asset response missing asset"))
    }
}
