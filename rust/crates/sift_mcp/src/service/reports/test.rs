use sift_rs::reports::v1::{
    ListReportsResponse, Report, report_service_server::ReportServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::reports::v1::MockReportServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::ReportService;
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

    (ReportService::new(channel), handle)
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
