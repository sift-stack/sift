use bytes::Bytes;
use pbjson_types::{Any, Timestamp};
use prost::Message;
use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer},
    channels::v3::{Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer},
    data::v2::{
        DoubleValue, DoubleValues, GetDataResponse, Metadata,
        data_service_server::DataServiceServer, metadata,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{
        assets::v1::MockAssetServiceImpl, channels::v3::MockChannelServiceImpl,
        data::v2::MockDataServiceImpl,
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
    server_with_all_mocks(assets, channels, MockDataServiceImpl::new()).await
}

async fn server_with_all_mocks(
    assets: MockAssetServiceImpl,
    channels: MockChannelServiceImpl,
    data: MockDataServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(assets))
            .add_service(ChannelServiceServer::new(channels))
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
        asset_name: Some("bench".into()),
        asset_id: None,
        run_name: None,
        start_time_unix_nanos: Some(0),
        end_time_unix_nanos: Some(1),
        sample_ms: 0,
        channel_names: None,
        channel_regex: Some(channel_regex.into()),
        output: std::env::temp_dir().join("sift-mcp-get-data-test-never-written.parquet"),
    })
}

/// `list_runs` hands the caller an `asset_id`, not an asset name, so `get_data`
/// must accept the id directly. The withf below is the real assertion: the
/// asset lookup must filter on `asset_id`, not `name`.
#[tokio::test]
async fn get_data_resolves_the_asset_by_id() {
    let mut assets = MockAssetServiceImpl::new();
    assets
        .expect_list_assets()
        .withf(|req| req.get_ref().filter == r#"asset_id == "asset-1""#)
        .returning(|_| {
            Ok(Response::new(ListAssetsResponse {
                assets: vec![Asset {
                    asset_id: "asset-1".into(),
                    name: "bench".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let mut channels = MockChannelServiceImpl::new();
    channels.expect_list_channels().returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: Vec::new(),
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mocks(assets, channels).await;

    let mut params = get_data_params("channel\\..*");
    params.0.asset_name = None;
    params.0.asset_id = Some("asset-1".into());

    // No channels are mocked to match, so the call still errs — but reaching
    // RESOURCE_NOT_FOUND for channels proves the id-filtered asset lookup
    // succeeded (a name-filtered lookup would have failed the withf).
    let err = server
        .get_data(params)
        .await
        .expect_err("no matching channels is an error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(
        err.message.contains("no channels matched"),
        "should fail at channel resolution, not asset resolution: {}",
        err.message
    );
}

/// Exactly one of `asset_name` / `asset_id` must be provided; the error must
/// name both fields so the caller knows how to recover.
#[tokio::test]
async fn get_data_requires_exactly_one_asset_identifier() {
    let (server, _h) =
        server_with_mocks(MockAssetServiceImpl::new(), MockChannelServiceImpl::new()).await;

    let mut both = get_data_params("channel\\..*");
    both.0.asset_id = Some("asset-1".into());
    let err = server
        .get_data(both)
        .await
        .expect_err("asset_name and asset_id together must be rejected");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(
        err.message.contains("asset_name") && err.message.contains("asset_id"),
        "error should name both fields: {}",
        err.message
    );

    let mut neither = get_data_params("channel\\..*");
    neither.0.asset_name = None;
    let err = server
        .get_data(neither)
        .await
        .expect_err("missing both asset_name and asset_id must be rejected");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(
        err.message.contains("asset_name") && err.message.contains("asset_id"),
        "error should name both fields: {}",
        err.message
    );
}

/// A truncated channel selection would produce a Parquet file that is missing
/// channels with no warning. The tool must refuse loudly instead. Truncation is
/// the service reporting more matches beyond the cap, not the result merely
/// reaching it — see the companion test below.
#[tokio::test]
async fn get_data_rejects_a_truncated_channel_selection() {
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
            next_page_token: "more-channels".into(),
        }))
    });

    let (server, _h) = server_with_mocks(one_asset_mock(), channels).await;

    let err = server
        .get_data(get_data_params("channel\\..*"))
        .await
        .expect_err("a truncated channel selection must be rejected");

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

/// The old guard inferred truncation from the result reaching the cap, so a
/// selection that matched exactly that many channels — with nothing left
/// upstream — was rejected as incomplete when it was whole. The service reports
/// the cut directly now, so this case must get through.
#[tokio::test]
async fn get_data_accepts_a_full_page_with_nothing_left() {
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

    // Data retrieval is not mocked here, so the call still fails — but it must
    // get past channel selection rather than being turned away as truncated.
    let err = server
        .get_data(get_data_params("channel\\..*"))
        .await
        .expect_err("no data service is wired in this test");

    assert!(
        !err.message.contains("incomplete") && !err.message.contains("narrow"),
        "a complete selection must not be rejected as truncated: {}",
        err.message
    );
}

fn double_page(channel_id: &str, channel_name: &str, ts_nanos: i64, value: f64) -> Any {
    let payload = DoubleValues {
        metadata: Some(Metadata {
            channel: Some(metadata::Channel {
                channel_id: channel_id.into(),
                name: channel_name.into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        values: vec![DoubleValue {
            timestamp: Some(Timestamp {
                seconds: ts_nanos / 1_000_000_000,
                nanos: (ts_nanos % 1_000_000_000) as i32,
            }),
            value,
        }],
        extras: vec![],
    };

    Any {
        type_url: "sift.data.v2.DoubleValues".into(),
        value: Bytes::from(payload.encode_to_vec()),
    }
}

/// `name in [...]` matches what it can and says nothing about the rest, so a
/// misspelled channel used to come back as a narrower table reported as a
/// complete fetch.
#[tokio::test]
async fn get_data_reports_channel_names_that_matched_nothing() {
    let mut channels = MockChannelServiceImpl::new();
    channels.expect_list_channels().returning(|_| {
        // Only two of the three requested names exist on this asset.
        Ok(Response::new(ListChannelsResponse {
            channels: vec![
                Channel {
                    channel_id: "ch-1".into(),
                    name: "pressure".into(),
                    ..Default::default()
                },
                Channel {
                    channel_id: "ch-2".into(),
                    name: "temperature".into(),
                    ..Default::default()
                },
            ],
            next_page_token: String::new(),
        }))
    });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data().returning(|_| {
        Ok(Response::new(GetDataResponse {
            data: vec![
                double_page("ch-1", "pressure", 1_000_000_000, 1.0),
                double_page("ch-2", "temperature", 1_000_000_000, 2.0),
            ],
            next_page_token: String::new(),
        }))
    });

    let dir = TempDir::new("sift-mcp-get-data").expect("failed to create temp dir");
    let (server, _h) = server_with_all_mocks(one_asset_mock(), channels, data).await;

    let resp = server
        .get_data(Parameters(GetDataParams {
            asset_name: Some("bench".into()),
            asset_id: None,
            run_name: None,
            start_time_unix_nanos: Some(0),
            end_time_unix_nanos: Some(2_000_000_000),
            sample_ms: 0,
            channel_names: Some(vec![
                "pressure".into(),
                "temperature".into(),
                "presure".into(),
            ]),
            channel_regex: None,
            output: dir.path().join("out.parquet"),
        }))
        .await
        .expect("get_data should still succeed for the channels that matched");

    let body = structured(resp);
    assert_eq!(
        body["unmatched_channel_names"],
        serde_json::json!(["presure"])
    );
    assert!(
        body.get("empty_channels").is_none(),
        "both matched channels returned samples: {body}"
    );

    // The structured field alone is not enough. `next_step` is what the calling
    // model reads before it answers, so the miss has to appear there too.
    let next_step = body["next_step"].as_str().expect("next_step");
    assert!(next_step.contains("presure"), "{next_step}");
    assert!(
        !next_step.starts_with("Wrote channel data to") || next_step.contains("does NOT"),
        "a partial fetch must not read as a clean success: {next_step}"
    );
}

/// A regex selection carries no per-name expectation, so there is nothing to
/// report as unmatched.
#[tokio::test]
async fn get_data_reports_no_unmatched_names_for_a_regex_selection() {
    let mut channels = MockChannelServiceImpl::new();
    channels.expect_list_channels().returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![Channel {
                channel_id: "ch-1".into(),
                name: "pressure".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data().returning(|_| {
        Ok(Response::new(GetDataResponse {
            data: vec![double_page("ch-1", "pressure", 1_000_000_000, 1.0)],
            next_page_token: String::new(),
        }))
    });

    let dir = TempDir::new("sift-mcp-get-data").expect("failed to create temp dir");
    let (server, _h) = server_with_all_mocks(one_asset_mock(), channels, data).await;

    let resp = server
        .get_data(Parameters(GetDataParams {
            asset_name: Some("bench".into()),
            asset_id: None,
            run_name: None,
            start_time_unix_nanos: Some(0),
            end_time_unix_nanos: Some(2_000_000_000),
            sample_ms: 0,
            channel_names: None,
            channel_regex: Some("press.*".into()),
            output: dir.path().join("out.parquet"),
        }))
        .await
        .expect("get_data failed");

    let body = structured(resp);
    assert!(body.get("unmatched_channel_names").is_none(), "{body}");
    assert!(body.get("empty_channels").is_none(), "{body}");
}

/// A channel that matched but returned nothing has no column in the file, so the
/// tool has to name it or the caller cannot tell it was ever requested.
#[tokio::test]
async fn get_data_reports_matched_channels_that_returned_no_samples() {
    let mut channels = MockChannelServiceImpl::new();
    channels.expect_list_channels().returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![
                Channel {
                    channel_id: "ch-1".into(),
                    name: "pressure".into(),
                    ..Default::default()
                },
                Channel {
                    channel_id: "ch-2".into(),
                    name: "temperature".into(),
                    ..Default::default()
                },
            ],
            next_page_token: String::new(),
        }))
    });

    let mut data = MockDataServiceImpl::new();
    data.expect_get_data().returning(|_| {
        // Only one of the two matched channels has samples in this window.
        Ok(Response::new(GetDataResponse {
            data: vec![double_page("ch-1", "pressure", 1_000_000_000, 1.0)],
            next_page_token: String::new(),
        }))
    });

    let dir = TempDir::new("sift-mcp-get-data").expect("failed to create temp dir");
    let (server, _h) = server_with_all_mocks(one_asset_mock(), channels, data).await;

    let resp = server
        .get_data(Parameters(GetDataParams {
            asset_name: Some("bench".into()),
            asset_id: None,
            run_name: None,
            start_time_unix_nanos: Some(0),
            end_time_unix_nanos: Some(2_000_000_000),
            sample_ms: 0,
            channel_names: Some(vec!["pressure".into(), "temperature".into()]),
            channel_regex: None,
            output: dir.path().join("out.parquet"),
        }))
        .await
        .expect("a partially empty window is still a successful fetch");

    let body = structured(resp);
    assert_eq!(body["empty_channels"], serde_json::json!(["temperature"]));
    assert!(body.get("unmatched_channel_names").is_none(), "{body}");

    let next_step = body["next_step"].as_str().expect("next_step");
    assert!(next_step.contains("temperature"), "{next_step}");
    assert!(next_step.contains("no samples"), "{next_step}");
}
