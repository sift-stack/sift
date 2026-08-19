use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::{
    calculated_channels::v2::CalculatedChannelAbstractChannelReference, metadata::v1::MetadataValue,
};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::calculated_channels::{
        CalculatedChannelUpdate, CalculatedChannelWrite, NewCalculatedChannel,
    },
    tool::common::{ListParams, MetadataEntry},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CalculatedChannelVersionListParams {
    calculated_channel_id: String,
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCalculatedChannelParams {
    name: String,
    expression: String,
    expression_channel_references_json: String,
    description: Option<String>,
    user_notes: Option<String>,
    units: Option<String>,
    client_key: Option<String>,
    all_assets: Option<bool>,
    asset_ids: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCalculatedChannelParams {
    calculated_channel_id: String,
    name: Option<String>,
    description: Option<String>,
    units: Option<String>,
    expression: Option<String>,
    expression_channel_references_json: Option<String>,
    all_assets: Option<bool>,
    asset_ids: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
    user_notes: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CalculatedChannelArchiveParams {
    calculated_channel_id: String,
}

#[tool_router(router = calculated_channels_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_calculated_channels",
        description = "
            List calculated channels in Sift, optionally filtered by a CEL expression and ordered by one or
            more fields. A calculated channel is a derived channel: a SEL expression over other channels,
            scoped to a set of assets.

            Output:
              - `{ \"calculated_channels\": [CalculatedChannel, ...] }`. Each item is the full Sift
                `CalculatedChannel` shape including `calculated_channel_id`, `version_id`, `version`, `name`,
                `description`, `units`, `client_key`, `calculated_channel_configuration` (the asset scope plus
                the SEL expression and its channel references), `metadata`, `folder_ids`, `is_archived`,
                `archived_date`, and timestamps.
              - Fields at their proto3 default are OMITTED from the JSON: a missing `is_archived` key means
                `false`, not \"unknown\".

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `calculated_channel_id`, `organization_id`, `client_key`, `name`, `description`, `asset_id`,
                `asset_name`, `tag_id`, `tag_name`, `units`, `calculated_channel_version_id`, `created_date`,
                `modified_date`, `created_by_user_id`, `modified_by_user_id`, `is_archived`, `archived_date`.
                Folder membership is filterable via `folders` and `activeFolders` (folder-id lists;
                `activeFolders` excludes archived folders): `\"<folder_id>\" in folders` returns calculated
                channels in a folder, `size(activeFolders) == 0` returns uncategorized ones.
                When filtering or searching, use `name.matches(\"(?i)thrust\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Thrust\")` silently misses `thrust_margin`. Calculated channel names can embed `.`,
                a regex wildcard, so match a full literal name with `contains`, not `matches`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `modified_date`, `name`, `description`, `units`, `archived_date`. Default sort is
                `created_date desc` (newest first). Example: `\"created_date desc,modified_date\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Default add `is_archived == false` to the filter. Include archived calculated channels only when
                the user explicitly asks for them.
              - Scope with `asset_id == \"...\"` when the asset is known — it is the most selective field.
              - Calculated channels store a definition, not stored samples: each one is a SEL expression evaluated
                per asset. Use this tool to answer what derived channels exist, what each one computes (read
                `calculated_channel_configuration`), and which assets it is scoped to.
        ",
        annotations(
            title = "calculated_channels/list_calculated_channels",
            read_only_hint = true
        )
    )]
    pub async fn list_calculated_channels(
        &self,
        params: Parameters<ListParams>,
    ) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .calculated_channel_service
            .list_calculated_channels(filter, order_by, limit)
            .await
            .map(|channels| serde_json::json!({ "calculated_channels": channels }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "list_calculated_channel_versions",
        description = "
            List the version history of a single calculated channel. Every update creates a new version, so this
            is how you see what changed and when.

            Output:
              - `{ \"calculated_channel_versions\": [CalculatedChannel, ...], \"next_step\": string }`. Each item
                is a full `CalculatedChannel` snapshot of that version, including `version`, `version_id`,
                `change_message`, `user_notes`, `calculated_channel_configuration`, and
                `modified_by_user_id` — not a reduced version record.

            Parameters:
              - `calculated_channel_id`: required. The calculated channel whose versions to list. Resolve it with
                `list_calculated_channels` first if you only have the name.
              - `filter`: CEL expression. Filterable fields: `calculated_channel_id`, `organization_id`,
                `client_key`, `name`, `description`, `asset_id`, `asset_name`, `tag_id`, `tag_name`, `version`,
                `units`, `calculated_channel_version_id`, `created_date`, `modified_date`, `created_by_user_id`,
                `modified_by_user_id`, `is_archived`, `archived_date`. Pass an empty string to list all
                versions. When filtering or searching text, use `name.matches(\"(?i)thrust\")`, not `==`. Use `==`
                only for an exact value from a prior result. `contains`/`startsWith`/`endsWith` are
                case-SENSITIVE: `contains(\"Thrust\")` silently misses `thrust_margin`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `version`,
                `created_date`, `modified_date`, `name`, `description`, `units`, `archived_date`. Default sort is
                `created_date` ascending (oldest first) — note this differs from `list_calculated_channels`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.

            Errors:
              - `INVALID_PARAMS` if `calculated_channel_id` is empty or `filter` is not a valid CEL expression.
              - `RESOURCE_NOT_FOUND` if no calculated channel matches `calculated_channel_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Use `order_by: \"version desc\"` with `limit: 1` to fetch just the most recent version.
              - Read the prior version's `calculated_channel_configuration` before calling
                `update_calculated_channel`, so you know what the expression and asset scope currently are.
        ",
        annotations(
            title = "calculated_channels/list_calculated_channel_versions",
            read_only_hint = true
        )
    )]
    pub async fn list_calculated_channel_versions(
        &self,
        params: Parameters<CalculatedChannelVersionListParams>,
    ) -> error::McpResult {
        let Parameters(CalculatedChannelVersionListParams {
            calculated_channel_id,
            filter,
            order_by,
            limit,
        }) = params;

        require_id(&calculated_channel_id)?;

        let versions = self
            .calculated_channel_service
            .list_calculated_channel_versions(calculated_channel_id, filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Listed {} calculated channel versions. Surface the version history to the user, \
             highlighting what changed between versions.",
            versions.len(),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "calculated_channel_versions": versions,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "create_calculated_channel",
        description = "
            Create a calculated channel: a derived channel defined by a SEL expression over other channels,
            scoped to a set of assets. This is a WRITE.

            Output:
              - `{ \"calculated_channel\": CalculatedChannel, \"inapplicable_assets\": [...],
                \"next_step\": string }`. `inapplicable_assets` lists in-scope assets that do NOT have every
                channel the expression references; each entry carries `asset_id`, `asset_name`, `tag_names`, and
                `missing_channels`. A non-empty list means the channel was created but will not evaluate on those
                assets.

            Parameters:
              - `name`: required. The calculated channel's name.
              - `expression`: required. A SEL expression whose channel operands are placeholders (`$1`, `$2`, …)
                resolved by `expression_channel_references_json`. Example: `\"$1 - $2\"`.
              - `expression_channel_references_json`: required. A JSON array string mapping each placeholder to a
                channel. Each entry is
                `{ \"channel_reference\": \"$1\", \"channel_identifier\": \"<channel name>\" }`. To reference
                another calculated channel instead of a raw channel, replace `channel_identifier` with
                `\"calculated_channel_version_id\": \"<version id>\"`. Pass `[]` only for an expression with no
                channel operands. This parameter is a JSON STRING, not an object.
              - `description`: optional. Free-text description.
              - `user_notes`: optional. Notes recorded against this version.
              - `units`: optional. Units of the computed output.
              - `client_key`: optional. A caller-defined identifier. Immutable after creation.
              - `all_assets`: optional. `true` scopes the channel to every asset in the organization.
              - `asset_ids` / `tag_ids`: optional. Scope the channel to specific assets and/or tagged assets.
              - Exactly one scope form is allowed: either `all_assets: true`, or a non-empty `asset_ids` /
                `tag_ids` selection. Setting both is rejected.
              - `metadata`: optional. Array of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries.

            Errors:
              - `INVALID_PARAMS` if `name` or `expression` is empty, `expression_channel_references_json` is not a
                valid reference array, no asset scope is given, or both scope forms are set.
              - `INVALID_REQUEST` if the server was launched without `--allow-create`.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. a referenced channel does not exist).

            Guidance:
              - Resolve the exact channel names with `list_channels` before writing the references — a typo makes
                the channel inapplicable on every asset rather than failing outright.
              - This creates a live resource. Confirm the name, the expression, and the asset scope with the user
                before calling.
              - Report any `inapplicable_assets` back to the user; that list is the API telling you the scope and
                the expression disagree.
        ",
        annotations(
            title = "calculated_channels/create_calculated_channel",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_calculated_channel(
        &self,
        params: Parameters<CreateCalculatedChannelParams>,
    ) -> error::McpResult {
        self.require_create()?;

        let Parameters(CreateCalculatedChannelParams {
            name,
            expression,
            expression_channel_references_json,
            description,
            user_notes,
            units,
            client_key,
            all_assets,
            asset_ids,
            tag_ids,
            metadata,
        }) = params;

        if name.is_empty() {
            return Err(ErrorData::invalid_params("`name` must not be empty", None));
        }
        if expression.is_empty() {
            return Err(ErrorData::invalid_params(
                "`expression` must not be empty",
                None,
            ));
        }

        let expression_channel_references =
            parse_channel_references(&expression_channel_references_json)?;
        check_scope_exclusive(all_assets, &asset_ids, &tag_ids)?;

        let asset_ids = asset_ids.unwrap_or_default();
        let tag_ids = tag_ids.unwrap_or_default();
        let all_assets = all_assets.unwrap_or_default();
        if !all_assets && asset_ids.is_empty() && tag_ids.is_empty() {
            return Err(ErrorData::invalid_params(
                "set `all_assets` to true, or name at least one `asset_ids` or `tag_ids` entry",
                None,
            ));
        }

        let written = self
            .calculated_channel_service
            .create_calculated_channel(NewCalculatedChannel {
                name,
                description,
                user_notes,
                units,
                client_key,
                metadata: metadata_values(metadata),
                expression,
                expression_channel_references,
                all_assets,
                asset_ids,
                tag_ids,
            })
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Created calculated channel `{}` (`{}`).{} Tell the user the new id. If they haven't \
             indicated a next step, offer to confirm it with `list_calculated_channels` \
             (filter `calculated_channel_id == \"{}\"`).",
            written.calculated_channel.name,
            written.calculated_channel.calculated_channel_id,
            inapplicable_clause(&written),
            written.calculated_channel.calculated_channel_id,
        );

        Ok(write_result(written, next_step, None))
    }

    #[tool(
        name = "update_calculated_channel",
        description = "
            Update an existing calculated channel, creating a new version. This is a WRITE. Only the fields you
            set are changed; the tool reads the current channel, overlays your changes, and saves the result, so
            unspecified fields are preserved.

            Output:
              - `{ \"calculated_channel\": CalculatedChannel, \"inapplicable_assets\": [...],
                \"next_step\": string }`. The returned channel is the new version's post-update state.
                `inapplicable_assets` lists in-scope assets missing a channel the expression references.

            Parameters:
              - `calculated_channel_id`: required. The calculated channel to update.
              - `name`: optional. New name.
              - `description`: optional. New description.
              - `units`: optional. New units.
              - `expression`: optional. New SEL expression.
              - `expression_channel_references_json`: optional. New reference array, same shape as in
                `create_calculated_channel`. `expression` and `expression_channel_references_json` must be
                supplied together — a new expression with stale references silently misbinds its operands.
              - `all_assets`: optional. `true` rescopes the channel to every asset.
              - `asset_ids` / `tag_ids`: optional. REPLACE the corresponding list on the channel's selection
                scope; the list you omit is preserved. Setting either alongside `all_assets: true` is rejected.
              - `metadata`: optional. REPLACES the full metadata list. Pass `[]` to clear.
              - `user_notes`: optional. Notes recorded against this new version. This ANNOTATES a change; it
                cannot be the change. Sending only `user_notes` is rejected, because it would write nothing.
              - At least one of `name`, `description`, `units`, `metadata`, `expression`, `all_assets`,
                `asset_ids`, or `tag_ids` must be set.

            Errors:
              - `INVALID_PARAMS` if no updatable field is set (including a `user_notes`-only call), `expression` and
                `expression_channel_references_json` are not supplied together, the references JSON is invalid, or
                both asset-scope forms are set.
              - `INVALID_REQUEST` if the server was launched without `--allow-destructive`.
              - `RESOURCE_NOT_FOUND` if no calculated channel matches `calculated_channel_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Read the current definition with `list_calculated_channel_versions` before rewriting the
                expression or the asset scope, and confirm the change with the user.
              - This does not archive. Use `archive_calculated_channel` to retire a channel.
              - Every update adds a version; the previous version stays readable through
                `list_calculated_channel_versions`.
        ",
        annotations(
            title = "calculated_channels/update_calculated_channel",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn update_calculated_channel(
        &self,
        params: Parameters<UpdateCalculatedChannelParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(UpdateCalculatedChannelParams {
            calculated_channel_id,
            name,
            description,
            units,
            expression,
            expression_channel_references_json,
            all_assets,
            asset_ids,
            tag_ids,
            metadata,
            user_notes,
        }) = params;

        require_id(&calculated_channel_id)?;

        if expression.is_some() != expression_channel_references_json.is_some() {
            return Err(ErrorData::invalid_params(
                "`expression` and `expression_channel_references_json` must be provided together",
                None,
            ));
        }
        check_scope_exclusive(all_assets, &asset_ids, &tag_ids)?;
        if all_assets == Some(false) && asset_ids.is_none() && tag_ids.is_none() {
            return Err(ErrorData::invalid_params(
                "`all_assets: false` needs `asset_ids` or `tag_ids` to define the new scope",
                None,
            ));
        }

        let expression_channel_references = expression_channel_references_json
            .as_deref()
            .map(parse_channel_references)
            .transpose()?;

        let changes = CalculatedChannelUpdate {
            name,
            description,
            units,
            metadata: metadata.map(|m| m.into_iter().map(MetadataValue::from).collect()),
            expression,
            expression_channel_references,
            all_assets,
            asset_ids,
            tag_ids,
            user_notes,
        };

        if changes.is_empty() {
            return Err(ErrorData::invalid_params(
                "at least one changed field besides `calculated_channel_id` must be set; \
                 `user_notes` only annotates a change, so it must accompany at least one of \
                 `name`, `description`, `units`, `metadata`, `expression`, `all_assets`, \
                 `asset_ids`, or `tag_ids`",
                None,
            ));
        }

        let written = self
            .calculated_channel_service
            .update_calculated_channel(calculated_channel_id, changes)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Updated calculated channel `{}` to version {}.{} Surface the new definition to the user \
             and confirm it matches their intent.",
            written.calculated_channel.calculated_channel_id,
            written.calculated_channel.version,
            inapplicable_clause(&written),
        );

        Ok(write_result(written, next_step, None))
    }

    #[tool(
        name = "archive_calculated_channel",
        description = "
            Archive a calculated channel so it stops being offered for plotting and querying. This is a WRITE.
            Reversible with `unarchive_calculated_channel`.

            Output:
              - `{ \"archived\": true, \"calculated_channel\": CalculatedChannel, \"next_step\": string }`. The
                returned channel carries the `archived_date` the server recorded.

            Parameters:
              - `calculated_channel_id`: required. The calculated channel to archive.

            Errors:
              - `INVALID_PARAMS` if `calculated_channel_id` is empty.
              - `INVALID_REQUEST` if the server was launched without `--allow-destructive`.
              - `RESOURCE_NOT_FOUND` if no calculated channel matches `calculated_channel_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Archiving does not delete the definition or its version history, and
                `unarchive_calculated_channel` restores it. Confirm the target with the user before calling.
              - Other calculated channels and rules may reference this one. Check with
                `list_calculated_channels` before archiving something that looks like a shared building block.
        ",
        annotations(
            title = "calculated_channels/archive_calculated_channel",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn archive_calculated_channel(
        &self,
        params: Parameters<CalculatedChannelArchiveParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id,
        }) = params;

        require_id(&calculated_channel_id)?;

        let written = self
            .calculated_channel_service
            .archive_calculated_channel(calculated_channel_id)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Archived calculated channel `{}`. Tell the user it is archived and no longer offered \
             for plotting or querying, and that `unarchive_calculated_channel` restores it.",
            written.calculated_channel.calculated_channel_id,
        );

        Ok(write_result(written, next_step, Some(("archived", true))))
    }

    #[tool(
        name = "unarchive_calculated_channel",
        description = "
            Restore a previously archived calculated channel so it is offered again. This is a WRITE.

            Output:
              - `{ \"unarchived\": true, \"calculated_channel\": CalculatedChannel, \"next_step\": string }`. The
                returned channel has no `archived_date`.

            Parameters:
              - `calculated_channel_id`: required. The calculated channel to restore.

            Errors:
              - `INVALID_PARAMS` if `calculated_channel_id` is empty.
              - `INVALID_REQUEST` if the server was launched without `--allow-destructive`.
              - `RESOURCE_NOT_FOUND` if no calculated channel matches `calculated_channel_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Find archived channels with `list_calculated_channels` filtered by `is_archived == true`.
              - Confirm the target with the user before calling.
        ",
        annotations(
            title = "calculated_channels/unarchive_calculated_channel",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn unarchive_calculated_channel(
        &self,
        params: Parameters<CalculatedChannelArchiveParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id,
        }) = params;

        require_id(&calculated_channel_id)?;

        let written = self
            .calculated_channel_service
            .unarchive_calculated_channel(calculated_channel_id)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Unarchived calculated channel `{}`. Tell the user it is restored and available again.",
            written.calculated_channel.calculated_channel_id,
        );

        Ok(write_result(written, next_step, Some(("unarchived", true))))
    }
}

/// Reject an empty `calculated_channel_id` before any RPC.
fn require_id(calculated_channel_id: &str) -> Result<(), ErrorData> {
    if calculated_channel_id.is_empty() {
        return Err(ErrorData::invalid_params(
            "`calculated_channel_id` must not be empty",
            None,
        ));
    }
    Ok(())
}

/// Deserialize the JSON-string channel reference array, mapping any parse error
/// to `INVALID_PARAMS` so the agent can correct it. Nested channel references
/// are the one irreducibly nested input on these tools.
fn parse_channel_references(
    references_json: &str,
) -> Result<Vec<CalculatedChannelAbstractChannelReference>, ErrorData> {
    serde_json::from_str::<Vec<CalculatedChannelAbstractChannelReference>>(references_json).map_err(
        |e| {
            ErrorData::invalid_params(
                format!(
                    "`expression_channel_references_json` is not a valid channel reference array: {e}"
                ),
                None,
            )
        },
    )
}

/// `all_assets: true` and an explicit asset/tag selection are two spellings of
/// the same oneof, so only one may be set.
fn check_scope_exclusive(
    all_assets: Option<bool>,
    asset_ids: &Option<Vec<String>>,
    tag_ids: &Option<Vec<String>>,
) -> Result<(), ErrorData> {
    if all_assets == Some(true) && (asset_ids.is_some() || tag_ids.is_some()) {
        return Err(ErrorData::invalid_params(
            "`all_assets` and `asset_ids`/`tag_ids` are mutually exclusive; set one scope form",
            None,
        ));
    }
    Ok(())
}

fn metadata_values(metadata: Option<Vec<MetadataEntry>>) -> Vec<MetadataValue> {
    metadata
        .unwrap_or_default()
        .into_iter()
        .map(MetadataValue::from)
        .collect()
}

/// A trailing `next_step` clause naming the assets the API reported the channel
/// cannot evaluate on. Empty when every in-scope asset applies.
fn inapplicable_clause(written: &CalculatedChannelWrite) -> String {
    if written.inapplicable_assets.is_empty() {
        return String::new();
    }
    format!(
        " {} in-scope asset(s) are missing a referenced channel and will not evaluate this \
         calculated channel; see `inapplicable_assets`.",
        written.inapplicable_assets.len()
    )
}

/// Shape a write result: the stored channel, the inapplicable assets when the
/// API reported any, `next_step` on both the structured body and the content
/// block, plus an optional state flag (`archived` / `unarchived`).
fn write_result(
    written: CalculatedChannelWrite,
    next_step: String,
    flag: Option<(&str, bool)>,
) -> CallToolResult {
    let mut body = serde_json::json!({
        "calculated_channel": written.calculated_channel,
        "next_step": next_step.clone(),
    });
    match flag {
        Some((key, value)) => {
            body[key] = serde_json::json!(value);
        }
        None => {
            body["inapplicable_assets"] = serde_json::json!(written.inapplicable_assets);
        }
    }

    let mut result = CallToolResult::structured(body);
    result.content = vec![ContentBlock::text(next_step)];
    result
}
