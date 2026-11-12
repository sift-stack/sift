use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use rmcp::{
    handler::server::wrapper::Parameters,
    model::{ErrorCode, Tool},
};
use sift_rs::annotations::v1::{
    Annotation, ArchiveAnnotationResponse, CreateAnnotationResponse, ListAnnotationsResponse,
    UnarchiveAnnotationResponse, UpdateAnnotationResponse,
    annotation_service_server::AnnotationServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::annotations::v1::MockAnnotationServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{
    AnnotationListParams, AnnotationStateParam, AnnotationTypeParam, CreateAnnotationItem,
    UpdateAnnotationParams,
};
use crate::{
    server::SiftMcpServer,
    tool::common::test_support::{structured, structured_field},
};

fn annotation_tool(name: &str) -> Tool {
    SiftMcpServer::annotations_router()
        .list_all()
        .into_iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("missing annotation tool `{name}`"))
}

fn contains_enum(schema: &serde_json::Value, node: &serde_json::Value, expected: &[&str]) -> bool {
    let expected_values = expected
        .iter()
        .map(|value| serde_json::Value::String((*value).to_string()))
        .collect::<Vec<_>>();
    if node
        .get("enum")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| values == &expected_values)
    {
        return true;
    }

    if let Some(reference) = node.get("$ref").and_then(serde_json::Value::as_str)
        && let Some(name) = reference.strip_prefix("#/$defs/")
        && contains_enum(schema, &schema["$defs"][name], expected)
    {
        return true;
    }

    ["anyOf", "oneOf", "allOf"].into_iter().any(|key| {
        node.get(key)
            .and_then(serde_json::Value::as_array)
            .is_some_and(|nodes| {
                nodes
                    .iter()
                    .any(|node| contains_enum(schema, node, expected))
            })
    })
}

fn assert_property_enum(tool: &Tool, property: &str, expected: &[&str]) {
    let schema = serde_json::Value::Object(tool.input_schema.as_ref().clone());
    // Create tools nest each resource's fields inside `items`.
    let item = schema["properties"]["items"]["items"]["$ref"]
        .as_str()
        .and_then(|reference| reference.strip_prefix("#/$defs/"));
    let property_schema = match item {
        Some(name) => &schema["$defs"][name]["properties"][property],
        None => &schema["properties"][property],
    };
    assert!(
        contains_enum(&schema, property_schema, expected),
        "`{}` must define `{property}` with enum values {expected:?}; schema: {schema}",
        tool.name
    );
}

#[test]
fn annotation_tool_schemas_publish_exact_enum_values() {
    let create = annotation_tool("create_annotation");
    assert_property_enum(
        &create,
        "annotation_type",
        &["ANNOTATION_TYPE_DATA_REVIEW", "ANNOTATION_TYPE_PHASE"],
    );
    assert_property_enum(
        &create,
        "state",
        &[
            "ANNOTATION_STATE_OPEN",
            "ANNOTATION_STATE_FLAGGED",
            "ANNOTATION_STATE_RESOLVED",
        ],
    );

    let update = annotation_tool("update_annotation");
    assert_property_enum(
        &update,
        "state",
        &[
            "ANNOTATION_STATE_OPEN",
            "ANNOTATION_STATE_FLAGGED",
            "ANNOTATION_STATE_RESOLVED",
        ],
    );
}

#[test]
fn list_annotations_description_publishes_filter_enum_values() {
    let tool = annotation_tool("list_annotations");
    let description = tool.description.expect("tool must have a description");

    for value in [
        "ANNOTATION_STATE_OPEN",
        "ANNOTATION_STATE_FLAGGED",
        "ANNOTATION_STATE_RESOLVED",
        "ANNOTATION_TYPE_DATA_REVIEW",
        "ANNOTATION_TYPE_PHASE",
    ] {
        assert!(
            description.contains(value),
            "list_annotations description must name `{value}`"
        );
    }
}

async fn server_with_mock(mock: MockAnnotationServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AnnotationServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://app.test.local"), true, true),
        handle,
    )
}

fn create_params() -> CreateAnnotationItem {
    CreateAnnotationItem {
        name: "review window".into(),
        description: None,
        start_time_unix_nanos: 1_000_000_000,
        end_time_unix_nanos: 2_000_000_000,
        annotation_type: AnnotationTypeParam::DataReview,
        state: None,
        assets: None,
        tags: None,
        linked_channel_ids: None,
        run_id: None,
        assign_to_user_id: None,
        metadata: None,
        organization_id: None,
    }
}

fn update_params(annotation_id: &str) -> UpdateAnnotationParams {
    UpdateAnnotationParams {
        annotation_ids: vec![annotation_id.into()],
        selected_indices: None,
        name: None,
        description: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        assigned_to_user_id: None,
        state: None,
        tags: None,
        linked_channel_ids: None,
        metadata: None,
        is_archived: None,
    }
}

#[tokio::test]
async fn update_annotation_approves_two_of_three_ids() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .times(2)
        .withf(|req| req.get_ref().annotation.as_ref().unwrap().annotation_id != "ann2")
        .returning(|req| {
            Ok(Response::new(UpdateAnnotationResponse {
                annotation: req.into_inner().annotation,
            }))
        });
    let (server, _h) = server_with_mock(mock).await;
    let server = server.with_require_selection(true);
    let mut params = update_params("ann1");
    params.annotation_ids.extend(["ann2".into(), "ann3".into()]);
    params.selected_indices = Some(vec![0, 2]);
    params.name = Some("new name".into());
    let body = structured(server.update_annotation(Parameters(params)).await.unwrap());
    assert_eq!(body["requested_count"], 3);
    assert_eq!(body["selected_count"], 2);
    assert_eq!(body["updated_count"], 2);
    assert_eq!(body["not_selected"], serde_json::json!([1]));
    assert_eq!(body["successes"][0]["index"], 0);
    assert_eq!(body["successes"][1]["index"], 2);
    assert_eq!(
        body["successes"][1]["result"]["annotation"]["url"],
        "https://app.test.local/annotation/ann3"
    );
}

#[tokio::test]
async fn update_annotation_reports_original_positions_for_selected_failures_and_unstarted_ids() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .times(50)
        .returning(|_| Err(Status::unavailable("offline")));
    let (server, _h) = server_with_mock(mock).await;
    let mut params = update_params("ann0");
    params
        .annotation_ids
        .extend((1..100).map(|i| format!("ann{i}")));
    params.selected_indices = Some((0..50).chain(std::iter::once(99)).collect());
    params.name = Some("new name".into());
    let body = structured(
        server
            .with_require_selection(true)
            .update_annotation(Parameters(params))
            .await
            .unwrap(),
    );
    assert_eq!(body["selected_count"], 51);
    assert_eq!(body["failures"][49]["index"], 49);
    assert_eq!(body["not_attempted"], serde_json::json!([99]));
    assert_eq!(body["not_selected"][0], 50);
}

#[tokio::test]
async fn update_annotation_chat_requires_a_mask_and_at_most_100_ids() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;
    let server = server.with_require_selection(true);
    let mut params = update_params("ann1");
    params.name = Some("new name".into());
    let error = server
        .update_annotation(Parameters(params))
        .await
        .unwrap_err();
    assert!(error.message.contains("approval host"));
    let mut params = update_params("ann0");
    params
        .annotation_ids
        .extend((1..101).map(|i| format!("ann{i}")));
    params.selected_indices = Some(vec![0]);
    params.is_archived = Some(true);
    let error = server
        .update_annotation(Parameters(params))
        .await
        .unwrap_err();
    assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
    assert!(
        error
            .message
            .contains("split the work into calls of at most 100 ids")
    );
}

#[tokio::test]
async fn update_annotation_accepts_100_ids_in_chat_and_1000_without_a_host() {
    for (count, chat) in [(100, true), (1000, false)] {
        let mut mock = MockAnnotationServiceImpl::new();
        mock.expect_update_annotation()
            .times(count)
            .returning(|req| {
                Ok(Response::new(UpdateAnnotationResponse {
                    annotation: req.into_inner().annotation,
                }))
            });
        let (server, _h) = server_with_mock(mock).await;
        let server = server.with_require_selection(chat);
        let mut params = update_params("ann0");
        params
            .annotation_ids
            .extend((1..count).map(|i| format!("ann{i}")));
        params.selected_indices = chat.then(|| (0..count).collect());
        params.name = Some("new name".into());
        let body = structured(server.update_annotation(Parameters(params)).await.unwrap());
        assert_eq!(body["updated_count"], count);
        assert_eq!(body["successes"][count - 1]["index"], count - 1);
    }
}

#[tokio::test]
async fn update_annotation_rejects_duplicates_and_invalid_masks_before_any_rpc() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;
    let mut duplicate = update_params("ann1");
    duplicate.annotation_ids.push("ann1".into());
    duplicate.is_archived = Some(true);
    assert_eq!(
        server
            .update_annotation(Parameters(duplicate))
            .await
            .unwrap_err()
            .code,
        ErrorCode::INVALID_PARAMS
    );
    for mask in [vec![0, 0], vec![1]] {
        let mut params = update_params("ann1");
        params.name = Some("new name".into());
        params.selected_indices = Some(mask);
        assert_eq!(
            server
                .update_annotation(Parameters(params))
                .await
                .unwrap_err()
                .code,
            ErrorCode::INVALID_PARAMS
        );
    }
    let mut params = update_params("ann1");
    params.name = Some("new name".into());
    params.selected_indices = Some(vec![]);
    let body = structured(server.update_annotation(Parameters(params)).await.unwrap());
    assert_eq!(body["updated_count"], 0);
    assert_eq!(body["not_selected"], serde_json::json!([0]));
}

#[test]
fn update_annotation_params_accept_bulk_ids() {
    let params = serde_json::from_value::<UpdateAnnotationParams>(serde_json::json!({
        "annotation_ids": ["ann1", "ann2"],
        "name": "renamed",
    }));

    assert!(params.is_ok());
}

#[tokio::test]
async fn list_annotations_returns_single_page() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_list_annotations()
        .withf(|req| req.get_ref().filter == "name == \"liftoff\"")
        .returning(|_| {
            Ok(Response::new(ListAnnotationsResponse {
                annotations: vec![Annotation {
                    annotation_id: "ann1".into(),
                    name: "liftoff".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_annotations(Parameters(AnnotationListParams {
            filter: "name == \"liftoff\"".into(),
            order_by: None,
            limit: None,
            organization_id: None,
            fields: None,
        }))
        .await
        .expect("list_annotations failed");

    let annotations = structured_field(resp, "annotations");
    assert_eq!(annotations.as_array().unwrap().len(), 1);
    assert_eq!(annotations[0]["annotationId"], "ann1");
    assert_eq!(
        annotations[0]["url"],
        "https://app.test.local/annotation/ann1"
    );
}

#[tokio::test]
async fn create_annotation_happy_path() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation().returning(|_| {
        Ok(Response::new(CreateAnnotationResponse {
            annotation: Some(Annotation {
                annotation_id: "ann-new".into(),
                name: "review window".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .create_annotation_item(Parameters(create_params()))
        .await
        .expect("create_annotation failed");

    let annotation = structured_field(resp, "annotation");
    assert_eq!(annotation["annotationId"], "ann-new");
}

#[tokio::test]
async fn create_annotation_rejects_empty_name() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let mut params = create_params();
    params.name = String::new();

    let err = server
        .create_annotation_item(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_annotation_rejects_inverted_time_range() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let mut params = create_params();
    params.start_time_unix_nanos = 5;
    params.end_time_unix_nanos = 1;

    let err = server
        .create_annotation_item(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[test]
fn create_annotation_params_reject_unknown_type() {
    let params = serde_json::from_value::<CreateAnnotationItem>(serde_json::json!({
        "name": "review window",
        "start_time_unix_nanos": 1_000_000_000,
        "end_time_unix_nanos": 2_000_000_000,
        "annotation_type": "bogus",
    }));

    assert!(params.is_err());
}

#[test]
fn annotation_params_accept_published_enum_values() {
    let create = serde_json::from_value::<CreateAnnotationItem>(serde_json::json!({
        "name": "review window",
        "start_time_unix_nanos": 1_000_000_000,
        "end_time_unix_nanos": 2_000_000_000,
        "annotation_type": "ANNOTATION_TYPE_DATA_REVIEW",
        "state": "ANNOTATION_STATE_FLAGGED",
    }))
    .expect("published create enum values must deserialize");
    assert!(matches!(
        create.annotation_type,
        AnnotationTypeParam::DataReview
    ));
    assert!(matches!(create.state, Some(AnnotationStateParam::Flagged)));

    let update = serde_json::from_value::<UpdateAnnotationParams>(serde_json::json!({
        "annotation_ids": ["ann1"],
        "state": "ANNOTATION_STATE_RESOLVED",
    }))
    .expect("published update enum values must deserialize");
    assert!(matches!(update.state, Some(AnnotationStateParam::Resolved)));
}

#[test]
fn annotation_params_accept_state_aliases() {
    let failed = serde_json::from_value::<UpdateAnnotationParams>(serde_json::json!({
        "annotation_ids": ["ann1"],
        "state": "failed",
    }))
    .expect("`failed` must alias the flagged state");
    assert!(matches!(failed.state, Some(AnnotationStateParam::Flagged)));

    let accepted = serde_json::from_value::<UpdateAnnotationParams>(serde_json::json!({
        "annotation_ids": ["ann1"],
        "state": "accepted",
    }))
    .expect("`accepted` must alias the resolved state");
    assert!(matches!(
        accepted.state,
        Some(AnnotationStateParam::Resolved)
    ));
}

#[tokio::test]
async fn create_annotation_rejects_state_on_phase() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let mut params = create_params();
    params.annotation_type = AnnotationTypeParam::Phase;
    params.state = Some(AnnotationStateParam::Open);

    let err = server
        .create_annotation_item(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_annotation_happy_path() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation().returning(|_| {
        Ok(Response::new(UpdateAnnotationResponse {
            annotation: Some(Annotation {
                annotation_id: "ann1".into(),
                name: "renamed".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.name = Some("renamed".into());

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("update_annotation failed");

    let successes = structured_field(resp, "successes");
    assert_eq!(successes[0]["result"]["annotation"]["name"], "renamed");
    assert_eq!(
        successes[0]["result"]["annotation"]["url"],
        "https://app.test.local/annotation/ann1"
    );
}

#[tokio::test]
async fn update_annotation_updates_each_annotation() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation().times(2).returning(|req| {
        let annotation = req.into_inner().annotation.unwrap();
        Ok(Response::new(UpdateAnnotationResponse {
            annotation: Some(annotation),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.name = Some("renamed".into());

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("update_annotation failed");

    let successes = structured_field(resp, "successes");
    assert_eq!(successes[0]["result"]["annotation"]["annotationId"], "ann1");
    assert_eq!(successes[1]["result"]["annotation"]["annotationId"], "ann2");
}

#[tokio::test]
async fn update_annotation_reports_partial_failures_and_continues() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation().times(3).returning(|req| {
        let annotation = req.into_inner().annotation.unwrap();
        if annotation.annotation_id == "ann2" {
            return Err(Status::not_found("no such annotation"));
        }
        Ok(Response::new(UpdateAnnotationResponse {
            annotation: Some(annotation),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.extend(["ann2".into(), "ann3".into()]);
    params.name = Some("renamed".into());

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("partial update result should be returned");

    assert_eq!(resp.is_error, Some(true));
    let content = serde_json::to_string(&resp.content).expect("content should serialize");
    assert_eq!(resp.content.len(), 2);
    assert!(content.contains("no such annotation"));
    let body = structured(resp);
    assert_eq!(
        body["successes"][0]["result"]["annotation"]["annotationId"],
        "ann1"
    );
    assert_eq!(
        body["successes"][1]["result"]["annotation"]["annotationId"],
        "ann3"
    );
    assert_eq!(body["successes"][1]["index"], 2);
    assert_eq!(body["failures"][0]["index"], 1);
    assert_eq!(
        body["failures"][0]["error"]["code"],
        ErrorCode::RESOURCE_NOT_FOUND.0
    );
    assert_eq!(body["not_attempted"], serde_json::json!([]));
}

#[tokio::test]
async fn update_annotation_reports_ids_not_attempted_after_backend_wide_failure() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .times(50)
        .returning(|_| Err(Status::resource_exhausted("slow down")));

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann0");
    params
        .annotation_ids
        .extend((1..60).map(|index| format!("ann{index}")));
    params.name = Some("renamed".into());

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("partial update result should be returned");

    assert_eq!(resp.is_error, Some(true));
    let content = serde_json::to_string(&resp.content).expect("content should serialize");
    assert_eq!(resp.content.len(), 2);
    assert!(content.contains("RateLimited"));
    let body = structured(resp);
    assert_eq!(body["failures"].as_array().unwrap().len(), 50);
    assert_eq!(body["not_attempted"].as_array().unwrap().len(), 10);
    assert_eq!(body["not_attempted"][0], 50);
    assert_eq!(body["not_attempted"][9], 59);
}

#[tokio::test]
async fn update_annotation_archives_each_annotation() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_archive_annotation().times(2).returning(|req| {
        Ok(Response::new(ArchiveAnnotationResponse {
            annotation: Some(Annotation {
                annotation_id: req.into_inner().annotation_id,
                is_archived: true,
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.is_archived = Some(true);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("update_annotation failed");

    let successes = structured_field(resp, "successes");
    assert_eq!(successes.as_array().unwrap().len(), 2);
    assert_eq!(successes[0]["result"]["annotation"]["isArchived"], true);
}

#[tokio::test]
async fn update_annotation_updates_fields_before_archiving() {
    let mut mock = MockAnnotationServiceImpl::new();
    let updated_count = Arc::new(AtomicUsize::new(0));

    let update_count = Arc::clone(&updated_count);
    mock.expect_update_annotation()
        .times(2)
        .returning(move |req| {
            update_count.fetch_add(1, Ordering::SeqCst);
            let annotation = req.into_inner().annotation.unwrap();
            Ok(Response::new(UpdateAnnotationResponse {
                annotation: Some(annotation),
            }))
        });

    let archive_count = Arc::clone(&updated_count);
    mock.expect_archive_annotation()
        .times(2)
        .withf(move |_| archive_count.load(Ordering::SeqCst) >= 1)
        .returning(|req| {
            Ok(Response::new(ArchiveAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: req.into_inner().annotation_id,
                    is_archived: true,
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.name = Some("renamed".into());
    params.is_archived = Some(true);

    server
        .update_annotation(Parameters(params))
        .await
        .expect("update_annotation failed");
}

#[tokio::test]
async fn update_annotation_skips_archive_after_partial_field_failure() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation().times(2).returning(|req| {
        let annotation = req.into_inner().annotation.unwrap();
        if annotation.annotation_id == "ann2" {
            return Err(Status::not_found("no such annotation"));
        }
        Ok(Response::new(UpdateAnnotationResponse {
            annotation: Some(annotation),
        }))
    });
    mock.expect_archive_annotation()
        .times(1)
        .withf(|req| req.get_ref().annotation_id == "ann1")
        .returning(|req| {
            Ok(Response::new(ArchiveAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: req.into_inner().annotation_id,
                    is_archived: true,
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.name = Some("renamed".into());
    params.is_archived = Some(true);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("partial update result should be returned");

    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert!(body.get("archive_skipped").is_none());
    assert_eq!(
        body["successes"][0]["result"]["annotation"]["annotationId"],
        "ann1"
    );
    assert_eq!(
        body["successes"][0]["result"]["annotation"]["isArchived"],
        true
    );
    assert_eq!(body["failures"][0]["index"], 1);
}

#[tokio::test]
async fn update_annotation_reports_one_archive_error_without_stopping_others() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_archive_annotation().times(2).returning(|req| {
        let annotation_id = req.into_inner().annotation_id;
        if annotation_id == "ann1" {
            return Err(Status::permission_denied("not the creator"));
        }
        Ok(Response::new(ArchiveAnnotationResponse {
            annotation: Some(Annotation {
                annotation_id,
                is_archived: true,
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.is_archived = Some(true);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("per-id archive failure details should be returned");

    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(body["updated_count"], 1);
    assert_eq!(body["failures"][0]["index"], 0);
    assert_eq!(body["successes"][0]["index"], 1);
    assert_eq!(
        body["failures"][0]["error"]["data"]["reason"],
        "PermissionDenied"
    );
    assert!(body.get("batch_archive_error").is_none());
}

#[tokio::test]
async fn update_annotation_reports_partial_unarchive_failures() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_unarchive_annotation()
        .times(2)
        .returning(|req| {
            let annotation_id = req.into_inner().annotation_id;
            if annotation_id == "ann2" {
                return Err(Status::not_found("no such annotation"));
            }
            Ok(Response::new(UnarchiveAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id,
                    is_archived: false,
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.is_archived = Some(false);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("partial unarchive result should be returned");

    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(
        body["successes"][0]["result"]["annotation"]["annotationId"],
        "ann1"
    );
    assert_eq!(body["failures"][0]["index"], 1);
    assert_eq!(
        body["failures"][0]["error"]["code"],
        ErrorCode::RESOURCE_NOT_FOUND.0
    );
    assert!(body.get("batch_archive_error").is_none());
}

#[tokio::test]
async fn update_annotation_unarchives_each_annotation() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_unarchive_annotation()
        .times(2)
        .returning(|req| {
            Ok(Response::new(UnarchiveAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: req.into_inner().annotation_id,
                    is_archived: false,
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.is_archived = Some(false);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("update_annotation failed");

    let successes = structured_field(resp, "successes");
    assert_eq!(successes.as_array().unwrap().len(), 2);
    assert_eq!(successes[0]["result"]["annotation"]["annotationId"], "ann1");
    assert_eq!(successes[1]["result"]["annotation"]["annotationId"], "ann2");
}

#[tokio::test]
async fn update_annotation_reports_unarchive_ids_not_attempted_after_backend_wide_failure() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_unarchive_annotation()
        .times(50)
        .returning(|_| Err(Status::unavailable("backend unavailable")));

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params
        .annotation_ids
        .extend((2..=51).map(|index| format!("ann{index}")));
    params.is_archived = Some(false);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("partial unarchive result should be returned");

    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(body["failures"].as_array().unwrap().len(), 50);
    assert_eq!(body["not_attempted"], serde_json::json!([50]));
    assert!(body.get("batch_archive_error").is_none());
}

#[tokio::test]
async fn update_annotation_updates_fields_before_unarchiving() {
    let mut mock = MockAnnotationServiceImpl::new();
    let updated_count = Arc::new(AtomicUsize::new(0));

    let update_count = Arc::clone(&updated_count);
    mock.expect_update_annotation()
        .times(2)
        .returning(move |req| {
            let req = req.into_inner();
            assert_eq!(req.update_mask.unwrap().paths, ["name"]);
            update_count.fetch_add(1, Ordering::SeqCst);
            Ok(Response::new(UpdateAnnotationResponse {
                annotation: req.annotation,
            }))
        });
    mock.expect_unarchive_annotation()
        .times(2)
        .withf(move |_| updated_count.load(Ordering::SeqCst) >= 1)
        .returning(|req| {
            Ok(Response::new(UnarchiveAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: req.into_inner().annotation_id,
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.name = Some("renamed".into());
    params.is_archived = Some(false);

    server
        .update_annotation(Parameters(params))
        .await
        .expect("update_annotation failed");
}

#[tokio::test]
async fn update_annotation_rejects_empty_ids() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let mut params = update_params("ann1");
    params.annotation_ids.clear();

    let err = server
        .update_annotation(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_annotation_rejects_more_than_1000_ids_without_requests() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation().times(0);
    mock.expect_archive_annotation().times(0);

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann0");
    params
        .annotation_ids
        .extend((1..=1_000).map(|index| format!("ann{index}")));
    params.name = Some("renamed".into());

    let err = server
        .update_annotation(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_annotation_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let err = server
        .update_annotation(Parameters(update_params("")))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_annotation_rejects_no_fields() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let err = server
        .update_annotation(Parameters(update_params("ann1")))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_annotation_propagates_grpc_error() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .returning(|_| Err(Status::not_found("no such annotation")));

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.name = Some("x".into());

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("failure details should be returned");

    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(body["failures"][0]["index"], 0);
    assert_eq!(
        body["failures"][0]["error"]["code"],
        ErrorCode::RESOURCE_NOT_FOUND.0
    );
}
