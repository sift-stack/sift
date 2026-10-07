use rmcp::handler::server::wrapper::Parameters;
use serde_json::json;
use sift_rs::test_reports::v1::{
    CountTestStepsResponse, CreateTestMeasurementsResponse, CreateTestReportResponse, TestReport,
    test_report_service_server::TestReportServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::test_reports::v1::MockTestReportServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::{AppendMeasurementsParams, CreateTestReportParams};
use crate::{
    server::SiftMcpServer,
    tool::common::{
        ListParams,
        test_support::{structured, structured_field},
    },
};

async fn server_with_mock(mock: MockTestReportServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(TestReportServiceServer::new(mock))
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
async fn create_test_report_surfaces_url() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_create_test_report().returning(|_| {
        Ok(Response::new(CreateTestReportResponse {
            test_report: Some(TestReport {
                test_report_id: "tr1".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let params = CreateTestReportParams {
        report_json: r#"{"name":"n","test_system_name":"s","test_case":"c"}"#.into(),
    };
    let resp = server
        .create_test_report(Parameters(params))
        .await
        .expect("create_test_report failed");

    let report_url = structured_field(resp, "report_url");
    assert_eq!(report_url, "https://app.test.local/test-results/tr1");
}

#[tokio::test]
async fn append_test_measurements_surfaces_url() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_create_test_measurements().returning(|_| {
        Ok(Response::new(CreateTestMeasurementsResponse {
            measurements_created_count: 1,
            measurement_ids: vec!["m1".into()],
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let params = AppendMeasurementsParams {
        test_report_id: "tr1".into(),
        test_step_id: "ts1".into(),
        measurements_json: r#"[{"name":"v","numeric_value":1.0}]"#.into(),
    };
    let resp = server
        .append_test_measurements(Parameters(params))
        .await
        .expect("append_test_measurements failed");

    let report_url = structured_field(resp, "report_url");
    assert_eq!(report_url, "https://app.test.local/test-results/tr1");
}

#[tokio::test]
async fn list_test_steps_count_only_uses_the_count_rpc() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_steps().times(0);
    mock.expect_count_test_steps()
        .withf(|req| req.get_ref().filter == "test_report_id == \"tr1\"")
        .returning(|_| Ok(Response::new(CountTestStepsResponse { count: 1234 })));

    let (server, _h) = server_with_mock(mock).await;
    let resp = server
        .list_test_steps(Parameters(ListParams {
            filter: "test_report_id == \"tr1\"".into(),
            order_by: None,
            limit: None,
            fields: None,
            count_only: Some(true),
        }))
        .await
        .expect("list_test_steps failed");

    assert_eq!(
        structured(resp),
        json!({ "count": 1234, "has_more": false })
    );
}
