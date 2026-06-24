use sift_rs::test_reports::v1::{
    CountTestMeasurementsResponse, CountTestStepsResponse, ListTestMeasurementsResponse,
    ListTestReportsResponse, ListTestStepsResponse, TestMeasurement, TestReport, TestStep,
    test_report_service_server::TestReportServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::test_reports::v1::MockTestReportServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::TestReportService;
use crate::policy::RetryPolicy;
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(
    mock: MockTestReportServiceImpl,
) -> (TestReportService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(TestReportServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (TestReportService::new(channel, RetryPolicy::default()), handle)
}

#[tokio::test]
async fn list_test_reports_returns_single_page() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_reports()
        .withf(|req| req.get_ref().filter == "name == \"nightly\"")
        .returning(|_| {
            Ok(Response::new(ListTestReportsResponse {
                test_reports: vec![
                    TestReport {
                        test_report_id: "r1".into(),
                        name: "nightly".into(),
                        ..Default::default()
                    },
                    TestReport {
                        test_report_id: "r2".into(),
                        name: "nightly".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_test_reports("name == \"nightly\"".to_string(), None, None)
        .await
        .expect("list_test_reports failed");

    assert_eq!(reports.len(), 2);
    assert_eq!(reports[0].test_report_id, "r1");
    assert_eq!(reports[1].test_report_id, "r2");
}

#[tokio::test]
async fn list_test_reports_paginates_until_token_empty() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_reports().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (test_reports, next) = match req.page_token.as_str() {
            "" => (
                vec![TestReport {
                    test_report_id: "r1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![TestReport {
                    test_report_id: "r2".into(),
                    ..Default::default()
                }],
                "page-3".to_string(),
            ),
            "page-3" => (
                vec![TestReport {
                    test_report_id: "r3".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListTestReportsResponse {
            test_reports,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_test_reports(String::new(), None, None)
        .await
        .expect("list_test_reports failed");

    let ids: Vec<&str> = reports.iter().map(|r| r.test_report_id.as_str()).collect();
    assert_eq!(ids, vec!["r1", "r2", "r3"]);
}

#[tokio::test]
async fn list_test_reports_respects_limit() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_reports().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListTestReportsResponse {
            test_reports: vec![
                TestReport {
                    test_report_id: "r1".into(),
                    ..Default::default()
                },
                TestReport {
                    test_report_id: "r2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let reports = service
        .list_test_reports(String::new(), None, Some(2))
        .await
        .expect("list_test_reports failed");

    assert_eq!(reports.len(), 2);
}

#[tokio::test]
async fn list_test_reports_propagates_grpc_error() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_reports()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_test_reports("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query test reports"));
}

#[tokio::test]
async fn list_test_steps_returns_single_page() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_steps()
        .withf(|req| req.get_ref().filter == "test_report_id == \"r1\"")
        .returning(|_| {
            Ok(Response::new(ListTestStepsResponse {
                test_steps: vec![
                    TestStep {
                        test_step_id: "s1".into(),
                        test_report_id: "r1".into(),
                        step_path: "1".into(),
                        ..Default::default()
                    },
                    TestStep {
                        test_step_id: "s2".into(),
                        test_report_id: "r1".into(),
                        step_path: "1.1".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let steps = service
        .list_test_steps("test_report_id == \"r1\"".to_string(), None, None)
        .await
        .expect("list_test_steps failed");

    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].step_path, "1");
    assert_eq!(steps[1].step_path, "1.1");
}

#[tokio::test]
async fn list_test_measurements_returns_single_page() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_measurements()
        .withf(|req| req.get_ref().filter == "test_step_id == \"s1\"")
        .returning(|_| {
            Ok(Response::new(ListTestMeasurementsResponse {
                test_measurements: vec![TestMeasurement {
                    measurement_id: "m1".into(),
                    test_step_id: "s1".into(),
                    test_report_id: "r1".into(),
                    passed: true,
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let measurements = service
        .list_test_measurements("test_step_id == \"s1\"".to_string(), None, None)
        .await
        .expect("list_test_measurements failed");

    assert_eq!(measurements.len(), 1);
    assert_eq!(measurements[0].measurement_id, "m1");
    assert!(measurements[0].passed);
}

#[tokio::test]
async fn count_test_steps_returns_count() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_count_test_steps()
        .withf(|req| {
            req.get_ref().filter == "test_report_id == \"r1\" && status == TEST_STATUS_FAILED"
        })
        .returning(|_| Ok(Response::new(CountTestStepsResponse { count: 7 })));

    let (service, _h) = service_with_mock(mock).await;

    let count = service
        .count_test_steps(
            "test_report_id == \"r1\" && status == TEST_STATUS_FAILED".to_string(),
        )
        .await
        .expect("count_test_steps failed");

    assert_eq!(count, 7);
}

#[tokio::test]
async fn count_test_steps_propagates_grpc_error() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_count_test_steps()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .count_test_steps("nope".to_string())
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to count test steps"));
}

#[tokio::test]
async fn count_test_measurements_returns_count() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_count_test_measurements()
        .withf(|req| req.get_ref().filter == "passed == false")
        .returning(|_| Ok(Response::new(CountTestMeasurementsResponse { count: 3 })));

    let (service, _h) = service_with_mock(mock).await;

    let count = service
        .count_test_measurements("passed == false".to_string())
        .await
        .expect("count_test_measurements failed");

    assert_eq!(count, 3);
}
