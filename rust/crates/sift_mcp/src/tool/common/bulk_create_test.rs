use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use serde_json::{Value, json};
use sift_rs::annotations::v1::{
    Annotation, CreateAnnotationResponse, annotation_service_server::AnnotationServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::annotations::v1::MockAnnotationServiceImpl};
use tonic::{Response, Status, transport::Server};

use crate::{server::SiftMcpServer, tool::common::test_support::structured};

async fn server(
    mock: MockAnnotationServiceImpl,
    allow_create: bool,
) -> (SiftMcpServer, tokio::task::JoinHandle<()>) {
    let (client, incoming) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AnnotationServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(incoming)))
            .await
            .unwrap();
    });
    (
        SiftMcpServer::new(
            channel,
            "https://app.test.local".into(),
            allow_create,
            false,
        ),
        handle,
    )
}

fn annotation(name: &str) -> Value {
    json!({"name": name, "start_time_unix_nanos": 1_000_000_000_i64,
        "end_time_unix_nanos": 2_000_000_000_i64, "annotation_type": "data_review"})
}

fn params(items: Value) -> crate::tool::annotations::CreateAnnotationsParams {
    serde_json::from_value(
        json!({"items_json": items.to_string(), "run_id": "run-1", "organization_id": "org-1",
            "selected_indices": (0..items.as_array().unwrap().len()).collect::<Vec<_>>() }),
    )
    .unwrap()
}

#[tokio::test]
async fn every_bulk_create_is_gated_before_parsing_or_rpc() {
    let (server, handle) = server(MockAnnotationServiceImpl::new(), false).await;
    macro_rules! check {
        ($method:ident) => {{
            let params = serde_json::from_value(json!({"items_json": "not json"})).unwrap();
            let error = server.$method(Parameters(params)).await.unwrap_err();
            assert_eq!(error.code, ErrorCode::INVALID_REQUEST, stringify!($method));
            assert!(error.message.contains("--allow-create"));
        }};
    }
    check!(create_annotations);
    check!(create_rules);
    check!(create_calculated_channels);
    check!(create_user_defined_functions);
    check!(create_reports);
    check!(create_report_templates);
    check!(create_test_reports);
    check!(create_artifacts);
    handle.abort();
}

#[tokio::test]
async fn every_bulk_create_rejects_empty_batches_before_rpc() {
    let (server, handle) = server(MockAnnotationServiceImpl::new(), true).await;
    macro_rules! check {
        ($method:ident) => {{
            let params = serde_json::from_value(json!({"items_json": "[]"})).unwrap();
            let error = server.$method(Parameters(params)).await.unwrap_err();
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS, stringify!($method));
        }};
    }
    check!(create_annotations);
    check!(create_rules);
    check!(create_calculated_channels);
    check!(create_user_defined_functions);
    check!(create_reports);
    check!(create_report_templates);
    check!(create_test_reports);
    check!(create_artifacts);
    handle.abort();
}

#[tokio::test]
async fn twenty_annotations_preserve_shared_run_and_org_on_every_rpc() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation()
        .times(20)
        .withf(|request| {
            let request = request.get_ref();
            request.run_id.as_deref() == Some("run-1") && request.organization_id == "org-1"
        })
        .returning(|request| {
            Ok(Response::new(CreateAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: format!("id-{}", request.get_ref().name),
                    name: request.get_ref().name.clone(),
                    ..Default::default()
                }),
            }))
        });
    let (server, handle) = server(mock, true).await;
    let items = Value::Array((0..20).map(|i| annotation(&format!("range-{i}"))).collect());
    let result = server
        .create_annotations(Parameters(params(items)))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false));
    let body = structured(result);
    assert_eq!(body["created_count"], 20);
    assert_eq!(body["successes"][19]["index"], 19);
    assert_eq!(
        body["successes"][0]["result"]["annotation_url"],
        "https://app.test.local/annotation/id-range-0"
    );
    assert_eq!(body["failures"], json!([]));
    handle.abort();
}

#[tokio::test]
async fn annotation_item_failure_does_not_block_other_items() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation()
        .times(3)
        .returning(|request| {
            if request.get_ref().name == "bad" {
                return Err(Status::invalid_argument("bad item"));
            }
            Ok(Response::new(CreateAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: request.get_ref().name.clone(),
                    ..Default::default()
                }),
            }))
        });
    let (server, handle) = server(mock, true).await;
    let result = server
        .create_annotations(Parameters(params(json!([
            annotation("good"),
            annotation("bad"),
            annotation("also-good")
        ]))))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true));
    let body = structured(result);
    assert_eq!(body["created_count"], 2);
    assert_eq!(body["failures"][0]["index"], 1);
    assert_eq!(body["successes"][1]["index"], 2);
    assert_eq!(body["not_attempted"], json!([]));
    handle.abort();
}

#[tokio::test]
async fn annotation_backend_failure_is_not_retried_and_stops_later_items() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation()
        .times(1)
        .returning(|_| Err(Status::unavailable("lost response")));
    let (server, handle) = server(mock, true).await;
    let body = structured(
        server
            .create_annotations(Parameters(params(json!([
                annotation("first"),
                annotation("second"),
                annotation("third")
            ]))))
            .await
            .unwrap(),
    );
    assert_eq!(body["created_count"], 0);
    assert_eq!(body["not_attempted"], json!([1, 2]));
    assert_eq!(body["failures"][0]["outcome_unknown"], true);
    handle.abort();
}

#[tokio::test]
async fn annotation_association_mistakes_reject_whole_batch_without_writes() {
    let (server, handle) = server(MockAnnotationServiceImpl::new(), true).await;
    let mut item = annotation("bad");
    item["run_id"] = json!("different-run");
    assert_eq!(
        server
            .create_annotations(Parameters(params(json!([annotation("valid"), item]))))
            .await
            .unwrap_err()
            .code,
        ErrorCode::INVALID_PARAMS
    );
    let mut item = annotation("typo");
    item["runId"] = json!("run-1");
    assert_eq!(
        server
            .create_annotations(Parameters(params(json!([item]))))
            .await
            .unwrap_err()
            .code,
        ErrorCode::INVALID_PARAMS
    );
    let params = serde_json::from_value(
        json!({"items_json": json!([annotation("no association")]).to_string()}),
    )
    .unwrap();
    assert_eq!(
        server
            .create_annotations(Parameters(params))
            .await
            .unwrap_err()
            .code,
        ErrorCode::INVALID_PARAMS
    );
    handle.abort();
}

#[tokio::test]
async fn rule_batch_rejects_update_identifiers_before_writing() {
    let (server, handle) = server(MockAnnotationServiceImpl::new(), true).await;
    let params = serde_json::from_value(json!({"items_json": json!([
        {"rule_json": json!({"name":"new", "description":""}).to_string()},
        {"rule_json": json!({"rule_id":"existing", "name":"do not edit", "description":""}).to_string()}
    ]).to_string()})).unwrap();
    let error = server.create_rules(Parameters(params)).await.unwrap_err();
    assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
    assert!(error.message.contains("create-only"));
    handle.abort();
}

#[tokio::test]
async fn selected_annotations_preserve_shared_associations_without_creating_the_rejected_item() {
    let mut mock = MockAnnotationServiceImpl::new();
    mock.expect_create_annotation()
        .times(2)
        .withf(|request| {
            let request = request.get_ref();
            request.name != "second"
                && request.run_id.as_deref() == Some("run-1")
                && request.organization_id == "org-1"
        })
        .returning(|request| {
            Ok(Response::new(CreateAnnotationResponse {
                annotation: Some(Annotation {
                    annotation_id: request.get_ref().name.clone(),
                    ..Default::default()
                }),
            }))
        });
    let (server, handle) = server(mock, true).await;
    let items = json!([
        annotation("first"),
        annotation("second"),
        annotation("third")
    ]);
    let params = serde_json::from_value(json!({"items_json": items.to_string(),
        "run_id":"run-1", "organization_id":"org-1", "selected_indices":[0,2]}))
    .unwrap();
    let body = structured(server.create_annotations(Parameters(params)).await.unwrap());
    assert_eq!(body["created_count"], 2);
    assert_eq!(body["not_selected"], json!([1]));
    assert_eq!(body["successes"][1]["index"], 2);
    handle.abort();
}

#[tokio::test]
async fn selecting_two_rules_issues_exactly_two_create_rpcs_and_never_the_rejected_rule() {
    use sift_rs::rules::v1::{CreateRuleResponse, rule_service_server::RuleServiceServer};
    use sift_test_util::mock::rules::v1::MockRuleServiceImpl;
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_create_rule()
        .times(2)
        .withf(|request| {
            request
                .get_ref()
                .update
                .as_ref()
                .is_some_and(|rule| rule.name != "second")
        })
        .returning(|request| {
            Ok(Response::new(CreateRuleResponse {
                rule_id: request.get_ref().update.as_ref().unwrap().name.clone(),
            }))
        });
    let (client, incoming) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(incoming)))
            .await
            .unwrap();
    });
    let server = SiftMcpServer::new(channel, "https://app.test.local".into(), true, false);
    let items = ["first", "second", "third"]
        .map(|name| json!({"rule_json":json!({"name":name,"description":""}).to_string()}));
    let params = serde_json::from_value(
        json!({"items_json":serde_json::to_string(&items).unwrap(), "selected_indices":[0,2]}),
    )
    .unwrap();
    let response = server.create_rules(Parameters(params)).await.unwrap();
    let body = structured(response.clone());
    assert_eq!(body["created_count"], 2);
    assert_eq!(body["successes"][0]["result"]["rule_id"], "first");
    assert_eq!(body["successes"][1]["index"], 2);
    assert_eq!(body["successes"][1]["result"]["rule_id"], "third");
    assert_eq!(body["not_selected"], json!([1]));
    assert!(
        serde_json::to_string(&response.content)
            .unwrap()
            .contains("not_selected")
    );
    handle.abort();
}

#[tokio::test]
async fn all_eight_tools_refuse_missing_selection_and_respect_an_empty_selection() {
    let (server, handle) = server(MockAnnotationServiceImpl::new(), true).await;
    macro_rules! check {
        ($method:ident, $item:expr) => {{
            let items_json = json!([$item]).to_string();
            let mut input = json!({"items_json": items_json});
            if stringify!($method) == "create_annotations" { input["run_id"] = json!("run-1"); }
            let params = serde_json::from_value(input.clone()).unwrap();
            let error = server.$method(Parameters(params)).await.unwrap_err();
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains("selected_indices"), "{}", error.message);
            input["selected_indices"] = json!([]);
            let params = serde_json::from_value(input).unwrap();
            let body = structured(server.$method(Parameters(params)).await.unwrap());
            assert_eq!(body["created_count"], 0, stringify!($method));
            assert_eq!(body["not_selected"], json!([0]), stringify!($method));
        }};
    }
    check!(create_annotations, annotation("first"));
    check!(create_rules, json!({"rule_json":"{}"}));
    check!(
        create_calculated_channels,
        json!({"name":"channel", "expression":"1", "expression_channel_references_json":"[]"})
    );
    check!(
        create_user_defined_functions,
        json!({"name":"function", "expression":"1", "function_inputs_json":"[]"})
    );
    check!(create_reports, json!({"name":"report", "run_id":"run-1"}));
    check!(create_report_templates, json!({"name":"template"}));
    check!(create_test_reports, json!({"report_json":"{}"}));
    check!(create_artifacts, json!({"title":"artifact"}));
    handle.abort();
}

#[test]
fn every_plural_tool_is_create_only_and_has_an_event() {
    let routers = [
        (SiftMcpServer::annotations_router(), "create_annotations"),
        (SiftMcpServer::rules_router(), "create_rules"),
        (
            SiftMcpServer::calculated_channels_router(),
            "create_calculated_channels",
        ),
        (
            SiftMcpServer::user_defined_functions_router(),
            "create_user_defined_functions",
        ),
        (SiftMcpServer::reports_router(), "create_reports"),
        (
            SiftMcpServer::report_templates_router(),
            "create_report_templates",
        ),
        (SiftMcpServer::test_reports_router(), "create_test_reports"),
        (SiftMcpServer::artifacts_router(), "create_artifacts"),
    ];
    for (router, name) in routers {
        let tool = router
            .list_all()
            .into_iter()
            .find(|tool| tool.name == name)
            .unwrap();
        assert!(
            tool.input_schema["properties"]
                .get("selected_indices")
                .is_some(),
            "{name}"
        );
        let annotations = tool.annotations.unwrap();
        assert_eq!(annotations.read_only_hint, Some(false), "{name}");
        assert_eq!(annotations.destructive_hint, Some(false), "{name}");
        assert_eq!(annotations.idempotent_hint, Some(false), "{name}");
        assert!(
            crate::client_event::event_for_tool(name).is_some(),
            "{name}"
        );
    }
}
