use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::{
    common::r#type::v1::{FunctionDataType, FunctionInput},
    metadata::v1::MetadataValue,
};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::user_defined_functions::UdfUpdate,
    tool::common::{ListParams, MetadataEntry, list_body, to_values},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserDefinedFunctionVersionListParams {
    pub(crate) user_defined_function_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateUserDefinedFunctionParams {
    pub(crate) name: String,
    pub(crate) expression: String,
    pub(crate) function_inputs_json: String,
    pub(crate) description: Option<String>,
    pub(crate) user_notes: Option<String>,
    pub(crate) metadata: Option<Vec<MetadataEntry>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateUserDefinedFunctionParams {
    pub(crate) user_defined_function_id: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) expression: Option<String>,
    pub(crate) function_inputs_json: Option<String>,
    pub(crate) metadata: Option<Vec<MetadataEntry>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveUserDefinedFunctionParams {
    pub(crate) user_defined_function_id: String,
}

/// One entry of the `function_inputs_json` array. The proto's `FunctionInput` is
/// a nested message, so it arrives as a documented JSON string scalar and is
/// parsed here.
#[derive(Debug, Deserialize)]
struct FunctionInputSpec {
    identifier: String,
    data_type: String,
    #[serde(default)]
    constant: bool,
}

#[tool_router(router = user_defined_functions_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_user_defined_functions",
        description = "
            List the latest version of each user defined function in Sift, optionally filtered by a CEL
            expression and ordered by one or more fields. A user defined function is a named, reusable
            expression that calculated channels and rules can call.

            Output:
              - `{ \"user_defined_functions\": [UserDefinedFunction, ...] }`. Each item carries
                `user_defined_function_id`, `name`, `description`, `expression`, `function_inputs`
                (`identifier`, `data_type`, `constant`), `function_output_type`, `function_dependencies`
                (the version ids of other functions this one calls), `user_defined_function_version_id`,
                `version`, `change_message`, `user_notes`, `metadata`, timestamps, author ids, and archive
                state.
              - Fields at their proto3 default are OMITTED from the JSON: a missing `is_archived` or
                `version` key means `false` / `0`, not \"unknown\".
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many items match `filter`.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — narrow `filter` or raise `limit` and ask again.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `user_defined_function_id`, `name`, `archived_date`, `is_archived`.
                `name` is the only free-text field. When filtering or searching it, use
                `name.matches(\"(?i)rms\")`, not `==`. Use `==` only for an exact value from a prior
                result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE: `contains(\"RMS\")`
                silently misses `rms_window`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields:
                `created_date`, `modified_date`, `name`. Default sort is `created_date desc`
                (newest first). Example: `\"name,modified_date desc\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens, so `asset_id`, `assetId` and
                `asset-id` all work. Any name that matched nothing on any returned item
                is listed in `unmatched_fields`; an empty page reports none, since it
                says nothing about whether a name was spelled right.
                Reach for this whenever you need only a few fields: full objects are wide,
                and a large listing can exceed the response size limit without it.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an
                unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Call this BEFORE authoring an expression that depends on a user defined function. The
                function's exact `name`, its `function_inputs` order, and its `function_output_type` are
                what a caller has to match; guessing them produces an expression the server rejects.
              - Default add `is_archived == false` to the filter. Include archived functions only when
                the user explicitly asks for them.
        ",
        annotations(
            title = "user_defined_functions/list_user_defined_functions",
            read_only_hint = true
        )
    )]
    pub async fn list_user_defined_functions(
        &self,
        params: Parameters<ListParams>,
    ) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
            fields,
        }) = params;

        let page = self
            .user_defined_function_service
            .list_user_defined_functions(filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let functions = to_values(&page.items)?;

        Ok(CallToolResult::structured(list_body(
            "user_defined_functions",
            functions,
            fields,
            page.has_more,
        )))
    }

    #[tool(
        name = "list_user_defined_function_versions",
        description = "
            List the version history of one user defined function. Every accepted update creates a new
            version and leaves the previous one intact, so this is how you read what changed and when.

            Output:
              - `{ \"user_defined_function_versions\": [UserDefinedFunction, ...] }`. Each item is one
                version with the same shape `list_user_defined_functions` returns, including
                `user_defined_function_version_id`, `version`, `expression`, `function_inputs`,
                `change_message` (server-generated summary of the change), `user_notes`, and
                `modified_by_user_id`.
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many items match `filter`.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — narrow `filter` or raise `limit` and ask again.

            Parameters:
              - `user_defined_function_id`: optional. The id of the function whose versions to list.
              - `name`: optional. The name of the function whose versions to list.
              - Exactly one of `user_defined_function_id` or `name` must be set.
              - `filter`: CEL expression. Pass an empty string to list every version.
                Filterable fields: `user_defined_function_id`, `name`, `version`, `archived_date`,
                `is_archived`. `name` is the only free-text field; when filtering or searching it, use
                `name.matches(\"(?i)rms\")`, not `==`. Use `==` only for an exact value from a prior
                result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE: `contains(\"RMS\")`
                silently misses `rms_window`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields:
                `created_date`, `modified_date`, `name`, `version`. When empty, items come back ordered
                by `name` ascending — pass `\"version desc\"` for newest-first.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens, so `asset_id`, `assetId` and
                `asset-id` all work. Any name that matched nothing on any returned item
                is listed in `unmatched_fields`; an empty page reports none, since it
                says nothing about whether a name was spelled right.
                Reach for this whenever you need only a few fields: full objects are wide,
                and a large listing can exceed the response size limit without it.

            Errors:
              - `INVALID_PARAMS` if neither or both of `user_defined_function_id` and `name` are set, or
                if `filter` is not a valid CEL expression.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Use this to find the `user_defined_function_version_id` a calculated channel or another
                function pins, or to show the user how an expression evolved. Resolve the id with
                `list_user_defined_functions` first when you only have a partial name.
        ",
        annotations(
            title = "user_defined_functions/list_user_defined_function_versions",
            read_only_hint = true
        )
    )]
    pub async fn list_user_defined_function_versions(
        &self,
        params: Parameters<UserDefinedFunctionVersionListParams>,
    ) -> error::McpResult {
        let Parameters(UserDefinedFunctionVersionListParams {
            user_defined_function_id,
            name,
            filter,
            order_by,
            limit,
            fields,
        }) = params;

        let (user_defined_function_id, name) = function_identifier(user_defined_function_id, name)?;

        let page = self
            .user_defined_function_service
            .list_user_defined_function_versions(
                user_defined_function_id,
                name,
                filter,
                order_by,
                limit,
            )
            .await
            .map_err(from_anyhow)?;

        let versions = to_values(&page.items)?;

        Ok(CallToolResult::structured(list_body(
            "user_defined_function_versions",
            versions,
            fields,
            page.has_more,
        )))
    }

    #[tool(
        name = "create_user_defined_function",
        description = "
            Create a user defined function: a named, reusable expression that calculated channels, rules,
            and other user defined functions can call. This is a WRITE.

            Output:
              - `{ \"user_defined_function\": UserDefinedFunction, \"user_defined_function_id\": \"<id>\",
                \"next_step\": \"...\" }`. The returned function is the server's post-create state,
                including the resolved `function_output_type` and `version` 1.

            Parameters:
              - `name`: required. The name callers use to reference the function. Must be non-empty.
              - `expression`: required. The function body. Reference each declared input by its
                `identifier`. Mirror an existing function retrieved with `list_user_defined_functions`
                rather than authoring the syntax blind.
              - `function_inputs_json`: required. A JSON array string declaring the function's inputs, in
                the order callers pass them. Each element is
                `{ \"identifier\": \"<name>\", \"data_type\": \"numeric\"|\"string\"|\"bool\",
                \"constant\": <bool> }`. `data_type` is matched case-insensitively; no other spelling is
                accepted. `constant` defaults to `false` and marks an input that takes a literal value
                rather than a channel. Pass `[]` for a function that takes no inputs.
              - `description`: optional. Human-readable summary.
              - `user_notes`: optional. Notes recorded against this first version.
              - `metadata`: optional. Array of `{ \"name\": \"<key>\", \"value\": <scalar> }` where
                `value` is a string, number, or boolean. A `name` that does not yet exist in the
                organization's metadata schema is created on the fly with the type inferred from `value`;
                for an existing key the type must match.

            Errors:
              - `INVALID_PARAMS` if `name` or `expression` is empty, `function_inputs_json` is not a JSON
                array of input objects, an `identifier` is empty, a `data_type` is not one of `numeric`,
                `string`, or `bool`, or the server rejects the expression (unresolved identifier, type
                mismatch, duplicate function name).
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - This creates a live resource. Confirm the name, the expression, and the input list with
                the user before calling.
              - An expression may call other user defined functions. List them first so you match their
                real names, input order, and output types; the server records the calls as
                `function_dependencies`, and those dependencies then restrict what can be updated later.
        ",
        annotations(
            title = "user_defined_functions/create_user_defined_function",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_user_defined_function(
        &self,
        params: Parameters<CreateUserDefinedFunctionParams>,
    ) -> error::McpResult {
        self.require_create()?;

        let Parameters(CreateUserDefinedFunctionParams {
            name,
            expression,
            function_inputs_json,
            description,
            user_notes,
            metadata,
        }) = params;

        if name.trim().is_empty() {
            return Err(ErrorData::invalid_params("`name` must not be empty", None));
        }
        if expression.trim().is_empty() {
            return Err(ErrorData::invalid_params(
                "`expression` must not be empty",
                None,
            ));
        }

        let function_inputs = parse_function_inputs(&function_inputs_json)?;
        let metadata = metadata_values(metadata).unwrap_or_default();

        let function = self
            .user_defined_function_service
            .create_user_defined_function(
                name,
                description,
                expression,
                function_inputs,
                user_notes,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Created user defined function `{}` with id `{}` at version {}. Tell the user the new id \
             and confirm the expression matches their intent. Reference it from a calculated channel \
             or rule by name.",
            function.name, function.user_defined_function_id, function.version,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "user_defined_function": function,
            "user_defined_function_id": function.user_defined_function_id,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_user_defined_function",
        description = "
            Update a user defined function. This is a WRITE. Only the fields you set are written; the
            rest of the function is left as it is. The update creates a NEW version and leaves the
            previous version intact, so callers pinned to an older version are unaffected.

            Output:
              - `{ \"user_defined_function\": UserDefinedFunction, \"next_step\": \"...\" }`. The
                returned function is the newly created version, with its own
                `user_defined_function_version_id` and incremented `version`.

            Parameters:
              - `user_defined_function_id`: required. The function to update.
              - `name`: optional. New name. The API applies a rename BY ITSELF and ignores any other
                field in the same call, so this tool rejects `name` combined with another field — send
                the rename as its own call.
              - `description`: optional. New description.
              - `expression`: optional. New function body.
              - `function_inputs_json`: optional. REPLACES the declared input list. Same array shape as
                `create_user_defined_function`.
              - `metadata`: optional. REPLACES the function's full metadata list. Same entry shape as
                `create_user_defined_function`; pass `[]` to clear.
              - At least one field besides `user_defined_function_id` must be set.
              - Archive state is not settable here. Use `archive_user_defined_function` /
                `unarchive_user_defined_function`.

            Errors:
              - `INVALID_PARAMS` if no updatable field is set, `name` is combined with another field, or
                `function_inputs_json` is not a valid input array. The server also rejects updates that
                its dependency rules forbid: `name` cannot change once the function has ever had
                dependencies, `function_inputs` cannot change while any function or calculated channel
                depends on this one, and `expression` cannot change the output type while dependents
                exist.
              - `RESOURCE_NOT_FOUND` if no function matches `user_defined_function_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Confirm the target function and the exact changes with the user before calling.
              - `function_inputs_json` and `metadata` are REPLACE, not merge. Read the current values
                with `list_user_defined_functions` (filter `user_defined_function_id == \"<id>\"`) and
                send the full intended list.
              - There is no version precondition on this RPC, so a concurrent edit is not detected. Read
                the function immediately before updating when a change may be racing another author.
        ",
        annotations(
            title = "user_defined_functions/update_user_defined_function",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn update_user_defined_function(
        &self,
        params: Parameters<UpdateUserDefinedFunctionParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(UpdateUserDefinedFunctionParams {
            user_defined_function_id,
            name,
            description,
            expression,
            function_inputs_json,
            metadata,
        }) = params;

        if user_defined_function_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`user_defined_function_id` must not be empty",
                None,
            ));
        }

        let function_inputs = function_inputs_json
            .as_deref()
            .map(parse_function_inputs)
            .transpose()?;

        let changes = UdfUpdate {
            name,
            description,
            expression,
            function_inputs,
            metadata: metadata_values(metadata),
        };

        let others_set = changes.description.is_some()
            || changes.expression.is_some()
            || changes.function_inputs.is_some()
            || changes.metadata.is_some();

        if changes.name.is_none() && !others_set {
            return Err(ErrorData::invalid_params(
                "at least one of `name`, `description`, `expression`, `function_inputs_json`, or \
                 `metadata` must be set",
                None,
            ));
        }
        if changes.name.is_some() && others_set {
            return Err(ErrorData::invalid_params(
                "the API applies a `name` change on its own and ignores every other field in the \
                 same request; send the rename as a separate call",
                None,
            ));
        }

        let function = self
            .user_defined_function_service
            .update_user_defined_function(user_defined_function_id, changes)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Updated user defined function `{}` ({}); it is now at version {}. Earlier versions are \
             untouched. Surface the new state to the user and confirm nothing was unintentionally \
             replaced — input and metadata lists are REPLACE operations.",
            function.name, function.user_defined_function_id, function.version,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "user_defined_function": function,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "archive_user_defined_function",
        description = "
            Archive a user defined function so it stops appearing as an available function. This is a
            WRITE. Reversible with `unarchive_user_defined_function`.

            Output:
              - `{ \"archived\": true, \"user_defined_function\": UserDefinedFunction,
                \"next_step\": \"...\" }`. The returned function is the post-archive state.

            Parameters:
              - `user_defined_function_id`: required. The function to archive.

            Errors:
              - `INVALID_PARAMS` if `user_defined_function_id` is empty.
              - `RESOURCE_NOT_FOUND` if no function matches `user_defined_function_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Archiving does not delete the function and does not rewrite anything that already calls
                it. Calculated channels and rules that call it keep referencing it. Confirm the target
                with the user before calling.
              - This toolset cannot enumerate what depends on a function. `function_dependencies` on a
                `list_user_defined_functions` row is the reverse direction — the functions this one
                calls. Treat the archive as reversible and let the user tell you what else is affected.
        ",
        annotations(
            title = "user_defined_functions/archive_user_defined_function",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn archive_user_defined_function(
        &self,
        params: Parameters<ArchiveUserDefinedFunctionParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(ArchiveUserDefinedFunctionParams {
            user_defined_function_id,
        }) = params;

        let function = self.set_archived(user_defined_function_id, true).await?;

        let next_step = format!(
            "Archived user defined function `{}` ({}). Tell the user it no longer appears as an \
             available function and that `unarchive_user_defined_function` restores it.",
            function.name, function.user_defined_function_id,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "archived": true,
            "user_defined_function": function,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "unarchive_user_defined_function",
        description = "
            Restore a previously archived user defined function. This is a WRITE.

            Output:
              - `{ \"unarchived\": true, \"user_defined_function\": UserDefinedFunction,
                \"next_step\": \"...\" }`. The returned function is the post-unarchive state.

            Parameters:
              - `user_defined_function_id`: required. The function to unarchive.

            Errors:
              - `INVALID_PARAMS` if `user_defined_function_id` is empty.
              - `RESOURCE_NOT_FOUND` if no function matches `user_defined_function_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Confirm the target function with the user before calling. Find archived functions with
                `list_user_defined_functions` and filter `is_archived == true`.
        ",
        annotations(
            title = "user_defined_functions/unarchive_user_defined_function",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn unarchive_user_defined_function(
        &self,
        params: Parameters<ArchiveUserDefinedFunctionParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(ArchiveUserDefinedFunctionParams {
            user_defined_function_id,
        }) = params;

        let function = self.set_archived(user_defined_function_id, false).await?;

        let next_step = format!(
            "Unarchived user defined function `{}` ({}). Tell the user it is available again.",
            function.name, function.user_defined_function_id,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "unarchived": true,
            "user_defined_function": function,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}

impl SiftMcpServer {
    /// Shared body of the archive and unarchive tools. Both flip the same
    /// `is_archived` field through the update mask, so only the validation and
    /// the reported wording differ.
    async fn set_archived(
        &self,
        user_defined_function_id: String,
        is_archived: bool,
    ) -> Result<sift_rs::common::r#type::v1::UserDefinedFunction, ErrorData> {
        if user_defined_function_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`user_defined_function_id` must not be empty",
                None,
            ));
        }

        self.user_defined_function_service
            .set_user_defined_function_archived(user_defined_function_id, is_archived)
            .await
            .map_err(from_anyhow)
    }
}

/// Resolve the `(user_defined_function_id, name)` request fields from the
/// mutually exclusive optional params. The proto silently ignores `name` when an
/// id is present, so reject the ambiguous call rather than pick for the caller.
fn function_identifier(
    user_defined_function_id: Option<String>,
    name: Option<String>,
) -> Result<(String, String), ErrorData> {
    match (user_defined_function_id, name) {
        (Some(id), None) => Ok((id, String::new())),
        (None, Some(name)) => Ok((String::new(), name)),
        (Some(_), Some(_)) => Err(ErrorData::invalid_params(
            "exactly one of `user_defined_function_id` or `name` must be set, not both",
            None,
        )),
        (None, None) => Err(ErrorData::invalid_params(
            "one of `user_defined_function_id` or `name` must be set",
            None,
        )),
    }
}

/// Parse the documented `function_inputs_json` array into proto inputs, mapping
/// every shape error to `INVALID_PARAMS` so the agent can correct its input.
fn parse_function_inputs(function_inputs_json: &str) -> Result<Vec<FunctionInput>, ErrorData> {
    let specs: Vec<FunctionInputSpec> =
        serde_json::from_str(function_inputs_json).map_err(|e| {
            ErrorData::invalid_params(
                format!(
                    "`function_inputs_json` is not a JSON array of \
                     {{\"identifier\", \"data_type\", \"constant\"}} objects: {e}"
                ),
                None,
            )
        })?;

    specs
        .into_iter()
        .map(|spec| {
            if spec.identifier.trim().is_empty() {
                return Err(ErrorData::invalid_params(
                    "every `function_inputs_json` entry needs a non-empty `identifier`",
                    None,
                ));
            }
            Ok(FunctionInput {
                identifier: spec.identifier,
                data_type: parse_function_data_type(&spec.data_type)?.into(),
                constant: spec.constant,
            })
        })
        .collect()
}

fn parse_function_data_type(data_type: &str) -> Result<FunctionDataType, ErrorData> {
    match data_type.to_ascii_lowercase().as_str() {
        "numeric" => Ok(FunctionDataType::Numeric),
        "string" => Ok(FunctionDataType::String),
        "bool" => Ok(FunctionDataType::Bool),
        other => Err(ErrorData::invalid_params(
            format!("unknown `data_type` `{other}`; expected `numeric`, `string`, or `bool`"),
            None,
        )),
    }
}

fn metadata_values(metadata: Option<Vec<MetadataEntry>>) -> Option<Vec<MetadataValue>> {
    metadata.map(|entries| entries.into_iter().map(MetadataValue::from).collect())
}
