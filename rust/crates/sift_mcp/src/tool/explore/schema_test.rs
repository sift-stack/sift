use rmcp::handler::server::wrapper::Parameters;

use super::test::{serve, server_with_rest, structured};
use super::*;
use crate::service::declarative::test::http_response;

#[test]
fn get_chart_schema_is_read_only_with_only_an_optional_definitions_parameter() {
    let tools = SiftMcpServer::explore_router().list_all();
    let tool = tools
        .iter()
        .find(|tool| tool.name == "get_chart_schema")
        .unwrap();
    let annotations = tool.annotations.as_ref().unwrap();
    assert_eq!(annotations.read_only_hint, Some(true));
    assert_eq!(
        annotations.title.as_deref(),
        Some("explore/get_chart_schema")
    );
    let schema = serde_json::to_value(schemars::schema_for!(GetChartSchemaParams)).unwrap();
    assert_eq!(schema["properties"].as_object().unwrap().len(), 1);
    assert!(schema["properties"].get("definitions").is_some());
    assert!(schema.get("required").is_none());
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        crate::client_event::event_for_tool("get_chart_schema"),
        Some("CLIENT_EVENT_USER_CALLED_MCP_TOOL_GET_CHART_SCHEMA")
    );
    for params in [
        serde_json::json!({"full": true}),
        serde_json::json!({"version": "0"}),
    ] {
        assert!(serde_json::from_value::<GetChartSchemaParams>(params).is_err());
    }
}

#[tokio::test]
async fn get_chart_schema_without_parameters_returns_an_index() {
    let (rest_uri, server_task) =
        serve(crate::service::declarative::test::chart_schema_responses()).await;
    let server = server_with_rest(rest_uri).await;
    let params = serde_json::from_value(serde_json::json!({})).unwrap();

    let result = server.get_chart_schema(Parameters(params)).await.unwrap();
    let fixture = crate::service::declarative::test::chart_schema_fixture();
    let snapshot = server.declarative_service.chart_schema().await.unwrap();
    assert!(snapshot.path.is_absolute());
    assert_eq!(
        tokio::fs::read(&snapshot.path).await.unwrap(),
        fixture.to_string().as_bytes()
    );
    let expected = serde_json::json!({
        "$id": "https://example.test/chart.schema.json",
        "title": "Declarative chart",
        "summary": "A chart specification.",
        "schema_path": snapshot.path,
        "root": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "required": ["version"],
            "additionalProperties": false,
            "properties": fixture["properties"],
            "if": {"required": ["chart"]},
            "then": {"not": {"required": ["charts"]}, "properties": {"layout": false}}
        },
        "references": ["chart", "layout"],
        "definitions": {
            "chart": "A chart with channels.",
            "channelIdBinding": "A channel ID.",
            "axis": "An axis scale.",
            "scalar": "",
            "layout": "A layout grid."
        },
        "save_command": "sift-cli get chart-schema -o <path>"
    });
    assert_eq!(structured(&result), expected);
    let content = serde_json::to_value(&result.content[0]).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(content["text"].as_str().unwrap()).unwrap(),
        expected
    );
    let requests = server_task.await.unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests[0].starts_with("GET /api/packages/v1/schemas/declarative-chart/versions "));
    assert!(
        requests[1].starts_with(
            "GET /download/schemas/declarative-chart/0/sift.declarative.schema.v0.json "
        )
    );
}

#[tokio::test]
async fn get_chart_schema_index_includes_root_constraints_and_sorted_references() {
    let (rest_uri, server_task) =
        serve(crate::service::declarative::test::chart_schema_responses()).await;
    let server = server_with_rest(rest_uri).await;
    let params = GetChartSchemaParams { definitions: None };

    let result = server.get_chart_schema(Parameters(params)).await.unwrap();
    let sc = structured(&result);

    // Root is nested under "root", not merged into the top level.
    let root = sc["root"].as_object().expect("root is an object");
    assert_eq!(root["type"], "object", "root type");
    assert_eq!(
        root["required"],
        serde_json::json!(["version"]),
        "root required"
    );
    assert_eq!(
        root["additionalProperties"], false,
        "root additionalProperties"
    );
    assert!(root.get("if").is_some(), "root if clause");
    let then = root["then"].as_object().expect("root then clause");
    assert_eq!(
        then["properties"]["layout"], false,
        "layout: false mutual exclusion in then.properties"
    );
    // $defs must not appear anywhere in the index.
    assert!(
        sc.get("$defs").is_none(),
        "$defs must not appear at index top level"
    );
    assert!(
        root.get("$defs").is_none(),
        "$defs must not appear inside root"
    );

    // References are sorted and deduplicated.
    let refs = sc["references"].as_array().expect("references is an array");
    let ref_names: Vec<&str> = refs.iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(
        ref_names,
        vec!["chart", "layout"],
        "sorted refs from root properties"
    );
    let dedup: std::collections::BTreeSet<_> = ref_names.iter().collect();
    assert_eq!(
        dedup.len(),
        ref_names.len(),
        "references must be deduplicated"
    );

    // Definitions index is name → one-sentence summary, not full definition.
    let defs = sc["definitions"]
        .as_object()
        .expect("definitions is an object");
    assert!(defs.contains_key("chart"));
    assert!(
        defs["chart"].is_string(),
        "definition value is a summary string"
    );
    assert!(defs.contains_key("layout"));

    server_task.await.unwrap();
}

#[tokio::test]
async fn get_chart_schema_returns_selected_definitions_and_unique_deep_reference_names() {
    let (rest_uri, server_task) =
        serve(crate::service::declarative::test::chart_schema_responses()).await;
    let server = server_with_rest(rest_uri).await;
    let params = GetChartSchemaParams {
        definitions: Some(vec!["chart".into(), "axis".into(), "chart".into()]),
    };

    let result = server.get_chart_schema(Parameters(params)).await.unwrap();
    let schema = crate::service::declarative::test::chart_schema_fixture();
    let snapshot = server.declarative_service.chart_schema().await.unwrap();
    assert!(snapshot.path.is_file());
    assert_eq!(
        structured(&result),
        serde_json::json!({
            "$id": schema["$id"],
            "title": schema["title"],
            "schema_path": snapshot.path,
            "definitions": {
                "chart": schema["$defs"]["chart"],
                "axis": schema["$defs"]["axis"]
            },
            "references": ["axis", "channelIdBinding"]
        })
    );
    assert_eq!(server_task.await.unwrap().len(), 2);
}

#[tokio::test]
async fn get_chart_schema_returns_invalid_params_and_close_matches_for_unknown_names() {
    let (rest_uri, server_task) =
        serve(crate::service::declarative::test::chart_schema_responses()).await;
    let server = server_with_rest(rest_uri).await;

    for (name, matches) in [
        ("chanelIdBinding", vec!["channelIdBinding"]),
        ("unrelatedDefinition", vec![]),
    ] {
        let error = server
            .get_chart_schema(Parameters(GetChartSchemaParams {
                definitions: Some(vec![name.to_owned()]),
            }))
            .await
            .unwrap_err();
        assert_eq!(error.code, rmcp::model::ErrorCode::INVALID_PARAMS);
        assert!(error.message.contains(name));
        for candidate in &matches {
            assert!(error.message.contains(candidate));
        }
        assert_eq!(
            error.data,
            Some(serde_json::json!({"name": name, "close_matches": matches}))
        );
    }
    assert_eq!(server_task.await.unwrap().len(), 2);
}

#[tokio::test]
async fn get_chart_schema_reports_when_the_deployment_has_no_schema() {
    for response in [
        http_response("404 Not Found", ""),
        http_response("200 OK", r#"{"versions":[{"version":"1"}]}"#),
    ] {
        let (rest_uri, server_task) = serve(vec![response]).await;
        let server = server_with_rest(rest_uri).await;
        let error = server
            .get_chart_schema(Parameters(GetChartSchemaParams { definitions: None }))
            .await
            .unwrap_err();

        assert_eq!(error.code, rmcp::model::ErrorCode::INTERNAL_ERROR);
        assert!(
            error
                .message
                .contains("This deployment does not serve the declarative chart schema.")
        );
        assert_eq!(server_task.await.unwrap().len(), 1);
    }
}
