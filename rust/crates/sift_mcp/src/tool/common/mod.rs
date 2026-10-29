use rmcp::ErrorData;
use rmcp::model::{CallToolResult, ContentBlock};
use rmcp::schemars::{self, JsonSchema};
use serde::ser::SerializeMap;
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

/// A trailing clause for a tool's `next_step` that tells the model how to
/// present an entity's Sift web link. The Sift chat renders a Markdown link as
/// a chip whose text must be the entity's own name: `[name](url)`. When the
/// caller has no name in scope, the model is told to supply it. Empty when the
/// URL is `None`, so URL derivation never fails an operation.
pub(crate) fn url_clause(kind: &str, name: Option<&str>, url: Option<&str>) -> String {
    let Some(url) = url else {
        return String::new();
    };
    match name {
        Some(name) => format!(
            " Present it to the user as the Markdown link [{}]({url}).",
            escape_link_text(name)
        ),
        None => format!(
            " Present the {kind} to the user as a Markdown link to {url} whose text is the {kind}'s \
             name, never a bare id or the word \"link\"."
        ),
    }
}

/// Backslash-escape the characters that end or nest Markdown link text. An
/// unbalanced bracket, as in `results v2].csv`, otherwise stops the text from
/// rendering as a link at all.
fn escape_link_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\\' | '[' | ']') {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
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

/// Normalize a field name for matching so `asset_id`, `assetId`, `asset-id` and
/// `AssetId` all address the same key: case is folded and `_` and `-` are
/// dropped. Callers carry names between tools that spell them differently, and a
/// name that quietly matches nothing is the failure this projection exists to
/// avoid.
///
/// Two source keys on the same object can therefore normalize alike — `asset_id`
/// beside `assetId` would both reduce to `assetid`. The first in key order wins,
/// deterministically. Proto3 JSON emits one casing per field so our shapes do
/// not hit this, but a serializer change could, and picking one is better than
/// erroring on an ambiguity the caller cannot see.
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
/// requested name that matched no key on any item — empty when the page itself
/// is empty, since an absent row proves nothing about a field name. Values that are not objects
/// pass through untouched. Key order is unchanged from an unprojected response
/// (`serde_json::Map` sorts), so callers read by name, not position.
///
/// Proto3 omits fields sitting at their default value, so a name can be absent
/// from one item and present on another. Only a name absent from every item
/// counts as unmatched.
pub(crate) fn project_fields(items: Vec<Value>, fields: &[String]) -> (Vec<Value>, Vec<String>) {
    // Nothing came back, so nothing can be said about the field names. Reporting
    // them all as unmatched would read as a spelling problem and send the caller
    // off renaming fields that were fine — the filter simply matched no rows.
    if items.is_empty() {
        return (items, Vec::new());
    }

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

/// The key under which `list_body` lists every field requested in `fields`
/// that matched no key on any returned item.
pub(crate) const UNMATCHED_FIELDS_KEY: &str = "unmatched_fields";

/// Build the structured body for a `list_*` tool. Projects the items when
/// `fields` is set, and reports any requested field that matched nothing under
/// [`UNMATCHED_FIELDS_KEY`], so a mistyped name is visible rather than silently
/// narrowing the response. An empty `fields` array is treated as no projection.
///
/// Every body carries `count`, the number of items in this response, and
/// `has_more`, whether the service stopped at `limit` with matches left over.
/// `count` is the size of the page, not the match total — the two fields
/// together are what let a caller tell a complete result from a truncated one
/// instead of guessing from whether `count` happens to equal `limit`. Counting a JSON
/// array by eye is arithmetic a caller should not have to do: an agent asked
/// how many channels an asset has read a 120-row response and reported 122,
/// then explained the discrepancy away rather than trusting its own correct
/// enumeration. The number is free here and exact, so hand it over.
pub(crate) fn list_body(
    key: &str,
    items: Vec<Value>,
    fields: Option<Vec<String>>,
    has_more: bool,
) -> Value {
    let (items, unmatched) = match fields.as_deref() {
        Some(fields) if !fields.is_empty() => project_fields(items, fields),
        _ => (items, Vec::new()),
    };

    let mut body = serde_json::Map::new();
    body.insert("count".to_string(), Value::from(items.len()));
    body.insert("has_more".to_string(), Value::Bool(has_more));
    body.insert(key.to_string(), Value::Array(items));
    if !unmatched.is_empty() {
        body.insert(
            UNMATCHED_FIELDS_KEY.to_string(),
            Value::Array(unmatched.into_iter().map(Value::String).collect()),
        );
    }
    Value::Object(body)
}

/// Shape a `list_*` tool's result so its metadata survives truncation. Agent
/// harnesses cut an oversized tool result from the end, and `serde_json::Map`
/// sorts keys, so `count`, `has_more` and [`UNMATCHED_FIELDS_KEY`] land after
/// the items whenever the items key sorts first (`channels`, `assets`, ...).
/// That loses them on exactly the large pages where they matter.
///
/// The text block agents read writes every key except `items_key` first, in
/// their usual order, then the items. `structuredContent` keeps `body` as is,
/// so the two differ only in key order. Pass `next_step` for a tool that also
/// leads with an instruction; it becomes its own text block ahead of the JSON,
/// as in [`read_result`].
pub(crate) fn list_result(
    body: Value,
    items_key: &str,
    next_step: Option<String>,
) -> CallToolResult {
    let text = ItemsLast {
        body: &body,
        items_key,
    };
    let text = serde_json::to_string(&text).unwrap_or_else(|_| body.to_string());
    let mut content = Vec::with_capacity(2);
    if let Some(next_step) = next_step {
        content.push(ContentBlock::text(next_step));
    }
    content.push(ContentBlock::text(text));

    let mut result = CallToolResult::structured(body);
    result.content = content;
    result
}

/// Serializes a JSON object with one key moved to the end. `SerializeMap`
/// writes entries in the order they are given, which `serde_json::Map` would
/// otherwise re-sort.
struct ItemsLast<'a> {
    body: &'a Value,
    items_key: &'a str,
}

impl Serialize for ItemsLast<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Value::Object(map) = self.body else {
            return self.body.serialize(serializer);
        };
        let mut out = serializer.serialize_map(Some(map.len()))?;
        for (key, value) in map.iter().filter(|(key, _)| *key != self.items_key) {
            out.serialize_entry(key, value)?;
        }
        if let Some(items) = map.get(self.items_key) {
            out.serialize_entry(self.items_key, items)?;
        }
        out.end()
    }
}

/// Shape a read tool's result so the calling agent sees both `next_step` and
/// the payload. `CallToolResult::structured` already mirrors `body` into a text
/// content block; replacing `content` with `next_step` alone, as write tools
/// do, leaves the payload only in `structuredContent`, which clients such as
/// Claude Code do not show the model. `preview_rule` hit exactly this: agents
/// saw an annotation count but none of the annotations.
pub(crate) fn read_result(body: Value, next_step: String) -> CallToolResult {
    let mut result = CallToolResult::structured(body);
    result.content.insert(0, ContentBlock::text(next_step));
    result
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

    /// Collect the text of every content block, which is what clients such as
    /// Claude Code show the model.
    pub(crate) fn content_texts(result: &CallToolResult) -> Vec<String> {
        result
            .content
            .iter()
            .map(|block| {
                block
                    .as_text()
                    .expect("expected a text content block")
                    .text
                    .clone()
            })
            .collect()
    }

    /// Assert that every key in `earlier` is written before `later` in `text`.
    /// List tools write their metadata ahead of the items so it survives an
    /// agent harness truncating the result from the end.
    pub(crate) fn assert_keys_before(text: &str, earlier: &[&str], later: &str) {
        let offset = |key: &str| {
            text.find(&format!("\"{key}\":"))
                .unwrap_or_else(|| panic!("missing key `{key}` in {text}"))
        };
        let later_at = offset(later);
        for key in earlier {
            assert!(
                offset(key) < later_at,
                "`{key}` must come before `{later}` in {text}"
            );
        }
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
