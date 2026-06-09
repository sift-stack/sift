use rmcp::handler::server::wrapper::Parameters;
use serde_json::Value;
use sift_test_util::grpc::memory_sift_channel;

use super::*;
use crate::server::SiftMcpServer;

const REST_URI: &str = "https://api.siftstack.com";

fn empty_params() -> ExploreUrlParams {
    ExploreUrlParams {
        assets: None,
        runs: None,
        channels: None,
        panel_type: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        explore_host: None,
    }
}

async fn server_for_explore(rest_uri: &str) -> SiftMcpServer {
    let (client, _server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    SiftMcpServer::new(channel, rest_uri.to_string())
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
fn full_url_with_all_params() {
    let mut p = empty_params();
    p.assets = Some(vec![String::from("Engine-7")]);
    p.runs = Some(vec![String::from("2025-thrust-test")]);
    p.channels = Some(vec![String::from("temperature"), String::from("pressure")]);
    p.panel_type = Some(String::from("scatter-plot"));
    p.start_time_unix_nanos = Some(0);
    p.end_time_unix_nanos = Some(1_700_000_000_000_000_000);
    let url = build_explore_url(REST_URI, p).unwrap();
    assert_eq!(
        url,
        "https://app.siftstack.com/explore?method=single\
         &assets=Engine-7\
         &runs=2025-thrust-test\
         &channels=temperature,pressure\
         &panelType=scatter-plot\
         &startTime=1970-01-01T00:00:00.000Z\
         &endTime=2023-11-14T22:13:20.000Z"
    );
}

#[test]
fn axis_prefix_colon_is_preserved() {
    let mut p = empty_params();
    p.channels = Some(vec![
        String::from("L1:temperature"),
        String::from("L2:pressure"),
    ]);
    let url = build_explore_url(REST_URI, p).unwrap();
    assert!(
        url.contains("&channels=L1:temperature,L2:pressure"),
        "got {url}"
    );
}

#[test]
fn comma_inside_single_value_is_encoded() {
    let mut p = empty_params();
    p.channels = Some(vec![String::from("weird,name")]);
    let url = build_explore_url(REST_URI, p).unwrap();
    assert!(url.contains("&channels=weird%2Cname"), "got {url}");
}

#[test]
fn unknown_panel_type_is_rejected() {
    let mut p = empty_params();
    p.assets = Some(vec![String::from("a")]);
    p.panel_type = Some(String::from("bogus"));
    let err = build_explore_url(REST_URI, p).unwrap_err();
    assert_eq!(err.code.0, -32602);
    assert!(err.message.contains("bogus"), "got `{}`", err.message);
}

#[test]
fn empty_request_is_rejected() {
    let err = build_explore_url(REST_URI, empty_params()).unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn empty_vecs_are_treated_as_missing() {
    let mut p = empty_params();
    p.assets = Some(vec![]);
    p.runs = Some(vec![]);
    p.channels = Some(vec![]);
    let err = build_explore_url(REST_URI, p).unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn host_derivation_strips_rest_uri_path() {
    let mut p = empty_params();
    p.assets = Some(vec![String::from("a")]);
    let url = build_explore_url("https://api.siftstack.com/v1", p).unwrap();
    assert!(
        url.starts_with("https://app.siftstack.com/explore?"),
        "got {url}"
    );
}

#[test]
fn unsupported_rest_uri_without_explore_host_errors() {
    let mut p = empty_params();
    p.assets = Some(vec![String::from("a")]);
    let err = build_explore_url("https://my-self-hosted.example", p).unwrap_err();
    assert_eq!(err.code.0, -32602);
    assert!(
        err.message.contains("explore_host"),
        "expected guidance to point at explore_host, got `{}`",
        err.message
    );
}

#[tokio::test]
async fn handler_returns_structured_url_and_text_content() {
    let server = server_for_explore(REST_URI).await;
    let mut p = empty_params();
    p.assets = Some(vec![String::from("Engine-7")]);

    let result = server.explore_url(Parameters(p)).await.unwrap();
    let expected_url = "https://app.siftstack.com/explore?method=single&assets=Engine-7";

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
        "expected one Content::text wrapping the next_step"
    );
}
