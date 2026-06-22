use rmcp::model::ErrorCode;
use sift_rs::runs::v2::{ListRunsResponse, Run, run_service_server::RunServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::runs::v2::MockRunServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    tool::common::test_support::{list_params, structured_field},
};

async fn server_with_mock(mock: MockRunServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RunServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
        handle,
    )
}

#[tokio::test]
async fn list_runs_returns_single_page() {
    let mut run_mock = MockRunServiceImpl::new();
    run_mock
        .expect_list_runs()
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

    let (server, _h) = server_with_mock(run_mock).await;

    let resp = server
        .list_runs(list_params("name == \"launch\"", None))
        .await
        .expect("list_runs failed");

    let runs = structured_field(resp, "runs");
    assert_eq!(runs.as_array().unwrap().len(), 1);
    assert_eq!(runs[0]["runId"], "r1");
}

#[tokio::test]
async fn list_runs_paginates_until_token_empty() {
    let mut run_mock = MockRunServiceImpl::new();
    run_mock.expect_list_runs().returning(|req| {
        let token = req.into_inner().page_token;
        let (runs, next) = match token.as_str() {
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

    let (server, _h) = server_with_mock(run_mock).await;

    let resp = server
        .list_runs(list_params("", None))
        .await
        .expect("list_runs failed");

    let runs = structured_field(resp, "runs");
    let ids: Vec<&str> = runs
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["runId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["r1", "r2"]);
}

#[tokio::test]
async fn list_runs_truncates_to_limit_across_pages() {
    let mut run_mock = MockRunServiceImpl::new();
    run_mock.expect_list_runs().returning(|req| {
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

    let (server, _h) = server_with_mock(run_mock).await;

    let resp = server
        .list_runs(list_params("", Some(3)))
        .await
        .expect("list_runs failed");

    let runs = structured_field(resp, "runs");
    let ids = runs
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["runId"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["r1", "r2", "r3"]);
}

#[tokio::test]
async fn list_runs_propagates_grpc_error() {
    let mut run_mock = MockRunServiceImpl::new();
    run_mock
        .expect_list_runs()
        .returning(|_| Err(Status::not_found("no such run")));

    let (server, _h) = server_with_mock(run_mock).await;

    let err = server
        .list_runs(list_params("", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("no such run"));
}
