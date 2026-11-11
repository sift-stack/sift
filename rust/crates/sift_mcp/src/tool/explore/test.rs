use rmcp::handler::server::wrapper::Parameters;
use serde_json::Value;
use sift_test_util::grpc::memory_sift_channel;

use super::*;
use crate::{
    server::SiftMcpServer,
    service::declarative::test::{http_response, request_json, service, start_sequence_server},
    tool::common::test_support::{content_texts, dir_entries},
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
    create_chart_with(server, params(spec)).await
}

/// Params with only `spec` set. Override the rest with struct update syntax.
fn params(spec: &str) -> CreateDeclarativeChartParams {
    CreateDeclarativeChartParams {
        spec: spec.into(),
        surface: None,
        output: None,
        accept_warnings: None,
    }
}

async fn create_chart_with(
    server: &SiftMcpServer,
    params: CreateDeclarativeChartParams,
) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    server.create_declarative_chart(Parameters(params)).await
}

/// A validate response with no errors or warnings.
fn valid() -> Vec<u8> {
    reply("200 OK", r#"{"valid":true}"#)
}

fn share_link(short_link: &str) -> Vec<u8> {
    reply("200 OK", &format!(r#"{{"shortLink":"{short_link}"}}"#))
}

/// The top-level keys of a JSON object, sorted.
fn keys(value: &Value) -> Vec<&str> {
    let mut keys: Vec<_> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    keys
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
    assert_eq!(properties.len(), 4);
    assert_eq!(properties["spec"]["type"], "string");
    for (optional, kind) in [
        ("surface", "string"),
        ("output", "string"),
        ("accept_warnings", "boolean"),
    ] {
        assert_eq!(
            properties[optional]["type"],
            serde_json::json!([kind, "null"]),
            "{optional}"
        );
    }
    let annotations = create_tool.annotations.unwrap();
    assert_eq!(annotations.read_only_hint, Some(true));
    assert_eq!(annotations.destructive_hint, Some(false));
    assert_eq!(annotations.idempotent_hint, Some(true));
    let description = create_tool.description.unwrap();
    assert!(description.contains("read-only policy exception"));
    assert!(description.contains("no create-access flag or confirmation required"));
    assert!(!description.contains("--allow-create"));

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
async fn chart_share_links_work_in_every_access_mode_without_enabling_other_writes() {
    let root = tempdir::TempDir::new("explore-access").unwrap();
    for (allow_create, allow_destructive) in
        [(false, false), (true, false), (false, true), (true, true)]
    {
        let (uri, _http) = start_sequence_server(vec![valid(), share_link("abc")]).await;
        let server = server_with_rest(&uri, &root, allow_create, allow_destructive).await;
        if !allow_create && !allow_destructive {
            assert!(server.require_create().is_err());
            assert!(server.require_destructive().is_err());
        }
        let result = create_chart(&server, "spec").await.unwrap();
        assert_eq!(structured(&result)["valid"], true);
    }
}

#[tokio::test]
async fn missing_app_uri_is_rejected_before_persisting_a_share_link() {
    use std::time::Duration;
    use tokio::{net::TcpListener, time::timeout};

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let uri = format!("http://{}", listener.local_addr().unwrap());
    let root = tempdir::TempDir::new("explore-no-app-uri").unwrap();
    let mut server = server_with_rest(&uri, &root, false, false).await;
    server.url_service = crate::service::url::UrlService::new(String::new());

    let error = timeout(Duration::from_secs(1), create_chart(&server, "spec"))
        .await
        .expect("app_uri must be checked before any network request")
        .unwrap_err();
    assert_eq!(error.code, rmcp::model::ErrorCode::INVALID_PARAMS);
    assert!(error.message.contains("app_uri"));
    assert!(
        timeout(Duration::from_millis(20), listener.accept())
            .await
            .is_err()
    );
}

fn structured(result: &rmcp::model::CallToolResult) -> Value {
    result.structured_content.clone().unwrap()
}

#[tokio::test]
async fn create_declarative_chart_returns_a_small_success_payload() {
    let (uri, http) = start_sequence_server(vec![valid(), share_link("a/b")]).await;
    let root = tempdir::TempDir::new("explore-ok").unwrap();
    let server = server_with_rest(&uri, &root, true, false).await;

    let result = create_chart(&server, "charts: []\n").await.unwrap();

    let structured = structured(&result);
    assert_eq!(
        keys(&structured),
        ["exploreUrl", "next_step", "shortLink", "surface", "valid"]
    );
    assert_eq!(structured["valid"], true);
    assert_eq!(structured["surface"], "explore");
    assert_eq!(structured["shortLink"], "a/b");
    assert_eq!(
        structured["exploreUrl"],
        "https://app.siftstack.com/share/a%2Fb"
    );
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("https://app.siftstack.com/share/a%2Fb"));
    // Without `output`, every surface offers a local copy.
    assert!(
        text.contains("Offer to save the spec as a local YAML file"),
        "{text}"
    );
    let requests = http.await.unwrap();
    assert_eq!(
        request_json(&requests[0]),
        serde_json::json!({ "spec": "charts: []\n", "strict": true, "surface": "DECLARATIVE_SURFACE_EXPLORE" })
    );
    let share = String::from_utf8(requests[1].clone()).unwrap();
    assert!(share.ends_with(r#"{"spec":"charts: []\n"}"#), "{share}");
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
async fn non_validation_bad_requests_are_not_reported_as_invalid_specs() {
    let root = tempdir::TempDir::new("explore-proxy-400").unwrap();
    for body in [
        "<html>The plain HTTP request was sent to HTTPS port</html>",
        r#"{"error":"Bad Request - WAF"}"#,
        r#"{"message":"proxy rejected the request"}"#,
        r#"{"code":13,"message":"proxy failure"}"#,
        r#"{"code":3,"message":null}"#,
        r#"{"code":3,"message":""}"#,
        r#"{"code":"3","message":"not a gateway status"}"#,
    ] {
        let (uri, _http) = start_sequence_server(vec![reply("400 Bad Request", body)]).await;
        let server = server_with_rest(&uri, &root, false, false).await;

        let error = create_chart(&server, "spec").await.unwrap_err();

        assert!(error.message.contains("400"), "{body}: {error}");
        assert!(
            !error.message.contains("resend the complete spec"),
            "{body}: {error}"
        );
        assert!(
            !error.message.contains("The spec is not valid"),
            "{body}: {error}"
        );
    }
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

#[tokio::test]
async fn an_invalid_spec_returns_each_error_with_its_path_and_saves_nothing() {
    let body = r#"{"errors":[{"path":"/chart/type","message":"unknown chart type"}]}"#;
    let (uri, http) = start_sequence_server(vec![reply("200 OK", body)]).await;
    let root = tempdir::TempDir::new("explore-invalid").unwrap();
    let server = server_with_rest(&uri, &root, false, false).await;

    let result = create_chart(&server, "bad").await.unwrap();

    let structured = structured(&result);
    assert_eq!(structured["valid"], false);
    assert_eq!(structured["reason"], "invalid_spec");
    assert_eq!(
        structured["issues"],
        serde_json::json!([{ "path": "/chart/type", "message": "unknown chart type" }])
    );
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("/chart/type: unknown chart type"), "{text}");
    assert_eq!(
        http.await.unwrap().len(),
        1,
        "no share link for an invalid spec"
    );
}

#[tokio::test]
async fn canvas_and_edge_check_the_spec_and_return_loading_steps_without_a_link() {
    let root = tempdir::TempDir::new("explore-surfaces").unwrap();
    for (surface, proto, step) in [
        (
            "canvas",
            "DECLARATIVE_SURFACE_CANVAS",
            "Charts > Timeseries",
        ),
        ("edge", "DECLARATIVE_SURFACE_EDGE", "Load view or spec"),
    ] {
        let (uri, http) = start_sequence_server(vec![valid()]).await;
        let mut server = server_with_rest(&uri, &root, false, false).await;
        // Canvas and Edge never build an Explore link, so they need no app URI.
        server.url_service = crate::service::url::UrlService::new(String::new());

        let result = create_chart_with(
            &server,
            CreateDeclarativeChartParams {
                surface: Some(surface.into()),
                ..params("spec")
            },
        )
        .await
        .unwrap();

        let structured = structured(&result);
        assert_eq!(
            keys(&structured),
            ["next_step", "surface", "valid"],
            "{surface}"
        );
        assert_eq!(structured["surface"], surface);
        let next_step = structured["next_step"].as_str().unwrap();
        assert!(next_step.contains(step), "{surface}: {next_step}");
        let requests = http.await.unwrap();
        assert_eq!(requests.len(), 1, "{surface}: no share link");
        assert_eq!(request_json(&requests[0])["surface"], proto);
    }
}

/// A validate response that accepts the spec with one contract warning, which every surface gets.
fn with_warnings() -> Vec<u8> {
    reply(
        "200 OK",
        r#"{"valid":true,"warnings":[{"path":"/charts/a","message":"skipped"}]}"#,
    )
}

#[tokio::test]
async fn warnings_block_delivery_on_every_surface() {
    let root = tempdir::TempDir::new("explore-warnings").unwrap();
    for surface in ["explore", "canvas", "edge"] {
        let (uri, http) = start_sequence_server(vec![with_warnings()]).await;
        let server = server_with_rest(&uri, &root, false, false).await;

        let result = create_chart_with(
            &server,
            CreateDeclarativeChartParams {
                surface: Some(surface.into()),
                ..params("spec")
            },
        )
        .await
        .unwrap();

        let structured = structured(&result);
        assert_eq!(
            keys(&structured),
            ["next_step", "reason", "surface", "valid", "warnings"],
            "{surface}"
        );
        assert_eq!(structured["reason"], "warnings", "{surface}");
        assert_eq!(
            structured["warnings"],
            serde_json::json!([{ "path": "/charts/a", "message": "skipped" }])
        );
        let next_step = structured["next_step"].as_str().unwrap();
        assert!(
            next_step.contains("nothing was delivered")
                && next_step.contains("`accept_warnings: true`"),
            "{surface}: {next_step}"
        );
        // Clients that show only text content still see the warnings.
        let text = content_texts(&result).concat();
        assert!(text.contains("skipped"), "{surface}: {text}");
        assert_eq!(http.await.unwrap().len(), 1, "{surface}: no share link");
    }
}

#[tokio::test]
async fn accepted_warnings_deliver_the_spec_and_report_them() {
    let (uri, _http) = start_sequence_server(vec![with_warnings(), share_link("abc")]).await;
    let root = tempdir::TempDir::new("explore-accept").unwrap();
    let server = server_with_rest(&uri, &root, false, false).await;
    let path = root.path().join("future.yaml");

    let blocked = create_chart(&server, "spec").await.unwrap();
    let accepted = create_chart_with(
        &server,
        CreateDeclarativeChartParams {
            output: Some(path.clone()),
            accept_warnings: Some(true),
            ..params("spec")
        },
    )
    .await
    .unwrap();

    assert_eq!(structured(&blocked)["reason"], "warnings");
    let structured = structured(&accepted);
    assert_eq!(structured["shortLink"], "abc");
    assert_eq!(
        structured["warnings"],
        serde_json::json!([{ "path": "/charts/a", "message": "skipped" }])
    );
    let next_step = structured["next_step"].as_str().unwrap();
    assert!(
        next_step.contains("does not draw the parts in `warnings` yet"),
        "{next_step}"
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "spec");
}

#[tokio::test]
async fn saving_a_delivered_spec_repeats_no_request() {
    let (uri, http) = start_sequence_server(vec![valid(), share_link("abc")]).await;
    let root = tempdir::TempDir::new("explore-save-after").unwrap();
    let server = server_with_rest(&uri, &root, false, false).await;
    let path = root.path().join("chart.yaml");

    let delivered = create_chart(&server, "spec").await.unwrap();
    let saved = create_chart_with(
        &server,
        CreateDeclarativeChartParams {
            output: Some(path.clone()),
            ..params("spec")
        },
    )
    .await
    .unwrap();

    assert_eq!(
        structured(&saved)["shortLink"],
        structured(&delivered)["shortLink"]
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "spec");
    assert_eq!(http.await.unwrap().len(), 2, "one validate, one share link");
}

#[tokio::test]
async fn output_writes_the_accepted_spec_as_sent_on_every_surface() {
    let root = tempdir::TempDir::new("explore-output").unwrap();
    let spec = "version: 0\n# comments and key order survive\nchart: {type: table}\n";
    for surface in ["explore", "canvas", "edge"] {
        let mut replies = vec![valid()];
        if surface == "explore" {
            replies.push(share_link("abc"));
        }
        let (uri, _http) = start_sequence_server(replies).await;
        let server = server_with_rest(&uri, &root, false, false).await;
        let path = root.path().join("charts").join(format!("{surface}.yaml"));

        let result = create_chart_with(
            &server,
            CreateDeclarativeChartParams {
                surface: Some(surface.into()),
                output: Some(path.clone()),
                ..params(spec)
            },
        )
        .await
        .unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), spec, "{surface}");
        let structured = structured(&result);
        let shown = path.to_string_lossy();
        assert_eq!(structured["output"], *shown, "{surface}");
        let next_step = structured["next_step"].as_str().unwrap();
        assert!(
            next_step.contains(&format!("saved at `{shown}`")),
            "{surface}: {next_step}"
        );
        assert!(
            !next_step.contains("Offer to save"),
            "{surface}: {next_step}"
        );
    }
}

#[tokio::test]
async fn an_undelivered_spec_keeps_the_existing_output_and_leaves_no_partial_file() {
    let root = tempdir::TempDir::new("explore-output-kept").unwrap();
    // Apart from the service's cache root, so only the output can appear here.
    let out = tempdir::TempDir::new("explore-output-kept-out").unwrap();
    let path = out.path().join("chart.yaml");
    std::fs::write(&path, "earlier spec").unwrap();
    let invalid = reply(
        "200 OK",
        r#"{"errors":[{"path":"/chart","message":"bad"}]}"#,
    );
    let share_link_fails = reply("500 Internal Server Error", "boom");
    for replies in [
        vec![invalid],
        vec![with_warnings()],
        vec![valid(), share_link_fails],
    ] {
        let (uri, _http) = start_sequence_server(replies).await;
        let server = server_with_rest(&uri, &root, false, false).await;

        let _ = create_chart_with(
            &server,
            CreateDeclarativeChartParams {
                output: Some(path.clone()),
                ..params("new spec")
            },
        )
        .await;

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "earlier spec");
        assert_eq!(dir_entries(&out), vec![path.clone()]);
    }
}

#[tokio::test]
async fn an_unwritable_output_fails_before_the_share_link() {
    let root = tempdir::TempDir::new("explore-output-blocked").unwrap();
    let blocker = root.path().join("blocker");
    std::fs::write(&blocker, "a file, not a directory").unwrap();
    let (uri, http) = start_sequence_server(vec![valid()]).await;
    let server = server_with_rest(&uri, &root, false, false).await;

    let result = create_chart_with(
        &server,
        CreateDeclarativeChartParams {
            output: Some(blocker.join("chart.yaml")),
            ..params("spec")
        },
    )
    .await;

    assert!(result.is_err());
    assert_eq!(http.await.unwrap().len(), 1, "no share link");
}

#[tokio::test]
async fn a_deployment_without_the_surface_still_checks_the_schema() {
    // Another surface's gap shows the deployment ignored `edge`; it must not block delivery.
    let unscoped = reply(
        "200 OK",
        r#"{"valid":true,"warnings":[{"path":"/a","message":"x","surfaces":["DECLARATIVE_SURFACE_EXPLORE"]}]}"#,
    );
    let (uri, _http) = start_sequence_server(vec![unscoped]).await;
    let root = tempdir::TempDir::new("explore-old-deployment").unwrap();
    let server = server_with_rest(&uri, &root, false, false).await;

    let result = create_chart_with(
        &server,
        CreateDeclarativeChartParams {
            surface: Some("edge".into()),
            ..params("spec")
        },
    )
    .await
    .unwrap();

    let structured = structured(&result);
    assert_eq!(structured["valid"], true);
    assert_eq!(structured["surfaceGapsChecked"], false);
    assert!(
        structured.get("reason").is_none(),
        "delivered: {structured}"
    );
    assert!(
        structured["next_step"]
            .as_str()
            .unwrap()
            .contains("checked against the schema only")
    );
}

#[tokio::test]
async fn an_unknown_surface_is_rejected_before_any_request() {
    let root = tempdir::TempDir::new("explore-bad-surface").unwrap();
    let server = server_with_rest("http://127.0.0.1:1", &root, false, false).await;

    let error = create_chart_with(
        &server,
        CreateDeclarativeChartParams {
            surface: Some("agents".into()),
            ..params("spec")
        },
    )
    .await
    .unwrap_err();

    assert_eq!(error.code, rmcp::model::ErrorCode::INVALID_PARAMS);
    assert!(
        error.message.contains("explore, canvas, edge"),
        "{}",
        error.message
    );
}
