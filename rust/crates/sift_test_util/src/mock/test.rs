use crate::mock::assets::v1::MockAssetServiceImpl;
use sift_rs::assets::v1::{
    Asset, GetAssetRequest, GetAssetResponse, ListAssetsRequest, ListAssetsResponse,
    asset_service_server::AssetService,
};
use tonic::{Code, Request, Status};

/// The simplest case: stub a single method to return a fixed response.
#[tokio::test]
async fn returns_a_canned_response() {
    let mut mock = MockAssetServiceImpl::new();

    mock.expect_get_asset().returning(|_req| {
        Ok(tonic::Response::new(GetAssetResponse {
            asset: Some(Asset {
                asset_id: "asset-1".into(),
                name: "engine".into(),
                ..Default::default()
            }),
        }))
    });

    let resp = mock
        .get_asset(Request::new(GetAssetRequest {
            asset_id: "asset-1".into(),
        }))
        .await
        .unwrap()
        .into_inner();

    let asset = resp.asset.unwrap();
    assert_eq!(asset.asset_id, "asset-1");
    assert_eq!(asset.name, "engine");
}

/// Match on request arguments with `withf(...)` so the mock only fires for
/// the expected input. A second expectation handles the "not found" case.
/// `withf` is used instead of `with(eq(...))` because `tonic::Request` does
/// not implement `PartialEq`.
#[tokio::test]
async fn matches_on_request_arguments() {
    let mut mock = MockAssetServiceImpl::new();

    mock.expect_get_asset()
        .withf(|req| req.get_ref().asset_id == "asset-1")
        .returning(|_| {
            Ok(tonic::Response::new(GetAssetResponse {
                asset: Some(Asset {
                    asset_id: "asset-1".into(),
                    name: "engine".into(),
                    ..Default::default()
                }),
            }))
        });

    mock.expect_get_asset()
        .withf(|req| req.get_ref().asset_id == "missing")
        .returning(|_| Err(Status::not_found("asset not found")));

    let found = mock
        .get_asset(Request::new(GetAssetRequest {
            asset_id: "asset-1".into(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(found.asset.unwrap().name, "engine");

    let err = mock
        .get_asset(Request::new(GetAssetRequest {
            asset_id: "missing".into(),
        }))
        .await
        .unwrap_err();
    assert_eq!(err.code(), Code::NotFound);
}

/// Assert the method was called exactly N times. The expectation panics on
/// drop if the count doesn't match, so this doubles as a usage check.
#[tokio::test]
async fn enforces_call_count() {
    let mut mock = MockAssetServiceImpl::new();

    mock.expect_list_assets().times(2).returning(|_| {
        Ok(tonic::Response::new(ListAssetsResponse {
            assets: vec![],
            next_page_token: String::new(),
        }))
    });

    for _ in 0..2 {
        mock.list_assets(Request::new(ListAssetsRequest::default()))
            .await
            .unwrap();
    }
}

/// Compute the response from the request. Useful when behavior depends on
/// input rather than being a fixed value.
#[tokio::test]
async fn computes_response_from_request() {
    let mut mock = MockAssetServiceImpl::new();

    mock.expect_get_asset().returning(|req| {
        let asset_id = req.into_inner().asset_id;
        Ok(tonic::Response::new(GetAssetResponse {
            asset: Some(Asset {
                name: format!("name-for-{asset_id}"),
                asset_id,
                ..Default::default()
            }),
        }))
    });

    let resp = mock
        .get_asset(Request::new(GetAssetRequest {
            asset_id: "abc".into(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.asset.unwrap().name, "name-for-abc");
}

/// Combine a page-aware mock with pagination on the client side. Shows how a
/// single expectation with a dynamic closure can simulate multi-page state.
#[tokio::test]
async fn simulates_pagination() {
    let mut mock = MockAssetServiceImpl::new();

    mock.expect_list_assets().returning(|req| {
        let page = req.into_inner().page_token;
        let (assets, next) = match page.as_str() {
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
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(tonic::Response::new(ListAssetsResponse {
            assets,
            next_page_token: next,
        }))
    });

    let mut ids = Vec::new();
    let mut token = String::new();
    loop {
        let resp = mock
            .list_assets(Request::new(ListAssetsRequest {
                page_token: token.clone(),
                ..Default::default()
            }))
            .await
            .unwrap()
            .into_inner();
        ids.extend(resp.assets.into_iter().map(|a| a.asset_id));
        if resp.next_page_token.is_empty() {
            break;
        }
        token = resp.next_page_token;
    }
    assert_eq!(ids, vec!["a1", "a2"]);
}
