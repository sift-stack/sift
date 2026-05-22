use sift_rs::runs::v2::{ListRunsResponse, Run, run_service_server::RunServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::runs::v2::MockRunServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::RunService;
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(mock: MockRunServiceImpl) -> (RunService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RunServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (RunService::new(channel), handle)
}

#[tokio::test]
async fn list_runs_returns_single_page() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_list_runs()
        .withf(|req| req.get_ref().filter == "name == \"launch\"")
        .returning(|_| {
            Ok(Response::new(ListRunsResponse {
                runs: vec![Run {
                    run_id: "r1".into(),
                    name: "launch".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let runs = service
        .list_runs("name == \"launch\"".to_string(), None, None)
        .await
        .expect("list_runs failed");

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].run_id, "r1");
}

#[tokio::test]
async fn list_runs_paginates_until_token_empty() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_list_runs().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (runs, next) = match req.page_token.as_str() {
            "" => (
                vec![Run {
                    run_id: "r1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Run {
                    run_id: "r2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListRunsResponse {
            runs,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let runs = service
        .list_runs(String::new(), None, None)
        .await
        .expect("list_runs failed");

    let ids: Vec<&str> = runs.iter().map(|r| r.run_id.as_str()).collect();
    assert_eq!(ids, vec!["r1", "r2"]);
}

#[tokio::test]
async fn list_runs_respects_limit() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_list_runs().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListRunsResponse {
            runs: vec![
                Run {
                    run_id: "r1".into(),
                    ..Default::default()
                },
                Run {
                    run_id: "r2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let runs = service
        .list_runs(String::new(), None, Some(2))
        .await
        .expect("list_runs failed");

    assert_eq!(runs.len(), 2);
}

#[tokio::test]
async fn list_runs_truncates_to_limit_across_pages() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_list_runs().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (runs, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Run {
                        run_id: "r1".into(),
                        ..Default::default()
                    },
                    Run {
                        run_id: "r2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Run {
                        run_id: "r3".into(),
                        ..Default::default()
                    },
                    Run {
                        run_id: "r4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListRunsResponse {
            runs,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let runs = service
        .list_runs(String::new(), None, Some(3))
        .await
        .expect("list_runs failed");

    let ids: Vec<&str> = runs.iter().map(|r| r.run_id.as_str()).collect();
    assert_eq!(ids, vec!["r1", "r2", "r3"]);
}

#[tokio::test]
async fn list_runs_breaks_on_empty_page() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_list_runs().times(1).returning(|_| {
        Ok(Response::new(ListRunsResponse {
            runs: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let runs = service
        .list_runs(String::new(), None, None)
        .await
        .expect("list_runs failed");

    assert!(runs.is_empty());
}

#[tokio::test]
async fn list_runs_propagates_grpc_error() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_list_runs()
        .returning(|_| Err(Status::not_found("no such run")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_runs(String::new(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query runs"));
}
