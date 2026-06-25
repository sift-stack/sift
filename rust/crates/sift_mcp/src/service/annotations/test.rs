use sift_rs::annotations::v1::{
    Annotation, AnnotationState, AnnotationType, CreateAnnotationResponse, ListAnnotationsResponse,
    UpdateAnnotationResponse, annotation_service_server::AnnotationServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::annotations::v1::MockAnnotationServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::AnnotationService;
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(mock: MockAnnotationServiceImpl) -> (AnnotationService, JoinHandle<()>) {
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
        AnnotationService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
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

    let (service, _h) = service_with_mock(mock).await;

    let annotations = service
        .list_annotations("name == \"liftoff\"".to_string(), None, None, None)
        .await
        .expect("list_annotations failed");

    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].annotation_id, "ann1");
}

#[tokio::test]
async fn list_annotations_forwards_organization_id() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_list_annotations()
        .withf(|req| req.get_ref().organization_id == "org-123")
        .returning(|_| {
            Ok(Response::new(ListAnnotationsResponse {
                annotations: vec![Annotation {
                    annotation_id: "ann1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let annotations = service
        .list_annotations(String::new(), None, None, Some("org-123".to_string()))
        .await
        .expect("list_annotations failed");

    assert_eq!(annotations.len(), 1);
}

#[tokio::test]
async fn list_annotations_paginates_until_token_empty() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_list_annotations().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (annotations, next) = match req.page_token.as_str() {
            "" => (
                vec![Annotation {
                    annotation_id: "ann1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Annotation {
                    annotation_id: "ann2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListAnnotationsResponse {
            annotations,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let annotations = service
        .list_annotations(String::new(), None, None, None)
        .await
        .expect("list_annotations failed");

    let ids: Vec<&str> = annotations
        .iter()
        .map(|a| a.annotation_id.as_str())
        .collect();
    assert_eq!(ids, vec!["ann1", "ann2"]);
}

#[tokio::test]
async fn list_annotations_truncates_to_limit_across_pages() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_list_annotations().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (annotations, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Annotation {
                        annotation_id: "ann1".into(),
                        ..Default::default()
                    },
                    Annotation {
                        annotation_id: "ann2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Annotation {
                        annotation_id: "ann3".into(),
                        ..Default::default()
                    },
                    Annotation {
                        annotation_id: "ann4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListAnnotationsResponse {
            annotations,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let annotations = service
        .list_annotations(String::new(), None, Some(3), None)
        .await
        .expect("list_annotations failed");

    let ids: Vec<&str> = annotations
        .iter()
        .map(|a| a.annotation_id.as_str())
        .collect();
    assert_eq!(ids, vec!["ann1", "ann2", "ann3"]);
}

#[tokio::test]
async fn list_annotations_propagates_grpc_error() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_list_annotations()
        .returning(|_| Err(Status::not_found("no such annotation")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_annotations(String::new(), None, None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query annotations"));
}

#[tokio::test]
async fn create_annotation_maps_fields() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation()
        .withf(|req| {
            let req = req.get_ref();
            req.name == "review window"
                && req.annotation_type == AnnotationType::DataReview as i32
                && req.state == Some(AnnotationState::Open as i32)
                && req.start_time.as_ref().map(|t| t.seconds) == Some(1)
                && req.end_time.as_ref().map(|t| t.seconds) == Some(2)
                && req.linked_channels.len() == 1
                && req.assets == vec!["rover".to_string()]
        })
        .returning(|_| {
            Ok(Response::new(CreateAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: "ann-new".into(),
                    name: "review window".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let annotation = service
        .create_annotation(
            "review window".to_string(),
            None,
            1_000_000_000,
            2_000_000_000,
            AnnotationType::DataReview,
            Some(AnnotationState::Open),
            Some(vec!["rover".to_string()]),
            None,
            Some(vec!["chan-1".to_string()]),
            None,
            None,
            None,
            None,
        )
        .await
        .expect("create_annotation failed");

    assert_eq!(annotation.annotation_id, "ann-new");
}

#[tokio::test]
async fn create_annotation_propagates_grpc_error() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation()
        .returning(|_| Err(Status::invalid_argument("bad input")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_annotation(
            "x".to_string(),
            None,
            1,
            2,
            AnnotationType::Phase,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to create annotation"));
}

#[tokio::test]
async fn update_annotation_builds_mask_from_provided_fields() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .withf(|req| {
            let req = req.get_ref();
            let paths = &req.update_mask.as_ref().unwrap().paths;
            let ann = req.annotation.as_ref().unwrap();
            ann.annotation_id == "ann1"
                && ann.name == "renamed"
                && paths == &vec!["name".to_string(), "tags".to_string()]
        })
        .returning(|_| {
            Ok(Response::new(UpdateAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: "ann1".into(),
                    name: "renamed".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let annotation = service
        .update_annotation(
            "ann1".to_string(),
            Some("renamed".to_string()),
            None,
            None,
            None,
            None,
            None,
            Some(vec!["important".to_string()]),
            None,
            None,
        )
        .await
        .expect("update_annotation failed");

    assert_eq!(annotation.name, "renamed");
}

#[tokio::test]
async fn update_annotation_propagates_grpc_error() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .returning(|_| Err(Status::not_found("no such annotation")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_annotation(
            "ann1".to_string(),
            Some("x".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to update annotation"));
}
