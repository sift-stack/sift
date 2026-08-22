use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::annotations::v1::{
    Annotation, BatchArchiveAnnotationsResponse, CreateAnnotationResponse, ListAnnotationsResponse,
    UpdateAnnotationResponse, annotation_service_server::AnnotationServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::annotations::v1::MockAnnotationServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{AnnotationListParams, CreateAnnotationParams, UpdateAnnotationParams};
use crate::{
    server::SiftMcpServer,
    tool::common::test_support::{structured, structured_field},
};

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

fn create_params() -> CreateAnnotationParams {
    CreateAnnotationParams {
        name: "review window".into(),
        description: None,
        start_time_unix_nanos: 1_000_000_000,
        end_time_unix_nanos: 2_000_000_000,
        annotation_type: "data_review".into(),
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
        .create_annotation(Parameters(create_params()))
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
        .create_annotation(Parameters(params))
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
        .create_annotation(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_annotation_rejects_unknown_type() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let mut params = create_params();
    params.annotation_type = "bogus".into();

    let err = server
        .create_annotation(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_annotation_rejects_state_on_phase() {
    let (server, _h) = server_with_mock(MockAnnotationServiceImpl::new()).await;

    let mut params = create_params();
    params.annotation_type = "phase".into();
    params.state = Some("open".into());

    let err = server
        .create_annotation(Parameters(params))
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

    let annotations = structured_field(resp, "annotations");
    assert_eq!(annotations[0]["name"], "renamed");
    assert_eq!(
        annotations[0]["url"],
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

    let annotations = structured_field(resp, "annotations");
    assert_eq!(annotations[0]["annotationId"], "ann1");
    assert_eq!(annotations[1]["annotationId"], "ann2");
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
    assert!(content.contains("ann2"));
    assert!(content.contains("no such annotation"));
    let body = structured(resp);
    assert_eq!(body["annotations"][0]["annotationId"], "ann1");
    assert_eq!(body["annotations"][1]["annotationId"], "ann3");
    assert_eq!(body["failures"][0]["annotation_id"], "ann2");
    assert_eq!(body["failures"][0]["code"], ErrorCode::RESOURCE_NOT_FOUND.0);
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
    assert!(content.contains("ann0"));
    assert!(content.contains("ann59"));
    let body = structured(resp);
    assert_eq!(body["failures"].as_array().unwrap().len(), 50);
    assert_eq!(body["not_attempted"].as_array().unwrap().len(), 10);
    assert_eq!(body["not_attempted"][0], "ann50");
    assert_eq!(body["not_attempted"][9], "ann59");
}

#[tokio::test]
async fn update_annotation_batch_archives_annotations() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_batch_archive_annotations()
        .withf(|req| req.get_ref().annotation_ids == ["ann1", "ann2"])
        .returning(|_| {
            Ok(Response::new(BatchArchiveAnnotationsResponse {
                annotations: vec![
                    Annotation {
                        annotation_id: "ann1".into(),
                        is_archived: true,
                        ..Default::default()
                    },
                    Annotation {
                        annotation_id: "ann2".into(),
                        is_archived: true,
                        ..Default::default()
                    },
                ],
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

    let annotations = structured_field(resp, "annotations");
    assert_eq!(annotations.as_array().unwrap().len(), 2);
    assert_eq!(annotations[0]["isArchived"], true);
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
    mock.expect_batch_archive_annotations()
        .withf(move |_| archive_count.load(Ordering::SeqCst) == 2)
        .returning(|req| {
            let annotations = req
                .into_inner()
                .annotation_ids
                .into_iter()
                .map(|annotation_id| Annotation {
                    annotation_id,
                    is_archived: true,
                    ..Default::default()
                })
                .collect();
            Ok(Response::new(BatchArchiveAnnotationsResponse {
                annotations,
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
    mock.expect_batch_archive_annotations().times(0);

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
    assert_eq!(body["archive_skipped"], true);
    assert_eq!(body["annotations"][0]["annotationId"], "ann1");
    assert_eq!(body["failures"][0]["annotation_id"], "ann2");
}

#[tokio::test]
async fn update_annotation_reports_opaque_batch_archive_error() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_batch_archive_annotations()
        .returning(|_| Err(Status::not_found("one target was not found")));

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params("ann1");
    params.annotation_ids.push("ann2".into());
    params.is_archived = Some(true);

    let resp = server
        .update_annotation(Parameters(params))
        .await
        .expect("batch archive failure details should be returned");

    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(
        body["batch_archive_error"]["annotation_ids"],
        serde_json::json!(["ann1", "ann2"])
    );
    assert_eq!(
        body["batch_archive_error"]["code"],
        ErrorCode::RESOURCE_NOT_FOUND.0
    );
}

#[tokio::test]
async fn update_annotation_unarchives_each_annotation() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .times(2)
        .withf(|req| {
            let req = req.get_ref();
            !req.annotation.as_ref().unwrap().is_archived
                && req.update_mask.as_ref().unwrap().paths == ["is_archived"]
        })
        .returning(|req| {
            let annotation = req.into_inner().annotation.unwrap();
            Ok(Response::new(UpdateAnnotationResponse {
                annotation: Some(annotation),
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

    let annotations = structured_field(resp, "annotations");
    assert_eq!(annotations.as_array().unwrap().len(), 2);
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
    assert_eq!(body["failures"][0]["annotation_id"], "ann1");
    assert_eq!(body["failures"][0]["code"], ErrorCode::RESOURCE_NOT_FOUND.0);
}
