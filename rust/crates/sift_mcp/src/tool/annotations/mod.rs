use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::{
    annotations::v1::{AnnotationState, AnnotationType},
    metadata::v1::MetadataValue,
};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::{MetadataEntry, url_clause, with_urls},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AnnotationListParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) organization_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateAnnotationParams {
    name: String,
    description: Option<String>,
    start_time_unix_nanos: i64,
    end_time_unix_nanos: i64,
    annotation_type: String,
    state: Option<String>,
    assets: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    linked_channel_ids: Option<Vec<String>>,
    run_id: Option<String>,
    assign_to_user_id: Option<String>,
    metadata: Option<Vec<MetadataEntry>>,
    organization_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateAnnotationParams {
    annotation_id: String,
    name: Option<String>,
    description: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
    assigned_to_user_id: Option<String>,
    state: Option<String>,
    tags: Option<Vec<String>>,
    linked_channel_ids: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
}

fn parse_annotation_type(s: &str) -> Result<AnnotationType, ErrorData> {
    match s.to_ascii_lowercase().as_str() {
        "data_review" => Ok(AnnotationType::DataReview),
        "phase" => Ok(AnnotationType::Phase),
        other => Err(ErrorData::invalid_params(
            format!("unknown `annotation_type` `{other}`; expected `data_review` or `phase`"),
            None,
        )),
    }
}

fn parse_annotation_state(s: &str) -> Result<AnnotationState, ErrorData> {
    match s.to_ascii_lowercase().as_str() {
        "open" => Ok(AnnotationState::Open),
        "flagged" => Ok(AnnotationState::Flagged),
        "resolved" => Ok(AnnotationState::Resolved),
        other => Err(ErrorData::invalid_params(
            format!("unknown `state` `{other}`; expected `open`, `flagged`, or `resolved`"),
            None,
        )),
    }
}

#[tool_router(router = annotations_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_annotations",
        description = "
            List annotations in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"annotations\": [Annotation, ...] }`. Each item is the full Sift `Annotation` shape including
                `annotation_id`, `name`, `description`, `start_time`, `end_time`, `state`, `annotation_type`,
                `run_id`, `asset_ids`, `tags`, `linked_channels`, metadata, timestamps, and archive state, plus an
                added `url` field with the annotation's Sift web link (`<host>/annotation/<annotation_id>`). `url`
                is omitted when the host can't be derived. Surface these links to the user when presenting
                annotations.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `annotation_id`, `start_time`, `end_time`, `created_date`, `modified_date`, `run_id`, `name`,
                `description`, `state`, `created_by_user_id`, `created_by_rule_condition_version_id`, `rule_id`,
                `annotation_type`, `tag_name`, `report_id`, `asset_id`, `asset_name`, `pending`, `assignee`,
                `campaign_reports`, `metadata`, `archived_date`, `is_archived`. Reference metadata entries as
                `metadata.{key}` (e.g. `metadata.severity == \"high\"`).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `modified_date`, `start_time`, `end_time`, `name`, `description`. Default sort is `created_date desc`
                (newest first). Example: `\"start_time desc,name\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching annotations (paginated server-side).
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Narrow with `run_id == \"...\"` or `asset_id == \"...\"` when known — those are the most selective.
              - Use `is_archived == false` to exclude archived annotations unless they're explicitly needed.
        ",
        annotations(title = "annotations_router/list_annotations", read_only_hint = true)
    )]
    pub async fn list_annotations(
        &self,
        params: Parameters<AnnotationListParams>,
    ) -> error::McpResult {
        let Parameters(AnnotationListParams {
            filter,
            order_by,
            limit,
            organization_id,
        }) = params;

        let annotations = self
            .annotation_service
            .list_annotations(filter, order_by, limit, organization_id)
            .await
            .map_err(from_anyhow)?;

        let annotations = with_urls(&annotations, |a| {
            self.url_service.build_annotation_url(&a.annotation_id).ok()
        })?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "annotations": annotations }),
        ))
    }

    #[tool(
        name = "create_annotation",
        description = "
            Create a new annotation over a time range. Wraps `annotations/v1 CreateAnnotation`.

            Output:
              - `{ \"annotation\": Annotation, \"annotation_url\": string|null, \"next_step\": string }`. The
                returned `Annotation` is the server-assigned state including its new `annotation_id`;
                `annotation_url` is its Sift web link (`<host>/annotation/<annotation_id>`), or null when the host
                can't be derived.

            Parameters:
              - `name`: required; the annotation's display name.
              - `description`: optional free-text description.
              - `start_time_unix_nanos` / `end_time_unix_nanos`: required time bounds in Unix nanoseconds.
                `end_time_unix_nanos` must be >= `start_time_unix_nanos`.
              - `annotation_type`: required; one of `data_review` or `phase`.
              - `state`: optional; one of `open`, `flagged`, `resolved`. MUST be omitted when `annotation_type`
                is `phase` (the server rejects a phase annotation with a state).
              - `assets`: optional list of asset NAMES to associate.
              - `tags`: optional list of tag names to associate. Names that do not yet exist are created.
              - `linked_channel_ids`: optional list of channel ids to link. Only plain channels are supported;
                bit-field and calculated-channel links are not exposed here.
              - `run_id`: optional id of the run to associate.
              - `assign_to_user_id`: optional id of the user to assign the annotation to.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries; `value` is a
                string, number, or boolean. A `name` that does not yet exist in the organization's metadata
                schema is created on the fly with type inferred from `value`; for an existing key, `value`'s
                type must match the key's current type.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.

            Errors:
              - `INVALID_PARAMS` if `name` is empty, the time range is inverted, `annotation_type`/`state` is not a
                recognized value, a `state` is supplied for a `phase` annotation, the `metadata` list contains
                duplicate key names, or a value's type does not match an existing metadata key's type.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. missing run/asset).

            Guidance:
              - This is a write. CONFIRM the time range, type, and associations with the user before invoking.
        ",
        annotations(title = "annotations_router/create_annotation", read_only_hint = false)
    )]
    pub async fn create_annotation(
        &self,
        params: Parameters<CreateAnnotationParams>,
    ) -> error::McpResult {
        let Parameters(CreateAnnotationParams {
            name,
            description,
            start_time_unix_nanos,
            end_time_unix_nanos,
            annotation_type,
            state,
            assets,
            tags,
            linked_channel_ids,
            run_id,
            assign_to_user_id,
            metadata,
            organization_id,
        }) = params;

        if name.is_empty() {
            return Err(ErrorData::invalid_params("`name` must not be empty", None));
        }
        if end_time_unix_nanos < start_time_unix_nanos {
            return Err(ErrorData::invalid_params(
                "`end_time_unix_nanos` must be >= `start_time_unix_nanos`",
                None,
            ));
        }

        let annotation_type = parse_annotation_type(&annotation_type)?;
        let state = state.map(|s| parse_annotation_state(&s)).transpose()?;

        if annotation_type == AnnotationType::Phase && state.is_some() {
            return Err(ErrorData::invalid_params(
                "`state` must be omitted when `annotation_type` is `phase`",
                None,
            ));
        }

        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let annotation = self
            .annotation_service
            .create_annotation(
                name,
                description,
                start_time_unix_nanos,
                end_time_unix_nanos,
                annotation_type,
                state,
                assets,
                tags,
                linked_channel_ids,
                run_id,
                assign_to_user_id,
                metadata,
                organization_id,
            )
            .await
            .map_err(from_anyhow)?;

        let annotation_url = self
            .url_service
            .build_annotation_url(&annotation.annotation_id)
            .ok();
        let next_step = format!(
            "Created annotation `{}` ({}).{} Surface the new annotation to the user and confirm it \
             matches their intent.",
            annotation.name,
            annotation.annotation_id,
            url_clause(annotation_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "annotation": annotation,
            "annotation_url": annotation_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_annotation",
        description = "
            Update an existing annotation. Wraps `annotations/v1 UpdateAnnotation`.

            Output:
              - `{ \"annotation\": Annotation, \"annotation_url\": string|null, \"next_step\": string }`. The
                returned `Annotation` is the post-update state from the server; `annotation_url` is its Sift web
                link, or null when the host can't be derived.

            Parameters:
              - `annotation_id`: required; the id of the annotation to update.
              - `name`: optional new name.
              - `description`: optional new description.
              - `start_time_unix_nanos` / `end_time_unix_nanos`: optional new time bounds in Unix nanoseconds.
              - `assigned_to_user_id`: optional new assignee user id.
              - `state`: optional; one of `open`, `flagged`, `resolved`.
              - `tags`: optional; REPLACES the full tag list. Pass `[]` to clear all tags.
              - `linked_channel_ids`: optional; REPLACES the full linked-channel list with plain channel links.
                Pass `[]` to clear. Bit-field and calculated-channel links are not exposed here.
              - `metadata`: optional; REPLACES the full metadata list. Each entry is
                `{ \"name\": \"<key>\", \"value\": <scalar> }`. Pass `[]` to clear.

              At least one updatable field must be set; otherwise the tool returns `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `annotation_id` is empty, `state` is unrecognized, or no updatable field is set.
              - `RESOURCE_NOT_FOUND` if no annotation matches `annotation_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - This is a write. CONFIRM the target and the full proposed values with the user before invoking —
                `tags`, `linked_channel_ids`, and `metadata` are REPLACE operations, not merges.
              - For appends, read the current annotation via `list_annotations` filtered by
                `annotation_id == \"<id>\"`, then send the union.
        ",
        annotations(title = "annotations_router/update_annotation", read_only_hint = false)
    )]
    pub async fn update_annotation(
        &self,
        params: Parameters<UpdateAnnotationParams>,
    ) -> error::McpResult {
        let Parameters(UpdateAnnotationParams {
            annotation_id,
            name,
            description,
            start_time_unix_nanos,
            end_time_unix_nanos,
            assigned_to_user_id,
            state,
            tags,
            linked_channel_ids,
            metadata,
        }) = params;

        if annotation_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`annotation_id` must not be empty",
                None,
            ));
        }

        let has_update = name.is_some()
            || description.is_some()
            || start_time_unix_nanos.is_some()
            || end_time_unix_nanos.is_some()
            || assigned_to_user_id.is_some()
            || state.is_some()
            || tags.is_some()
            || linked_channel_ids.is_some()
            || metadata.is_some();
        if !has_update {
            return Err(ErrorData::invalid_params(
                "at least one updatable field must be provided",
                None,
            ));
        }

        let state = state.map(|s| parse_annotation_state(&s)).transpose()?;
        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let annotation = self
            .annotation_service
            .update_annotation(
                annotation_id,
                name,
                description,
                start_time_unix_nanos,
                end_time_unix_nanos,
                assigned_to_user_id,
                state,
                tags,
                linked_channel_ids,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        let annotation_url = self
            .url_service
            .build_annotation_url(&annotation.annotation_id)
            .ok();
        let next_step = format!(
            "Updated annotation `{}` ({}).{} Surface the new state to the user and confirm the change \
             matches their intent. Remember: tags, linked channels, and metadata are REPLACE operations.",
            annotation.name,
            annotation.annotation_id,
            url_clause(annotation_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "annotation": annotation,
            "annotation_url": annotation_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}
