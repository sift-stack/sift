use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CampaignListParams {
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
    include_archived: Option<bool>,
    organization_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReviewCampaignsParams {
    campaign_ids: Vec<String>,
    organization_id: Option<String>,
}

#[tool_router(router = campaigns_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_campaigns",
        description = "
            List campaigns in Sift, optionally filtered by a CEL expression and ordered by one or more fields.
            Report review summaries are always skipped here for speed; use `review_campaigns` to fetch them.

            Output:
              - `{ \"campaigns\": [Campaign, ...] }`. Each item is the full Sift `Campaign` shape including
                `campaign_id`, `client_key`, `name`, `description`, `is_archived`, `archived_date`,
                `created_by_user_id`, `modified_by_user_id`, `created_from_campaign_id`, `tags`, `metadata`,
                and timestamps, plus `reports`: an array of `{ report_id, report_name }` per report in the
                campaign.
              - `reports_include_summaries` is always `false` in this tool's output, and each report's `num_*`
                fields (`num_annotations`, `num_passed_rules`, `num_accepted_rules`, `num_failed_rules`,
                `num_open_rules`) are omitted. Call `review_campaigns` with the `campaign_id`s you need to get
                those counts.
              - Fields at their proto3 default are OMITTED from the JSON: a missing `is_archived` key means
                `false`, not \"unknown\".

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `campaign_id`, `client_key`, `name`, `description`, `created_by_user_id`, `tag_id`, `tag_name`,
                `report_id`, `report_name`, `run_id`, `is_archived`, and `metadata`. Reference metadata entries
                as `metadata.{key}` (e.g. `metadata.program == \"artemis\"`).
                When filtering or searching, use `name.matches(\"(?i)launch\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Launch\")` silently misses `launch-2024`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`. Default sort is `created_date desc` (newest first).
                Example: `\"created_date desc,name\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `include_archived`: optional, defaults to `false`. Archived campaigns are excluded from the
                query entirely unless this is `true` -- filtering on `is_archived == true` alone is not enough.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Campaign review summaries are expensive to compute; fetch them with `review_campaigns` only
                when the user actually wants the rollup, not on every listing.
              - Default add `is_archived == false` to the filter. Include archived campaigns only when the user
                explicitly asks for them, and set `include_archived` to `true` at the same time.
        ",
        annotations(title = "campaigns/list_campaigns", read_only_hint = true)
    )]
    pub async fn list_campaigns(&self, params: Parameters<CampaignListParams>) -> error::McpResult {
        let Parameters(CampaignListParams {
            filter,
            order_by,
            limit,
            include_archived,
            organization_id,
        }) = params;

        let campaigns = self
            .campaign_service
            .list_campaigns(filter, order_by, limit, include_archived, organization_id)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "campaigns": campaigns }),
        ))
    }

    #[tool(
        name = "review_campaigns",
        description = "
            Fetch report review summaries -- annotation totals and rule pass/fail classifications -- for one or
            more campaigns, identified by `campaign_id`. This is the expensive counterpart to `list_campaigns`,
            which always omits this data. Resolve campaign ids with `list_campaigns` first if you only have
            campaign names.

            Output:
              - `{ \"summaries\": { \"<campaign_id>\": [CampaignReport, ...], ... } }`. Each `CampaignReport`
                has `report_id`, `report_name`, `num_annotations` (total annotations across the report's rules),
                `num_passed_rules` (rules never triggered), `num_accepted_rules` (rules whose annotations are all
                accepted), `num_failed_rules` (rules with any failed annotation), and `num_open_rules` (rules
                with an open annotation and none failed).
              - Fields at their proto3 default (`0`) are OMITTED from the JSON: a missing `num_annotations` key
                means `0`, not \"unknown\".
              - A `campaign_id` with no reports, or that does not exist, may be absent from the map or map to an
                empty list; do not assume every requested id is present.

            Parameters:
              - `campaign_ids`: required, one or more campaign ids. Must be non-empty.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.

            Errors:
              - `INVALID_PARAMS` if `campaign_ids` is empty.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Campaign review summaries are expensive; fetch only when the user asks for the rollup of
                annotations and rule outcomes, not as part of routine listing.
              - Pass every `campaign_id` you need in one call rather than calling this once per campaign.
        ",
        annotations(title = "campaigns/review_campaigns", read_only_hint = true)
    )]
    pub async fn review_campaigns(
        &self,
        params: Parameters<ReviewCampaignsParams>,
    ) -> error::McpResult {
        let Parameters(ReviewCampaignsParams {
            campaign_ids,
            organization_id,
        }) = params;

        if campaign_ids.is_empty() {
            return Err(ErrorData::invalid_params(
                "`campaign_ids` must include at least one campaign id",
                None,
            ));
        }

        let summaries = self
            .campaign_service
            .review_campaigns(campaign_ids, organization_id)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "summaries": summaries }),
        ))
    }
}
