use sift_rs::{
    reports::v1::{
        GetReportResponse, ListReportRuleSummariesResponse, ListReportsResponse, Report,
        ReportRuleSummary, UpdateReportResponse, report_service_server::ReportServiceServer,
    },
    rule_evaluation::v1::{
        EvaluateRulesResponse, evaluate_rules_request,
        rule_evaluation_service_server::RuleEvaluationServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{
        reports::v1::MockReportServiceImpl, rule_evaluation::v1::MockRuleEvaluationServiceImpl,
    },
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{ReportService, ReportSource, RuleIdentifier};
use crate::service::common::DEFAULT_LIMIT;

async fn service_with_mock(mock: MockReportServiceImpl) -> (ReportService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ReportServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        ReportService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
}

async fn service_with_dual_mocks(
    report_mock: MockReportServiceImpl,
    eval_mock: MockRuleEvaluationServiceImpl,
) -> (ReportService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ReportServiceServer::new(report_mock))
            .add_service(RuleEvaluationServiceServer::new(eval_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        ReportService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
}

#[tokio::test]
async fn list_reports_returns_single_page() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports()
        .withf(|req| req.get_ref().filter == "name == \"nightly\"")
        .returning(|_| {
            Ok(Response::new(ListReportsResponse {
                reports: vec![Report {
                    report_id: "rep1".into(),
                    name: "nightly".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_reports("name == \"nightly\"".to_string(), None, None, None)
        .await
        .expect("list_reports failed");

    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].report_id, "rep1");
}

#[tokio::test]
async fn list_reports_forwards_organization_id() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports()
        .withf(|req| req.get_ref().organization_id == "org-123")
        .returning(|_| {
            Ok(Response::new(ListReportsResponse {
                reports: vec![Report {
                    report_id: "rep1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_reports(String::new(), None, None, Some("org-123".to_string()))
        .await
        .expect("list_reports failed");

    assert_eq!(reports.len(), 1);
}

#[tokio::test]
async fn list_reports_paginates_until_token_empty() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (reports, next) = match req.page_token.as_str() {
            "" => (
                vec![Report {
                    report_id: "rep1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Report {
                    report_id: "rep2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListReportsResponse {
            reports,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_reports(String::new(), None, None, None)
        .await
        .expect("list_reports failed");

    let ids: Vec<&str> = reports.iter().map(|r| r.report_id.as_str()).collect();
    assert_eq!(ids, vec!["rep1", "rep2"]);
}

#[tokio::test]
async fn list_reports_respects_limit() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListReportsResponse {
            reports: vec![
                Report {
                    report_id: "rep1".into(),
                    ..Default::default()
                },
                Report {
                    report_id: "rep2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_reports(String::new(), None, Some(2), None)
        .await
        .expect("list_reports failed");

    assert_eq!(reports.len(), 2);
}

#[tokio::test]
async fn list_reports_truncates_to_limit_across_pages() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (reports, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Report {
                        report_id: "rep1".into(),
                        ..Default::default()
                    },
                    Report {
                        report_id: "rep2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Report {
                        report_id: "rep3".into(),
                        ..Default::default()
                    },
                    Report {
                        report_id: "rep4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListReportsResponse {
            reports,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_reports(String::new(), None, Some(3), None)
        .await
        .expect("list_reports failed");

    let ids: Vec<&str> = reports.iter().map(|r| r.report_id.as_str()).collect();
    assert_eq!(ids, vec!["rep1", "rep2", "rep3"]);
}

#[tokio::test]
async fn list_reports_breaks_on_empty_page() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports().times(1).returning(|_| {
        Ok(Response::new(ListReportsResponse {
            reports: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_reports(String::new(), None, None, None)
        .await
        .expect("list_reports failed");

    assert!(reports.is_empty());
}

#[tokio::test]
async fn list_reports_propagates_grpc_error() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_reports()
        .returning(|_| Err(Status::not_found("no such report")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_reports(String::new(), None, None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query reports"));
}

#[tokio::test]
async fn list_report_rule_summaries_returns_single_page() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_report_rule_summaries()
        .withf(|req| req.get_ref().report_id == "rep1")
        .returning(|_| {
            Ok(Response::new(ListReportRuleSummariesResponse {
                report_rule_summaries: vec![ReportRuleSummary {
                    rule_id: "rule-1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let summaries = service
        .list_report_rule_summaries("rep1".to_string(), String::new(), None, None)
        .await
        .expect("list_report_rule_summaries failed");

    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].rule_id, "rule-1");
}

#[tokio::test]
async fn list_report_rule_summaries_paginates_until_token_empty() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_report_rule_summaries().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (summaries, next) = match req.page_token.as_str() {
            "" => (
                vec![ReportRuleSummary {
                    rule_id: "rule-1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![ReportRuleSummary {
                    rule_id: "rule-2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListReportRuleSummariesResponse {
            report_rule_summaries: summaries,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let summaries = service
        .list_report_rule_summaries("rep1".to_string(), String::new(), None, None)
        .await
        .expect("list_report_rule_summaries failed");

    let ids: Vec<&str> = summaries.iter().map(|s| s.rule_id.as_str()).collect();
    assert_eq!(ids, vec!["rule-1", "rule-2"]);
}

#[tokio::test]
async fn list_report_rule_summaries_respects_limit() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_report_rule_summaries()
        .times(1)
        .returning(|req| {
            assert_eq!(req.get_ref().page_size, 1);
            Ok(Response::new(ListReportRuleSummariesResponse {
                report_rule_summaries: vec![ReportRuleSummary {
                    rule_id: "rule-1".into(),
                    ..Default::default()
                }],
                next_page_token: "page-2".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let summaries = service
        .list_report_rule_summaries("rep1".to_string(), String::new(), None, Some(1))
        .await
        .expect("list_report_rule_summaries failed");

    assert_eq!(summaries.len(), 1);
}

#[tokio::test]
async fn list_report_rule_summaries_propagates_grpc_error() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_list_report_rule_summaries()
        .returning(|_| Err(Status::not_found("no such report")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_report_rule_summaries("rep1".to_string(), String::new(), None, None)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to query report rule summaries")
    );
}

#[tokio::test]
async fn create_report_from_rule_ids() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules()
        .withf(|req| {
            let req = req.get_ref();
            let run_ok = matches!(
                &req.time,
                Some(evaluate_rules_request::Time::Run(rid))
                    if matches!(&rid.identifier,
                        Some(sift_rs::common::r#type::v1::resource_identifier::Identifier::Id(id))
                            if id == "run-1")
            );
            let mode_ok = matches!(
                &req.mode,
                Some(evaluate_rules_request::Mode::Rules(inner))
                    if matches!(
                        &inner.rules.as_ref().and_then(|r| r.identifiers.as_ref()),
                        Some(sift_rs::common::r#type::v1::resource_identifiers::Identifiers::Ids(ids))
                            if ids.ids == vec!["rule-1".to_string()]
                    )
            );
            req.report_name.as_deref() == Some("nightly report") && run_ok && mode_ok
        })
        .returning(|_| {
            Ok(Response::new(EvaluateRulesResponse {
                report_id: Some("rep-new".into()),
                job_id: Some("job-1".into()),
                created_annotation_count: 0,
            }))
        });

    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_get_report()
        .withf(|req| req.get_ref().report_id == "rep-new")
        .returning(|_| {
            Ok(Response::new(GetReportResponse {
                report: Some(Report {
                    report_id: "rep-new".into(),
                    name: "nightly report".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_dual_mocks(report_mock, eval_mock).await;

    let output = service
        .create_report(
            None,
            "run-1".to_string(),
            "nightly report".to_string(),
            None,
            vec![],
            ReportSource::Rules {
                rules: RuleIdentifier::RuleIds(vec!["rule-1".to_string()]),
            },
        )
        .await
        .expect("create_report failed");

    assert_eq!(output.report.report_id, "rep-new");
    assert_eq!(output.job_id.as_deref(), Some("job-1"));
    assert_eq!(output.created_annotation_count, 0);
}

#[tokio::test]
async fn create_report_from_rule_client_keys() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules()
        .withf(|req| {
            matches!(
                &req.get_ref().mode,
                Some(evaluate_rules_request::Mode::Rules(inner))
                    if matches!(
                        inner.rules.as_ref().and_then(|r| r.identifiers.as_ref()),
                        Some(sift_rs::common::r#type::v1::resource_identifiers::Identifiers::ClientKeys(k))
                            if k.client_keys == vec!["ck-1".to_string()]
                    )
            )
        })
        .returning(|_| {
            Ok(Response::new(EvaluateRulesResponse {
                report_id: Some("rep-new".into()),
                job_id: None,
                created_annotation_count: 0,
            }))
        });

    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_get_report().returning(|_| {
        Ok(Response::new(GetReportResponse {
            report: Some(Report {
                report_id: "rep-new".into(),
                ..Default::default()
            }),
        }))
    });

    let (service, _h) = service_with_dual_mocks(report_mock, eval_mock).await;

    service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            None,
            vec![],
            ReportSource::Rules {
                rules: RuleIdentifier::RuleClientKeys(vec!["ck-1".to_string()]),
            },
        )
        .await
        .expect("create_report failed");
}

#[tokio::test]
async fn create_report_from_rule_version_ids() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules()
        .withf(|req| {
            matches!(
                &req.get_ref().mode,
                Some(evaluate_rules_request::Mode::RuleVersions(v))
                    if v.rule_version_ids == vec!["rv-1".to_string()]
            )
        })
        .returning(|_| {
            Ok(Response::new(EvaluateRulesResponse {
                report_id: Some("rep-new".into()),
                job_id: None,
                created_annotation_count: 0,
            }))
        });

    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_get_report().returning(|_| {
        Ok(Response::new(GetReportResponse {
            report: Some(Report {
                report_id: "rep-new".into(),
                ..Default::default()
            }),
        }))
    });

    let (service, _h) = service_with_dual_mocks(report_mock, eval_mock).await;

    service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            None,
            vec![],
            ReportSource::Rules {
                rules: RuleIdentifier::RuleVersionIds(vec!["rv-1".to_string()]),
            },
        )
        .await
        .expect("create_report failed");
}

#[tokio::test]
async fn create_report_from_template() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules()
        .withf(|req| {
            matches!(
                &req.get_ref().mode,
                Some(evaluate_rules_request::Mode::ReportTemplate(t))
                    if matches!(
                        t.report_template.as_ref().and_then(|r| r.identifier.as_ref()),
                        Some(sift_rs::common::r#type::v1::resource_identifier::Identifier::Id(id))
                            if id == "tmpl-1"
                    )
            )
        })
        .returning(|_| {
            Ok(Response::new(EvaluateRulesResponse {
                report_id: Some("rep-tmpl".into()),
                job_id: Some("job-2".into()),
                created_annotation_count: 0,
            }))
        });

    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_get_report().returning(|_| {
        Ok(Response::new(GetReportResponse {
            report: Some(Report {
                report_id: "rep-tmpl".into(),
                ..Default::default()
            }),
        }))
    });

    let (service, _h) = service_with_dual_mocks(report_mock, eval_mock).await;

    let output = service
        .create_report(
            None,
            "run-1".to_string(),
            "from template".to_string(),
            None,
            vec![],
            ReportSource::Template {
                report_template_id: "tmpl-1".to_string(),
            },
        )
        .await
        .expect("create_report failed");

    assert_eq!(output.report.report_id, "rep-tmpl");
    assert_eq!(output.job_id.as_deref(), Some("job-2"));
}

#[tokio::test]
async fn create_report_applies_description() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock.expect_evaluate_rules().returning(|_| {
        Ok(Response::new(EvaluateRulesResponse {
            report_id: Some("rep-new".into()),
            job_id: Some("job-3".into()),
            created_annotation_count: 0,
        }))
    });

    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_update_report()
        .withf(|req| {
            let req = req.get_ref();
            let paths = req
                .update_mask
                .as_ref()
                .map(|m| m.paths.clone())
                .unwrap_or_default();
            let report = req.report.as_ref().expect("report");
            paths == vec!["description".to_string()]
                && report.report_id == "rep-new"
                && report.description.as_deref() == Some("what this evaluates")
        })
        .times(1)
        .returning(|_| Ok(Response::new(UpdateReportResponse {})));
    report_mock.expect_get_report().returning(|_| {
        Ok(Response::new(GetReportResponse {
            report: Some(Report {
                report_id: "rep-new".into(),
                description: Some("what this evaluates".into()),
                ..Default::default()
            }),
        }))
    });

    let (service, _h) = service_with_dual_mocks(report_mock, eval_mock).await;

    let output = service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            Some("what this evaluates".to_string()),
            vec![],
            ReportSource::Rules {
                rules: RuleIdentifier::RuleIds(vec!["rule-1".to_string()]),
            },
        )
        .await
        .expect("create_report failed");

    assert_eq!(
        output.report.description.as_deref(),
        Some("what this evaluates")
    );
}

#[tokio::test]
async fn create_report_skips_metadata_write_when_none_provided() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock.expect_evaluate_rules().returning(|_| {
        Ok(Response::new(EvaluateRulesResponse {
            report_id: Some("rep-new".into()),
            job_id: None,
            created_annotation_count: 0,
        }))
    });

    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_get_report().returning(|_| {
        Ok(Response::new(GetReportResponse {
            report: Some(Report {
                report_id: "rep-new".into(),
                ..Default::default()
            }),
        }))
    });

    let (service, _h) = service_with_dual_mocks(report_mock, eval_mock).await;

    service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            None,
            vec![],
            ReportSource::Rules {
                rules: RuleIdentifier::RuleIds(vec!["rule-1".to_string()]),
            },
        )
        .await
        .expect("create_report failed");
}

#[tokio::test]
async fn create_report_errors_when_no_report_id_returned() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock.expect_evaluate_rules().returning(|_| {
        Ok(Response::new(EvaluateRulesResponse {
            report_id: None,
            job_id: None,
            created_annotation_count: 0,
        }))
    });

    let (service, _h) = service_with_dual_mocks(MockReportServiceImpl::new(), eval_mock).await;

    let err = service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            None,
            vec![],
            ReportSource::Rules {
                rules: RuleIdentifier::RuleIds(vec!["rule-1".to_string()]),
            },
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("missing report_id"));
}

#[tokio::test]
async fn create_report_propagates_grpc_error() {
    let mut eval_mock = MockRuleEvaluationServiceImpl::new();
    eval_mock
        .expect_evaluate_rules()
        .returning(|_| Err(Status::invalid_argument("bad input")));

    let (service, _h) = service_with_dual_mocks(MockReportServiceImpl::new(), eval_mock).await;

    let err = service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            None,
            vec![],
            ReportSource::Template {
                report_template_id: "tmpl-1".to_string(),
            },
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to create report"));
}

#[tokio::test]
async fn update_report_sets_metadata_mask_and_refetches() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_update_report()
        .withf(|req| {
            let req = req.get_ref();
            req.update_mask.as_ref().unwrap().paths == vec!["metadata".to_string()]
                && req.report.as_ref().unwrap().report_id == "rep1"
        })
        .returning(|_| Ok(Response::new(UpdateReportResponse {})));
    mock.expect_get_report()
        .withf(|req| req.get_ref().report_id == "rep1")
        .returning(|_| {
            Ok(Response::new(GetReportResponse {
                report: Some(Report {
                    report_id: "rep1".into(),
                    name: "after update".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let report = service
        .update_report("rep1".to_string(), vec![])
        .await
        .expect("update_report failed");

    assert_eq!(report.name, "after update");
}

#[tokio::test]
async fn update_report_propagates_grpc_error() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_update_report()
        .returning(|_| Err(Status::not_found("no such report")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_report("rep1".to_string(), vec![])
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to update report"));
}
