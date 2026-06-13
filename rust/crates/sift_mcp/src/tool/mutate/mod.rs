use std::collections::HashMap;

use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::calculated_channels::v2::{
    CalculatedChannelAbstractChannelReference, CalculatedChannelAssetConfiguration,
    CalculatedChannelConfiguration, CalculatedChannelQueryConfiguration,
    calculated_channel_abstract_channel_reference::CalculatedChannelReference,
    calculated_channel_asset_configuration::{AssetScope, AssetSelection},
    calculated_channel_query_configuration::{Query, Sel},
};
use sift_rs::common::r#type::v1::{FunctionDataType, FunctionInput};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::{
        assets::AssetService, calculated_channels::CalculatedChannelUpdate,
        user_defined_functions::UserDefinedFunctionUpdate,
    },
    tool::common::{MetadataEntry, cel_escape},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateUserDefinedFunctionParams {
    name: String,
    description: Option<String>,
    expression: String,
    input_identifiers: Vec<String>,
    input_data_types: Vec<String>,
    input_constants: Vec<bool>,
    user_notes: Option<String>,
    metadata: Option<Vec<MetadataEntry>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateUserDefinedFunctionParams {
    user_defined_function_id: String,
    name: Option<String>,
    description: Option<String>,
    expression: Option<String>,
    input_identifiers: Option<Vec<String>>,
    input_data_types: Option<Vec<String>>,
    input_constants: Option<Vec<bool>>,
    metadata: Option<Vec<MetadataEntry>>,
}

/// Map the parallel `input_*` arrays onto `FunctionInput`s, enforcing equal
/// length and a known `data_type` spelling.
fn build_function_inputs(
    identifiers: Vec<String>,
    data_types: Vec<String>,
    constants: Vec<bool>,
) -> Result<Vec<FunctionInput>, ErrorData> {
    if identifiers.len() != data_types.len() || identifiers.len() != constants.len() {
        return Err(ErrorData::invalid_params(
            format!(
                "`input_identifiers` ({}), `input_data_types` ({}), and `input_constants` ({}) must be the same length",
                identifiers.len(),
                data_types.len(),
                constants.len(),
            ),
            None,
        ));
    }

    identifiers
        .into_iter()
        .zip(data_types)
        .zip(constants)
        .map(|((identifier, data_type), constant)| {
            let data_type = parse_function_data_type(&data_type)?;
            Ok(FunctionInput {
                identifier,
                data_type: data_type as i32,
                constant,
            })
        })
        .collect()
}

fn parse_function_data_type(value: &str) -> Result<FunctionDataType, ErrorData> {
    match value.to_ascii_lowercase().as_str() {
        "numeric" => Ok(FunctionDataType::Numeric),
        "string" => Ok(FunctionDataType::String),
        "bool" | "boolean" => Ok(FunctionDataType::Bool),
        other => Err(ErrorData::invalid_params(
            format!(
                "unknown function input data_type `{other}`; expected one of `numeric`, `string`, `bool`"
            ),
            None,
        )),
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCalculatedChannelParams {
    name: String,
    description: Option<String>,
    units: Option<String>,
    client_key: Option<String>,
    user_notes: Option<String>,
    expression: String,
    all_assets: Option<bool>,
    asset_ids: Option<Vec<String>>,
    asset_names: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
    channel_references_json: String,
    metadata: Option<Vec<MetadataEntry>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCalculatedChannelParams {
    calculated_channel_id: Option<String>,
    client_key: Option<String>,
    name: Option<String>,
    description: Option<String>,
    units: Option<String>,
    expression: Option<String>,
    channel_references_json: Option<String>,
    all_assets: Option<bool>,
    asset_ids: Option<Vec<String>>,
    asset_names: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
    user_notes: Option<String>,
}

/// One entry of `channel_references_json`: the token used in the expression
/// plus what it resolves to. `channel_identifier` and `calculated_channel_version_id`
/// are mutually exclusive (the inline-expression reference variant is unsupported).
#[derive(Debug, Deserialize, JsonSchema)]
struct ChannelReferenceInput {
    channel_reference: String,
    channel_identifier: Option<String>,
    calculated_channel_version_id: Option<String>,
}

/// The inline-expression reference variant (`calculated_channel`) is intentionally
/// unsupported; only a `channel_identifier` or a `calculated_channel_version_id`
/// target is accepted.
fn parse_channel_references(
    json: &str,
) -> Result<Vec<CalculatedChannelAbstractChannelReference>, ErrorData> {
    let inputs: Vec<ChannelReferenceInput> = serde_json::from_str(json).map_err(|e| {
        ErrorData::invalid_params(
            format!("`channel_references_json` is not valid JSON: {e}"),
            None,
        )
    })?;

    inputs
        .into_iter()
        .map(|input| {
            let calculated_channel_reference =
                match (input.channel_identifier.as_deref(), input.calculated_channel_version_id) {
                    (Some(_), Some(_)) => {
                        return Err(ErrorData::invalid_params(
                            format!(
                                "channel reference `{}` sets both `channel_identifier` and `calculated_channel_version_id`; use exactly one",
                                input.channel_reference
                            ),
                            None,
                        ));
                    }
                    (_, Some(version_id)) => {
                        Some(CalculatedChannelReference::CalculatedChannelVersionId(version_id))
                    }
                    (_, None) => None,
                };
            Ok(CalculatedChannelAbstractChannelReference {
                channel_reference: input.channel_reference,
                channel_identifier: input.channel_identifier.unwrap_or_default(),
                calculated_channel_reference,
            })
        })
        .collect()
}

/// Resolve exact asset names to ids via the asset service in a single query.
/// Errors if any name matches no asset. Output ids follow `names` order.
async fn resolve_asset_names(
    asset_service: &AssetService,
    names: &[String],
) -> Result<Vec<String>, ErrorData> {
    if names.is_empty() {
        return Ok(Vec::new());
    }

    let items = names
        .iter()
        .map(|n| format!("\"{}\"", cel_escape(n)))
        .collect::<Vec<_>>()
        .join(", ");
    let filter = format!("name in [{items}]");
    let assets = asset_service
        .list_assets(filter, None, None)
        .await
        .map_err(from_anyhow)?;

    let by_name: HashMap<&str, &str> = assets
        .iter()
        .map(|a| (a.name.as_str(), a.asset_id.as_str()))
        .collect();

    names
        .iter()
        .map(|name| {
            by_name
                .get(name.as_str())
                .map(|id| id.to_string())
                .ok_or_else(|| ErrorData::invalid_params(format!("asset '{name}' not found"), None))
        })
        .collect()
}

/// Build a calculated-channel asset configuration from the scope params,
/// resolving `asset_names` to ids. Returns `None` when no scope field is set
/// (the caller decides whether that is an error).
async fn build_asset_configuration(
    asset_service: &AssetService,
    all_assets: Option<bool>,
    asset_ids: Option<Vec<String>>,
    asset_names: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
) -> Result<Option<CalculatedChannelAssetConfiguration>, ErrorData> {
    let mut selection_ids = asset_ids.unwrap_or_default();
    let tag_ids = tag_ids.unwrap_or_default();
    if let Some(names) = asset_names {
        selection_ids.extend(resolve_asset_names(asset_service, &names).await?);
    }
    let has_selection = !selection_ids.is_empty() || !tag_ids.is_empty();

    match (all_assets, has_selection) {
        (Some(true), true) => Err(ErrorData::invalid_params(
            "set either `all_assets` or an asset/tag selection, not both",
            None,
        )),
        (Some(true), false) => Ok(Some(CalculatedChannelAssetConfiguration {
            asset_scope: Some(AssetScope::AllAssets(true)),
        })),
        (_, true) => Ok(Some(CalculatedChannelAssetConfiguration {
            asset_scope: Some(AssetScope::Selection(AssetSelection {
                asset_ids: selection_ids,
                tag_ids,
            })),
        })),
        (_, false) => Ok(None),
    }
}

#[tool_router(router = mutate_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "create_user_defined_function",
        description = "
            Create a user-defined function (UDF): a named, reusable expression with typed inputs that calculated
            channels and other UDFs can reference.

            Output:
              - `{ \"user_defined_function\": UserDefinedFunction }` for the created UDF, including its
                server-assigned `user_defined_function_id`, `version`, and `function_output_type`.

            Parameters:
              - `name`: UDF name. Unique within the organization.
              - `description`: optional human description.
              - `expression`: the function body in Sift Expression Language, referencing the declared input
                identifiers by name (e.g. with inputs `x` and `offset`: `x * 2 + offset`).
              - `input_identifiers`: identifiers for each input, in order. These are the names the `expression`
                references.
              - `input_data_types`: data type per input, one per identifier. Each is one of `numeric`, `string`,
                `bool` (case-insensitive).
              - `input_constants`: whether each input is a constant, one per identifier (`true`/`false`).
              - `user_notes`: optional notes describing this creation.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries; `value` may be a
                string, number, or boolean — the type is inferred from the JSON literal.

            The three `input_*` arrays are parallel and MUST be the same length; element `i` of each describes one input.

            Errors:
              - `INVALID_PARAMS` if the `input_*` arrays differ in length, an `input_data_types` value is not one of
                the accepted spellings, or the server rejects the expression/inputs.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - The server validates the expression and inputs; a malformed expression returns `INVALID_PARAMS` with
                the validation message. Fix the expression and retry.
        ",
        annotations(
            title = "mutate_router/create_user_defined_function",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false
        )
    )]
    pub async fn create_user_defined_function(
        &self,
        params: Parameters<CreateUserDefinedFunctionParams>,
    ) -> error::McpResult {
        let Parameters(CreateUserDefinedFunctionParams {
            name,
            description,
            expression,
            input_identifiers,
            input_data_types,
            input_constants,
            user_notes,
            metadata,
        }) = params;

        let function_inputs =
            build_function_inputs(input_identifiers, input_data_types, input_constants)?;
        let metadata = metadata
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        let out = self
            .udf_service
            .create_user_defined_function(
                name,
                description,
                expression,
                function_inputs,
                user_notes,
                metadata,
            )
            .await
            .map(|udf| serde_json::json!({ "user_defined_function": udf }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "update_user_defined_function",
        description = "
            Update fields of an existing user-defined function. Only the fields you provide change; omitted fields
            keep their current values (read-modify-write).

            Output:
              - `{ \"user_defined_function\": UserDefinedFunction }` for the updated UDF (new `version`).

            Parameters:
              - `user_defined_function_id`: id of the UDF to update (required).
              - `name`: optional new name. A rename is applied exclusively by the server, so `name` MUST be the
                only field changed in the call; combining it with other fields is rejected.
              - `description`: optional new description.
              - `expression`: optional new expression body.
              - `input_identifiers` / `input_data_types` / `input_constants`: optional. To change inputs, provide ALL
                THREE arrays together; they are parallel and MUST be the same length. Providing some but not all is an
                error. Data types are `numeric`, `string`, `bool` (case-insensitive).
              - `metadata`: optional replacement list of `{ \"name\", \"value\" }` entries; replaces existing
                metadata. `value` may be a string, number, or boolean — the type is inferred from the JSON literal.

            Errors:
              - `INVALID_PARAMS` if no updatable field is provided, `name` is combined with any other field, the
                `input_*` arrays are partially supplied or differ in length, a data type is unknown, or the server
                rejects the change (e.g. changing inputs on a UDF that has dependents).
              - `RESOURCE_NOT_FOUND` if `user_defined_function_id` does not exist.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Some changes are restricted once the UDF has dependents (name and input changes); the server returns
                `INVALID_PARAMS` with the reason.
        ",
        annotations(
            title = "mutate_router/update_user_defined_function",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true
        )
    )]
    pub async fn update_user_defined_function(
        &self,
        params: Parameters<UpdateUserDefinedFunctionParams>,
    ) -> error::McpResult {
        let Parameters(UpdateUserDefinedFunctionParams {
            user_defined_function_id,
            name,
            description,
            expression,
            input_identifiers,
            input_data_types,
            input_constants,
            metadata,
        }) = params;

        let function_inputs = match (input_identifiers, input_data_types, input_constants) {
            (None, None, None) => None,
            (Some(ids), Some(types), Some(constants)) => {
                Some(build_function_inputs(ids, types, constants)?)
            }
            _ => {
                return Err(ErrorData::invalid_params(
                    "to update inputs, provide all of `input_identifiers`, `input_data_types`, and `input_constants`",
                    None,
                ));
            }
        };

        let metadata = metadata.map(|m| m.into_iter().map(Into::into).collect::<Vec<_>>());

        let update = UserDefinedFunctionUpdate {
            name,
            description,
            expression,
            function_inputs,
            metadata,
        };

        let out = self
            .udf_service
            .update_user_defined_function(user_defined_function_id, update)
            .await
            .map(|udf| serde_json::json!({ "user_defined_function": udf }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "create_calculated_channel",
        description = "
            Create a calculated channel: a derived channel defined by a Sift Expression Language expression over
            referenced channels, scoped to one or more assets.

            Output:
              - `{ \"calculated_channel\": CalculatedChannel, \"inapplicable_assets\": [...] }`. `calculated_channel`
                is the created channel with its server-assigned `calculated_channel_id` and `version`.
                `inapplicable_assets` lists scoped assets the channel could not be applied to (e.g. missing
                referenced channels); empty when it applied everywhere.

            Parameters:
              - `name`: channel name.
              - `description`: optional description.
              - `units`: optional unit string.
              - `client_key`: optional caller-assigned identifier (enables later get/update by key).
              - `user_notes`: optional notes describing this creation.
              - `expression`: Sift Expression Language body referencing the tokens declared in
                `channel_references_json` (e.g. `$1 + $2`).
              - Asset scope (exactly one form required): set `all_assets` to `true` for all assets, OR provide an
                `asset_ids` / `asset_names` and/or `tag_ids` selection. `asset_names` are resolved to ids. Setting
                `all_assets` together with a selection is an error.
              - `channel_references_json`: JSON array mapping each expression token to a channel. Each element is
                `{ \"channel_reference\": \"$1\", \"channel_identifier\": \"<channel name>\" }`. Use
                `\"calculated_channel_version_id\"` instead of `channel_identifier` to reference another calculated
                channel version; the two are mutually exclusive.
              - `metadata`: optional list of `{ \"name\", \"value\" }` entries; `value` may be a string, number, or
                boolean — the type is inferred from the JSON literal.

            Errors:
              - `INVALID_PARAMS` if no asset scope (or both forms) is given, `channel_references_json` is malformed
                or a reference sets both `channel_identifier` and `calculated_channel_version_id`, an `asset_names`
                entry resolves to no asset, or the server rejects the expression/configuration.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - The server validates the expression and channel references; a malformed expression returns
                `INVALID_PARAMS` with the validation message.
        ",
        annotations(
            title = "mutate_router/create_calculated_channel",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false
        )
    )]
    pub async fn create_calculated_channel(
        &self,
        params: Parameters<CreateCalculatedChannelParams>,
    ) -> error::McpResult {
        let Parameters(CreateCalculatedChannelParams {
            name,
            description,
            units,
            client_key,
            user_notes,
            expression,
            all_assets,
            asset_ids,
            asset_names,
            tag_ids,
            channel_references_json,
            metadata,
        }) = params;

        let expression_channel_references = parse_channel_references(&channel_references_json)?;

        let asset_configuration = build_asset_configuration(
            &self.asset_service,
            all_assets,
            asset_ids,
            asset_names,
            tag_ids,
        )
        .await?
        .ok_or_else(|| {
            ErrorData::invalid_params(
                "an asset scope is required: set `all_assets` or an `asset_ids`/`asset_names`/`tag_ids` selection",
                None,
            )
        })?;

        let configuration = CalculatedChannelConfiguration {
            asset_configuration: Some(asset_configuration),
            query_configuration: Some(CalculatedChannelQueryConfiguration {
                query: Some(Query::Sel(Sel {
                    expression,
                    expression_channel_references,
                })),
            }),
        };

        let metadata = metadata
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        let resp = self
            .calculated_channel_service
            .create_calculated_channel(
                name,
                description.unwrap_or_default(),
                units,
                client_key,
                user_notes.unwrap_or_default(),
                configuration,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(serde_json::json!({
            "calculated_channel": resp.calculated_channel,
            "inapplicable_assets": resp.inapplicable_assets,
        })))
    }

    #[tool(
        name = "update_calculated_channel",
        description = "
            Update fields of an existing calculated channel. Only the fields you provide change; omitted fields keep
            their current values (read-modify-write).

            Output:
              - `{ \"calculated_channel\": CalculatedChannel, \"inapplicable_assets\": [...] }` for the updated
                channel (new `version`). `inapplicable_assets` lists scoped assets the change could not apply to.

            Parameters:
              - `calculated_channel_id` / `client_key`: identify the channel to update; exactly one MUST be set.
              - `name`: optional new name.
              - `description`: optional new description.
              - `units`: optional new units.
              - `expression`: optional new expression. Changing only the expression preserves the existing channel
                references; provide `channel_references_json` too if the referenced channels change.
              - `channel_references_json`: optional replacement references (same shape as in
                `create_calculated_channel`). Changing only the references preserves the existing expression.
              - Asset scope: optionally set `all_assets` OR an `asset_ids` / `asset_names` / `tag_ids` selection to
                replace the channel's asset scope. `asset_names` are resolved to ids. Omit all to leave scope unchanged.
              - `metadata`: optional replacement list of `{ \"name\", \"value\" }` entries.
              - `user_notes`: optional notes describing this change.

            Errors:
              - `INVALID_PARAMS` if neither/both id and client_key are set, no updatable field is provided,
                `channel_references_json` is malformed, both asset-scope forms are set, an `asset_names` entry
                resolves to no asset, or the server rejects the change.
              - `RESOURCE_NOT_FOUND` if the channel does not exist.
              - `INTERNAL_ERROR` for upstream gRPC failures.
        ",
        annotations(
            title = "mutate_router/update_calculated_channel",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true
        )
    )]
    pub async fn update_calculated_channel(
        &self,
        params: Parameters<UpdateCalculatedChannelParams>,
    ) -> error::McpResult {
        let Parameters(UpdateCalculatedChannelParams {
            calculated_channel_id,
            client_key,
            name,
            description,
            units,
            expression,
            channel_references_json,
            all_assets,
            asset_ids,
            asset_names,
            tag_ids,
            metadata,
            user_notes,
        }) = params;

        let (id, client_key) = match (calculated_channel_id, client_key) {
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "exactly one of `calculated_channel_id` or `client_key` must be set, not both",
                    None,
                ));
            }
            (None, None) => {
                return Err(ErrorData::invalid_params(
                    "one of `calculated_channel_id` or `client_key` must be set",
                    None,
                ));
            }
            (Some(id), None) => (id, String::new()),
            (None, Some(key)) => (String::new(), key),
        };

        let channel_references = match channel_references_json {
            Some(json) => Some(parse_channel_references(&json)?),
            None => None,
        };

        let asset_configuration = build_asset_configuration(
            &self.asset_service,
            all_assets,
            asset_ids,
            asset_names,
            tag_ids,
        )
        .await?;

        let metadata = metadata.map(|m| m.into_iter().map(Into::into).collect::<Vec<_>>());

        let update = CalculatedChannelUpdate {
            name,
            description,
            units,
            expression,
            channel_references,
            asset_configuration,
            metadata,
            user_notes,
        };

        let resp = self
            .calculated_channel_service
            .update_calculated_channel(id, client_key, update)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(serde_json::json!({
            "calculated_channel": resp.calculated_channel,
            "inapplicable_assets": resp.inapplicable_assets,
        })))
    }
}
