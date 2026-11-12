use super::*;
use crate::tool::common::{bulk::BulkItems, test_support::structured};
use serde_json::{Value, json};
use sift_rs::assets::v1::UpdateAssetResponse;

fn params(
    items: Value,
    selected: Option<Vec<usize>>,
) -> Parameters<BulkItems<super::super::UpdateAssetItem>> {
    Parameters(
        serde_json::from_value(json!({"items": items, "selected_indices": selected})).unwrap(),
    )
}

#[tokio::test]
async fn selected_subset_only_updates_selected_assets() {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_update_asset()
        .times(2)
        .withf(|req| req.get_ref().asset.as_ref().unwrap().asset_id != "skip")
        .returning(|req| {
            Ok(Response::new(UpdateAssetResponse {
                asset: req.into_inner().asset,
            }))
        });
    let (server, _h) = server_with_mock(mock).await;
    let body = structured(server.update_asset(params(json!([
        {"asset_id":"a","tags":[]}, {"asset_id":"skip","tags":[]}, {"asset_id":"c","tags":[]}
    ]), Some(vec![0,2]))).await.unwrap());
    assert_eq!(body["updated_count"], 2);
    assert_eq!(body["not_selected"], json!([1]));
    assert_eq!(body["successes"][1]["index"], 2);
    assert!(
        body["successes"][0]["result"]["asset_url"]
            .as_str()
            .unwrap()
            .contains("/asset/a")
    );
}

#[tokio::test]
async fn item_errors_continue_but_unavailable_stops_the_tail() {
    for code in [
        tonic::Code::NotFound,
        tonic::Code::PermissionDenied,
        tonic::Code::Unavailable,
    ] {
        let mut mock = MockAssetServiceImpl::new();
        mock.expect_update_asset()
            .withf(|req| req.get_ref().asset.as_ref().unwrap().asset_id == "bad")
            .returning(move |_| Err(Status::new(code, "failed")));
        if code != tonic::Code::Unavailable {
            mock.expect_update_asset()
                .times(1)
                .withf(|req| req.get_ref().asset.as_ref().unwrap().asset_id == "good")
                .returning(|req| {
                    Ok(Response::new(UpdateAssetResponse {
                        asset: req.into_inner().asset,
                    }))
                });
        }
        let (server, _h) = server_with_mock(mock).await;
        let result = server
            .update_asset(params(
                json!([{"asset_id":"bad","tags":[]},{"asset_id":"good","tags":[]}]),
                None,
            ))
            .await
            .unwrap();
        assert_eq!(result.is_error, Some(true));
        let body = structured(result);
        assert_eq!(body["failures"].as_array().unwrap().len(), 1);
        assert_eq!(
            body["not_attempted"].as_array().unwrap().len(),
            usize::from(code == tonic::Code::Unavailable)
        );
        assert_eq!(
            body["updated_count"],
            usize::from(code != tonic::Code::Unavailable)
        );
    }
}

#[tokio::test]
async fn invalid_calls_and_missing_chat_masks_write_nothing() {
    let (server, _h) = server_with_mock(MockAssetServiceImpl::new()).await;
    for items in [
        json!([{"asset_id":"a","tags":[]},{"asset_id":"a","tags":[]}]),
        json!([{"asset_id":"a","tags":[]},{"asset_id":" ","tags":[]}]),
        json!([{"asset_id":"a","tags":[]},{"asset_id":"b"}]),
        json!([{"asset_id":"a","tags":[],"unknown":true}]),
    ] {
        assert_eq!(
            server
                .update_asset(params(items, None))
                .await
                .unwrap_err()
                .code,
            ErrorCode::INVALID_PARAMS
        );
    }
    let error = server
        .with_require_selection(true)
        .update_asset(params(json!([{"asset_id":"a","tags":[]}]), None))
        .await
        .unwrap_err();
    assert!(error.message.contains("approval host"));
}
