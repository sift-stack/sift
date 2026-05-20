use sift_rs::assets::v1::{
    Asset, GetAssetRequest, GetAssetResponse, asset_service_client::AssetServiceClient,
    asset_service_server::AssetServiceServer,
};
use tonic::{Request, Response, transport::Server};

use super::memory_sift_channel;
use crate::mock::assets::v1::MockAssetServiceImpl;

/// Demonstrates how to use [memory_sift_channel] for testing.
#[tokio::test]
async fn test_memory_sift_channel() {
    // Setup the mock
    let asset_grpc = {
        let mut mock = MockAssetServiceImpl::new();

        mock.expect_get_asset().returning(|_req| {
            Ok(Response::new(GetAssetResponse {
                asset: Some(Asset {
                    asset_id: "asset-1".into(),
                    name: "engine".into(),
                    ..Default::default()
                }),
            }))
        });

        AssetServiceServer::new(mock)
    };

    let (client, server) = tokio::io::duplex(1024);

    let sift_channel = memory_sift_channel(client).await;

    // Spawn the server
    tokio::spawn(async move {
        Server::builder()
            .add_service(asset_grpc)
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    let mut client = AssetServiceClient::new(sift_channel);

    let resp = client
        .get_asset(Request::new(GetAssetRequest {
            asset_id: "asset-1".into(),
        }))
        .await
        .expect("get_asset rpc failed")
        .into_inner();

    let asset = resp.asset.expect("expected asset in response");
    assert_eq!(asset.asset_id, "asset-1");
    assert_eq!(asset.name, "engine");
}
