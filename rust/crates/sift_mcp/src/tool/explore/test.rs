use rmcp::handler::server::wrapper::Parameters;
use serde_json::Value;
use sift_test_util::grpc::memory_sift_channel;

use super::*;
use crate::{
    server::SiftMcpServer,
    service::declarative::test::{http_response, service, start_sequence_server},
};

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

const SCHEMA: &str = "{ \"$defs\" : { \"chart\" : {} } }\n";

/// A server whose declarative service talks to `rest_uri` and caches under `root`.
async fn server_with_rest(
    rest_uri: &str,
    root: &tempdir::TempDir,
    allow_create: bool,
    allow_destructive: bool,
) -> SiftMcpServer {
    let (client, _server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    let mut server = SiftMcpServer::new(channel, APP_URI.into(), allow_create, allow_destructive);
    server.declarative_service = service(rest_uri, root.path());
    server
}

fn reply(status: &str, body: &str) -> Vec<u8> {
    http_response(status, body)
}

async fn create_chart(
    server: &SiftMcpServer,
    spec: &str,
) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    server
        .create_declarative_chart(Parameters(CreateDeclarativeChartParams {
            spec: spec.into(),
        }))
        .await
}

fn tool_named(name: &str) -> rmcp::model::Tool {
    SiftMcpServer::explore_router()
        .list_all()
        .into_iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("{name} is routed"))
}

#[test]
fn chart_tools_have_flat_schemas_and_honest_annotations() {
    let schema_tool = tool_named("get_chart_schema");
    let properties = schema_tool.input_schema.get("properties");
    assert!(properties.is_none_or(|p| p.as_object().is_some_and(|p| p.is_empty())));
    assert_eq!(schema_tool.annotations.unwrap().read_only_hint, Some(true));

    let create_tool = tool_named("create_declarative_chart");
    let properties = create_tool.input_schema["properties"].as_object().unwrap();
    assert_eq!(properties.len(), 1);
    assert_eq!(properties["spec"]["type"], "string");
    let annotations = create_tool.annotations.unwrap();
    assert_eq!(annotations.read_only_hint, Some(false));
    assert_eq!(annotations.destructive_hint, Some(false));
    assert_eq!(annotations.idempotent_hint, Some(false));

    let explore = tool_named("explore_url");
    assert!(
        explore
            .description
            .unwrap()
            .contains("`create_declarative_chart`")
    );
}

#[tokio::test]
async fn get_chart_schema_returns_the_cached_file_path_and_small_metadata() {
    let (uri, _http) = start_sequence_server(vec![reply("200 OK", SCHEMA)]).await;
    let root = tempdir::TempDir::new("explore-schema").unwrap();
    let server = server_with_rest(&uri, &root, false, false).await;

    let result = server.get_chart_schema().await.unwrap();

    let structured = result.structured_content.unwrap();
    let keys: Vec<_> = structured.as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys, ["next_step", "schema_path", "schema_version"]);
    assert_eq!(structured["schema_version"], 0);
    let path = structured["schema_path"].as_str().unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), SCHEMA);
    assert!(result.content[0].as_text().unwrap().text.contains(path));

    let no_rest = server_for_explore(APP_URI).await;
    assert!(no_rest.get_chart_schema().await.is_err());
}

#[tokio::test]
async fn create_declarative_chart_is_gated_on_allow_create() {
    let root = tempdir::TempDir::new("explore-gate").unwrap();
    let blocked = server_with_rest("http://127.0.0.1:1", &root, false, false).await;
    let error = create_chart(&blocked, "spec").await.unwrap_err();
    assert_eq!(error.code, rmcp::model::ErrorCode::INVALID_REQUEST);
    assert!(
        error.message.contains("--allow-create"),
        "{}",
        error.message
    );

    // `allow_create` alone and `allow_destructive` alone (which implies it) both pass.
    for (allow_create, allow_destructive) in [(true, false), (false, true)] {
        let (uri, _http) =
            start_sequence_server(vec![reply("200 OK", r#"{"shortLink":"abc"}"#)]).await;
        let server = server_with_rest(&uri, &root, allow_create, allow_destructive).await;
        let result = create_chart(&server, "spec").await.unwrap();
        assert_eq!(structured(&result)["valid"], true);
    }
}

fn structured(result: &rmcp::model::CallToolResult) -> Value {
    result.structured_content.clone().unwrap()
}

#[tokio::test]
async fn create_declarative_chart_returns_a_small_success_payload() {
    let (uri, http) = start_sequence_server(vec![reply("200 OK", r#"{"shortLink":"a/b"}"#)]).await;
    let root = tempdir::TempDir::new("explore-ok").unwrap();
    let server = server_with_rest(&uri, &root, true, false).await;

    let result = create_chart(&server, "charts: []\n").await.unwrap();

    let structured = structured(&result);
    let mut keys: Vec<_> = structured.as_object().unwrap().keys().cloned().collect();
    keys.sort();
    assert_eq!(keys, ["exploreUrl", "next_step", "shortLink", "valid"]);
    assert_eq!(structured["valid"], true);
    assert_eq!(structured["shortLink"], "a/b");
    assert_eq!(
        structured["exploreUrl"],
        "https://app.siftstack.com/share/a%2Fb"
    );
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("https://app.siftstack.com/share/a%2Fb"));
    let request = String::from_utf8(http.await.unwrap().remove(0)).unwrap();
    assert!(request.ends_with(r#"{"spec":"charts: []\n"}"#), "{request}");
}

#[tokio::test]
async fn a_rejected_spec_returns_issues_to_repair() {
    let body = r#"{"code":3,"message":"charts[0].type: unknown"}"#;
    let (uri, _http) = start_sequence_server(vec![reply("400 Bad Request", body)]).await;
    let root = tempdir::TempDir::new("explore-400").unwrap();
    let server = server_with_rest(&uri, &root, true, false).await;

    let result = create_chart(&server, "bad").await.unwrap();

    let structured = structured(&result);
    assert_eq!(structured["valid"], false);
    assert_eq!(structured["reason"], "invalid_spec");
    assert_eq!(
        structured["issues"][0]["message"],
        "charts[0].type: unknown"
    );
    let next_step = structured["next_step"].as_str().unwrap();
    assert!(next_step.contains("resend the complete spec") && next_step.contains('3'));
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("charts[0].type: unknown"));
}

#[tokio::test]
async fn other_http_failures_never_suggest_repairing_the_spec() {
    let root = tempdir::TempDir::new("explore-fail").unwrap();
    for (status, stop) in [
        ("401 Unauthorized", true),
        ("403 Forbidden", true),
        ("404 Not Found", true),
        ("500 Internal Server Error", false),
        ("429 Too Many Requests", false),
    ] {
        let (uri, _http) = start_sequence_server(vec![reply(status, "boom")]).await;
        let server = server_with_rest(&uri, &root, true, false).await;

        let error = create_chart(&server, "spec").await.unwrap_err();

        let number = &status[..3];
        assert!(
            error.message.contains(number),
            "{status}: {}",
            error.message
        );
        assert!(
            !error.message.contains("resend"),
            "{status}: {}",
            error.message
        );
        assert_eq!(
            error.message.contains("Do not retry"),
            stop,
            "{status}: {}",
            error.message
        );
        assert_eq!(
            error.data.is_some_and(|d| d["status"] == "stopped"),
            stop,
            "{status}"
        );
    }

    // A transport failure is not a spec problem either.
    let server = server_with_rest("http://127.0.0.1:1", &root, true, false).await;
    let error = create_chart(&server, "spec").await.unwrap_err();
    assert!(!error.message.contains("resend"), "{}", error.message);
}
