use std::path::PathBuf;

use bytes::Bytes;
use pbjson_types::{Any, Timestamp};
use prost::Message;
use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer},
    calculated_channels::{
        v1::{ExpressionChannelReference, ExpressionRequest},
        v2::{
            BatchResolveCalculatedChannelsResponse, CalculatedChannel,
            ListCalculatedChannelsResponse, ResolveCalculatedChannelResponse,
            ResolvedCalculatedChannel, UnresolvedCalculatedChannel,
            calculated_channel_service_server::CalculatedChannelServiceServer,
        },
    },
    channels::v3::{Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer},
    data::v2::{
        DoubleValue, DoubleValues, GetDataResponse, Metadata,
        data_service_server::DataServiceServer, metadata, query::Query as QueryKind,
    },
    runs::v2::{ListRunsResponse, Run, run_service_server::RunServiceServer},
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{
        assets::v1::MockAssetServiceImpl,
        calculated_channels::v2::MockCalculatedChannelServiceImpl,
        channels::v3::MockChannelServiceImpl, data::v2::MockDataServiceImpl,
        runs::v2::MockRunServiceImpl,
    },
};
use tempdir::TempDir;
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::GetDataParams;
use crate::{
    server::SiftMcpServer, service::common::PAGE_SIZE, tool::common::test_support::structured,
};

async fn server_with_mocks(
    assets: MockAssetServiceImpl,
    channels: MockChannelServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(assets))
            .add_service(ChannelServiceServer::new(channels))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://api.test.local"),
            false,
            false,
        ),
        handle,
    )
}

fn one_asset_mock() -> MockAssetServiceImpl {
    let mut assets = MockAssetServiceImpl::new();
    assets.expect_list_assets().returning(|_| {
        Ok(Response::new(ListAssetsResponse {
            assets: vec![Asset {
                asset_id: "asset-1".into(),
                name: "bench".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    assets
}

fn get_data_params(channel_regex: &str) -> Parameters<GetDataParams> {
    Parameters(GetDataParams {
        asset_name: "bench".into(),
        run_name: None,
        start_time_unix_nanos: Some(0),
        end_time_unix_nanos: Some(1),
        sample_ms: 0,
        channel_names: None,
        channel_regex: Some(channel_regex.into()),
        output: std::env::temp_dir().join("sift-mcp-get-data-test-never-written.parquet"),
    })
}

/// A channel selection that fills the service's record cap may have been
/// silently truncated, which would produce a Parquet file that is missing
/// channels with no warning. The tool must refuse loudly instead.
#[tokio::test]
async fn get_data_rejects_channel_selection_at_cap() {
    let mut channels = MockChannelServiceImpl::new();
    channels.expect_list_channels().returning(|req| {
        let page_size = req.into_inner().page_size as usize;
        Ok(Response::new(ListChannelsResponse {
            channels: (0..page_size)
                .map(|i| Channel {
                    channel_id: format!("ch-{i}"),
                    name: format!("channel.{i}"),
                    ..Default::default()
                })
                .collect(),
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mocks(one_asset_mock(), channels).await;

    let err = server
        .get_data(get_data_params("channel\\..*"))
        .await
        .expect_err("a capped channel selection must be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(
        err.message.contains(&PAGE_SIZE.to_string()),
        "error should name the cap: {}",
        err.message
    );
    assert!(
        err.message.contains("channel_regex") || err.message.contains("narrow"),
        "error should tell the caller how to recover: {}",
        err.message
    );
}

/// The channel resolution inside `get_data` must request the service maximum,
/// not the default list limit; otherwise selections between the default and
/// the cap are silently incomplete.
#[tokio::test]
async fn get_data_channel_resolution_requests_service_maximum() {
    let mut channels = MockChannelServiceImpl::new();
    channels
        .expect_list_channels()
        .withf(|req| req.get_ref().page_size == PAGE_SIZE)
        .returning(|_| {
            Ok(Response::new(ListChannelsResponse {
                channels: Vec::new(),
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mocks(one_asset_mock(), channels).await;

    let err = server
        .get_data(get_data_params("nothing-matches"))
        .await
        .expect_err("no matching channels is an error");

    // The withf above is the real assertion: a request with the wrong
    // page_size fails the mock expectation. Reaching RESOURCE_NOT_FOUND
    // proves the request passed the filter.
    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}

/// Server wired to every service `get_data` touches, including the calculated
/// channel and data services the saved-calculation path needs.
async fn server_with_calculation_mocks(
    assets: MockAssetServiceImpl,
    channels: MockChannelServiceImpl,
    runs: MockRunServiceImpl,
    calculated: MockCalculatedChannelServiceImpl,
    data: MockDataServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(assets))
            .add_service(ChannelServiceServer::new(channels))
            .add_service(RunServiceServer::new(runs))
            .add_service(CalculatedChannelServiceServer::new(calculated))
            .add_service(DataServiceServer::new(data))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://api.test.local"),
            false,
            false,
        ),
        handle,
    )
}

fn named_params(
    names: &[&str],
    run_name: Option<&str>,
    output: PathBuf,
) -> Parameters<GetDataParams> {
    Parameters(GetDataParams {
        asset_name: "bench".into(),
        run_name: run_name.map(String::from),
        start_time_unix_nanos: Some(0),
        end_time_unix_nanos: Some(3_000_000_000),
        sample_ms: 0,
        channel_names: Some(names.iter().map(|n| (*n).to_string()).collect()),
        channel_regex: None,
        output,
    })
}

fn raw_channel_mock(names: &[&str]) -> MockChannelServiceImpl {
    let channels = names
        .iter()
        .enumerate()
        .map(|(i, name)| Channel {
            channel_id: format!("ch-{i}"),
            name: (*name).to_string(),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels().returning(move |_| {
        Ok(Response::new(ListChannelsResponse {
            channels: channels.clone(),
            next_page_token: String::new(),
        }))
    });
    mock
}

fn one_run_mock() -> MockRunServiceImpl {
    let mut runs = MockRunServiceImpl::new();
    runs.expect_list_runs().returning(|_| {
        Ok(Response::new(ListRunsResponse {
            runs: vec![Run {
                run_id: "run-1".into(),
                name: "hotfire-3".into(),
                start_time: Some(Timestamp {
                    seconds: 0,
                    nanos: 0,
                }),
                stop_time: Some(Timestamp {
                    seconds: 3,
                    nanos: 0,
                }),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    runs
}

/// A calculated channel service that finds `name` and resolves it for the
/// asset.
fn resolving_calculation_mock(name: &'static str) -> MockCalculatedChannelServiceImpl {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(move |_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![CalculatedChannel {
                calculated_channel_id: "cc1".into(),
                name: name.into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels()
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: None,
                    resolved: vec![ResolvedCalculatedChannel {
                        asset_name: "bench".into(),
                        asset_id: "asset-1".into(),
                        expression_request: Some(ExpressionRequest {
                            expression: "$1 * 2".into(),
                            expression_channel_references: vec![ExpressionChannelReference {
                                channel_reference: "$1".into(),
                                channel_id: "ch-9".into(),
                                calculated_channel_reference: None,
                            }],
                            ..Default::default()
                        }),
                        output_data_type: 0,
                    }],
                    unresolved: vec![],
                }],
            }))
        });
    mock
}

fn double_page(channel_id: &str, channel_name: &str, samples: Vec<(i64, f64)>) -> Any {
    let payload = DoubleValues {
        metadata: Some(Metadata {
            channel: Some(metadata::Channel {
                channel_id: channel_id.into(),
                name: channel_name.into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        values: samples
            .into_iter()
            .map(|(ts_nanos, value)| DoubleValue {
                timestamp: Some(Timestamp {
                    seconds: ts_nanos / 1_000_000_000,
                    nanos: (ts_nanos % 1_000_000_000) as i32,
                }),
                value,
            })
            .collect(),
    };

    Any {
        type_url: "sift.data.v2.DoubleValues".into(),
        value: Bytes::from(payload.encode_to_vec()),
    }
}

/// A channel name with no raw channel on the asset is served as a saved
/// calculated channel: the query carries the resolved expression, not a
/// channel id.
#[tokio::test]
async fn get_data_queries_saved_calculated_channel() {
    let mut data = MockDataServiceImpl::new();
    data.expect_get_data()
        .times(1)
        .withf(|req| {
            let queries = &req.get_ref().queries;
            queries.len() == 1
                && match queries[0].query.as_ref() {
                    Some(QueryKind::CalculatedChannel(query)) => {
                        let expression = query.expression.as_ref();
                        query.channel_key == "thrust_margin"
                            && expression.is_some_and(|e| {
                                e.expression == "$1 * 2"
                                    && e.expression_channel_references
                                        .first()
                                        .is_some_and(|r| r.channel_id == "ch-9")
                            })
                    }
                    _ => false,
                }
        })
        .returning(|_| {
            Ok(Response::new(GetDataResponse {
                data: vec![double_page("", "thrust_margin", vec![(1_000_000_000, 4.0)])],
                next_page_token: String::new(),
            }))
        });

    let dir = TempDir::new("sift-mcp-cc-data").expect("temp dir");
    let output = dir.path().join("out.parquet");

    let (server, _h) = server_with_calculation_mocks(
        one_asset_mock(),
        raw_channel_mock(&[]),
        MockRunServiceImpl::new(),
        resolving_calculation_mock("thrust_margin"),
        data,
    )
    .await;

    let result = server
        .get_data(named_params(&["thrust_margin"], None, output.clone()))
        .await
        .expect("get_data should serve a saved calculated channel");

    let body = structured(result);
    assert_eq!(
        body.get("output").and_then(|v| v.as_str()),
        Some(output.to_string_lossy().as_ref()),
    );
    assert!(
        body.get("unresolved_calculated_channels").is_none(),
        "nothing should be reported as unresolved: {body}",
    );
    assert!(output.exists(), "parquet file should be written");
}

/// One requested name resolves and one does not. The file is still written, but
/// the response must name the channel that did not resolve along with the asset
/// and run it was requested for.
#[tokio::test]
async fn get_data_reports_unresolved_calculated_channel() {
    let mut calculated = MockCalculatedChannelServiceImpl::new();
    calculated.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![CalculatedChannel {
                calculated_channel_id: "cc1".into(),
                name: "chamber_delta".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    calculated
        .expect_batch_resolve_calculated_channels()
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: None,
                    resolved: vec![],
                    unresolved: vec![UnresolvedCalculatedChannel {
                        asset_name: "bench".into(),
                        error_message: "asset is missing channel chamber_pressure".into(),
                    }],
                }],
            }))
        });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data().times(1).returning(|_| {
        Ok(Response::new(GetDataResponse {
            data: vec![double_page(
                "ch-0",
                "temperature_c",
                vec![(1_000_000_000, 20.0)],
            )],
            next_page_token: String::new(),
        }))
    });

    let dir = TempDir::new("sift-mcp-cc-partial").expect("temp dir");
    let output = dir.path().join("out.parquet");

    let (server, _h) = server_with_calculation_mocks(
        one_asset_mock(),
        raw_channel_mock(&["temperature_c"]),
        one_run_mock(),
        calculated,
        data,
    )
    .await;

    let result = server
        .get_data(named_params(
            &["temperature_c", "chamber_delta"],
            Some("hotfire-3"),
            output.clone(),
        ))
        .await
        .expect("a partial resolution should still return the resolved data");

    let body = structured(result);
    let reported = body
        .get("unresolved_calculated_channels")
        .expect("partial resolution must be reported")
        .to_string();
    assert!(
        reported.contains("chamber_delta") && reported.contains("chamber_pressure"),
        "report should name the channel and the reason: {reported}",
    );

    let next_step = body
        .get("next_step")
        .and_then(|v| v.as_str())
        .expect("next_step")
        .to_string();
    assert!(
        next_step.contains("calculated channel data was not resolved for asset 'bench'")
            && next_step.contains("hotfire-3")
            && next_step.contains("chamber_delta"),
        "next_step must name what did not resolve, and for which asset and run: {next_step}",
    );
}

/// When nothing the caller named can be served, the call fails and the error
/// names every channel that did not resolve.
#[tokio::test]
async fn get_data_errors_when_no_channel_resolves() {
    let mut calculated = MockCalculatedChannelServiceImpl::new();
    calculated.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![],
            next_page_token: String::new(),
        }))
    });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data().times(0);

    let dir = TempDir::new("sift-mcp-cc-none").expect("temp dir");
    let output = dir.path().join("out.parquet");

    let (server, _h) = server_with_calculation_mocks(
        one_asset_mock(),
        raw_channel_mock(&[]),
        MockRunServiceImpl::new(),
        calculated,
        data,
    )
    .await;

    let err = server
        .get_data(named_params(&["chamber_delta"], None, output))
        .await
        .expect_err("a request where nothing resolves must fail");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(
        err.message
            .contains("calculated channel data was not resolved for asset 'bench'")
            && err.message.contains("chamber_delta"),
        "error must name what did not resolve: {}",
        err.message,
    );
}

/// A name that exists as both a raw channel and a saved calculated channel is
/// served as the raw channel; the calculated channel service is never consulted.
#[tokio::test]
async fn get_data_prefers_raw_channel_over_calculated_channel() {
    let mut calculated = MockCalculatedChannelServiceImpl::new();
    calculated.expect_list_calculated_channels().times(0);
    calculated
        .expect_batch_resolve_calculated_channels()
        .times(0);

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data()
        .times(1)
        .withf(|req| {
            let queries = &req.get_ref().queries;
            queries.len() == 1
                && matches!(
                    queries[0].query.as_ref(),
                    Some(QueryKind::Channel(query)) if query.channel_id == "ch-0"
                )
        })
        .returning(|_| {
            Ok(Response::new(GetDataResponse {
                data: vec![double_page(
                    "ch-0",
                    "thrust_margin",
                    vec![(1_000_000_000, 1.0)],
                )],
                next_page_token: String::new(),
            }))
        });

    let dir = TempDir::new("sift-mcp-cc-precedence").expect("temp dir");
    let output = dir.path().join("out.parquet");

    let (server, _h) = server_with_calculation_mocks(
        one_asset_mock(),
        raw_channel_mock(&["thrust_margin"]),
        MockRunServiceImpl::new(),
        calculated,
        data,
    )
    .await;

    server
        .get_data(named_params(&["thrust_margin"], None, output))
        .await
        .expect("the raw channel should be served without any calculated lookup");
}

/// A window with no samples still has to tell the caller which calculated
/// channels never resolved; otherwise the failure reads as "the asset has no
/// data" when part of the request was never queried at all.
#[tokio::test]
async fn get_data_reports_unresolved_calculated_channel_when_no_samples() {
    let mut calculated = MockCalculatedChannelServiceImpl::new();
    calculated.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![CalculatedChannel {
                calculated_channel_id: "cc1".into(),
                name: "chamber_delta".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    calculated
        .expect_batch_resolve_calculated_channels()
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: Some("cc1".into()),
                    resolved: vec![],
                    unresolved: vec![UnresolvedCalculatedChannel {
                        asset_name: "bench".into(),
                        error_message: "asset is missing channel chamber_pressure".into(),
                    }],
                }],
            }))
        });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data().times(1).returning(|_| {
        Ok(Response::new(GetDataResponse {
            data: vec![],
            next_page_token: String::new(),
        }))
    });

    let dir = TempDir::new("sift-mcp-cc-nodata").expect("temp dir");
    let output = dir.path().join("out.parquet");

    let (server, _h) = server_with_calculation_mocks(
        one_asset_mock(),
        raw_channel_mock(&["temperature_c"]),
        MockRunServiceImpl::new(),
        calculated,
        data,
    )
    .await;

    let err = server
        .get_data(named_params(
            &["temperature_c", "chamber_delta"],
            None,
            output,
        ))
        .await
        .expect_err("an empty window is still an error");

    assert!(
        err.message
            .contains("calculated channel data was not resolved for asset 'bench'")
            && err.message.contains("chamber_delta"),
        "the empty-window error must still name what did not resolve: {}",
        err.message,
    );
}

/// The same calculated channel named twice must be queried once. Two identical
/// queries share a channel key, so their pages merge into one column and repeat
/// timestamps in the output file.
#[tokio::test]
async fn get_data_deduplicates_repeated_calculated_channel_name() {
    let mut calculated = MockCalculatedChannelServiceImpl::new();
    calculated.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![CalculatedChannel {
                calculated_channel_id: "cc1".into(),
                name: "thrust_margin".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    calculated
        .expect_batch_resolve_calculated_channels()
        .times(1)
        .withf(|req| req.get_ref().requests.len() == 1)
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: Some("cc1".into()),
                    resolved: vec![ResolvedCalculatedChannel {
                        asset_name: "bench".into(),
                        asset_id: "asset-1".into(),
                        expression_request: Some(ExpressionRequest {
                            expression: "$1 * 2".into(),
                            expression_channel_references: vec![ExpressionChannelReference {
                                channel_reference: "$1".into(),
                                channel_id: "ch-9".into(),
                                calculated_channel_reference: None,
                            }],
                            ..Default::default()
                        }),
                        output_data_type: 0,
                    }],
                    unresolved: vec![],
                }],
            }))
        });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data()
        .times(1)
        .withf(|req| req.get_ref().queries.len() == 1)
        .returning(|_| {
            Ok(Response::new(GetDataResponse {
                data: vec![double_page("thrust_margin", "", vec![(1_000_000_000, 4.0)])],
                next_page_token: String::new(),
            }))
        });

    let dir = TempDir::new("sift-mcp-cc-dedupe").expect("temp dir");
    let output = dir.path().join("out.parquet");

    let (server, _h) = server_with_calculation_mocks(
        one_asset_mock(),
        raw_channel_mock(&[]),
        MockRunServiceImpl::new(),
        calculated,
        data,
    )
    .await;

    server
        .get_data(named_params(
            &["thrust_margin", "thrust_margin"],
            None,
            output,
        ))
        .await
        .expect("a repeated name must resolve and query once");
}
