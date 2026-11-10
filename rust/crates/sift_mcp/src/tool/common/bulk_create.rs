use std::future::Future;

use rmcp::{
    ErrorData,
    model::{CallToolResult, ContentBlock, ErrorCode},
    schemars::{self, JsonSchema},
};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{error::McpResult, service::batch};

pub const MAX_CREATE_ITEMS: usize = 100;

/// Shared item list and approval mask for every create tool.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateItems<T> {
    /// 1–100 items to create. Pass one item to create one resource.
    #[schemars(length(min = 1, max = 100))]
    #[schemars(with = "Vec<T>")]
    pub items: Vec<Value>,
    /// Approval host's zero-based selection of `items` to create. Omit when proposing.
    /// An empty selection performs no writes.
    pub selected_indices: Option<Vec<usize>>,
    #[serde(skip)]
    #[schemars(skip)]
    _item: std::marker::PhantomData<T>,
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
    if items.is_empty() || items.len() > MAX_CREATE_ITEMS {
        return Err(ErrorData::invalid_params(
            format!("`items` must contain 1–{MAX_CREATE_ITEMS} items"),
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

fn reason(error: &ErrorData) -> Option<&str> {
    error.data.as_ref()?.get("reason")?.as_str()
}

fn should_stop(error: &ErrorData) -> bool {
    error.code != ErrorCode::INVALID_PARAMS
        && error.code != ErrorCode::RESOURCE_NOT_FOUND
        && !matches!(reason(error), Some("AlreadyExists" | "Conflict"))
}

pub async fn create_many<T, F, Fut>(items: Vec<T>, selection: Selection, operation: F) -> McpResult
where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = McpResult>,
{
    create_many_with_partial_writes(items, selection, operation, false).await
}

/// The approval mask and whether this server requires one.
#[derive(Debug, Clone)]
pub struct Selection {
    pub indices: Option<Vec<usize>>,
    /// Set when an approval host runs this server (`sift-cli mcp --chat`). A missing mask
    /// then fails closed so a host regression cannot create unapproved items.
    pub required: bool,
}

pub async fn create_many_with_partial_writes<T, F, Fut>(
    items: Vec<T>,
    selection: Selection,
    operation: F,
    partial_writes: bool,
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
                "Creates require explicit `selected_indices` from the approval host; no unapproved items were created",
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
    let outcome = batch::execute(items, operation, should_stop).await;
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
            // an error. Never instruct callers to blindly replay a failed create.
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
        "Created {} items; {} failed; {} not attempted. \
         On success, confirm in one short sentence using resource names or links; omit unrequested metadata, \
         internal indices/masks, and approval/retry narration. Report actionable warnings and failures. \
         Never replay the batch: successful writes are committed; `outcome_unknown` items may also exist \
         or be partially created. Inspect uncertain outcomes before retrying, follow each error's guidance, \
         and do not retry backend-wide failures in this prompt.",
        successes.len(),
        failures.len(),
        not_attempted.len(),
    );
    let body = json!({
        "requested_count": requested_count,
        "selected_count": indices.len(),
        "not_selected": not_selected,
        "created_count": successes.len(),
        "successes": successes,
        "failures": failures,
        "not_attempted": not_attempted,
        "next_step": next_step,
    });
    let text = body.to_string();
    let mut result = CallToolResult::structured(body);
    result.is_error = Some(has_errors);
    // Models in some MCP clients cannot see structuredContent.
    result.content = vec![ContentBlock::text(next_step), ContentBlock::text(text)];
    Ok(result)
}

#[cfg(test)]
#[path = "bulk_create_test.rs"]
mod integration_test;

#[cfg(test)]
#[path = "bulk_create_retry_test.rs"]
mod retry_test;

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Deserialize, JsonSchema)]
    struct Item {
        name: String,
    }

    fn required(indices: Option<Vec<usize>>) -> Selection {
        Selection {
            indices,
            required: true,
        }
    }

    #[test]
    fn validates_the_entire_input_and_bounds_before_writes() {
        for input in [
            json!([]),
            json!([1]),
            json!([{"name": "ok"}, {"typo": "bad"}]),
            json!([{"name": 3}]),
        ] {
            let items: Vec<Value> = serde_json::from_value(input.clone()).unwrap();
            assert!(parse_items::<Item>(items).is_err(), "{input}");
        }
        assert!(parse_items::<Item>(vec![json!({"name":"x"}); 101]).is_err());
        let error =
            parse_items::<Item>(vec![json!({"name": "ok"}), json!({"typo": "bad"})]).unwrap_err();
        assert!(error.message.contains("item 1"), "{}", error.message);
        assert_eq!(
            parse_items::<Item>(vec![json!({"name":"ok"})]).unwrap()[0].name,
            "ok"
        );
    }

    #[tokio::test]
    async fn a_missing_selection_creates_every_item_unless_a_host_requires_one() {
        let result = create_many(
            vec![0, 1],
            Selection {
                indices: None,
                required: false,
            },
            |item| async move { Ok(CallToolResult::structured(json!({"item": item}))) },
        )
        .await
        .unwrap();
        let body = result.structured_content.unwrap();
        assert_eq!(body["created_count"], 2);
        assert_eq!(body["not_selected"], json!([]));
        assert!(
            create_many(vec![0], required(None), |_| async {
                Ok(CallToolResult::structured(json!({})))
            })
            .await
            .is_err()
        );
    }

    #[test]
    fn the_schema_is_a_bounded_array_of_items() {
        let schema = serde_json::to_value(schemars::schema_for!(CreateItems<Item>)).unwrap();
        let items = &schema["properties"]["items"];
        assert_eq!(items["type"], "array");
        assert_eq!(items["minItems"], 1);
        assert_eq!(items["maxItems"], 100);
        assert_eq!(schema["required"], json!(["items"]));
    }

    #[tokio::test]
    async fn reports_partial_successes_and_preserves_error_guidance() {
        let result = create_many(
            vec![0, 1, 2],
            required(Some(vec![0, 1, 2])),
            |i| async move {
                if i == 1 {
                    Err(ErrorData::invalid_params("bad item", None))
                } else {
                    Ok(CallToolResult::structured(json!({"id": i})))
                }
            },
        )
        .await
        .unwrap();
        assert_eq!(result.is_error, Some(true));
        let body = result.structured_content.unwrap();
        assert_eq!(body["created_count"], 2);
        assert_eq!(body["successes"][1]["index"], 2);
        assert_eq!(body["failures"][0]["index"], 1);
        assert_eq!(body["failures"][0]["outcome_unknown"], false);
    }

    #[tokio::test]
    async fn composite_create_can_fail_validation_after_partial_writes() {
        let result = create_many_with_partial_writes(
            vec![0],
            required(Some(vec![0])),
            |_| async {
                Err(ErrorData::invalid_params(
                    "report already created; invalid step",
                    None,
                ))
            },
            true,
        )
        .await
        .unwrap();
        let body = result.structured_content.unwrap();
        assert_eq!(body["failures"][0]["outcome_unknown"], true);
        assert_eq!(body["created_count"], 0);
    }

    #[tokio::test]
    async fn selecting_two_of_three_never_attempts_the_rejected_item() {
        let result = create_many(
            vec![0, 1, 2],
            required(Some(vec![2, 0])),
            |item| async move {
                assert_ne!(
                    item, 1,
                    "the rejected item must never reach a create handler"
                );
                Ok(CallToolResult::structured(json!({"created_item": item})))
            },
        )
        .await
        .unwrap();
        let body = result.structured_content.unwrap();
        assert_eq!(body["requested_count"], 3);
        assert_eq!(body["selected_count"], 2);
        assert_eq!(body["created_count"], 2);
        assert_eq!(body["not_selected"], json!([1]));
        assert_eq!(body["successes"][0]["index"], 0);
        assert_eq!(body["successes"][1]["index"], 2);
        assert_eq!(body["successes"][1]["result"]["created_item"], 2);
    }

    #[tokio::test]
    async fn confirmation_guidance_keeps_indices_internal() {
        let result = create_many(
            vec!["first", "second", "third"],
            required(Some(vec![0, 2])),
            |name| async move { Ok(CallToolResult::structured(json!({"name": name}))) },
        )
        .await
        .unwrap();
        let body = result.structured_content.unwrap();
        let guidance = body["next_step"].as_str().unwrap();
        assert!(guidance.contains("confirm in one short sentence using resource names or links"));
        assert!(guidance.contains(
            "omit unrequested metadata, internal indices/masks, and approval/retry narration"
        ));
        assert!(!guidance.contains("Present each indexed result"));
        assert_eq!(body["successes"][0]["index"], 0);
        assert_eq!(body["successes"][1]["index"], 2);
        assert_eq!(body["not_selected"], json!([1]));
    }

    #[tokio::test]
    async fn missing_or_invalid_selection_never_defaults_to_all_items() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let calls = AtomicUsize::new(0);
        for selection in [None, Some(vec![0, 0]), Some(vec![3])] {
            let result = create_many(vec![0, 1, 2], required(selection), |item| {
                calls.fetch_add(1, Ordering::SeqCst);
                async move { Ok(CallToolResult::structured(json!({"item": item}))) }
            })
            .await;
            assert!(result.is_err());
        }
        let result = create_many(vec![0, 1, 2], required(Some(vec![])), |item| {
            calls.fetch_add(1, Ordering::SeqCst);
            async move { Ok(CallToolResult::structured(json!({"item": item}))) }
        })
        .await
        .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let body = result.structured_content.unwrap();
        assert_eq!(body["created_count"], 0);
        assert_eq!(body["not_selected"], json!([0, 1, 2]));
    }

    #[tokio::test]
    async fn selected_partial_failures_and_unattempted_items_keep_original_indices() {
        let result = create_many(
            vec![0, 1, 2, 3, 4],
            required(Some(vec![0, 2, 4])),
            |item| async move {
                if item == 2 {
                    Err(crate::error::from_anyhow(
                        tonic::Status::unavailable("lost response").into(),
                    ))
                } else {
                    Ok(CallToolResult::structured(json!({"item": item})))
                }
            },
        )
        .await
        .unwrap();
        let body = result.structured_content.unwrap();
        assert_eq!(body["successes"][0]["index"], 0);
        assert_eq!(body["failures"][0]["index"], 2);
        assert_eq!(body["not_attempted"], json!([4]));
        assert_eq!(body["not_selected"], json!([1, 3]));
    }

    #[test]
    fn only_item_local_errors_allow_continuing() {
        assert!(!should_stop(&ErrorData::invalid_params("invalid", None)));
        assert!(!should_stop(&ErrorData::resource_not_found(
            "missing", None
        )));
        for code in [tonic::Code::AlreadyExists, tonic::Code::Aborted] {
            assert!(!should_stop(&crate::error::from_anyhow(
                tonic::Status::new(code, "item failure").into()
            )));
        }
        for code in [
            tonic::Code::Unavailable,
            tonic::Code::ResourceExhausted,
            tonic::Code::DeadlineExceeded,
            tonic::Code::Internal,
            tonic::Code::PermissionDenied,
            tonic::Code::Unauthenticated,
            tonic::Code::Cancelled,
            tonic::Code::Unknown,
        ] {
            assert!(should_stop(&crate::error::from_anyhow(
                tonic::Status::new(code, "stop").into()
            )));
        }
    }

    #[tokio::test]
    async fn backend_failure_stops_the_tail_and_marks_uncertain_writes() {
        let result = create_many(vec![0, 1, 2], required(Some(vec![0, 1, 2])), |_| async {
            Err(crate::error::from_anyhow(
                tonic::Status::unavailable("lost response").into(),
            ))
        })
        .await
        .unwrap();
        let body = result.structured_content.unwrap();
        assert_eq!(body["not_attempted"], json!([1, 2]));
        assert_eq!(body["failures"][0]["outcome_unknown"], true);
        assert_eq!(
            body["failures"][0]["error"]["data"]["reason"],
            "BackendUnreachable"
        );
    }
}
