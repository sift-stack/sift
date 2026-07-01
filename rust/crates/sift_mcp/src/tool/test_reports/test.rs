use rmcp::handler::server::wrapper::Parameters;
use sift_rs::test_reports::v1::{
    CreateTestMeasurementsResponse, CreateTestReportResponse, ListTestMeasurementsResponse,
    ListTestReportsResponse, ListTestStepsResponse, TestMeasurement, TestReport, TestStep,
    UpdateTestMeasurementResponse, UpdateTestReportResponse, UpdateTestStepResponse,
    test_report_service_server::TestReportServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::test_reports::v1::MockTestReportServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::{
    AppendMeasurementsParams, CreateTestReportParams, ExportTestReportParams,
    UpdateTestMeasurementParams, UpdateTestReportParams, UpdateTestStepParams,
};
use crate::{server::SiftMcpServer, tool::common::test_support::structured_field};

/// A `UpdateTestReportParams` with only the id set, so tests set just the fields they exercise.
fn report_params(test_report_id: &str) -> UpdateTestReportParams {
    UpdateTestReportParams {
        test_report_id: test_report_id.into(),
        status: None,
        name: None,
        test_system_name: None,
        test_case: None,
        start_time: None,
        end_time: None,
        serial_number: None,
        part_number: None,
        system_operator: None,
        run_id: None,
        is_archived: None,
    }
}

fn step_params(test_step_id: &str) -> UpdateTestStepParams {
    UpdateTestStepParams {
        test_step_id: test_step_id.into(),
        name: None,
        description: None,
        step_type: None,
        step_path: None,
        status: None,
        start_time: None,
        end_time: None,
        error_code: None,
        error_message: None,
        metadata: None,
    }
}

fn measurement_params(measurement_id: &str) -> UpdateTestMeasurementParams {
    UpdateTestMeasurementParams {
        measurement_id: measurement_id.into(),
        name: None,
        measurement_type: None,
        numeric_value: None,
        string_value: None,
        boolean_value: None,
        unit: None,
        numeric_bounds_min: None,
        numeric_bounds_max: None,
        string_expected: None,
        passed: None,
        timestamp: None,
        description: None,
        channel_names: None,
        metadata: None,
    }
}

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
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
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
async fn update_test_report_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockTestReportServiceImpl::new()).await;

    let mut params = report_params("");
    params.name = Some("x".into());
    let err = server
        .update_test_report(Parameters(params))
        .await
        .expect_err("expected empty-id rejection");
    assert!(err.message.contains("test_report_id"));
}

#[tokio::test]
async fn update_test_report_rejects_no_fields() {
    let (server, _h) = server_with_mock(MockTestReportServiceImpl::new()).await;

    let err = server
        .update_test_report(Parameters(report_params("tr1")))
        .await
        .expect_err("expected no-fields rejection");
    assert!(err.message.contains("at least one updatable field"));
}

#[tokio::test]
async fn update_test_report_surfaces_url() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_update_test_report().returning(|_| {
        Ok(Response::new(UpdateTestReportResponse {
            test_report: Some(TestReport {
                test_report_id: "tr1".into(),
                name: "renamed".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = report_params("tr1");
    params.name = Some("renamed".into());
    let resp = server
        .update_test_report(Parameters(params))
        .await
        .expect("update_test_report failed");

    let report_url = structured_field(resp, "report_url");
    assert_eq!(report_url, "https://app.test.local/test-results/tr1");
}

#[tokio::test]
async fn update_test_step_rejects_partial_error_info() {
    let (server, _h) = server_with_mock(MockTestReportServiceImpl::new()).await;

    let mut params = step_params("ts1");
    params.error_code = Some(7); // error_message missing
    let err = server
        .update_test_step(Parameters(params))
        .await
        .expect_err("expected partial error_info rejection");
    assert!(err.message.contains("error_code` and `error_message"));
}

#[tokio::test]
async fn update_test_step_surfaces_url() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_update_test_step().returning(|_| {
        Ok(Response::new(UpdateTestStepResponse {
            test_step: Some(TestStep {
                test_step_id: "ts1".into(),
                test_report_id: "tr1".into(),
                name: "step".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = step_params("ts1");
    params.description = Some("note".into());
    let resp = server
        .update_test_step(Parameters(params))
        .await
        .expect("update_test_step failed");

    let report_url = structured_field(resp, "report_url");
    assert_eq!(report_url, "https://app.test.local/test-results/tr1");
}

#[tokio::test]
async fn update_test_measurement_rejects_multiple_values() {
    let (server, _h) = server_with_mock(MockTestReportServiceImpl::new()).await;

    let mut params = measurement_params("m1");
    params.numeric_value = Some(1.0);
    params.boolean_value = Some(true);
    let err = server
        .update_test_measurement(Parameters(params))
        .await
        .expect_err("expected multiple-value rejection");
    assert!(err.message.contains("at most one"));
}

#[tokio::test]
async fn update_test_measurement_rejects_conflicting_bounds() {
    let (server, _h) = server_with_mock(MockTestReportServiceImpl::new()).await;

    let mut params = measurement_params("m1");
    params.numeric_bounds_max = Some(5.0);
    params.string_expected = Some("ok".into());
    let err = server
        .update_test_measurement(Parameters(params))
        .await
        .expect_err("expected conflicting-bounds rejection");
    assert!(err.message.contains("not both"));
}

#[tokio::test]
async fn update_test_measurement_surfaces_url() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_update_test_measurement().returning(|_| {
        Ok(Response::new(UpdateTestMeasurementResponse {
            test_measurement: Some(TestMeasurement {
                measurement_id: "m1".into(),
                test_report_id: "tr1".into(),
                name: "meas".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = measurement_params("m1");
    params.passed = Some(false);
    let resp = server
        .update_test_measurement(Parameters(params))
        .await
        .expect("update_test_measurement failed");

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
async fn export_test_report_writes_file_and_surfaces_url() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_list_test_reports().returning(|_| {
        Ok(Response::new(ListTestReportsResponse {
            test_reports: vec![TestReport {
                test_report_id: "tr1".into(),
                name: "nightly".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });
    mock.expect_list_test_steps().returning(|_| {
        Ok(Response::new(ListTestStepsResponse {
            test_steps: vec![],
            next_page_token: String::new(),
        }))
    });
    mock.expect_list_test_measurements().returning(|_| {
        Ok(Response::new(ListTestMeasurementsResponse {
            test_measurements: vec![],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let out = std::env::temp_dir().join("sift_mcp_export_test_report_test.json");
    let params = ExportTestReportParams {
        test_report_id: "tr1".into(),
        output: out.clone(),
        include_measurements: Some(true),
    };
    let resp = server
        .export_test_report(Parameters(params))
        .await
        .expect("export_test_report failed");

    let report_url = structured_field(resp, "report_url");
    assert_eq!(report_url, "https://app.test.local/test-results/tr1");

    let written = std::fs::read_to_string(&out).expect("snapshot file written");
    assert!(written.contains("\"test_report_id\": \"tr1\""));
    assert!(written.contains("\"name\": \"nightly\""));
    let _ = std::fs::remove_file(&out);
}
