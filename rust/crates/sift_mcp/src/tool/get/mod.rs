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
pub struct GetUserDefinedFunctionParams {
    user_defined_function_id: Option<String>,
    name: Option<String>,
}

#[tool_router(router = get_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "get_user_defined_function",
        description = "
            Retrieve a single user-defined function (UDF) by id or by exact name.

            Output:
              - `{ \"user_defined_function\": UserDefinedFunction }`. The full Sift `UserDefinedFunction` shape:
                `user_defined_function_id`, `name`, `description`, `expression`, `function_inputs`
                (`identifier`, `data_type`, `constant`), `function_output_type`, `version`,
                `user_defined_function_version_id`, `metadata`, `is_archived`, and timestamps.

            Parameters:
              - `user_defined_function_id`: exact UDF id. Mutually exclusive with `name`; exactly one MUST be set.
              - `name`: exact UDF name (not a pattern). Mutually exclusive with `user_defined_function_id`;
                exactly one MUST be set.

            Errors:
              - `INVALID_PARAMS` if neither or both of `user_defined_function_id` / `name` are set.
              - `RESOURCE_NOT_FOUND` if no UDF matches.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Prefer `user_defined_function_id` when known; it is unambiguous. Use `name` for human-driven lookups.
              - To discover ids/names first, call `list_user_defined_functions`.
        ",
        annotations(title = "get_router/get_user_defined_function", read_only_hint = true)
    )]
    pub async fn get_user_defined_function(
        &self,
        params: Parameters<GetUserDefinedFunctionParams>,
    ) -> error::McpResult {
        let Parameters(GetUserDefinedFunctionParams {
            user_defined_function_id,
            name,
        }) = params;

        let (id, name) = match (user_defined_function_id, name) {
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "exactly one of `user_defined_function_id` or `name` must be set, not both",
                    None,
                ));
            }
            (None, None) => {
                return Err(ErrorData::invalid_params(
                    "one of `user_defined_function_id` or `name` must be set",
                    None,
                ));
            }
            (Some(id), None) => (id, String::new()),
            (None, Some(name)) => (String::new(), name),
        };

        let out = self
            .udf_service
            .get_user_defined_function(id, name)
            .await
            .map(|udf| serde_json::json!({ "user_defined_function": udf }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
