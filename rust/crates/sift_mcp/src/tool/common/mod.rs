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
    pub(crate) fields: Option<Vec<String>>,
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

/// Normalize a field name for matching so `asset_id`, `assetId` and `AssetId`
/// all address the same key. Callers carry names between tools that spell them
/// differently, and a name that quietly matches nothing is the failure this
/// projection exists to avoid.
fn normalized(name: &str) -> String {
    name.chars()
        .filter(|c| *c != '_' && *c != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

/// Serialize each item to JSON, adding nothing. The `list_*` tools with no web
/// URL to attach use this where the others use [`with_urls`].
pub(crate) fn to_values<T: Serialize>(items: &[T]) -> Result<Vec<Value>, ErrorData> {
    items
        .iter()
        .map(|item| {
            serde_json::to_value(item).map_err(|e| {
                ErrorData::internal_error(format!("failed to serialize list item: {e}"), None)
            })
        })
        .collect()
}

/// Restrict each object to `fields`. Returns the projected items and any
/// requested name that matched no key on any item. Values that are not objects
/// pass through untouched. Key order is unchanged from an unprojected response
/// (`serde_json::Map` sorts), so callers read by name, not position.
///
/// Proto3 omits fields sitting at their default value, so a name can be absent
/// from one item and present on another. Only a name absent from every item
/// counts as unmatched.
pub(crate) fn project_fields(items: Vec<Value>, fields: &[String]) -> (Vec<Value>, Vec<String>) {
    let wanted: Vec<String> = fields.iter().map(|f| normalized(f)).collect();
    let mut matched = vec![false; wanted.len()];

    let projected = items
        .into_iter()
        .map(|item| match item {
            Value::Object(mut obj) => {
                let keys: Vec<(String, String)> =
                    obj.keys().map(|k| (normalized(k), k.clone())).collect();
                let mut out = serde_json::Map::new();
                for (i, want) in wanted.iter().enumerate() {
                    let Some((_, key)) = keys.iter().find(|(norm, _)| norm == want) else {
                        continue;
                    };
                    matched[i] = true;
                    if let Some(value) = obj.remove(key) {
                        out.insert(key.clone(), value);
                    }
                }
                Value::Object(out)
            }
            other => other,
        })
        .collect();

    let unmatched = fields
        .iter()
        .zip(&matched)
        .filter(|(_, hit)| !**hit)
        .map(|(name, _)| name.clone())
        .collect();

    (projected, unmatched)
}

/// Build the structured body for a `list_*` tool. Projects the items when
/// `fields` is set, and reports any requested field that matched nothing, so a
/// mistyped name is visible rather than silently narrowing the response. An
/// empty `fields` array is treated as no projection.
///
/// Every body carries `count`, the number of items in this response — the size
/// of the page, not the number matching the filter. The service caps results at
/// `limit` and does not report whether more exist, so a caller can only infer a
/// truncated page from `count` reaching its `limit`. Surfacing that properly
/// needs the truncation flag the service currently discards. Counting a JSON
/// array by eye is arithmetic a caller should not have to do: an agent asked
/// how many channels an asset has read a 120-row response and reported 122,
/// then explained the discrepancy away rather than trusting its own correct
/// enumeration. The number is free here and exact, so hand it over.
pub(crate) fn list_body(key: &str, items: Vec<Value>, fields: Option<Vec<String>>) -> Value {
    let (items, unmatched) = match fields.as_deref() {
        Some(fields) if !fields.is_empty() => project_fields(items, fields),
        _ => (items, Vec::new()),
    };

    let mut body = serde_json::Map::new();
    body.insert("count".to_string(), Value::from(items.len()));
    body.insert(key.to_string(), Value::Array(items));
    if !unmatched.is_empty() {
        body.insert(
            "unmatched_fields".to_string(),
            Value::Array(unmatched.into_iter().map(Value::String).collect()),
        );
    }
    Value::Object(body)
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
            fields: None,
        })
    }

    /// Build `Parameters<ListParams>` requesting a field projection.
    pub(crate) fn list_params_with_fields(filter: &str, fields: &[&str]) -> Parameters<ListParams> {
        Parameters(ListParams {
            filter: filter.into(),
            order_by: None,
            limit: None,
            fields: Some(fields.iter().map(|f| (*f).to_string()).collect()),
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
