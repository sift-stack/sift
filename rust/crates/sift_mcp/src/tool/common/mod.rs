use rmcp::schemars::{self, JsonSchema};
use serde::Deserialize;

/// Shared parameters for the simple `list_*` tools (assets, runs, channels).
/// Resources with extra knobs (e.g. reports' `organization_id`) define their own
/// params struct in their domain module.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::ListParams;
    use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult};
    use serde_json::Value;

    /// Build `Parameters<ListParams>` for a tool test.
    pub(crate) fn list_params(filter: &str, limit: Option<u32>) -> Parameters<ListParams> {
        Parameters(ListParams {
            filter: filter.into(),
            order_by: None,
            limit,
        })
    }

    /// Extract the structured JSON body from a tool result.
    pub(crate) fn structured(result: CallToolResult) -> Value {
        result
            .structured_content
            .expect("expected structured content")
    }

    /// Extract a single field from a tool result's structured JSON body.
    pub(crate) fn structured_field(result: CallToolResult, key: &str) -> Value {
        let mut value = structured(result);
        value
            .get_mut(key)
            .unwrap_or_else(|| panic!("missing key `{key}` in structured content"))
            .take()
    }
}
