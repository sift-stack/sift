use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    rule_evaluation::v1::{
        EvaluateRulesPreviewResponse, rule_evaluation_service_server::RuleEvaluationServiceServer,
    },
    rules::v1::{
        DryRunAnnotation, ListRulesResponse, Rule, rule_service_server::RuleServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{rule_evaluation::v1::MockRuleEvaluationServiceImpl, rules::v1::MockRuleServiceImpl},
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::PreviewRuleParams;
use crate::{server::SiftMcpServer, tool::common::test_support::structured_field};

fn preview_rule_params() -> PreviewRuleParams {
    PreviewRuleParams {
        run_id: "run-1".into(),
        rule_id: None,
        rule_name: None,
        draft_rule_config: None,
        organization_id: None,
    }
}

/// Registers only the evaluation mock. Sufficient for tests that never need to
/// resolve a rule name (rule_id-only or draft-config paths).
async fn server_with_eval_mock(
    mock: MockRuleEvaluationServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleEvaluationServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://app.test.local"), true, true),
        handle,
    )
}

/// Registers both the rule and evaluation mocks, for the rule-name-resolution path.
async fn server_with_dual_mocks(
    rule_mock: MockRuleServiceImpl,
    eval_mock: MockRuleEvaluationServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleServiceServer::new(rule_mock))
            .add_service(RuleEvaluationServiceServer::new(eval_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://app.test.local"), true, true),
        handle,
    )
}

#[tokio::test]
async fn preview_rule_saved_by_id_happy_path() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock.expect_evaluate_rules_preview().returning(|_| {
        Ok(Response::new(EvaluateRulesPreviewResponse {
            created_annotation_count: 2,
            dry_run_annotations: vec![DryRunAnnotation {
                condition_id: "cond-1".into(),
                name: "overtemp".into(),
                ..Default::default()
            }],
        }))
    });

    let (server, _h) = server_with_eval_mock(eval_mock).await;

    let mut params = preview_rule_params();
    params.rule_id = Some("rule-1".into());

    let resp = server
        .preview_rule(Parameters(params))
        .await
        .expect("preview_rule failed");

    let count = structured_field(resp.clone(), "created_annotation_count");
    assert_eq!(count, 2);
    let annotations = structured_field(resp, "dry_run_annotations");
    assert_eq!(annotations.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn preview_rule_saved_by_name_resolves_rule_id() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock
        .expect_list_rules()
        .withf(|req| req.get_ref().filter == "is_archived == false && name == \"overtemp\"")
        .returning(|_| {
            Ok(Response::new(ListRulesResponse {
                rules: vec![Rule {
                    rule_id: "rule-resolved".into(),
                    name: "overtemp".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock.expect_evaluate_rules_preview().returning(|_| {
        Ok(Response::new(EvaluateRulesPreviewResponse {
            created_annotation_count: 0,
            dry_run_annotations: vec![],
        }))
    });

    let (server, _h) = server_with_dual_mocks(rule_mock, eval_mock).await;

    let mut params = preview_rule_params();
    params.rule_name = Some("overtemp".into());

    let resp = server
        .preview_rule(Parameters(params))
        .await
        .expect("preview_rule failed");

    let count = structured_field(resp, "created_annotation_count");
    assert_eq!(count, 0);
}

#[tokio::test]
async fn preview_rule_saved_by_name_not_found() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock.expect_list_rules().returning(|_| {
        Ok(Response::new(ListRulesResponse {
            rules: vec![],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) =
        server_with_dual_mocks(rule_mock, MockRuleEvaluationServiceImpl::new()).await;

    let mut params = preview_rule_params();
    params.rule_name = Some("does-not-exist".into());

    let err = server
        .preview_rule(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}

#[tokio::test]
async fn preview_rule_draft_config_happy_path() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock.expect_evaluate_rules_preview().returning(|_| {
        Ok(Response::new(EvaluateRulesPreviewResponse {
            created_annotation_count: 1,
            dry_run_annotations: vec![DryRunAnnotation {
                condition_id: "cond-draft".into(),
                name: "draft rule".into(),
                ..Default::default()
            }],
        }))
    });

    let (server, _h) = server_with_eval_mock(eval_mock).await;

    let mut params = preview_rule_params();
    params.draft_rule_config =
        Some(r#"{ "name": "draft rule", "description": "ad-hoc" }"#.to_string());

    let resp = server
        .preview_rule(Parameters(params))
        .await
        .expect("preview_rule failed");

    let count = structured_field(resp, "created_annotation_count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn preview_rule_rejects_malformed_draft_json() {
    let (server, _h) = server_with_eval_mock(MockRuleEvaluationServiceImpl::new()).await;

    let mut params = preview_rule_params();
    params.draft_rule_config = Some("not json".to_string());

    let err = server
        .preview_rule(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn preview_rule_rejects_both_saved_and_draft() {
    let (server, _h) = server_with_eval_mock(MockRuleEvaluationServiceImpl::new()).await;

    let mut params = preview_rule_params();
    params.rule_id = Some("rule-1".into());
    params.draft_rule_config = Some(r#"{ "name": "x", "description": "y" }"#.to_string());

    let err = server
        .preview_rule(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn preview_rule_rejects_neither_saved_nor_draft() {
    let (server, _h) = server_with_eval_mock(MockRuleEvaluationServiceImpl::new()).await;

    let err = server
        .preview_rule(Parameters(preview_rule_params()))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn preview_rule_rejects_both_rule_id_and_rule_name() {
    let (server, _h) = server_with_eval_mock(MockRuleEvaluationServiceImpl::new()).await;

    let mut params = preview_rule_params();
    params.rule_id = Some("rule-1".into());
    params.rule_name = Some("overtemp".into());

    let err = server
        .preview_rule(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn preview_rule_rejects_empty_run_id() {
    let (server, _h) = server_with_eval_mock(MockRuleEvaluationServiceImpl::new()).await;

    let mut params = preview_rule_params();
    params.run_id = String::new();
    params.rule_id = Some("rule-1".into());

    let err = server
        .preview_rule(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn preview_rule_propagates_grpc_error() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules_preview()
        .returning(|_| Err(Status::not_found("run missing")));

    let (server, _h) = server_with_eval_mock(eval_mock).await;

    let mut params = preview_rule_params();
    params.rule_id = Some("rule-1".into());

    let err = server
        .preview_rule(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}
