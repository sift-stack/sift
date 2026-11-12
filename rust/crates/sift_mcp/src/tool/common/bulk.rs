use std::collections::HashMap;
use std::future::Future;

use rmcp::{
    ErrorData,
    model::{CallToolResult, ContentBlock, ErrorCode},
    schemars::{self, JsonSchema},
};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::{Map, Value, json};

use crate::{error::McpResult, service::batch};

pub const MAX_ITEMS: usize = 100;

/// Shared item list and approval mask for every bulk write tool.
#[derive(Debug, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct BulkItems<T> {
    /// 1–100 items. Pass one item to write one resource.
    #[schemars(length(min = 1, max = 100))]
    #[schemars(with = "Vec<T>")]
    pub items: Vec<Value>,
    /// Approval host's zero-based selection of `items` to write. Omit when proposing.
    /// An empty selection performs no writes.
    pub selected_indices: Option<Vec<usize>>,
    #[schemars(skip)]
    _item: std::marker::PhantomData<T>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BulkCall {
    items: Vec<Value>,
    selected_indices: Option<Vec<usize>>,
}

impl<'de, T> Deserialize<'de> for BulkItems<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let object = Map::<String, Value>::deserialize(deserializer)?;
        let BulkCall {
            items,
            selected_indices,
        } = bulk_call(object).map_err(serde::de::Error::custom)?;
        Ok(Self {
            items,
            selected_indices,
            _item: std::marker::PhantomData,
        })
    }
}

/// The single place that decides which top-level shapes a bulk tool accepts.
/// Only `items` (plus the host's `selected_indices`) is accepted. A call in the
/// old one-resource shape is rejected with a migration hint.
fn bulk_call(object: Map<String, Value>) -> Result<BulkCall, String> {
    if !object.contains_key("items") {
        return Err(
            "missing field `items`: pass the fields for each resource as one object in the \
             `items` array, for example `{\"items\": [{...}]}`"
                .into(),
        );
    }
    serde_json::from_value(Value::Object(object)).map_err(|error| error.to_string())
}

/// Prefix an item's validation error with its index in `items`.
pub fn at_item(index: usize, error: ErrorData) -> ErrorData {
    ErrorData::new(
        error.code,
        format!("`items` item {index}: {}", error.message),
        error.data,
    )
}

pub fn parse_items<T: DeserializeOwned + JsonSchema>(
    items: Vec<Value>,
) -> Result<Vec<T>, ErrorData> {
    if items.is_empty() || items.len() > MAX_ITEMS {
        return Err(ErrorData::invalid_params(
            format!("`items` must contain 1–{MAX_ITEMS} items"),
            None,
        ));
    }
    let schema = schemars::schema_for!(T);
    items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            if let Some(fields) = schema.get("properties").and_then(Value::as_object)
                && let Some(object) = item.as_object()
                && let Some(unknown) = object.keys().find(|key| !fields.contains_key(*key))
            {
                return Err(ErrorData::invalid_params(
                    format!("`items` item {index}: unknown field `{unknown}`"),
                    None,
                ));
            }
            serde_json::from_value(item).map_err(|error| {
                ErrorData::invalid_params(format!("`items` item {index}: {error}"), None)
            })
        })
        .collect()
}

/// Reject a call in which two items write the same resource, so the result
/// never depends on item order. `target` returns `None` for items without one.
pub fn reject_duplicate_targets<T>(
    items: &[T],
    resource: &str,
    target: impl Fn(&T) -> Option<String>,
) -> Result<(), ErrorData> {
    let mut seen = HashMap::new();
    for (index, item) in items.iter().enumerate() {
        let Some(key) = target(item) else { continue };
        if let Some(first) = seen.insert(key, index) {
            return Err(ErrorData::invalid_params(
                format!("`items` item {index}: targets the same {resource} as item {first}"),
                None,
            ));
        }
    }
    Ok(())
}

/// The approval mask and whether this server requires one.
#[derive(Debug, Clone)]
pub struct Selection {
    pub indices: Option<Vec<usize>>,
    /// Set when an approval host runs this server (`sift-cli mcp --chat`). A missing mask
    /// then fails closed so a host regression cannot write unapproved items.
    pub required: bool,
}

/// The kind of write a bulk tool performs. It sets the result's count field,
/// its wording, and which errors stop the remaining items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Create,
    Update,
    Archive,
    Unarchive,
}

impl Op {
    fn past(self) -> &'static str {
        match self {
            Self::Create => "Created",
            Self::Update => "Updated",
            Self::Archive => "Archived",
            Self::Unarchive => "Unarchived",
        }
    }

    fn count_key(self) -> &'static str {
        match self {
            Self::Create => "created_count",
            Self::Update => "updated_count",
            Self::Archive => "archived_count",
            Self::Unarchive => "unarchived_count",
        }
    }
}

fn reason(error: &ErrorData) -> Option<&str> {
    error.data.as_ref()?.get("reason")?.as_str()
}

/// Item-specific errors let the remaining items run. Anything that points at the
/// backend or the caller's session stops them. Permission on an existing resource
/// is scoped to that resource (rule assets, artifact creator), so it is item-specific
/// for writes to existing resources.
pub(crate) fn should_stop(op: Op, error: &ErrorData) -> bool {
    if error.code == ErrorCode::INVALID_PARAMS || error.code == ErrorCode::RESOURCE_NOT_FOUND {
        return false;
    }
    match reason(error) {
        Some("AlreadyExists" | "Conflict") => false,
        Some("PermissionDenied") => op == Op::Create,
        _ => true,
    }
}

pub async fn run<T, F, Fut>(
    op: Op,
    items: Vec<T>,
    selection: Selection,
    partial_writes: bool,
    operation: F,
) -> McpResult
where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = McpResult>,
{
    let requested_count = items.len();
    let selected_indices = match selection.indices {
        Some(indices) => indices,
        None if selection.required => {
            return Err(ErrorData::invalid_params(
                "Writes require explicit `selected_indices` from the approval host; nothing was written. \
                 The chat host is out of date for this tool. Tell the user the change could not be \
                 applied from chat yet.",
                None,
            ));
        }
        None => (0..requested_count).collect(),
    };
    let mut selected = vec![false; requested_count];
    for &index in &selected_indices {
        if index >= requested_count || selected[index] {
            return Err(ErrorData::invalid_params(
                "`selected_indices` must contain unique, in-range input indices",
                None,
            ));
        }
        selected[index] = true;
    }
    let indices = (0..requested_count)
        .filter(|&index| selected[index])
        .collect::<Vec<_>>();
    let not_selected = (0..requested_count)
        .filter(|&index| !selected[index])
        .collect::<Vec<_>>();
    let items = items
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| selected[index].then_some(item))
        .collect();
    let outcome = batch::execute(items, operation, |error| should_stop(op, error)).await;
    let not_attempted = outcome
        .not_attempted
        .into_iter()
        .map(|index| indices[index])
        .collect::<Vec<_>>();
    let successes = outcome
        .successes
        .into_iter()
        .map(
            |(index, result)| json!({"index": indices[index], "result": result.structured_content}),
        )
        .collect::<Vec<_>>();
    let failures = outcome
        .failures
        .into_iter()
        .map(|(index, error)| {
            // An interrupted write or a composite task can commit before returning
            // an error. Never instruct callers to blindly replay a failed write.
            let outcome_unknown = partial_writes
                || (error.code != ErrorCode::INVALID_PARAMS
                    && error.code != ErrorCode::RESOURCE_NOT_FOUND
                    && !matches!(
                        reason(&error),
                        Some("AlreadyExists" | "PermissionDenied" | "Unauthenticated")
                    ));
            json!({"index": indices[index], "error": error, "outcome_unknown": outcome_unknown})
        })
        .collect::<Vec<_>>();
    let has_errors = !failures.is_empty() || !not_attempted.is_empty();
    let next_step = format!(
        "{} {} items; {} failed; {} not attempted. \
         On success, confirm in one short sentence using resource names or links; omit unrequested metadata, \
         internal indices/masks, and approval/retry narration. Report actionable warnings and failures. \
         Never replay the batch: successful writes are committed, and the user rejected the items in \
         `not_selected`, so never retry those. `outcome_unknown` items may also have been written or \
         partially written. Inspect uncertain outcomes before retrying, follow each error's guidance, \
         and do not retry backend-wide failures in this prompt.",
        op.past(),
        successes.len(),
        failures.len(),
        not_attempted.len(),
    );
    let mut body = Map::new();
    body.insert("requested_count".into(), json!(requested_count));
    body.insert("selected_count".into(), json!(indices.len()));
    body.insert("not_selected".into(), json!(not_selected));
    body.insert(op.count_key().into(), json!(successes.len()));
    body.insert("successes".into(), json!(successes));
    body.insert("failures".into(), json!(failures));
    body.insert("not_attempted".into(), json!(not_attempted));
    body.insert("next_step".into(), json!(next_step));
    let body = Value::Object(body);
    let text = body.to_string();
    let mut result = CallToolResult::structured(body);
    result.is_error = Some(has_errors);
    // Models in some MCP clients cannot see structuredContent.
    result.content = vec![ContentBlock::text(next_step), ContentBlock::text(text)];
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Deserialize, JsonSchema)]
    struct Item {
        id: String,
    }

    fn all() -> Selection {
        Selection {
            indices: None,
            required: false,
        }
    }

    fn signal(code: tonic::Code) -> ErrorData {
        crate::error::from_anyhow(tonic::Status::new(code, "failure").into())
    }

    #[test]
    fn the_legacy_single_item_shape_is_rejected_with_a_migration_hint() {
        let error = serde_json::from_value::<BulkItems<Item>>(json!({"id": "a"})).unwrap_err();
        assert!(error.to_string().contains("`items` array"), "{error}");
        let error =
            serde_json::from_value::<BulkItems<Item>>(json!({"items": [], "id": "a"})).unwrap_err();
        assert!(error.to_string().contains("unknown field `id`"), "{error}");
        let call: BulkItems<Item> =
            serde_json::from_value(json!({"items": [{"id": "a"}], "selected_indices": [0]}))
                .unwrap();
        assert_eq!(call.items.len(), 1);
        assert_eq!(call.selected_indices, Some(vec![0]));
    }

    #[test]
    fn the_schema_is_a_bounded_array_that_requires_items() {
        let schema = serde_json::to_value(schemars::schema_for!(BulkItems<Item>)).unwrap();
        let items = &schema["properties"]["items"];
        assert_eq!(items["type"], "array");
        assert_eq!(items["minItems"], 1);
        assert_eq!(items["maxItems"], 100);
        assert_eq!(schema["required"], json!(["items"]));
        assert_eq!(schema["additionalProperties"], false);
    }

    #[test]
    fn duplicate_targets_name_both_items() {
        let items = ["a", "b", "a"].map(|id| Item { id: id.into() });
        let error =
            reject_duplicate_targets(&items, "rule", |item| Some(item.id.clone())).unwrap_err();
        assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
        assert!(error.message.contains("item 2"), "{}", error.message);
        assert!(error.message.contains("as item 0"), "{}", error.message);
        assert!(
            reject_duplicate_targets(&items[..2], "rule", |item| Some(item.id.clone())).is_ok()
        );
    }

    #[test]
    fn permission_errors_stop_creates_but_not_writes_to_existing_resources() {
        let denied = signal(tonic::Code::PermissionDenied);
        assert!(should_stop(Op::Create, &denied));
        for op in [Op::Update, Op::Archive, Op::Unarchive] {
            assert!(!should_stop(op, &denied), "{op:?}");
            assert!(!should_stop(
                op,
                &ErrorData::resource_not_found("missing", None)
            ));
            assert!(!should_stop(op, &signal(tonic::Code::Aborted)));
            for code in [
                tonic::Code::Unavailable,
                tonic::Code::ResourceExhausted,
                tonic::Code::DeadlineExceeded,
                tonic::Code::Internal,
                tonic::Code::Unauthenticated,
                tonic::Code::Cancelled,
                tonic::Code::Unknown,
            ] {
                assert!(should_stop(op, &signal(code)), "{op:?} {code:?}");
            }
        }
    }

    #[tokio::test]
    async fn each_op_reports_its_own_count_and_verb() {
        for (op, key, verb) in [
            (Op::Create, "created_count", "Created 2 items"),
            (Op::Update, "updated_count", "Updated 2 items"),
            (Op::Archive, "archived_count", "Archived 2 items"),
            (Op::Unarchive, "unarchived_count", "Unarchived 2 items"),
        ] {
            let result = run(op, vec![0, 1], all(), false, |item| async move {
                Ok(CallToolResult::structured(json!({"item": item})))
            })
            .await
            .unwrap();
            let body = result.structured_content.unwrap();
            assert_eq!(body[key], 2, "{op:?}");
            let next_step = body["next_step"].as_str().unwrap();
            assert!(next_step.starts_with(verb), "{next_step}");
            assert!(next_step.contains("never retry those"), "{next_step}");
        }
    }

    #[tokio::test]
    async fn a_permission_failure_on_one_archive_does_not_stop_the_rest() {
        let result = run(
            Op::Archive,
            vec![0, 1, 2],
            Selection {
                indices: Some(vec![0, 1, 2]),
                required: true,
            },
            false,
            |item| async move {
                if item == 1 {
                    Err(signal(tonic::Code::PermissionDenied))
                } else {
                    Ok(CallToolResult::structured(json!({"item": item})))
                }
            },
        )
        .await
        .unwrap();
        assert_eq!(result.is_error, Some(true));
        let body = result.structured_content.unwrap();
        assert_eq!(body["archived_count"], 2);
        assert_eq!(body["failures"][0]["index"], 1);
        assert_eq!(body["failures"][0]["outcome_unknown"], false);
        assert_eq!(body["not_attempted"], json!([]));
    }

    #[tokio::test]
    async fn a_required_mask_fails_closed_for_every_op() {
        for op in [Op::Update, Op::Archive, Op::Unarchive] {
            let error = run(
                op,
                vec![0],
                Selection {
                    indices: None,
                    required: true,
                },
                false,
                |_| async { panic!("no write may run without a mask") },
            )
            .await
            .unwrap_err();
            assert!(
                error.message.contains("nothing was written"),
                "{}",
                error.message
            );
        }
    }
}
