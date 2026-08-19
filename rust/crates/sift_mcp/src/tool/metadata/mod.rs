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
    tool::common::ListParams,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListMetadataValuesParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) metadata_key_name: String,
}

#[cfg(test)]
mod test;

#[tool_router(router = metadata_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_metadata_keys",
        description = "
            List metadata keys in Sift, optionally filtered by a CEL expression and ordered by one or more
            fields. Wraps `metadata/v1 ListMetadataKeys`.

            Output:
              - `{ \"metadata_keys\": [MetadataKey, ...] }`. Each item has `name`, `type` (one of `string`,
                `number`, `boolean`, `relation`), `archived_date`, `is_archived`, and `filter_field_type`
                (derived from `type`; use it to build a CEL filter against `metadata[\"<name>\"]` elsewhere).

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `name`, `archived_date`, `is_archived`.
                When filtering or searching, use `name.matches(\"(?i)vehicle\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Vehicle\")` silently misses `vehicle_type`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `name`. If left empty, items are ordered by `created_date` ascending (oldest first). Example:
                `\"name,created_date desc\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown
                field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Check here before adding metadata to an asset or run — a name that doesn't yet exist is created
                on the fly by `update_asset`/`update_run`, so a typo becomes a new, permanent key.
              - Default add `is_archived == false` to the filter. Include archived keys only when the user
                explicitly asks for them.
        ",
        annotations(title = "metadata/list_metadata_keys", read_only_hint = true)
    )]
    pub async fn list_metadata_keys(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .metadata_service
            .list_metadata_keys(filter, order_by, limit)
            .await
            .map(|keys| serde_json::json!({ "metadata_keys": keys }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "list_metadata_values",
        description = "
            List the distinct values recorded for one metadata key, optionally filtered by a CEL expression and
            ordered by one or more fields. Wraps `metadata/v1 ListMetadataValues`.

            Output:
              - `{ \"metadata_values\": [MetadataValue, ...] }`. Each item has `key` (the `MetadataKey` this
                value belongs to), exactly one of `string_value`/`number_value`/`boolean_value`/`relation_value`,
                `archived_date`, and `is_archived`.

            Parameters:
              - `metadata_key_name`: required; the name of the metadata key to list values for. Get valid names
                from `list_metadata_keys`.
              - `filter`: CEL expression. Pass an empty string to list everything for the key. Filterable
                fields: `value_string`, `value_number`, `value_boolean`, `archived_date`, `is_archived`.
                When filtering or searching a string value, use `value_string.matches(\"(?i)rover\")`, not `==`.
                Use `==` only for an exact value from a prior result. `contains`/`startsWith`/`endsWith` are
                case-SENSITIVE: `contains(\"Rover\")` silently misses `rover-01`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `name`. If left empty, items are ordered by `created_date` ascending (oldest first).
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.

            Errors:
              - `INVALID_PARAMS` if `metadata_key_name` is empty, `filter` is not a valid CEL expression, or
                `order_by` references an unknown field.
              - `RESOURCE_NOT_FOUND` if `metadata_key_name` does not match an existing key.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Use this before filtering on a value (e.g. `metadata.vehicle_type == \"rover\"` elsewhere) to
                confirm the exact spelling and casing already in use.
        ",
        annotations(title = "metadata/list_metadata_values", read_only_hint = true)
    )]
    pub async fn list_metadata_values(
        &self,
        params: Parameters<ListMetadataValuesParams>,
    ) -> error::McpResult {
        let Parameters(ListMetadataValuesParams {
            filter,
            order_by,
            limit,
            metadata_key_name,
        }) = params;

        if metadata_key_name.is_empty() {
            return Err(ErrorData::invalid_params(
                "`metadata_key_name` must not be empty",
                None,
            ));
        }

        let out = self
            .metadata_service
            .list_metadata_values(filter, order_by, limit, metadata_key_name)
            .await
            .map(|values| serde_json::json!({ "metadata_values": values }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "list_metadata_usage",
        description = "
            List where a metadata key or value is currently applied — which assets, runs, and other entities
            carry it. Wraps `metadata/v1 ListMetadataUsage`.

            Output:
              - `{ \"metadata_usages\": [MetadataUsage, ...] }`. Each item has `entity_id`, `entity_type` (e.g.
                `asset`, `run`, `report`), and `value` (the `MetadataValue`, including its `key`, present on that
                entity).

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `entity_name`, `entity_type`, `key_name`, `value_string`, `value_number`, `value_boolean`.
                When filtering or searching, use `entity_name.matches(\"(?i)rover\")`, not `==`. Use `==` only
                for an exact value from a prior result, such as `key_name == \"vehicle_type\"`.
                `contains`/`startsWith`/`endsWith` are case-SENSITIVE: `contains(\"Rover\")` silently misses
                `rover-01`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `entity_id`, `entity_type`. If left empty, items are ordered by `created_date` ascending
                (oldest first).
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown
                field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Filter on `key_name == \"<name>\"` to see everywhere a key is used before deciding whether it's
                safe to stop setting it going forward.
        ",
        annotations(title = "metadata/list_metadata_usage", read_only_hint = true)
    )]
    pub async fn list_metadata_usage(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .metadata_service
            .list_metadata_usage(filter, order_by, limit)
            .await
            .map(|usages| serde_json::json!({ "metadata_usages": usages }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
