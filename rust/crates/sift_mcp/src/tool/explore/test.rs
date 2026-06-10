use rmcp::handler::server::wrapper::Parameters;
use serde_json::Value;
use sift_test_util::grpc::memory_sift_channel;

use super::*;
use crate::server::SiftMcpServer;

const REST_URI: &str = "https://api.siftstack.com";

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

#[tokio::test]
async fn handler_returns_structured_url_and_text_content() {
    let server = server_for_explore(REST_URI).await;
    let params = ExploreUrlParams {
        assets: Some(vec![String::from("Engine-7")]),
        runs: None,
        channels: None,
        panel_type: None,
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
        explore_host: None,
    };

    let result = server.explore_url(Parameters(params)).await.unwrap();
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
