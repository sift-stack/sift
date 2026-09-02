use rmcp::handler::server::wrapper::Parameters;
use serde_json::Value;
use sift_test_util::grpc::memory_sift_channel;

use super::*;
use crate::server::SiftMcpServer;

const APP_URI: &str = "https://app.siftstack.com";

async fn server_for_explore(app_uri: &str) -> SiftMcpServer {
    let (client, _server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    SiftMcpServer::new(channel, app_uri.to_string(), true, true)
}

fn structured_field(result: rmcp::model::CallToolResult, key: &str) -> Value {
    let mut value = result
        .structured_content
        .expect("expected structured content");
    value
        .get_mut(key)
        .unwrap_or_else(|| panic!("missing key `{key}` in structured content"))
        .take()
}

#[test]
fn schema_requires_source_ids() {
    let schema = serde_json::to_value(schemars::schema_for!(ExploreUrlParams)).unwrap();
    let properties = schema["properties"].as_object().unwrap();

    assert!(properties.contains_key("asset_ids"));
    assert!(properties.contains_key("run_ids"));
    assert!(!properties.contains_key("assets"));
    assert!(!properties.contains_key("runs"));
}

#[tokio::test]
async fn handler_returns_structured_url_and_text_content() {
    let server = server_for_explore(APP_URI).await;
    let params = ExploreUrlParams {
        asset_ids: Some(vec![String::from("asset-id")]),
        run_ids: None,
        channels: None,
        panel_type: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        include_assets_and_runs: None,
    };

    let result = server.explore_url(Parameters(params)).await.unwrap();
    let expected_url = "https://app.siftstack.com/explore?method=single&assets=asset-id";

    let url = structured_field(result.clone(), "url");
    assert_eq!(url.as_str(), Some(expected_url));

    let next_step = structured_field(result.clone(), "next_step");
    assert!(
        next_step.as_str().is_some_and(|s| s.contains(expected_url)),
        "next_step should embed the URL verbatim, got {next_step}"
    );

    assert_eq!(
        result.content.len(),
        1,
        "expected one ContentBlock::text wrapping the next_step"
    );
}

#[tokio::test]
async fn handler_rejects_assets_and_runs_without_the_opt_in() {
    let server = server_for_explore(APP_URI).await;
    let params = ExploreUrlParams {
        asset_ids: Some(vec![String::from("asset-id")]),
        run_ids: Some(vec![String::from("run-id")]),
        channels: None,
        panel_type: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        include_assets_and_runs: None,
    };

    let err = server.explore_url(Parameters(params)).await.unwrap_err();
    assert_eq!(err.code.0, -32602);
    assert!(
        err.message.contains("include_assets_and_runs"),
        "the error should name the opt-in, got `{}`",
        err.message
    );
}

#[tokio::test]
async fn handler_keeps_both_source_types_when_the_opt_in_is_set() {
    let server = server_for_explore(APP_URI).await;
    let params = ExploreUrlParams {
        asset_ids: Some(vec![String::from("asset-id")]),
        run_ids: Some(vec![String::from("run-id")]),
        channels: None,
        panel_type: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        include_assets_and_runs: Some(true),
    };

    let result = server.explore_url(Parameters(params)).await.unwrap();
    assert_eq!(
        structured_field(result, "url").as_str(),
        Some("https://app.siftstack.com/explore?method=single&assets=asset-id&runs=run-id")
    );
}
