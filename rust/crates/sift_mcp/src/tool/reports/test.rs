use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::reports::v1::{
    ListReportsResponse, Report, report_service_server::ReportServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::reports::v1::MockReportServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::ReportListParams;
use crate::{server::SiftMcpServer, tool::common::test_support::structured_field};

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
