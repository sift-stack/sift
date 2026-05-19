use anyhow::Context;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::{
    assets::v1::{ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient},
    runs::v2::{ListRunsRequest, ListRunsResponse, run_service_client::RunServiceClient},
};

use crate::{error, server::SiftMcpServer};

#[cfg(test)]
mod test;

const PAGE_SIZE: u32 = 1000;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetParams {
    filter: String,
    limit: Option<u32>,
}

#[tool_router(router = resource_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "get_asset",
        description = "
            Retrieve and filter assets in Sift. The `filter` parameter is a Common Expression Language (CEL).
            Available fields to filter by are `asset_id`, `created_by_user_id`, `modified_by_user_id`,
            `created_date`, `modified_date`, `name`, 'name_lower', `tag_id`, `tag_name`, 'archived_date', `is_archived`, and `metadata`.
            Metadata can be used in filters by using `metadata.{metadata_key_name}` as the field name.
        ",
        annotations(title = "Resource/get_asset", read_only_hint = true)
    )]
    pub async fn get_asset(&self, params: Parameters<GetParams>) -> error::McpResult {
        let Parameters(GetParams { filter, limit }) = params;
        let (page_size, record_limit) = paging(limit);

        let mut client = AssetServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_assets(ListAssetsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    ..Default::default()
                })
                .await
                .map_err(error::from_grpc_status)?;

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
        let out = serde_json::to_value(&results)
            .context("failed to serialize assets")
            .map_err(error::from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "get_run",
        description = "
            Retrieve and filter runs in Sift. The `filter` parameter is a Common Expression Language (CEL).
            Available fields to filter by are `run_id` `organization_id`, `asset_id`, `asset_name`, `client_key`, `name`,
            `description`, `created_by_user_id`, `modified_by_user_id`, `created_date`, `modified_date`, `start_time`,
            `stop_time`, `tag_id`, `asset_tag_id`, `duration`, 'duration_string', `annotation_comments_count`, `annotation_state`,
            `archived_date`, `is_archived`, and `metadata`. Metadata can be used in filters by using
            `metadata.{metadata_key_name}` as the field name. `duration` is in the format of elapsed seconds and `duration_string`
            allows for `h`, `m`, `s`, `ms` suffixes (example: `duration_string > duration('10h')).
        ",
        annotations(title = "Resource/get_run", read_only_hint = true)
    )]
    pub async fn get_run(&self, params: Parameters<GetParams>) -> error::McpResult {
        let Parameters(GetParams { filter, limit }) = params;
        let (page_size, record_limit) = paging(limit);

        let mut client = RunServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_runs(ListRunsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    ..Default::default()
                })
                .await
                .map_err(error::from_grpc_status)?;

            let ListRunsResponse {
                runs,
                next_page_token,
            } = resp.into_inner();
            if runs.is_empty() {
                break;
            }
            results.extend(runs);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        let out = serde_json::to_value(&results)
            .context("failed to serialize runs")
            .map_err(error::from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}

fn paging(limit: Option<u32>) -> (u32, usize) {
    match limit {
        Some(lim) if lim <= PAGE_SIZE => (lim, lim as usize),
        _ => (PAGE_SIZE, usize::MAX),
    }
}
