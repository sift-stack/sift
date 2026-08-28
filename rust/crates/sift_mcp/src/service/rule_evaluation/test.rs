use sift_rs::{
    common::r#type::v1::{resource_identifier, resource_identifiers},
    rule_evaluation::v1::{
        EvaluateRulesPreviewResponse, evaluate_rules_preview_request,
        rule_evaluation_service_server::RuleEvaluationServiceServer,
    },
    rules::v1::{DryRunAnnotation, UpdateRuleRequest},
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::rule_evaluation::v1::MockRuleEvaluationServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{PreviewRuleSource, RuleEvaluationService};
use crate::policy::RetryPolicy;

async fn service_with_mock(
    mock: MockRuleEvaluationServiceImpl,
) -> (RuleEvaluationService, JoinHandle<()>) {
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
        RuleEvaluationService::new(channel, RetryPolicy::default()),
        handle,
    )
}

#[tokio::test]
async fn preview_rule_builds_saved_rule_request() {
    let mut mock = MockRuleEvaluationServiceImpl::new();
    mock.expect_evaluate_rules_preview()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let run_matches = matches!(
                &req.time,
                Some(evaluate_rules_preview_request::Time::Run(id))
                    if matches!(&id.identifier, Some(resource_identifier::Identifier::Id(v)) if v == "run-1")
            );
            let mode_matches = matches!(
                &req.mode,
                Some(evaluate_rules_preview_request::Mode::Rules(r))
                    if matches!(
                        r.rules.as_ref().and_then(|r| r.identifiers.as_ref()),
                        Some(resource_identifiers::Identifiers::Ids(ids)) if ids.ids == vec!["rule-1".to_string()]
                    )
            );
            run_matches && mode_matches
        })
        .returning(|_| {
            Ok(Response::new(EvaluateRulesPreviewResponse {
                created_annotation_count: 2,
                dry_run_annotations: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resp = service
        .preview_rule(
            "run-1".to_string(),
            PreviewRuleSource::SavedRuleId("rule-1".to_string()),
            None,
        )
        .await
        .expect("preview_rule failed");

    assert_eq!(resp.created_annotation_count, 2);
}

#[tokio::test]
async fn preview_rule_builds_draft_rule_config_request() {
    let mut mock = MockRuleEvaluationServiceImpl::new();
    mock.expect_evaluate_rules_preview()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            matches!(
                &req.mode,
                Some(evaluate_rules_preview_request::Mode::RuleConfigs(cfg))
                    if cfg.configs.len() == 1 && cfg.configs[0].name == "draft rule"
            )
        })
        .returning(|_| {
            Ok(Response::new(EvaluateRulesPreviewResponse {
                created_annotation_count: 1,
                dry_run_annotations: vec![DryRunAnnotation {
                    condition_id: "cond-1".into(),
                    name: "draft rule".into(),
                    ..Default::default()
                }],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let draft = UpdateRuleRequest {
        name: "draft rule".into(),
        ..Default::default()
    };

    let resp = service
        .preview_rule(
            "run-1".to_string(),
            PreviewRuleSource::DraftRuleConfig(Box::new(draft)),
            None,
        )
        .await
        .expect("preview_rule failed");

    assert_eq!(resp.created_annotation_count, 1);
    assert_eq!(resp.dry_run_annotations.len(), 1);
}

#[tokio::test]
async fn preview_rule_propagates_grpc_error() {
    let mut mock = MockRuleEvaluationServiceImpl::new();
    mock.expect_evaluate_rules_preview()
        .returning(|_| Err(Status::invalid_argument("bad rule config")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .preview_rule(
            "run-1".to_string(),
            PreviewRuleSource::SavedRuleId("rule-1".to_string()),
            None,
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to preview rule evaluation")
    );
}
