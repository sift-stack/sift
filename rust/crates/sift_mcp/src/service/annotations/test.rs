use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use sift_rs::annotations::v1::{
    Annotation, AnnotationState, AnnotationType, BatchArchiveAnnotationsResponse,
    CreateAnnotationResponse, ListAnnotationsResponse, UpdateAnnotationResponse,
    annotation_service_server::AnnotationServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::annotations::v1::MockAnnotationServiceImpl};
use tokio::{sync::Semaphore, task::JoinHandle};
use tonic::{Response, Status, transport::Server};

use super::{AnnotationService, fan_out_bounded};
use crate::service::common::DEFAULT_LIMIT;

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
async fn bounded_fan_out_limits_concurrency_and_preserves_order() {
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let started = Arc::new(AtomicUsize::new(0));
    let gate = Arc::new(Semaphore::new(0));

    let active_for_task = Arc::clone(&active);
    let max_for_task = Arc::clone(&max_active);
    let started_for_task = Arc::clone(&started);
    let gate_for_task = Arc::clone(&gate);
    let task = tokio::spawn(fan_out_bounded((0..120).collect(), 50, move |item| {
        let active = Arc::clone(&active_for_task);
        let max_active = Arc::clone(&max_for_task);
        let started = Arc::clone(&started_for_task);
        let gate = Arc::clone(&gate_for_task);
        async move {
            let current = active.fetch_add(1, Ordering::SeqCst) + 1;
            max_active.fetch_max(current, Ordering::SeqCst);
            started.fetch_add(1, Ordering::SeqCst);
            gate.acquire().await.unwrap().forget();
            active.fetch_sub(1, Ordering::SeqCst);
            item
        }
    }));

    tokio::time::timeout(Duration::from_secs(1), async {
        while started.load(Ordering::SeqCst) < 50 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("50 updates should start concurrently");
    assert_eq!(started.load(Ordering::SeqCst), 50);
    gate.add_permits(120);
    let results = task.await.unwrap();

    assert_eq!(results, (0..120).collect::<Vec<_>>());
    assert_eq!(max_active.load(Ordering::SeqCst), 50);
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
        .expect("list_annotations failed")
        .items;

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
        .expect("list_annotations failed")
        .items;

    assert_eq!(annotations.len(), 1);
}

#[tokio::test]
async fn list_annotations_paginates_until_token_empty() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_list_annotations().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
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
        .expect("list_annotations failed")
        .items;

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
        .expect("list_annotations failed")
        .items;

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
async fn batch_archive_annotations_forwards_ids() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_batch_archive_annotations()
        .withf(|req| req.get_ref().annotation_ids == ["ann1", "ann2"])
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

    let (service, _h) = service_with_mock(mock).await;

    let annotations = service
        .batch_archive_annotations(vec!["ann1".into(), "ann2".into()])
        .await
        .expect("batch archive failed");

    assert_eq!(annotations.len(), 2);
    assert!(annotations.iter().all(|annotation| annotation.is_archived));
}

#[tokio::test]
async fn batch_archive_annotations_propagates_grpc_error() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_batch_archive_annotations()
        .returning(|_| Err(Status::not_found("no such annotation")));

    let (service, _h) = service_with_mock(mock).await;

    let error = service
        .batch_archive_annotations(vec!["ann1".into()])
        .await
        .expect_err("expected batch archive error");

    assert!(
        error
            .to_string()
            .contains("failed to batch archive annotations")
    );
}

#[tokio::test]
async fn update_annotations_collects_failures_and_continues() {
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

    let (service, _h) = service_with_mock(mock).await;

    let outcome = service
        .update_annotations(
            vec!["ann1".into(), "ann2".into(), "ann3".into()],
            Some("renamed".into()),
            None,
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
        .expect("bulk update failed");

    let updated_ids = outcome
        .annotations
        .iter()
        .map(|annotation| annotation.annotation_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(updated_ids, ["ann1", "ann3"]);
    assert_eq!(outcome.failures.len(), 1);
    assert_eq!(outcome.failures[0].annotation_id, "ann2");
}

#[tokio::test]
async fn update_annotations_stops_after_backend_wide_failure() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_update_annotation()
        .times(50)
        .returning(|_| Err(Status::resource_exhausted("slow down")));

    let (service, _h) = service_with_mock(mock).await;
    let annotation_ids = (0..60)
        .map(|index| format!("ann{index}"))
        .collect::<Vec<_>>();

    let outcome = service
        .update_annotations(
            annotation_ids.clone(),
            Some("renamed".into()),
            None,
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
        .expect("bulk update failed");

    assert_eq!(outcome.failures.len(), 50);
    assert_eq!(outcome.not_attempted, annotation_ids[50..]);
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
            None,
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to update annotation"));
}
