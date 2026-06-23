use sift_rs::{
    assets::v1::{
        Asset, ListAssetsResponse, UpdateAssetResponse, asset_service_server::AssetServiceServer,
    },
    metadata::v1::{
        MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
    },
};
use sift_test_util::{grpc::memory_sift_channel, mock::assets::v1::MockAssetServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::AssetService;
use crate::policy::RetryPolicy;
use crate::service::common::PAGE_SIZE;

fn string_metadata(name: &str, value: &str) -> MetadataValue {
    MetadataValue {
        key: Some(MetadataKey {
            name: name.into(),
            r#type: MetadataKeyType::String.into(),
            ..Default::default()
        }),
        value: Some(MetadataValueInner::StringValue(value.into())),
        ..Default::default()
    }
}

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

#[tokio::test]
async fn update_asset_tags_only_sets_tags_mask() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let asset = req.asset.as_ref().expect("asset present");
            let mask = req.update_mask.as_ref().expect("mask present");
            asset.asset_id == "a1"
                && asset.tags == vec!["primary".to_string(), "prod".to_string()]
                && asset.metadata.is_empty()
                && mask.paths == vec!["tags".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            let mut asset = req.asset.expect("asset present");
            asset.name = "engine".into();
            Ok(Response::new(UpdateAssetResponse { asset: Some(asset) }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = service
        .update_asset(
            "a1".to_string(),
            Some(vec!["primary".into(), "prod".into()]),
            None,
        )
        .await
        .expect("update_asset failed");

    assert_eq!(updated.asset_id, "a1");
    assert_eq!(updated.name, "engine");
    assert_eq!(updated.tags, vec!["primary".to_string(), "prod".to_string()]);
}

#[tokio::test]
async fn update_asset_metadata_only_sets_metadata_mask() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let asset = req.asset.as_ref().expect("asset present");
            let mask = req.update_mask.as_ref().expect("mask present");
            asset.asset_id == "a2"
                && asset.tags.is_empty()
                && asset.metadata.len() == 1
                && asset
                    .metadata
                    .first()
                    .and_then(|v| v.key.as_ref().map(|k| k.name.as_str()))
                    == Some("mission")
                && mask.paths == vec!["metadata".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateAssetResponse { asset: req.asset }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = service
        .update_asset(
            "a2".to_string(),
            None,
            Some(vec![string_metadata("mission", "varda-m6")]),
        )
        .await
        .expect("update_asset failed");

    assert_eq!(updated.asset_id, "a2");
    assert_eq!(updated.metadata.len(), 1);
}

#[tokio::test]
async fn update_asset_both_fields_sets_both_mask_paths_in_order() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .times(1)
        .withf(|req| {
            let mask = req.get_ref().update_mask.as_ref().expect("mask present");
            mask.paths == vec!["tags".to_string(), "metadata".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateAssetResponse { asset: req.asset }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_asset(
            "a3".to_string(),
            Some(vec!["x".into()]),
            Some(vec![string_metadata("mission", "varda-m6")]),
        )
        .await
        .expect("update_asset failed");
}

#[tokio::test]
async fn update_asset_empty_tags_clears_tags_with_mask_set() {
    // Passing Some(vec![]) explicitly is the agent's way of clearing all tags.
    // The mask must still include `tags` so the server replaces the field.
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let asset = req.asset.as_ref().expect("asset present");
            let mask = req.update_mask.as_ref().expect("mask present");
            asset.tags.is_empty() && mask.paths == vec!["tags".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateAssetResponse { asset: req.asset }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_asset("a4".to_string(), Some(vec![]), None)
        .await
        .expect("update_asset failed");
}

#[tokio::test]
async fn update_asset_no_fields_sends_empty_mask() {
    // Tool handler validates this case before calling the service, but the
    // service contract is to send whatever the caller provided. Verify the
    // request carries an empty mask and an asset with only the id set.
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let asset = req.asset.as_ref().expect("asset present");
            let mask = req.update_mask.as_ref().expect("mask present");
            asset.asset_id == "a5"
                && asset.tags.is_empty()
                && asset.metadata.is_empty()
                && mask.paths.is_empty()
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateAssetResponse { asset: req.asset }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_asset("a5".to_string(), None, None)
        .await
        .expect("update_asset failed");
}

#[tokio::test]
async fn update_asset_propagates_grpc_error() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .returning(|_| Err(Status::not_found("asset missing")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_asset("missing".to_string(), Some(vec!["x".into()]), None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to update asset"));
}

#[tokio::test]
async fn update_asset_errors_when_response_missing_asset() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .returning(|_| Ok(Response::new(UpdateAssetResponse { asset: None })));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_asset("a6".to_string(), Some(vec!["x".into()]), None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("update_asset response missing asset"));
}
