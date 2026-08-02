use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer},
    channels::v3::{Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer},
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{assets::v1::MockAssetServiceImpl, channels::v3::MockChannelServiceImpl},
};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::GetDataParams;
use crate::{server::SiftMcpServer, service::common::PAGE_SIZE};

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
        SiftMcpServer::new(channel, String::from("https://api.test.local"), false),
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
