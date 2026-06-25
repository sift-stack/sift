use rmcp::model::ErrorCode;
use sift_rs::assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::assets::v1::MockAssetServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    service::common::PAGE_SIZE,
    tool::common::test_support::{list_params, structured_field},
};

async fn server_with_mock(mock: MockAssetServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://api.test.local"),
            String::new(),
        ),
        handle,
    )
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

    let (server, _h) = server_with_mock(asset_mock).await;

    let resp = server
        .list_assets(list_params("name == \"engine\"", None))
        .await
        .expect("list_assets failed");

    let assets = structured_field(resp, "assets");
    assert_eq!(assets.as_array().unwrap().len(), 2);
    assert_eq!(assets[0]["assetId"], "a1");
    assert_eq!(assets[0]["url"], "https://app.test.local/asset/a1");
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

    let (server, _h) = server_with_mock(asset_mock).await;

    let resp = server
        .list_assets(list_params("", None))
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

    let (server, _h) = server_with_mock(asset_mock).await;

    let resp = server
        .list_assets(list_params("", Some(2)))
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

    let (server, _h) = server_with_mock(asset_mock).await;

    let resp = server
        .list_assets(list_params("", None))
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

    let (server, _h) = server_with_mock(asset_mock).await;

    let err = server
        .list_assets(list_params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}
