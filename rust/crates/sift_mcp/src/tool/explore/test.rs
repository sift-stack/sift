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

#[test]
fn tool_description_explains_explore_url_millisecond_precision() {
    let tools = SiftMcpServer::explore_router().list_all();
    let tool = tools
        .iter()
        .find(|tool| tool.name == "explore_url")
        .expect("explore_url is routed");
    let description = tool
        .description
        .as_deref()
        .expect("explore_url has a description");

    assert!(
        description.contains("truncates each bound to the whole millisecond"),
        "the description must say the URL truncates to milliseconds"
    );
    assert!(
        description.contains("Truncate to whole milliseconds yourself before calling"),
        "the description must tell the agent to pre-truncate its nanos"
    );
    assert!(
        description.contains("report the link's time window to the user, report those echoed"),
        "the description must tell the agent to report the echoed bounds"
    );
}

#[test]
fn truncate_to_millis_floors_to_the_millisecond_the_url_encodes() {
    assert_eq!(
        truncate_to_millis(1_788_462_000_333_333_333),
        1_788_462_000_333_000_000
    );
    assert_eq!(
        truncate_to_millis(1_788_462_000_666_666_800),
        1_788_462_000_666_000_000
    );
    assert_eq!(truncate_to_millis(1_000_000), 1_000_000);
    assert_eq!(truncate_to_millis(0), 0);
    // Negative instants floor toward negative infinity, like chrono's sub-second field.
    assert_eq!(truncate_to_millis(-1), -1_000_000);
}

#[tokio::test]
async fn handler_echoes_millisecond_truncated_bounds_matching_the_url() {
    let server = server_for_explore(APP_URI).await;
    let params = ExploreUrlParams {
        asset_ids: None,
        run_ids: Some(vec![String::from("run-id")]),
        channels: Some(vec![String::from("L1:temperature")]),
        panel_type: None,
        start_time_unix_nanos: Some(1_788_462_000_333_333_333),
        end_time_unix_nanos: Some(1_788_462_000_666_666_666),
        include_assets_and_runs: None,
    };

    let result = server.explore_url(Parameters(params)).await.unwrap();

    let url = structured_field(result.clone(), "url");
    let url = url.as_str().unwrap();
    assert!(
        url.contains("startTime=2026-09-03T19:00:00.333Z"),
        "URL should truncate start to millis, got {url}"
    );
    assert!(
        url.contains("endTime=2026-09-03T19:00:00.666Z"),
        "URL should truncate end to millis, got {url}"
    );

    assert_eq!(
        structured_field(result.clone(), "start_time_unix_nanos").as_i64(),
        Some(1_788_462_000_333_000_000)
    );
    assert_eq!(
        structured_field(result.clone(), "end_time_unix_nanos").as_i64(),
        Some(1_788_462_000_666_000_000)
    );

    let next_step = structured_field(result, "next_step");
    let next_step = next_step.as_str().unwrap();
    assert!(
        next_step.contains("start_time_unix_nanos=1788462000333000000"),
        "next_step should quote the truncated start, got {next_step}"
    );
    assert!(
        next_step.contains("end_time_unix_nanos=1788462000666000000"),
        "next_step should quote the truncated end, got {next_step}"
    );
}

#[tokio::test]
async fn handler_omits_time_bounds_when_no_window_was_given() {
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
    let structured = result.structured_content.expect("structured content");
    assert!(structured.get("start_time_unix_nanos").is_none());
    assert!(structured.get("end_time_unix_nanos").is_none());
    assert!(
        !result.content.is_empty(),
        "text content should still carry the next_step"
    );
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
