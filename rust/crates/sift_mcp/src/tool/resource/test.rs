use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use serde_json::Value;
use sift_rs::{
    assets::v1::{
        Asset, ListAssetsResponse, asset_service_server::AssetServiceServer,
    },
    runs::v2::{
        ListRunsResponse, Run, run_service_server::RunServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{assets::v1::MockAssetServiceImpl, runs::v2::MockRunServiceImpl},
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{GetParams, PAGE_SIZE};
use crate::server::SiftMcpServer;

async fn server_with_mocks(
    asset_mock: MockAssetServiceImpl,
    run_mock: MockRunServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(asset_mock))
            .add_service(RunServiceServer::new(run_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (SiftMcpServer::new(channel), handle)
}

fn params(filter: &str, limit: Option<u32>) -> Parameters<GetParams> {
    Parameters(GetParams {
        filter: filter.into(),
        limit,
    })
}

fn structured(result: rmcp::model::CallToolResult) -> Value {
    result
        .structured_content
        .expect("expected structured content")
}

#[tokio::test]
async fn get_asset_returns_single_page() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock
        .expect_list_assets()
        .withf(|req| req.get_ref().filter == "name == \"engine\"")
        .returning(|_| {
            Ok(Response::new(ListAssetsResponse {
                assets: vec![
                    Asset {
                        asset_id: "a1".into(),
                        name: "engine".into(),
                        ..Default::default()
                    },
                    Asset {
                        asset_id: "a2".into(),
                        name: "engine".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mocks(asset_mock, MockRunServiceImpl::new()).await;

    let resp = server
        .get_asset(params("name == \"engine\"", None))
        .await
        .expect("get_asset failed");

    let assets = structured(resp);
    assert_eq!(assets.as_array().unwrap().len(), 2);
    assert_eq!(assets[0]["assetId"], "a1");
    assert_eq!(assets[1]["assetId"], "a2");
}

#[tokio::test]
async fn get_asset_paginates_until_token_empty() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock.expect_list_assets().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (assets, next) = match req.page_token.as_str() {
            "" => (
                vec![Asset {
                    asset_id: "a1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Asset {
                    asset_id: "a2".into(),
                    ..Default::default()
                }],
                "page-3".to_string(),
            ),
            "page-3" => (
                vec![Asset {
                    asset_id: "a3".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListAssetsResponse {
            assets,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_mocks(asset_mock, MockRunServiceImpl::new()).await;

    let resp = server
        .get_asset(params("", None))
        .await
        .expect("get_asset failed");

    let assets = structured(resp);
    let ids: Vec<&str> = assets
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["assetId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["a1", "a2", "a3"]);
}

#[tokio::test]
async fn get_asset_respects_limit() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock
        .expect_list_assets()
        .times(1)
        .returning(|req| {
            let req = req.into_inner();
            assert_eq!(req.page_size, 2);
            Ok(Response::new(ListAssetsResponse {
                assets: vec![
                    Asset {
                        asset_id: "a1".into(),
                        ..Default::default()
                    },
                    Asset {
                        asset_id: "a2".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: "page-2".into(),
            }))
        });

    let (server, _h) = server_with_mocks(asset_mock, MockRunServiceImpl::new()).await;

    let resp = server
        .get_asset(params("", Some(2)))
        .await
        .expect("get_asset failed");

    let assets = structured(resp);
    assert_eq!(assets.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn get_asset_breaks_on_empty_page() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock
        .expect_list_assets()
        .times(1)
        .returning(|_| {
            Ok(Response::new(ListAssetsResponse {
                assets: vec![],
                next_page_token: "ignored".into(),
            }))
        });

    let (server, _h) = server_with_mocks(asset_mock, MockRunServiceImpl::new()).await;

    let resp = server
        .get_asset(params("", None))
        .await
        .expect("get_asset failed");

    assert!(structured(resp).as_array().unwrap().is_empty());
}

#[tokio::test]
async fn get_asset_propagates_grpc_error() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock
        .expect_list_assets()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mocks(asset_mock, MockRunServiceImpl::new()).await;

    let err = server
        .get_asset(params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

#[tokio::test]
async fn get_run_returns_single_page() {
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

    let (server, _h) = server_with_mocks(MockAssetServiceImpl::new(), run_mock).await;

    let resp = server
        .get_run(params("name == \"launch\"", None))
        .await
        .expect("get_run failed");

    let runs = structured(resp);
    assert_eq!(runs.as_array().unwrap().len(), 1);
    assert_eq!(runs[0]["runId"], "r1");
}

#[tokio::test]
async fn get_run_paginates_until_token_empty() {
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

    let (server, _h) = server_with_mocks(MockAssetServiceImpl::new(), run_mock).await;

    let resp = server
        .get_run(params("", None))
        .await
        .expect("get_run failed");

    let runs = structured(resp);
    let ids: Vec<&str> = runs
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["runId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["r1", "r2"]);
}

#[tokio::test]
async fn get_run_truncates_to_limit_across_pages() {
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

    let (server, _h) = server_with_mocks(MockAssetServiceImpl::new(), run_mock).await;

    let resp = server
        .get_run(params("", Some(3)))
        .await
        .expect("get_run failed");

    let runs = structured(resp);
    let ids = runs
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["runId"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["r1", "r2", "r3"]);
}

#[tokio::test]
async fn get_run_propagates_grpc_error() {
    let mut run_mock = MockRunServiceImpl::new();
    run_mock
        .expect_list_runs()
        .returning(|_| Err(Status::not_found("no such run")));

    let (server, _h) = server_with_mocks(MockAssetServiceImpl::new(), run_mock).await;

    let err = server
        .get_run(params("", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("no such run"));
}
