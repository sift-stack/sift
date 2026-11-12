use super::*;
use crate::tool::common::bulk::BulkItems;
use serde_json::{Value, json};

fn params(
    items: Value,
    selected: Option<Vec<usize>>,
) -> Parameters<BulkItems<super::super::UpdateRunItem>> {
    Parameters(
        serde_json::from_value(json!({"items": items, "selected_indices": selected})).unwrap(),
    )
}

#[tokio::test]
async fn selected_subset_only_updates_selected_runs() {
    let mut mock = MockRunServiceImpl::new();
    mock.expect_update_run()
        .times(2)
        .withf(|req| req.get_ref().run.as_ref().unwrap().run_id != "skip")
        .returning(|req| {
            Ok(Response::new(UpdateRunResponse {
                run: req.into_inner().run,
            }))
        });
    let (server, _h) = server_with_mock(mock).await;
    let body = structured(server.update_run(params(json!([
        {"run_id":"a","name":"first"}, {"run_id":"skip","name":"second"}, {"run_id":"c","name":"third"}
    ]), Some(vec![0,2]))).await.unwrap());
    assert_eq!(body["updated_count"], 2);
    assert_eq!(body["not_selected"], json!([1]));
    assert_eq!(body["successes"][1]["index"], 2);
    assert!(
        body["successes"][0]["result"]["run_url"]
            .as_str()
            .unwrap()
            .contains("/run/a")
    );
}

#[tokio::test]
async fn item_errors_continue_but_unavailable_stops_the_tail() {
    for code in [
        tonic::Code::NotFound,
        tonic::Code::PermissionDenied,
        tonic::Code::Unavailable,
    ] {
        let mut mock = MockRunServiceImpl::new();
        mock.expect_update_run()
            .withf(|req| req.get_ref().run.as_ref().unwrap().run_id == "bad")
            .returning(move |_| Err(Status::new(code, "failed")));
        if code != tonic::Code::Unavailable {
            mock.expect_update_run()
                .times(1)
                .withf(|req| req.get_ref().run.as_ref().unwrap().run_id == "good")
                .returning(|req| {
                    Ok(Response::new(UpdateRunResponse {
                        run: req.into_inner().run,
                    }))
                });
        }
        let (server, _h) = server_with_mock(mock).await;
        let result = server
            .update_run(params(
                json!([{"run_id":"bad","name":"x"},{"run_id":"good","name":"y"}]),
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
    let (server, _h) = server_with_mock(MockRunServiceImpl::new()).await;
    for items in [
        json!([{"run_id":"a","name":"x"},{"run_id":"a","name":"y"}]),
        json!([{"run_id":"a","name":"x"},{"run_id":" " ,"name":"y"}]),
        json!([{"run_id":"a","name":"x"},{"run_id":"b"}]),
        json!([{"run_id":"a","name":"x","unknown":true}]),
    ] {
        assert_eq!(
            server
                .update_run(params(items, None))
                .await
                .unwrap_err()
                .code,
            ErrorCode::INVALID_PARAMS
        );
    }
    let error = server
        .with_require_selection(true)
        .update_run(params(json!([{"run_id":"a","name":"x"}]), None))
        .await
        .unwrap_err();
    assert!(error.message.contains("approval host"));
}
