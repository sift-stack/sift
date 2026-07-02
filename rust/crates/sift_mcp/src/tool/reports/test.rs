use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::reports::v1::{
    CreateReportResponse, GetReportResponse, ListReportRuleSummariesResponse, ListReportsResponse,
    Report, ReportRuleSummary, UpdateReportResponse, report_service_server::ReportServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::reports::v1::MockReportServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{
    CreateReportParams, ReportListParams, ReportRuleSummaryListParams, UpdateReportParams,
};
use crate::{server::SiftMcpServer, tool::common::test_support::structured_field};

fn create_report_params() -> CreateReportParams {
    CreateReportParams {
        run_id: "run-1".into(),
        name: "nightly report".into(),
        organization_id: None,
        metadata: None,
        report_template_id: None,
        description: None,
        tag_names: None,
        rule_ids: None,
        rule_client_keys: None,
        rule_version_ids: None,
    }
}

async fn server_with_mock(mock: MockReportServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
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
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
        handle,
    )
}

fn report_params(filter: &str, limit: Option<u32>) -> Parameters<ReportListParams> {
    Parameters(ReportListParams {
        filter: filter.into(),
        order_by: None,
        limit,
        organization_id: None,
    })
}

#[tokio::test]
async fn list_reports_returns_single_page() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_list_reports()
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

    let (server, _h) = server_with_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("name == \"nightly\"", None))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    assert_eq!(reports.as_array().unwrap().len(), 1);
    assert_eq!(reports[0]["reportId"], "rep1");
    assert_eq!(reports[0]["url"], "https://app.test.local/reports/rep1");
}

#[tokio::test]
async fn list_reports_forwards_organization_id() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_list_reports()
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

    let (server, _h) = server_with_mock(report_mock).await;

    let resp = server
        .list_reports(Parameters(ReportListParams {
            filter: String::new(),
            order_by: None,
            limit: None,
            organization_id: Some("org-123".into()),
        }))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    assert_eq!(reports.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn list_reports_paginates_until_token_empty() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_list_reports().returning(|req| {
        let token = req.into_inner().page_token;
        let (reports, next) = match token.as_str() {
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

    let (server, _h) = server_with_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("", None))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    let ids: Vec<&str> = reports
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["reportId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["rep1", "rep2"]);
}

#[tokio::test]
async fn list_reports_truncates_to_limit_across_pages() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_list_reports().returning(|req| {
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

    let (server, _h) = server_with_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("", Some(3)))
        .await
        .expect("list_reports failed");

    let reports = structured_field(resp, "reports");
    let ids = reports
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["reportId"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["rep1", "rep2", "rep3"]);
}

#[tokio::test]
async fn list_reports_breaks_on_empty_page() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock.expect_list_reports().times(1).returning(|_| {
        Ok(Response::new(ListReportsResponse {
            reports: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (server, _h) = server_with_mock(report_mock).await;

    let resp = server
        .list_reports(report_params("", None))
        .await
        .expect("list_reports failed");

    assert!(
        structured_field(resp, "reports")
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn list_reports_propagates_grpc_error() {
    let mut report_mock = MockReportServiceImpl::new();
    report_mock
        .expect_list_reports()
        .returning(|_| Err(Status::not_found("no such report")));

    let (server, _h) = server_with_mock(report_mock).await;

    let err = server
        .list_reports(report_params("", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("no such report"));
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

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_report_rule_summaries(Parameters(ReportRuleSummaryListParams {
            report_id: "rep1".into(),
            filter: String::new(),
            order_by: None,
            limit: None,
        }))
        .await
        .expect("list_report_rule_summaries failed");

    let summaries = structured_field(resp, "report_rule_summaries");
    assert_eq!(summaries.as_array().unwrap().len(), 1);
    assert_eq!(summaries[0]["ruleId"], "rule-1");
}

#[tokio::test]
async fn list_report_rule_summaries_rejects_empty_report_id() {
    let (server, _h) = server_with_mock(MockReportServiceImpl::new()).await;

    let err = server
        .list_report_rule_summaries(Parameters(ReportRuleSummaryListParams {
            report_id: String::new(),
            filter: String::new(),
            order_by: None,
            limit: None,
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_from_rules_happy_path() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_create_report().returning(|_| {
        Ok(Response::new(CreateReportResponse {
            report: Some(Report {
                report_id: "rep-new".into(),
                name: "nightly report".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = create_report_params();
    params.rule_ids = Some(vec!["rule-1".into()]);

    let resp = server
        .create_report(Parameters(params))
        .await
        .expect("create_report failed");

    let report_url = structured_field(resp.clone(), "report_url");
    assert_eq!(report_url, "https://app.test.local/reports/rep-new");
    let report = structured_field(resp, "report");
    assert_eq!(report["reportId"], "rep-new");
}

#[tokio::test]
async fn create_report_rejects_no_source() {
    let (server, _h) = server_with_mock(MockReportServiceImpl::new()).await;

    let err = server
        .create_report(Parameters(create_report_params()))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_rejects_both_sources() {
    let (server, _h) = server_with_mock(MockReportServiceImpl::new()).await;

    let mut params = create_report_params();
    params.report_template_id = Some("tmpl-1".into());
    params.rule_ids = Some(vec!["rule-1".into()]);

    let err = server
        .create_report(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_rejects_multiple_rule_identifiers() {
    let (server, _h) = server_with_mock(MockReportServiceImpl::new()).await;

    let mut params = create_report_params();
    params.rule_ids = Some(vec!["rule-1".into()]);
    params.rule_client_keys = Some(vec!["ck-1".into()]);

    let err = server
        .create_report(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_report_happy_path() {
    let mut mock = MockReportServiceImpl::new();
    mock.expect_update_report()
        .returning(|_| Ok(Response::new(UpdateReportResponse {})));
    mock.expect_get_report().returning(|_| {
        Ok(Response::new(GetReportResponse {
            report: Some(Report {
                report_id: "rep1".into(),
                name: "after update".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .update_report(Parameters(UpdateReportParams {
            report_id: "rep1".into(),
            metadata: vec![],
        }))
        .await
        .expect("update_report failed");

    let report = structured_field(resp, "report");
    assert_eq!(report["name"], "after update");
}

#[tokio::test]
async fn update_report_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockReportServiceImpl::new()).await;

    let err = server
        .update_report(Parameters(UpdateReportParams {
            report_id: String::new(),
            metadata: vec![],
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
