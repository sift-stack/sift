use rmcp::ErrorData;
use rmcp::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sift_rs::metadata::v1::{
    MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
};

/// Shared parameters for the simple `list_*` tools (assets, runs, channels).
/// Resources with extra knobs (e.g. reports' `organization_id`) define their own
/// params struct in their domain module.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
}

/// A single metadata scalar as it arrives over the wire. Flat (untagged) so the
/// value round-trips as a bare JSON string/number/bool, per the flat-params rule.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum MetadataScalar {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// A flat `{ "name": ..., "value": <scalar> }` metadata entry shared by every
/// tool that attaches metadata (data, assets, annotations, reports).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetadataEntry {
    pub(crate) name: String,
    pub(crate) value: MetadataScalar,
}

/// A trailing clause for a write tool's `next_step` that points at the operated
/// resource's Sift web URL. Empty when the URL is `None` — i.e. the host could
/// not be derived (e.g. self-hosted deployments without an `api.` subdomain) —
/// so URL derivation never fails an operation.
pub(crate) fn url_clause(url: Option<&str>) -> String {
    url.map(|u| format!(" View it in Sift: {u}"))
        .unwrap_or_default()
}

/// Serialize each item to JSON and inject a `url` field built by `url_of`, so a
/// listing surfaces a clickable Sift web link per row. Items whose url can't be
/// built (host underivable on self-hosted deployments) are returned unchanged,
/// without a `url` field. Mutates only object-shaped values.
pub(crate) fn with_urls<T: Serialize>(
    items: &[T],
    url_of: impl Fn(&T) -> Option<String>,
) -> Result<Vec<Value>, ErrorData> {
    items
        .iter()
        .map(|item| {
            let mut value = serde_json::to_value(item).map_err(|e| {
                ErrorData::internal_error(format!("failed to serialize list item: {e}"), None)
            })?;
            if let (Some(obj), Some(url)) = (value.as_object_mut(), url_of(item)) {
                obj.insert("url".to_string(), Value::String(url));
            }
            Ok(value)
        })
        .collect()
}

impl From<MetadataEntry> for MetadataValue {
    fn from(entry: MetadataEntry) -> Self {
        let (key_type, value) = match entry.value {
            MetadataScalar::String(s) => {
                (MetadataKeyType::String, MetadataValueInner::StringValue(s))
            }
            MetadataScalar::Number(n) => {
                (MetadataKeyType::Number, MetadataValueInner::NumberValue(n))
            }
            MetadataScalar::Boolean(b) => (
                MetadataKeyType::Boolean,
                MetadataValueInner::BooleanValue(b),
            ),
        };
        MetadataValue {
            key: Some(MetadataKey {
                name: entry.name,
                r#type: key_type.into(),
                ..Default::default()
            }),
            value: Some(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test;

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
