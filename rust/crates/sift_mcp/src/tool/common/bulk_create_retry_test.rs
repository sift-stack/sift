use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use rmcp::handler::server::wrapper::Parameters;
use serde_json::json;
use sift_test_util::grpc::memory_sift_channel;
use tonic::{Response, Status, transport::Server};

use crate::{server::SiftMcpServer, tool::common::test_support::structured};

macro_rules! check_retry_isolation {
    ($name:ident, $mock:path, $service:path, $expect:ident, $tool:ident, $item:expr) => {
        #[tokio::test]
        async fn $name() {
            use $mock as Mock;
            use $service as Service;

            let calls = Arc::new(AtomicUsize::new(0));
            let rpc_calls = calls.clone();
            let mut mock = Mock::new();
            mock.$expect().times(2).returning(move |_| {
                rpc_calls.fetch_add(1, Ordering::SeqCst);
                Err(Status::unavailable("response lost"))
            });
            let (client, incoming) = tokio::io::duplex(1024);
            let channel = memory_sift_channel(client).await;
            let handle = tokio::spawn(async move {
                Server::builder()
                    .add_service(Service::new(mock))
                    .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(incoming)))
                    .await
                    .unwrap();
            });
            let server = SiftMcpServer::new(channel, "https://app.test.local".into(), true, false);
            let item = $item;
            let mut input = json!({
                "items": [item.clone(), item.clone()],
                "selected_indices": [0, 1],
            });
            if stringify!($tool) == "create_annotation" {
                input["run_id"] = json!("run-1");
            }
            let mut unknown_item = $item;
            unknown_item["unexpected_parameter"] = json!(true);
            let mut invalid = input.clone();
            invalid["items"] = json!([item.clone(), unknown_item]);
            let params = serde_json::from_value(invalid).unwrap();
            let error = server.$tool(Parameters(params)).await.unwrap_err();
            assert!(error.message.contains("item 1"), "{}", error.message);
            assert!(error.message.contains("unexpected_parameter"));
            assert_eq!(calls.load(Ordering::SeqCst), 0);

            let params = serde_json::from_value(input.clone()).unwrap();
            let body = structured(server.$tool(Parameters(params)).await.unwrap());
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            assert_eq!(body["created_count"], 0);
            assert_eq!(body["failures"][0]["outcome_unknown"], true);
            assert_eq!(body["not_attempted"], json!([1]));

            let mut single = input;
            single["items"] = json!([item]);
            single["selected_indices"] = json!([0]);
            let params = serde_json::from_value(single).unwrap();
            let body = structured(server.$tool(Parameters(params)).await.unwrap());
            assert_eq!(
                body["failures"][0]["error"]["data"]["reason"],
                "BackendUnreachable"
            );
            assert_eq!(body["failures"][0]["outcome_unknown"], true);
            assert_eq!(calls.load(Ordering::SeqCst), 2);
            handle.abort();
        }
    };
}

check_retry_isolation!(
    annotations_create_once_per_item,
    sift_test_util::mock::annotations::v1::MockAnnotationServiceImpl,
    sift_rs::annotations::v1::annotation_service_server::AnnotationServiceServer,
    expect_create_annotation,
    create_annotation,
    json!({"name":"range", "start_time_unix_nanos":1_000_000_000_i64,
        "end_time_unix_nanos":2_000_000_000_i64, "annotation_type":"data_review"})
);
check_retry_isolation!(
    rules_create_once_per_item,
    sift_test_util::mock::rules::v1::MockRuleServiceImpl,
    sift_rs::rules::v1::rule_service_server::RuleServiceServer,
    expect_create_rule,
    create_rule,
    json!({"rule_json":json!({"name":"rule", "description":""}).to_string()})
);
check_retry_isolation!(
    calculated_channels_create_once_per_item,
    sift_test_util::mock::calculated_channels::v2::MockCalculatedChannelServiceImpl,
    sift_rs::calculated_channels::v2::calculated_channel_service_server::CalculatedChannelServiceServer,
    expect_create_calculated_channel,
    create_calculated_channel,
    json!({"name":"channel", "expression":"1", "expression_channel_references_json":"[]", "all_assets":true})
);
check_retry_isolation!(
    user_defined_functions_create_once_per_item,
    sift_test_util::mock::user_defined_functions::v1::MockUserDefinedFunctionServiceImpl,
    sift_rs::user_defined_functions::v1::user_defined_function_service_server::UserDefinedFunctionServiceServer,
    expect_create_user_defined_function,
    create_user_defined_function,
    json!({"name":"function", "expression":"1", "function_inputs_json":"[]"})
);
check_retry_isolation!(
    reports_create_once_per_item,
    sift_test_util::mock::rule_evaluation::v1::MockRuleEvaluationServiceImpl,
    sift_rs::rule_evaluation::v1::rule_evaluation_service_server::RuleEvaluationServiceServer,
    expect_evaluate_rules,
    create_report,
    json!({"name":"report", "run_id":"run-1", "rule_ids":["rule-1"]})
);
check_retry_isolation!(
    report_templates_create_once_per_item,
    sift_test_util::mock::report_templates::v1::MockReportTemplateServiceImpl,
    sift_rs::report_templates::v1::report_template_service_server::ReportTemplateServiceServer,
    expect_create_report_template,
    create_report_template,
    json!({"name":"template", "rule_ids":["rule-1"]})
);
check_retry_isolation!(
    test_reports_create_once_per_item,
    sift_test_util::mock::test_reports::v1::MockTestReportServiceImpl,
    sift_rs::test_reports::v1::test_report_service_server::TestReportServiceServer,
    expect_create_test_report,
    create_test_report,
    json!({"report_json":json!({"name":"report", "test_system_name":"system", "test_case":"case"}).to_string()})
);
check_retry_isolation!(
    artifacts_create_once_per_item,
    sift_test_util::mock::artifacts::v1::MockArtifactServiceImpl,
    sift_rs::artifacts::v1::artifact_service_server::ArtifactServiceServer,
    expect_create_artifact,
    create_artifact,
    json!({"title":"artifact"})
);

#[tokio::test]
async fn reports_retry_get_after_create() {
    use sift_rs::reports::v1::{
        GetReportResponse, Report, report_service_server::ReportServiceServer,
    };
    use sift_rs::rule_evaluation::v1::{
        EvaluateRulesResponse, rule_evaluation_service_server::RuleEvaluationServiceServer,
    };
    use sift_test_util::mock::{
        reports::v1::MockReportServiceImpl, rule_evaluation::v1::MockRuleEvaluationServiceImpl,
    };

    let evaluate_calls = Arc::new(AtomicUsize::new(0));
    let evaluate_count = evaluate_calls.clone();
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules()
        .times(2)
        .returning(move |_| {
            let number = evaluate_count.fetch_add(1, Ordering::SeqCst) + 1;
            Ok(Response::new(EvaluateRulesResponse {
                report_id: Some(format!("rep-{number}")),
                ..Default::default()
            }))
        });

    let get_calls = Arc::new(AtomicUsize::new(0));
    let get_count = get_calls.clone();
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_get_report()
        .times(3)
        .returning(move |request| {
            if get_count.fetch_add(1, Ordering::SeqCst) == 0 {
                Err(Status::unavailable("response lost"))
            } else {
                Ok(Response::new(GetReportResponse {
                    report: Some(Report {
                        report_id: request.into_inner().report_id,
                        name: "report".into(),
                        ..Default::default()
                    }),
                }))
            }
        });

    let (client, incoming) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleEvaluationServiceServer::new(eval_mock))
            .add_service(ReportServiceServer::new(report_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(incoming)))
            .await
            .unwrap();
    });
    let server = SiftMcpServer::new(channel, "https://app.test.local".into(), true, false);
    let params = serde_json::from_value(json!({
        "items": [
            {"name":"first", "run_id":"run-1", "rule_ids":["rule-1"]},
            {"name":"second", "run_id":"run-1", "rule_ids":["rule-1"]}
        ],
        "selected_indices": [0, 1]
    }))
    .unwrap();
    let body = structured(server.create_report(Parameters(params)).await.unwrap());
    assert_eq!(body["created_count"], 2);
    assert_eq!(get_calls.load(Ordering::SeqCst), 3);
    assert_eq!(evaluate_calls.load(Ordering::SeqCst), 2);
    handle.abort();
}
