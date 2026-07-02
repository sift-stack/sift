use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::annotations::v1::{
    Annotation, CreateAnnotationResponse, ListAnnotationsResponse, UpdateAnnotationResponse,
    annotation_service_server::AnnotationServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::annotations::v1::MockAnnotationServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{AnnotationListParams, CreateAnnotationParams, UpdateAnnotationParams};
use crate::{server::SiftMcpServer, tool::common::test_support::structured_field};

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
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
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
        annotation_id: annotation_id.into(),
        name: None,
        description: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        assigned_to_user_id: None,
        state: None,
        tags: None,
        linked_channel_ids: None,
        metadata: None,
    }
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

    let annotation = structured_field(resp, "annotation");
    assert_eq!(annotation["name"], "renamed");
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

    let err = server
        .update_annotation(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}
