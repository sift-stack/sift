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

use crate::tool::common::{
    bulk::{self, Op},
    bulk_create,
};
use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::{MetadataEntry, list_body, url_clause, with_urls},
};

#[cfg(test)]
mod test;

const MAX_UPDATE_ANNOTATIONS: usize = 1_000;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AnnotationListParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) organization_id: Option<String>,
    pub(crate) fields: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema)]
enum AnnotationTypeParam {
    #[serde(rename = "ANNOTATION_TYPE_DATA_REVIEW", alias = "data_review")]
    DataReview,
    #[serde(rename = "ANNOTATION_TYPE_PHASE", alias = "phase")]
    Phase,
}

impl From<AnnotationTypeParam> for AnnotationType {
    fn from(value: AnnotationTypeParam) -> Self {
        match value {
            AnnotationTypeParam::DataReview => Self::DataReview,
            AnnotationTypeParam::Phase => Self::Phase,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema)]
enum AnnotationStateParam {
    #[serde(rename = "ANNOTATION_STATE_OPEN", alias = "open")]
    Open,
    #[serde(
        rename = "ANNOTATION_STATE_FLAGGED",
        alias = "flagged",
        alias = "failed"
    )]
    Flagged,
    #[serde(
        rename = "ANNOTATION_STATE_RESOLVED",
        alias = "resolved",
        alias = "accepted"
    )]
    Resolved,
}

impl From<AnnotationStateParam> for AnnotationState {
    fn from(value: AnnotationStateParam) -> Self {
        match value {
            AnnotationStateParam::Open => Self::Open,
            AnnotationStateParam::Flagged => Self::Flagged,
            AnnotationStateParam::Resolved => Self::Resolved,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateAnnotationItem {
    name: String,
    description: Option<String>,
    start_time_unix_nanos: i64,
    end_time_unix_nanos: i64,
    annotation_type: AnnotationTypeParam,
    state: Option<AnnotationStateParam>,
    assets: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    linked_channel_ids: Option<Vec<String>>,
    /// Set from the shared top-level `run_id`.
    #[schemars(skip)]
    run_id: Option<String>,
    assign_to_user_id: Option<String>,
    metadata: Option<Vec<MetadataEntry>>,
    /// Set from the shared top-level `organization_id`.
    #[schemars(skip)]
    organization_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateAnnotationParams {
    /// 1–100 annotations to create. Pass one item to create one annotation.
    #[schemars(length(min = 1, max = 100))]
    #[schemars(with = "Vec<CreateAnnotationItem>")]
    items: Vec<serde_json::Value>,
    /// Approval host's zero-based selection of `items` to create. Omit when proposing.
    /// An empty selection performs no writes.
    selected_indices: Option<Vec<usize>>,
    /// Run associated with every item.
    run_id: Option<String>,
    /// Organization for every item, when the caller belongs to several.
    organization_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateAnnotationParams {
    annotation_ids: Vec<String>,
    selected_indices: Option<Vec<usize>>,
    name: Option<String>,
    description: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
    assigned_to_user_id: Option<String>,
    state: Option<AnnotationStateParam>,
    tags: Option<Vec<String>>,
    linked_channel_ids: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
    is_archived: Option<bool>,
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
                is omitted when the host can't be derived. Present each annotation to the user as a Markdown
                link with its name as the text and `url` as the target.
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
                `metadata[\"<key>\"]` (e.g. `metadata[\"severity\"] == \"high\"`).
                Compare `state` with `ANNOTATION_STATE_OPEN`, `ANNOTATION_STATE_FLAGGED`, or
                `ANNOTATION_STATE_RESOLVED`. Compare `annotation_type` with `ANNOTATION_TYPE_DATA_REVIEW` or
                `ANNOTATION_TYPE_PHASE`. Use these exact string enum values in quoted filter operands.
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

        Ok(list_body("annotations", annotations, fields, page.has_more).into_result(None))
    }

    #[tool(
        name = "create_annotation",
        description = "
            Create one or more annotations, each over a time range. Wraps `annotations/v1 CreateAnnotation`
            once per item.

            Output:
              - `{ requested_count, selected_count, not_selected, created_count, successes, failures,
                not_attempted, next_step }`. `successes` holds `{ index, result }` per created item, where
                `index` is the item's zero-based position in `items`. `failures` holds `{ index, error,
                outcome_unknown }`. `not_attempted` lists items skipped after a backend-wide error. Any
                failure sets `isError`. Each `result` is:
                - `{ \"annotation\": Annotation, \"annotation_url\": string|null, \"next_step\": string }`. The
                  returned `Annotation` is the server-assigned state including its new `annotation_id`;
                  `annotation_url` is its Sift web link (`<host>/annotation/<annotation_id>`), or null when the host
                  can't be derived.

            Parameters:
              - `items`: required array of 1–100 objects, one per annotation. Pass one item to create one
                annotation. Each item takes:
                - `name`: required; the annotation's display name.
                - `description`: optional free-text description.
                - `start_time_unix_nanos` / `end_time_unix_nanos`: required time bounds in Unix nanoseconds.
                  `end_time_unix_nanos` must be >= `start_time_unix_nanos`.
                - `annotation_type`: required; `ANNOTATION_TYPE_DATA_REVIEW` or `ANNOTATION_TYPE_PHASE`.
                - `state`: optional; `ANNOTATION_STATE_OPEN`, `ANNOTATION_STATE_FLAGGED`, or
                  `ANNOTATION_STATE_RESOLVED`. MUST be omitted when `annotation_type` is `ANNOTATION_TYPE_PHASE`
                  (the server rejects a phase annotation with a state).
                - `assets`: optional list of asset NAMES to associate.
                - `tags`: optional list of tag names to associate. Names that do not yet exist are created.
                - `linked_channel_ids`: optional list of channel ids to link. Only plain channels are supported;
                  bit-field and calculated-channel links are not exposed here.
                - `assign_to_user_id`: optional id of the user to assign the annotation to.
                - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries; `value` is a
                  string, number, or boolean. A `name` that does not yet exist in the organization's metadata
                  schema is created on the fly with type inferred from `value`; for an existing key, `value`'s
                  type must match the key's current type.
              - `run_id`: optional id of the run to associate with EVERY item. Set it here, never
                inside an item.
              - `organization_id`: optional, shared by every item. Required only when the caller belongs
                to multiple organizations.
              - `selected_indices`: zero-based positions in `items` to create. An approval host sets this
                from the user's per-item decisions; omit it when proposing. Omitted means every item,
                except in a Sift chat pod, where a missing value is rejected. `[]` creates nothing.

            Errors:
              - `INVALID_PARAMS` if `name` is empty, the time range is inverted, `annotation_type`/`state` is not a
                recognized value, a `state` is supplied for a `phase` annotation, the `metadata` list contains
                duplicate key names, or a value's type does not match an existing metadata key's type.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. missing run/asset).
              - `INVALID_PARAMS` before any write if `items` is empty or has more than 100 entries, or an
                item has an unknown field or fails local validation. The message names the item index.
              - Once writing starts, an item-specific error fails only that item. A backend-wide error
                stops the remaining items; earlier creates stay committed.

            Guidance:
              - This is a write. CONFIRM the time range, type, and associations with the user before invoking.
              - Items run sequentially, one create per item; this is not an atomic batch. Never replay
                the whole call after a partial failure. Retry only failed items, and check any
                `outcome_unknown` item first, since its create may have committed.
              - Use one call with several items, not repeated calls, when creating several annotations.
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
            items,
            selected_indices,
            run_id,
            organization_id,
        }) = params;
        if run_id.as_ref().is_some_and(|id| id.trim().is_empty())
            || organization_id
                .as_ref()
                .is_some_and(|id| id.trim().is_empty())
        {
            return Err(ErrorData::invalid_params(
                "shared `run_id` and `organization_id` must not be empty",
                None,
            ));
        }
        for (index, item) in items.iter().enumerate() {
            if let Some(key) = ["run_id", "organization_id"]
                .into_iter()
                .find(|key| item.get(key).is_some_and(|value| !value.is_null()))
            {
                return Err(ErrorData::invalid_params(
                    format!("`items` item {index}: set `{key}` at the top level, not per item"),
                    None,
                ));
            }
        }
        let mut items = bulk_create::parse_items::<CreateAnnotationItem>(items)?;
        for (index, item) in items.iter_mut().enumerate() {
            item.run_id = run_id.clone();
            item.organization_id = organization_id.clone();
            let has_association = run_id.is_some()
                || item
                    .assets
                    .as_ref()
                    .is_some_and(|ids| ids.iter().any(|id| !id.trim().is_empty()))
                || item
                    .linked_channel_ids
                    .as_ref()
                    .is_some_and(|ids| ids.iter().any(|id| !id.trim().is_empty()));
            if !has_association {
                return Err(ErrorData::invalid_params(
                    format!(
                        "`items` item {index}: supply a shared `run_id`, `assets`, or `linked_channel_ids`; tags are not an association"
                    ),
                    None,
                ));
            }
        }
        bulk_create::create_many(items, self.selection(selected_indices), |item| {
            self.create_annotation_item(Parameters(item))
        })
        .await
    }

    /// Create one item. Callers check access and validate the whole call first.
    pub(crate) async fn create_annotation_item(
        &self,
        params: Parameters<CreateAnnotationItem>,
    ) -> error::McpResult {
        self.require_create()?;

        let Parameters(CreateAnnotationItem {
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

        let annotation_type = AnnotationType::from(annotation_type);
        let state = state.map(AnnotationState::from);

        if annotation_type == AnnotationType::Phase && state.is_some() {
            return Err(ErrorData::invalid_params(
                "`state` must be omitted when `annotation_type` is `ANNOTATION_TYPE_PHASE`",
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
            url_clause(
                "annotation",
                Some(&annotation.name),
                annotation_url.as_deref()
            ),
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
            Update one or more existing annotations with shared changes and per-id approval.
            Each selected id receives a field update when requested, then an archive or unarchive when requested.
            These use the per-id UpdateAnnotation, ArchiveAnnotation and UnarchiveAnnotation RPCs.

            Output:
              - `{ requested_count, selected_count, not_selected, updated_count, successes, failures,
                not_attempted, next_step }`. Successes hold `{ index, result: { annotation } }`; the annotation
                includes a `url` field when the host can be derived. Failures hold `{ index, error, outcome_unknown }`.
                Every index is a zero-based position in `annotation_ids`. `not_attempted` lists positions not started
                after a backend-wide error. Any failure sets `isError`.

            Parameters:
              - `annotation_ids`: required list of 1–1000 unique, nonempty annotation ids, or 1–100 in chat.
                Split chat work into calls of at most 100 ids. Pass one id to update one annotation.
                The same changes apply to every selected annotation.
              - `selected_indices`: zero-based positions in `annotation_ids` supplied by the approval host;
                omit when proposing. Omitted selects all outside chat; chat requires a mask. `[]` writes nothing.
              - `name`: optional new name.
              - `description`: optional new description.
              - `start_time_unix_nanos` / `end_time_unix_nanos`: optional new time bounds in Unix nanoseconds.
              - `assigned_to_user_id`: optional new assignee user id.
              - `state`: optional; `ANNOTATION_STATE_OPEN`, `ANNOTATION_STATE_FLAGGED`, or
                `ANNOTATION_STATE_RESOLVED`.
              - `tags`: optional; REPLACES the full tag list. Pass `[]` to clear all tags.
              - `linked_channel_ids`: optional; REPLACES the full linked-channel list with plain channel links.
                Pass `[]` to clear. Bit-field and calculated-channel links are not exposed here.
              - `metadata`: optional; REPLACES the full metadata list. Each entry is
                `{ \"name\": \"<key>\", \"value\": <scalar> }`. Pass `[]` to clear.
              - `is_archived`: optional archive state. `true` archives each selected annotation; `false`
                unarchives it. When combined with other fields, each id's fields are updated first.
                If that field update fails, that id's archive-state change is skipped.

              At least one updatable field must be set; otherwise the tool returns `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` before any write for empty, duplicate or invalid ids; more than 1000 ids
                (100 in chat); an invalid selection mask; an unrecognized state; or no changes.
              - `INVALID_REQUEST` without `--allow-destructive`.
              - Item-local failures, including NotFound, PermissionDenied and FailedPrecondition, fail only that id.
              - Backend-wide failures stop later groups; ids not started are reported in `not_attempted`.

            Guidance:
              - This is a write. CONFIRM every target and the full proposed values with the user before invoking —
                `tags`, `linked_channel_ids`, and `metadata` are REPLACE operations, not merges.
              - Up to 50 ids run concurrently, with each id's field update before its archive change.
                The call is not atomic; successful writes stay committed even when other ids fail. Never replay
                the whole call after a partial failure. Check `outcome_unknown` items before retrying, and never
                retry `not_selected` ids. A failed archive can follow a committed field update; inspect that id
                before retrying its shared changes.
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
            selected_indices,
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

        let selection = self.selection(selected_indices);
        if selection.required && annotation_ids.len() > 100 {
            return Err(ErrorData::invalid_params(
                "`annotation_ids` must contain at most 100 ids in chat; split the work into calls of at most 100 ids",
                None,
            ));
        }
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

        if annotation_ids.iter().any(|id| id.trim().is_empty()) {
            return Err(ErrorData::invalid_params(
                "`annotation_ids` must not contain empty ids",
                None,
            ));
        }

        bulk::reject_duplicate_targets(&annotation_ids, "annotation", |id| Some(id.clone()))?;

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

        let state = state.map(AnnotationState::from);
        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());
        let requested_count = annotation_ids.len();
        let positions = annotation_ids
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, id)| (id, index))
            .collect::<std::collections::HashMap<_, _>>();
        let selected = bulk::select(requested_count, selection)?;
        let annotation_ids = selected.filter(annotation_ids);
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

        let annotations = with_urls(&outcome.annotations, |annotation| {
            self.url_service
                .build_annotation_url(&annotation.annotation_id)
                .ok()
        })?;
        let successes = outcome
            .annotations
            .iter()
            .zip(annotations)
            .map(|(annotation, value)| {
                (
                    positions[&annotation.annotation_id],
                    CallToolResult::structured(serde_json::json!({"annotation": value})),
                )
            })
            .collect();
        let failures = outcome
            .failures
            .into_iter()
            .map(|failure| (positions[&failure.annotation_id], failure.error))
            .collect();
        let not_attempted = outcome
            .not_attempted
            .into_iter()
            .map(|id| positions[&id])
            .collect();
        bulk::report(
            Op::Update,
            requested_count,
            &selected,
            successes,
            failures,
            not_attempted,
            false,
        )
    }
}
