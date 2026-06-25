use sift_rs::reports::v1::{
    CreateReportResponse, GetReportResponse, ListReportRuleSummariesResponse, ListReportsResponse,
    Report, ReportRuleSummary, UpdateReportResponse, create_report_request,
    report_service_server::ReportServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::reports::v1::MockReportServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{ReportService, ReportSource, RuleIdentifier};
use crate::service::common::PAGE_SIZE;

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
        assert_eq!(req.page_size, PAGE_SIZE);
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
        assert_eq!(req.page_size, PAGE_SIZE);
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
                report_rule_summaries: vec![
                    ReportRuleSummary {
                        rule_id: "rule-1".into(),
                        ..Default::default()
                    },
                    ReportRuleSummary {
                        rule_id: "rule-2".into(),
                        ..Default::default()
                    },
                ],
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
async fn create_report_from_rules_maps_oneof() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_create_report()
        .withf(|req| {
            let req = req.get_ref();
            req.run_id == "run-1"
                && req.name.as_deref() == Some("nightly report")
                && matches!(
                    req.request,
                    Some(create_report_request::Request::ReportFromRulesRequest(_))
                )
        })
        .returning(|_| {
            Ok(Response::new(CreateReportResponse {
                report: Some(Report {
                    report_id: "rep-new".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let report = service
        .create_report(
            None,
            "run-1".to_string(),
            "nightly report".to_string(),
            None,
            ReportSource::Rules {
                description: None,
                tag_names: vec![],
                rules: RuleIdentifier::RuleIds(vec!["rule-1".to_string()]),
            },
        )
        .await
        .expect("create_report failed");

    assert_eq!(report.report_id, "rep-new");
}

#[tokio::test]
async fn create_report_from_template_maps_oneof() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_create_report()
        .withf(|req| {
            matches!(
                req.get_ref().request,
                Some(create_report_request::Request::ReportFromReportTemplateRequest(_))
            )
        })
        .returning(|_| {
            Ok(Response::new(CreateReportResponse {
                report: Some(Report {
                    report_id: "rep-tmpl".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let report = service
        .create_report(
            None,
            "run-1".to_string(),
            "from template".to_string(),
            None,
            ReportSource::Template {
                report_template_id: "tmpl-1".to_string(),
            },
        )
        .await
        .expect("create_report failed");

    assert_eq!(report.report_id, "rep-tmpl");
}

#[tokio::test]
async fn create_report_propagates_grpc_error() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_create_report()
        .returning(|_| Err(Status::invalid_argument("bad input")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_report(
            None,
            "run-1".to_string(),
            "x".to_string(),
            None,
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
