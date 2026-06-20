use sift_rs::assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::assets::v1::MockAssetServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::AssetService;
use crate::policy::RetryPolicy;
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(mock: MockAssetServiceImpl) -> (AssetService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (AssetService::new(channel, RetryPolicy::default()), handle)
}

#[tokio::test]
async fn list_assets_returns_single_page() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets()
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

    let (service, _h) = service_with_mock(mock).await;

    let assets = service
        .list_assets("name == \"engine\"".to_string(), None, None)
        .await
        .expect("list_assets failed");

    assert_eq!(assets.len(), 2);
    assert_eq!(assets[0].asset_id, "a1");
    assert_eq!(assets[1].asset_id, "a2");
}

#[tokio::test]
async fn list_assets_paginates_until_token_empty() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets().returning(|req| {
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

    let (service, _h) = service_with_mock(mock).await;

    let assets = service
        .list_assets(String::new(), None, None)
        .await
        .expect("list_assets failed");

    let ids: Vec<&str> = assets.iter().map(|a| a.asset_id.as_str()).collect();
    assert_eq!(ids, vec!["a1", "a2", "a3"]);
}

#[tokio::test]
async fn list_assets_respects_limit() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets().times(1).returning(|req| {
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

    let (service, _h) = service_with_mock(mock).await;

    let assets = service
        .list_assets(String::new(), None, Some(2))
        .await
        .expect("list_assets failed");

    assert_eq!(assets.len(), 2);
}

#[tokio::test]
async fn list_assets_truncates_to_limit_across_pages() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (assets, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Asset {
                        asset_id: "a1".into(),
                        ..Default::default()
                    },
                    Asset {
                        asset_id: "a2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Asset {
                        asset_id: "a3".into(),
                        ..Default::default()
                    },
                    Asset {
                        asset_id: "a4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListAssetsResponse {
            assets,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let assets = service
        .list_assets(String::new(), None, Some(3))
        .await
        .expect("list_assets failed");

    let ids: Vec<&str> = assets.iter().map(|a| a.asset_id.as_str()).collect();
    assert_eq!(ids, vec!["a1", "a2", "a3"]);
}

#[tokio::test]
async fn list_assets_breaks_on_empty_page() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets().times(1).returning(|_| {
        Ok(Response::new(ListAssetsResponse {
            assets: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let assets = service
        .list_assets(String::new(), None, None)
        .await
        .expect("list_assets failed");

    assert!(assets.is_empty());
}

#[tokio::test]
async fn list_assets_propagates_grpc_error() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_assets("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query assets"));
}
