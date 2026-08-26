use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
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
    tool::common::{MetadataEntry, list_body, url_clause, with_urls},
};

#[cfg(test)]
mod test;

const MAX_UPDATE_ANNOTATIONS: usize = 1_000;

fn upstream_error_message(error: &anyhow::Error) -> String {
    if let Some(status) = error.downcast_ref::<tonic::Status>() {
        format!("{}: {}", status.code(), status.message())
    } else {
        error.to_string()
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AnnotationListParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) organization_id: Option<String>,
    pub(crate) fields: Option<Vec<String>>,
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
    annotation_ids: Vec<String>,
    name: Option<String>,
    description: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
    assigned_to_user_id: Option<String>,
    state: Option<String>,
    tags: Option<Vec<String>>,
    linked_channel_ids: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
    is_archived: Option<bool>,
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
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many items match `filter`.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — narrow `filter` or raise `limit` and ask again.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `annotation_id`, `start_time`, `end_time`, `created_date`, `modified_date`, `run_id`, `name`,
                `description`, `state`, `created_by_user_id`, `created_by_rule_condition_version_id`, `rule_id`,
                `annotation_type`, `tag_name`, `report_id`, `asset_id`, `asset_name`, `pending`, `assignee`,
                `campaign_reports`, `metadata`, `archived_date`, `is_archived`. Reference metadata entries as
                `metadata.{key}` (e.g. `metadata.severity == \"high\"`).
                When filtering or searching, use `name.matches(\"(?i)vibration\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Vibration\")` silently misses `vibration-check`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `modified_date`, `start_time`, `end_time`, `name`, `description`. Default sort is `created_date desc`
                (newest first). Example: `\"start_time desc,name\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens, so `asset_id`, `assetId` and
                `asset-id` all work. Any name that matched nothing on any returned item
                is listed in `unmatched_fields`; an empty page reports none, since it
                says nothing about whether a name was spelled right.
                Reach for this whenever you need only a few fields: full objects are wide,
                and a large listing can exceed the response size limit without it.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Narrow with `run_id == \"...\"` or `asset_id == \"...\"` when known — those are the most selective.
              - Default add `is_archived == false` to the filter. Include archived annotations only when the user
                explicitly asks for them.
        ",
        annotations(title = "annotations/list_annotations", read_only_hint = true)
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
            fields,
        }) = params;

        let page = self
            .annotation_service
            .list_annotations(filter, order_by, limit, organization_id)
            .await
            .map_err(from_anyhow)?;

        let annotations = with_urls(&page.items, |a| {
            self.url_service.build_annotation_url(&a.annotation_id).ok()
        })?;

        Ok(CallToolResult::structured(list_body(
            "annotations",
            annotations,
            fields,
            page.has_more,
        )))
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
        annotations(
            title = "annotations/create_annotation",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_annotation(
        &self,
        params: Parameters<CreateAnnotationParams>,
    ) -> error::McpResult {
        self.require_create()?;

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
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_annotation",
        description = "
            Update one or more existing annotations. Uses `annotations/v1 BatchArchiveAnnotations` when archiving
            and one `UpdateAnnotation` request per annotation when unarchiving.

            Output:
              - `{ \"annotations\": [Annotation, ...], \"failures\": [...], \"batch_archive_error\": object|null,
                \"not_attempted\": [string, ...], \"archive_skipped\": bool, \"next_step\": string }`. Successful
                annotations include a `url` field when the host can be derived. Each individual failure includes
                `annotation_id` and an upstream `message`. `batch_archive_error` includes `annotation_ids` and an
                upstream `message` only for a failed `BatchArchiveAnnotations` request. Partial failures set the tool result's
                `isError` flag. `archive_skipped` is true when the requested archive-state change was not attempted
                because field updates failed.

            Parameters:
              - `annotation_ids`: required list of 1 to 1000 annotation ids. The same changes are applied to every
                annotation. This replaces the former `annotation_id` parameter; pass a single annotation as a
                one-element list.
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
              - `is_archived`: optional archive state. `true` uses one batch-archive request; `false` uses one
                per-annotation update request that clears the annotation's delete date. When combined with other
                fields, annotations are updated before their archive state changes.

              At least one updatable field must be set; otherwise the tool returns `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `annotation_ids` is empty, contains an empty id, exceeds 1000 ids, `state` is
                unrecognized, or no updatable field is set.
              - Individual upstream failures, including unarchive failures, are returned in `failures` next to
                successful results. Follow each failure's guidance and retry only eligible failed ids.
              - Backend-wide failures stop later batches. Their ids are returned in `not_attempted` without an API
                request.
              - Batch archive failures are returned in `batch_archive_error`. The archive outcome may be unknown.

            Guidance:
              - This is a write. CONFIRM every target and the full proposed values with the user before invoking —
                `tags`, `linked_channel_ids`, and `metadata` are REPLACE operations, not merges.
              - General field updates and unarchive issue one API request per annotation and are not atomic. Requests
                run in batches of up to 50. A backend-wide failure stops later batches. Archive uses one batch API
                request. Requested archive-state changes begin only after all individual updates succeed; otherwise
                `archive_skipped` is true.
              - For appends, read the current annotation via `list_annotations` filtered by
                `annotation_id == \"<id>\"`, then send the union.
        ",
        annotations(
            title = "annotations/update_annotation",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn update_annotation(
        &self,
        params: Parameters<UpdateAnnotationParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(UpdateAnnotationParams {
            annotation_ids,
            name,
            description,
            start_time_unix_nanos,
            end_time_unix_nanos,
            assigned_to_user_id,
            state,
            tags,
            linked_channel_ids,
            metadata,
            is_archived,
        }) = params;

        if annotation_ids.is_empty() {
            return Err(ErrorData::invalid_params(
                "`annotation_ids` must contain at least one id",
                None,
            ));
        }

        if annotation_ids.len() > MAX_UPDATE_ANNOTATIONS {
            return Err(ErrorData::invalid_params(
                format!("`annotation_ids` must contain at most {MAX_UPDATE_ANNOTATIONS} ids"),
                None,
            ));
        }

        if annotation_ids.iter().any(String::is_empty) {
            return Err(ErrorData::invalid_params(
                "`annotation_ids` must not contain empty ids",
                None,
            ));
        }

        let has_field_update = name.is_some()
            || description.is_some()
            || start_time_unix_nanos.is_some()
            || end_time_unix_nanos.is_some()
            || assigned_to_user_id.is_some()
            || state.is_some()
            || tags.is_some()
            || linked_channel_ids.is_some()
            || metadata.is_some();
        if !has_field_update && is_archived.is_none() {
            return Err(ErrorData::invalid_params(
                "at least one updatable field must be provided",
                None,
            ));
        }

        let state = state.map(|s| parse_annotation_state(&s)).transpose()?;
        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());
        let requested_ids = annotation_ids.clone();
        let requested_count = annotation_ids.len();
        let outcome = self
            .annotation_service
            .update_annotations(
                annotation_ids,
                name,
                description,
                start_time_unix_nanos,
                end_time_unix_nanos,
                assigned_to_user_id,
                state,
                tags,
                linked_channel_ids,
                metadata,
                is_archived,
            )
            .await
            .map_err(from_anyhow)?;

        let updated_count = outcome.annotations.len();
        let failures = outcome
            .failures
            .into_iter()
            .map(|failure| {
                serde_json::json!({
                    "annotation_id": failure.annotation_id,
                    "message": upstream_error_message(&failure.error),
                })
            })
            .collect::<Vec<_>>();
        let failure_count = failures.len();
        let not_attempted = outcome.not_attempted;
        let not_attempted_count = not_attempted.len();
        let batch_archive_error = outcome.batch_archive_error.map(|error| {
            serde_json::json!({
                "annotation_ids": requested_ids,
                "message": upstream_error_message(&error),
            })
        });
        let has_errors =
            failure_count > 0 || not_attempted_count > 0 || batch_archive_error.is_some();

        let annotations = outcome.annotations;
        let annotations = with_urls(&annotations, |annotation| {
            self.url_service
                .build_annotation_url(&annotation.annotation_id)
                .ok()
        })?;
        let next_step = if batch_archive_error.is_some() {
            format!(
                "Completed field updates for {updated_count} of {requested_count} annotation(s), but batch archive \
                 failed. Archive state may be unknown. Verify the targets with `list_annotations` before retrying.",
            )
        } else if outcome.archive_skipped {
            if not_attempted_count > 0 {
                format!(
                    "Updated {updated_count} of {requested_count} annotation(s); {failure_count} failed and \
                     {not_attempted_count} were not attempted after a backend-wide failure. The archive state change was not \
                     attempted. Review `failures` before retrying eligible failed and not-attempted ids."
                )
            } else {
                format!(
                    "Updated {updated_count} of {requested_count} annotation(s); {failure_count} failed. The archive state change was \
                     not attempted because individual updates failed. Review `failures`, follow their guidance, and \
                     retry only eligible failed ids."
                )
            }
        } else if not_attempted_count > 0 {
            format!(
                "Updated {updated_count} of {requested_count} annotation(s); {failure_count} failed and \
                 {not_attempted_count} were not attempted after a backend-wide failure. Review `failures` before \
                 retrying eligible failed and not-attempted ids."
            )
        } else if failure_count > 0 {
            format!(
                "Updated {updated_count} of {requested_count} annotation(s); {failure_count} failed. Successful \
                 annotations are already changed. Review `failures`, follow their guidance, and retry only eligible \
                 failed ids."
            )
        } else {
            format!(
                "Updated {updated_count} annotation(s). Surface the new states and links to the user and confirm the \
                 changes match their intent. Remember: tags, linked channels, and metadata are REPLACE operations."
            )
        };

        let structured = serde_json::json!({
            "annotations": annotations,
            "failures": failures,
            "not_attempted": not_attempted,
            "batch_archive_error": batch_archive_error,
            "archive_skipped": outcome.archive_skipped,
            "next_step": next_step,
        });
        let mut result = if has_errors {
            CallToolResult::structured_error(structured)
        } else {
            CallToolResult::structured(structured)
        };
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}
