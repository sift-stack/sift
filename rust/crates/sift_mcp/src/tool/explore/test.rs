use rmcp::handler::server::wrapper::Parameters;
use serde_json::Value;
use sift_test_util::grpc::memory_sift_channel;

use super::*;
use crate::{
    server::SiftMcpServer,
    service::{
        declarative::{
            DeclarativeService,
            test::{Reply, fast_retry, http_response, start_sequence_server},
        },
        remote_files::RestConfig,
    },
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

const YAML_CHART: &str = "chart:\n  type: timeseries\n  title: Motor current\n  series:\n    - channel: a\n    - channel: b\n";
const MIGRATED_CHART: &str = "chart:\n  type: timeseries\n  title: Motor current\n  series: []\n";

/// Serves one canned response per connection, in order, and returns every request as text.
async fn serve(responses: Vec<Vec<u8>>) -> (String, tokio::task::JoinHandle<Vec<String>>) {
    let (rest_uri, server) =
        start_sequence_server(responses.into_iter().map(Reply::Respond).collect()).await;
    let requests = tokio::spawn(async move {
        server
            .await
            .unwrap()
            .into_iter()
            .map(|request| String::from_utf8(request).unwrap())
            .collect()
    });
    (rest_uri, requests)
}

fn request_body(request: &str) -> Value {
    serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap()
}

/// A successful migrate response with no warnings.
fn migrate_ok(migrated_spec: &str) -> Vec<u8> {
    http_response(
        "200 OK",
        &migrate_body(migrated_spec, serde_json::json!([])),
    )
}

fn migrate_body(migrated_spec: &str, warnings: Value) -> String {
    serde_json::json!({
        "originalSpec": "ignored",
        "migratedSpec": migrated_spec,
        "sourceVersion": "DECLARATIVE_SCHEMA_VERSION_V0",
        "targetVersion": "DECLARATIVE_SCHEMA_VERSION_V1",
        "warnings": warnings,
        "valid": true,
    })
    .to_string()
}

/// A successful share-link response carrying `short_link`.
fn share_link_ok(short_link: &str) -> Vec<u8> {
    http_response(
        "200 OK",
        &serde_json::json!({ "shortLink": short_link }).to_string(),
    )
}

async fn server_with_rest(rest_uri: String) -> SiftMcpServer {
    let mut server = server_for_explore(APP_URI).await;
    server.declarative_service =
        DeclarativeService::new(Some(RestConfig::new(rest_uri, "test-key".into())), "1.2.3")
            .with_retry(fast_retry());
    server
}

async fn create_chart(server: &SiftMcpServer, spec: &str) -> rmcp::model::CallToolResult {
    server
        .create_declarative_chart(Parameters(CreateDeclarativeChartParams {
            spec: spec.to_string(),
        }))
        .await
        .expect("invalid specs are results, not MCP errors")
}

fn structured(result: &rmcp::model::CallToolResult) -> Value {
    result
        .structured_content
        .clone()
        .expect("expected structured content")
}

fn assert_invalid_spec(result: &rmcp::model::CallToolResult, path: &str, message: &str) {
    let structured = structured(result);
    assert_eq!(structured["valid"], false, "{structured}");
    assert_eq!(structured["reason"], "invalid_spec", "{structured}");
    let issues = structured["issues"].as_array().unwrap();
    assert_eq!(issues.len(), 1, "{structured}");
    assert_eq!(issues[0]["path"], path, "{structured}");
    let actual = issues[0]["message"].as_str().unwrap();
    assert!(actual.contains(message), "{actual}");
    let next_step = structured["next_step"].as_str().unwrap();
    assert!(
        next_step.contains("resend the complete spec"),
        "{next_step}"
    );
    assert!(next_step.contains('3'), "{next_step}");
    assert_eq!(result.content.len(), 1, "content should carry the issues");
}

#[test]
fn create_declarative_chart_schema_is_one_flat_spec_string() {
    let schema = serde_json::to_value(schemars::schema_for!(CreateDeclarativeChartParams)).unwrap();
    let properties = schema["properties"].as_object().unwrap();

    assert_eq!(properties.len(), 1);
    assert_eq!(properties["spec"]["type"], "string");
}

#[test]
fn create_declarative_chart_is_registered_as_an_additive_write() {
    let tools = SiftMcpServer::explore_router().list_all();
    let tool = tools
        .iter()
        .find(|tool| tool.name == "create_declarative_chart")
        .expect("create_declarative_chart is routed");
    let annotations = tool.annotations.as_ref().expect("annotations are set");

    assert_eq!(
        annotations.title.as_deref(),
        Some("explore/create_declarative_chart")
    );
    assert_eq!(annotations.read_only_hint, Some(false));
    assert_eq!(annotations.destructive_hint, Some(false));
    assert_eq!(annotations.idempotent_hint, Some(false));

    let description = tool.description.as_deref().expect("has a description");
    for needle in [
        "list_runs",
        "list_channels",
        "list_calculated_channels",
        "calculatedChannelId",
        "exactly one chart displays inline",
        "After 3 failed attempts",
        "`maxGap`",
        "`exploreUrl`",
        "Do not paste the spec",
    ] {
        assert!(description.contains(needle), "missing `{needle}`");
    }
}

#[test]
fn explore_url_description_defers_charting_requests() {
    let tools = SiftMcpServer::explore_router().list_all();
    let tool = tools
        .iter()
        .find(|tool| tool.name == "explore_url")
        .expect("explore_url is routed");
    let description = tool.description.as_deref().expect("has a description");

    assert!(description.contains("call `create_declarative_chart` instead"));
    assert!(description.contains("belong to"));
}

#[test]
fn create_declarative_chart_has_a_client_event() {
    assert_eq!(
        crate::client_event::event_for_tool("create_declarative_chart"),
        Some("CLIENT_EVENT_USER_CALLED_MCP_TOOL_CREATE_DECLARATIVE_CHART")
    );
}

#[tokio::test]
async fn a_migrate_error_is_an_invalid_spec_result() {
    let (rest_uri, server_task) = serve(vec![http_response(
        "200 OK",
        r#"{"errors":[{"path":"/chart/series/0","message":"cannot migrate"},{"path":"","message":"unknown version"}]}"#,
    )])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    let requests = server_task.await.unwrap();

    assert_eq!(requests.len(), 1, "validate ran after a failed migrate");
    assert!(requests[0].starts_with("POST /api/v1/declarative:migrate "));
    let structured = structured(&result);
    assert_eq!(structured["valid"], false);
    assert_eq!(structured["reason"], "invalid_spec");
    assert_eq!(
        structured["issues"],
        serde_json::json!([
            {"path": "/chart/series/0", "message": "cannot migrate"},
            {"path": "/", "message": "unknown version"},
        ])
    );
    assert!(
        structured["next_step"]
            .as_str()
            .unwrap()
            .contains("resend the complete spec")
    );
}

#[tokio::test]
async fn a_validate_error_is_an_invalid_spec_result() {
    let (rest_uri, server_task) = serve(vec![
        migrate_ok(MIGRATED_CHART),
        http_response(
            "200 OK",
            r#"{"errors":[{"path":"/chart/series/0/channel","message":"is required"}]}"#,
        ),
    ])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    let requests = server_task.await.unwrap();

    assert!(requests[1].starts_with("POST /api/v1/declarative:validate "));
    assert_eq!(
        request_body(&requests[1]),
        serde_json::json!({"spec": MIGRATED_CHART, "strict": true}),
        "validate must receive the migrated spec in strict mode"
    );
    assert_invalid_spec(&result, "/chart/series/0/channel", "is required");
}

#[tokio::test]
async fn a_migrate_400_is_an_invalid_spec_result() {
    let (rest_uri, server_task) = serve(vec![http_response(
        "400 Bad Request",
        r#"{"message":"spec must not be empty"}"#,
    )])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, "  ").await;
    let requests = server_task.await.unwrap();

    assert_eq!(requests.len(), 1, "a 400 must not be retried");
    assert_invalid_spec(&result, "/", "spec must not be empty");
}

#[tokio::test]
async fn a_validate_400_is_an_invalid_spec_result() {
    let (rest_uri, server_task) = serve(vec![
        migrate_ok(MIGRATED_CHART),
        http_response("400 Bad Request", r#"{"message":"spec must not be empty"}"#),
    ])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    let requests = server_task.await.unwrap();

    assert_eq!(requests.len(), 2, "a 400 must not be retried");
    assert!(requests[1].starts_with("POST /api/v1/declarative:validate "));
    assert_invalid_spec(&result, "/", "spec must not be empty");
}

#[tokio::test]
async fn a_service_error_is_a_service_unavailable_result() {
    // The retry policy makes three attempts, so each needs an answer.
    let (rest_uri, server_task) = serve(vec![
        http_response(
            "503 Service Unavailable",
            r#"{"message":"down"}"#
        );
        3
    ])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    server_task.await.unwrap();

    let structured = structured(&result);
    assert_eq!(structured["valid"], false);
    assert_eq!(structured["reason"], "service_unavailable");
    let issues = structured["issues"].as_array().unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0]["path"], "/");
    assert!(issues[0]["message"].as_str().unwrap().contains("503"));
    assert!(structured["next_step"].as_str().is_some());
    assert_eq!(result.content.len(), 1);
}

#[tokio::test]
async fn a_validate_service_error_is_a_service_unavailable_result() {
    let mut responses = vec![migrate_ok(MIGRATED_CHART)];
    responses.extend(vec![
        http_response(
            "500 Internal Server Error",
            r#"{"message":"boom"}"#
        );
        3
    ]);
    let (rest_uri, server_task) = serve(responses).await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    server_task.await.unwrap();

    assert_eq!(structured(&result)["reason"], "service_unavailable");
}

#[tokio::test]
async fn a_share_link_service_error_is_a_service_unavailable_result() {
    let mut responses = vec![
        migrate_ok(MIGRATED_CHART),
        http_response("200 OK", r#"{"valid":true}"#),
    ];
    responses.extend(vec![
        http_response(
            "502 Bad Gateway",
            r#"{"message":"upstream"}"#
        );
        3
    ]);
    let (rest_uri, server_task) = serve(responses).await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    let requests = server_task.await.unwrap();

    assert!(requests[2].starts_with("POST /api/v1/declarative:sharelink "));
    let structured = structured(&result);
    assert_eq!(structured["valid"], false, "{structured}");
    assert_eq!(structured["reason"], "service_unavailable");
    assert_eq!(structured["issues"][0]["path"], "/");
    assert!(
        structured["issues"][0]["message"]
            .as_str()
            .unwrap()
            .contains("502")
    );
    assert!(structured.get("shortLink").is_none());
    assert!(structured.get("exploreUrl").is_none());
}

#[tokio::test]
async fn a_rejected_share_link_is_an_invalid_spec_result() {
    let (rest_uri, server_task) = serve(vec![
        migrate_ok(MIGRATED_CHART),
        http_response("200 OK", r#"{"valid":true}"#),
        http_response("400 Bad Request", r#"{"message":"spec cannot be shared"}"#),
    ])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    let requests = server_task.await.unwrap();

    assert!(requests[2].starts_with("POST /api/v1/declarative:sharelink "));
    assert_invalid_spec(&result, "/", "spec cannot be shared");
    let structured = structured(&result);
    assert_eq!(
        structured["issues"],
        serde_json::json!([{"path": "/", "message": "spec cannot be shared"}])
    );
    assert!(structured.get("shortLink").is_none());
    assert!(structured.get("exploreUrl").is_none());
}

#[tokio::test]
async fn a_server_without_a_rest_endpoint_reports_service_unavailable() {
    let server = server_for_explore(APP_URI).await;

    let result = create_chart(&server, YAML_CHART).await;

    let structured = structured(&result);
    assert_eq!(structured["reason"], "service_unavailable");
    assert!(
        structured["issues"][0]["message"]
            .as_str()
            .unwrap()
            .contains("without a REST endpoint")
    );
}

const JSON_CHART: &str = r#"{"chart": {"type": "timeseries", "series": [{"channel": "a"}]}}"#;
const MULTI_CHART: &str = "charts:\n  a:\n    type: timeseries\n  b:\n    type: stat\n";
const LAYOUT_CHART: &str = "layout:\n  rows: []\nchart:\n  type: timeseries\n";

/// Keys of a success result, sorted. Chart metadata is not among them.
const SUCCESS_KEYS: [&str; 8] = [
    "exploreUrl",
    "next_step",
    "shortLink",
    "sourceVersion",
    "spec",
    "targetVersion",
    "valid",
    "warnings",
];

/// Runs `spec` against a backend that accepts it and migrates it to `migrated`. Returns the
/// structured result and the three requests the backend saw.
async fn forward(spec: &str, migrated: &str) -> (Value, Vec<String>) {
    let (rest_uri, server_task) = serve(vec![
        migrate_ok(migrated),
        http_response("200 OK", r#"{"valid":true}"#),
        share_link_ok("aB3dE9x"),
    ])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, spec).await;
    (structured(&result), server_task.await.unwrap())
}

/// The backend, not the tool, judges `spec`: migrate gets it byte-for-byte, then validate and
/// the share link get the migrated spec, and the result has the success shape.
fn assert_forwarded(spec: &str, migrated: &str, structured: &Value, requests: &[String]) {
    assert_eq!(requests.len(), 3, "{requests:?}");
    assert!(requests[0].starts_with("POST /api/v1/declarative:migrate "));
    assert!(requests[1].starts_with("POST /api/v1/declarative:validate "));
    assert!(requests[2].starts_with("POST /api/v1/declarative:sharelink "));
    assert_eq!(
        request_body(&requests[0]),
        serde_json::json!({"spec": spec}),
        "migrate must receive the spec unchanged"
    );
    assert_eq!(
        request_body(&requests[1]),
        serde_json::json!({"spec": migrated, "strict": true}),
        "validate must receive the migrated spec"
    );
    assert_eq!(
        request_body(&requests[2]),
        serde_json::json!({"spec": migrated}),
        "the share link must be created from the migrated spec"
    );

    assert_eq!(structured["valid"], true, "{structured}");
    assert_eq!(structured["spec"], migrated);
    assert_eq!(structured["shortLink"], "aB3dE9x");
    let mut keys: Vec<&str> = structured
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, SUCCESS_KEYS);
}

#[tokio::test]
async fn yaml_and_json_specs_are_forwarded_unchanged_with_the_same_success_shape() {
    let (yaml_result, yaml_requests) = forward(YAML_CHART, MIGRATED_CHART).await;
    let (json_result, json_requests) = forward(JSON_CHART, MIGRATED_CHART).await;

    assert_forwarded(YAML_CHART, MIGRATED_CHART, &yaml_result, &yaml_requests);
    assert_forwarded(JSON_CHART, MIGRATED_CHART, &json_result, &json_requests);
    assert_eq!(yaml_result, json_result);
}

#[tokio::test]
async fn a_multi_chart_spec_is_forwarded_not_rejected_locally() {
    let migrated = "charts:\n  a:\n    type: timeseries\n  b:\n    type: stat\nversion: 0\n";
    let (result, requests) = forward(MULTI_CHART, migrated).await;

    assert_forwarded(MULTI_CHART, migrated, &result, &requests);
}

#[tokio::test]
async fn a_spec_with_layout_is_forwarded_not_rejected_locally() {
    let migrated = "layout:\n  rows: []\nchart:\n  type: timeseries\nversion: 0\n";
    let (result, requests) = forward(LAYOUT_CHART, migrated).await;

    assert_forwarded(LAYOUT_CHART, migrated, &result, &requests);
}

#[tokio::test]
async fn a_success_result_carries_warnings_the_link_and_a_next_step() {
    let (rest_uri, server_task) = serve(vec![
        http_response(
            "200 OK",
            &migrate_body(
                MIGRATED_CHART,
                serde_json::json!([{"path": "/chart/series", "message": "renamed"}]),
            ),
        ),
        http_response(
            "200 OK",
            r#"{"valid":true,"spec":"normalized","warnings":[{"path":"/chart/title","message":"long title"}]}"#,
        ),
        share_link_ok("aB3dE9x"),
    ])
    .await;
    let server = server_with_rest(rest_uri).await;

    let result = create_chart(&server, YAML_CHART).await;
    let requests = server_task.await.unwrap();

    assert_eq!(
        request_body(&requests[0]),
        serde_json::json!({"spec": YAML_CHART})
    );
    let structured = structured(&result);
    assert_eq!(structured["valid"], true, "{structured}");
    assert_eq!(structured["spec"], MIGRATED_CHART);
    assert_eq!(structured["sourceVersion"], "DECLARATIVE_SCHEMA_VERSION_V0");
    assert_eq!(structured["targetVersion"], "DECLARATIVE_SCHEMA_VERSION_V1");
    assert_eq!(
        structured["warnings"],
        serde_json::json!([
            {"path": "/chart/series", "message": "renamed"},
            {"path": "/chart/title", "message": "long title"},
        ])
    );
    let next_step = structured["next_step"].as_str().unwrap();
    assert!(next_step.contains("validated"), "{next_step}");
    assert_eq!(structured["shortLink"], "aB3dE9x");
    assert_eq!(
        structured["exploreUrl"],
        "https://app.siftstack.com/share/aB3dE9x"
    );
    assert!(
        next_step.contains("https://app.siftstack.com/share/aB3dE9x"),
        "{next_step}"
    );
    assert!(next_step.contains("clickable markdown link"), "{next_step}");
    assert!(next_step.contains("Mention it once"), "{next_step}");
    assert!(next_step.contains("Do not paste the spec"), "{next_step}");
    assert!(requests[2].starts_with("POST /api/v1/declarative:sharelink "));
    assert_eq!(
        request_body(&requests[2]),
        serde_json::json!({"spec": MIGRATED_CHART}),
        "the share link must be created from the migrated spec"
    );
    assert_eq!(result.content.len(), 1);
    let content = serde_json::to_value(&result.content[0]).unwrap();
    assert_eq!(content["text"], next_step);
}
