use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::metadata::v1::MetadataValue;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::{common::ListParams, data::MetadataEntry},
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateAssetParams {
    asset_id: String,
    tags: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
}

#[cfg(test)]
mod test;

#[tool_router(router = assets_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_assets",
        description = "
            List assets in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"assets\": [Asset, ...] }`. Each item is the full Sift `Asset` shape including `asset_id`, `name`,
                tags, metadata, timestamps, and archive state.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `asset_id`, `name`, `name_lower`, `tag_id`, `tag_name`, `created_date`, `modified_date`,
                `archived_date`, `is_archived`, `created_by_user_id`, `modified_by_user_id`, `metadata`.
                Reference metadata entries as `metadata.{key}` (e.g. `metadata.vehicle_type == \"rover\"`).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`, `archived_date`. Default sort is `created_date desc` (newest first).
                Example: `\"created_date desc,modified_date\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching assets (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Always prefer narrowing the filter over relying on `limit` — very large unfiltered listings can be slow.
              - Use `is_archived == false` to exclude archived assets unless they're explicitly needed.
        ",
        annotations(title = "assets_router/list_assets", read_only_hint = true)
    )]
    pub async fn list_assets(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            limit,
            order_by,
        }) = params;

        let out = self
            .asset_service
            .list_assets(filter, order_by, limit)
            .await
            .map(|assets| serde_json::json!({ "assets": assets }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "update_asset",
        description = "
            Update an existing asset's tags and/or metadata. Wraps `assets/v1 UpdateAsset`.

            Output:
              - `{ \"asset\": Asset, \"next_step\": string }`. The returned `Asset` is the
                post-update state from the server.

            Parameters:
              - `asset_id`: required; the id of the asset to update.
              - `tags`: optional; REPLACES the asset's full tag list with this array. To add a
                single tag, first read the current tags via `list_assets` filtered by
                `asset_id == \"<id>\"` and send the union. Pass `[]` to clear all tags.
              - `metadata`: optional; REPLACES the asset's full metadata list. Each entry is
                `{ \"name\": \"<key>\", \"value\": <scalar> }` where `value` is a string,
                number, or boolean. Pass `[]` to clear. The key must already exist in the
                organization's metadata schema (managed via the `metadata/v1` service); this
                tool attaches values to existing keys, it does not create new keys.

              At least one of `tags` or `metadata` must be set; otherwise the update is a no-op
              and the tool returns `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `asset_id` is empty or neither `tags` nor `metadata` is
                provided.
              - `RESOURCE_NOT_FOUND` if no asset matches `asset_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. server rejecting a metadata
                key that does not exist).

            Guidance:
              - This is a write. CONFIRM the target asset and the full proposed `tags` /
                `metadata` lists with the user before invoking — silent truncation is the most
                common foot-gun because the operation is REPLACE, not merge.
              - For appends, always read-modify-write: list the asset by id, append your new
                entry to the existing collection, then call this tool with the union.
              - The asset proto has no `description` field — there is no equivalent to update.
        ",
        annotations(title = "assets_router/update_asset", read_only_hint = false)
    )]
    pub async fn update_asset(
        &self,
        params: Parameters<UpdateAssetParams>,
    ) -> error::McpResult {
        let Parameters(UpdateAssetParams {
            asset_id,
            tags,
            metadata,
        }) = params;

        if asset_id.is_empty() {
            return Err(ErrorData::invalid_params("`asset_id` must not be empty", None));
        }
        if tags.is_none() && metadata.is_none() {
            return Err(ErrorData::invalid_params(
                "at least one of `tags` or `metadata` must be provided",
                None,
            ));
        }

        let metadata =
            metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let asset = self
            .asset_service
            .update_asset(asset_id, tags, metadata)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Updated asset `{}` ({}). Surface the new state to the user and confirm the change \
             matches their intent. Remember: tags and metadata are REPLACE operations — verify \
             nothing was unintentionally dropped.",
            asset.name, asset.asset_id,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "asset": asset,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}
