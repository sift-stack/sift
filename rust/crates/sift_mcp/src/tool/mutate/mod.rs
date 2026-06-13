use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::common::r#type::v1::{FunctionDataType, FunctionInput};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::user_defined_functions::UserDefinedFunctionUpdate,
    tool::common::MetadataEntry,
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
                identifiers (e.g. `$1 * 2` or `pressure + offset`).
              - `input_identifiers`: identifiers for each input, in order.
              - `input_data_types`: data type per input, one per identifier. Each is one of `numeric`, `string`,
                `bool` (case-insensitive).
              - `input_constants`: whether each input is a constant, one per identifier (`true`/`false`).
              - `user_notes`: optional notes describing this creation.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries; `value` may be a
                string, number, or boolean.

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
              - `name`: optional new name.
              - `description`: optional new description.
              - `expression`: optional new expression body.
              - `input_identifiers` / `input_data_types` / `input_constants`: optional. To change inputs, provide ALL
                THREE arrays together; they are parallel and MUST be the same length. Providing some but not all is an
                error. Data types are `numeric`, `string`, `bool` (case-insensitive).
              - `metadata`: optional replacement list of `{ \"name\", \"value\" }` entries; replaces existing metadata.

            Errors:
              - `INVALID_PARAMS` if no updatable field is provided, the `input_*` arrays are partially supplied or
                differ in length, a data type is unknown, or the server rejects the change (e.g. changing inputs on a
                UDF that has dependents).
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
}
