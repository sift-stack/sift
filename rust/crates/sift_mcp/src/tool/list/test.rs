use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use serde_json::Value;
use sift_rs::{
    assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer},
    channels::v3::{Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer},
    reports::v1::{ListReportsResponse, Report, report_service_server::ReportServiceServer},
    runs::v2::{ListRunsResponse, Run, run_service_server::RunServiceServer},
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{
        assets::v1::MockAssetServiceImpl, channels::v3::MockChannelServiceImpl,
        reports::v1::MockReportServiceImpl, runs::v2::MockRunServiceImpl,
    },
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{ListParams, ReportListParams};
use crate::{server::SiftMcpServer, service::common::PAGE_SIZE};

async fn server_with_mocks(
    asset_mock: MockAssetServiceImpl,
    run_mock: MockRunServiceImpl,
    channel_mock: MockChannelServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(asset_mock))
            .add_service(RunServiceServer::new(run_mock))
            .add_service(ChannelServiceServer::new(channel_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (SiftMcpServer::new(channel), handle)
}

fn params(filter: &str, limit: Option<u32>) -> Parameters<ListParams> {
    Parameters(ListParams {
        filter: filter.into(),
        order_by: None,
        limit,
    })
}

fn structured(result: rmcp::model::CallToolResult) -> Value {
    result
        .structured_content
        .expect("expected structured content")
}

fn structured_field(result: rmcp::model::CallToolResult, key: &str) -> Value {
    let mut value = structured(result);
    value
        .get_mut(key)
        .unwrap_or_else(|| panic!("missing key `{key}` in structured content"))
        .take()
}

#[tokio::test]
async fn list_assets_returns_single_page() {
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

    let (server, _h) = server_with_mocks(
        asset_mock,
        MockRunServiceImpl::new(),
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_assets(params("name == \"engine\"", None))
        .await
        .expect("list_assets failed");

    let assets = structured_field(resp, "assets");
    assert_eq!(assets.as_array().unwrap().len(), 2);
    assert_eq!(assets[0]["assetId"], "a1");
    assert_eq!(assets[1]["assetId"], "a2");
}

#[tokio::test]
async fn list_assets_paginates_until_token_empty() {
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

    let (server, _h) = server_with_mocks(
        asset_mock,
        MockRunServiceImpl::new(),
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_assets(params("", None))
        .await
        .expect("list_assets failed");

    let assets = structured_field(resp, "assets");
    let ids: Vec<&str> = assets
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["assetId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["a1", "a2", "a3"]);
}

#[tokio::test]
async fn list_assets_respects_limit() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock.expect_list_assets().times(1).returning(|req| {
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

    let (server, _h) = server_with_mocks(
        asset_mock,
        MockRunServiceImpl::new(),
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_assets(params("", Some(2)))
        .await
        .expect("list_assets failed");

    let assets = structured_field(resp, "assets");
    assert_eq!(assets.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn list_assets_breaks_on_empty_page() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock.expect_list_assets().times(1).returning(|_| {
        Ok(Response::new(ListAssetsResponse {
            assets: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (server, _h) = server_with_mocks(
        asset_mock,
        MockRunServiceImpl::new(),
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_assets(params("", None))
        .await
        .expect("list_assets failed");

    assert!(
        structured_field(resp, "assets")
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn list_assets_propagates_grpc_error() {
    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock
        .expect_list_assets()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mocks(
        asset_mock,
        MockRunServiceImpl::new(),
        MockChannelServiceImpl::new(),
    )
    .await;

    let err = server
        .list_assets(params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
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

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        run_mock,
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_runs(params("name == \"launch\"", None))
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

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        run_mock,
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_runs(params("", None))
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

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        run_mock,
        MockChannelServiceImpl::new(),
    )
    .await;

    let resp = server
        .list_runs(params("", Some(3)))
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

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        run_mock,
        MockChannelServiceImpl::new(),
    )
    .await;

    let err = server
        .list_runs(params("", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("no such run"));
}

#[tokio::test]
async fn list_channels_returns_single_page() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock
        .expect_list_channels()
        .withf(|req| req.get_ref().filter == "name == \"throttle\"")
        .returning(|_| {
            Ok(Response::new(ListChannelsResponse {
                channels: vec![
                    Channel {
                        channel_id: "c1".into(),
                        name: "throttle".into(),
                        ..Default::default()
                    },
                    Channel {
                        channel_id: "c2".into(),
                        name: "throttle".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        MockRunServiceImpl::new(),
        channel_mock,
    )
    .await;

    let resp = server
        .list_channels(params("name == \"throttle\"", None))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    assert_eq!(channels.as_array().unwrap().len(), 2);
    assert_eq!(channels[0]["channelId"], "c1");
    assert_eq!(channels[1]["channelId"], "c2");
}

#[tokio::test]
async fn list_channels_paginates_until_token_empty() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (channels, next) = match req.page_token.as_str() {
            "" => (
                vec![Channel {
                    channel_id: "c1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Channel {
                    channel_id: "c2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListChannelsResponse {
            channels,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        MockRunServiceImpl::new(),
        channel_mock,
    )
    .await;

    let resp = server
        .list_channels(params("", None))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    let ids: Vec<&str> = channels
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["channelId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["c1", "c2"]);
}

#[tokio::test]
async fn list_channels_respects_limit() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock
        .expect_list_channels()
        .times(1)
        .returning(|req| {
            let req = req.into_inner();
            assert_eq!(req.page_size, 2);
            Ok(Response::new(ListChannelsResponse {
                channels: vec![
                    Channel {
                        channel_id: "c1".into(),
                        ..Default::default()
                    },
                    Channel {
                        channel_id: "c2".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: "page-2".into(),
            }))
        });

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        MockRunServiceImpl::new(),
        channel_mock,
    )
    .await;

    let resp = server
        .list_channels(params("", Some(2)))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    assert_eq!(channels.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn list_channels_breaks_on_empty_page() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().times(1).returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        MockRunServiceImpl::new(),
        channel_mock,
    )
    .await;

    let resp = server
        .list_channels(params("", None))
        .await
        .expect("list_channels failed");

    assert!(
        structured_field(resp, "channels")
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn list_channels_propagates_grpc_error() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock
        .expect_list_channels()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mocks(
        MockAssetServiceImpl::new(),
        MockRunServiceImpl::new(),
        channel_mock,
    )
    .await;

    let err = server
        .list_channels(params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

async fn server_with_report_mock(
    report_mock: MockReportServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ReportServiceServer::new(report_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (SiftMcpServer::new(channel), handle)
}

fn report_params(filter: &str, limit: Option<u32>) -> Parameters<ReportListParams> {
    Parameters(ReportListParams {
        filter: filter.into(),
        order_by: None,
        limit,
        organization_id: None,
    })
}

#[tokio::test]
async fn list_reports_returns_single_page() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_list_reports()
        .withf(|req| req.get_ref().filter == "name == \"nightly\"")
        .returning(|_| {
            Ok(Response::new(ListReportsResponse {
                reports: vec![Report {
                    report_id: "rep1".into(),
                    name: "nightly".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_report_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("name == \"nightly\"", None))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    assert_eq!(reports.as_array().unwrap().len(), 1);
    assert_eq!(reports[0]["reportId"], "rep1");
}

#[tokio::test]
async fn list_reports_forwards_organization_id() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_list_reports()
        .withf(|req| req.get_ref().organization_id == "org-123")
        .returning(|_| {
            Ok(Response::new(ListReportsResponse {
                reports: vec![Report {
                    report_id: "rep1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_report_mock(report_mock).await;

    let resp = server
        .list_reports(Parameters(ReportListParams {
            filter: String::new(),
            order_by: None,
            limit: None,
            organization_id: Some("org-123".into()),
        }))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    assert_eq!(reports.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn list_reports_paginates_until_token_empty() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_list_reports().returning(|req| {
        let token = req.into_inner().page_token;
        let (reports, next) = match token.as_str() {
            "" => (
                vec![Report {
                    report_id: "rep1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Report {
                    report_id: "rep2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListReportsResponse {
            reports,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_report_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("", None))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    let ids: Vec<&str> = reports
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["reportId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["rep1", "rep2"]);
}

#[tokio::test]
async fn list_reports_truncates_to_limit_across_pages() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_list_reports().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (reports, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Report {
                        report_id: "rep1".into(),
                        ..Default::default()
                    },
                    Report {
                        report_id: "rep2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Report {
                        report_id: "rep3".into(),
                        ..Default::default()
                    },
                    Report {
                        report_id: "rep4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListReportsResponse {
            reports,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_report_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("", Some(3)))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    let ids = reports
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["reportId"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["rep1", "rep2", "rep3"]);
}

#[tokio::test]
async fn list_reports_breaks_on_empty_page() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_list_reports().times(1).returning(|_| {
        Ok(Response::new(ListReportsResponse {
            reports: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (server, _h) = server_with_report_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("", None))
        .await
        .expect("list_reports failed");

    assert!(
        structured_field(resp, "reports")
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn list_reports_propagates_grpc_error() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_list_reports()
        .returning(|_| Err(Status::not_found("no such report")));

    let (server, _h) = server_with_report_mock(report_mock).await;

    let err = server
        .list_reports(report_params("", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("no such report"));
}
