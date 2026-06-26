use sift_rs::test_reports::v1::{
    CountTestMeasurementsResponse, CountTestStepsResponse, CreateTestMeasurementsResponse,
    CreateTestReportResponse, CreateTestStepResponse, ListTestMeasurementsResponse,
    ListTestReportsResponse, ListTestStepsResponse, TestMeasurement, TestMeasurementType,
    TestReport, TestStatus, TestStep, TestStepType, test_measurement,
    test_report_service_server::TestReportServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::test_reports::v1::MockTestReportServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::TestReportService;
use super::spec;
use crate::policy::RetryPolicy;
use crate::service::common::PAGE_SIZE;

fn built_from_json(json: &str) -> spec::BuiltReport {
    let parsed = serde_json::from_str(json).expect("spec should parse");
    spec::build(parsed).expect("spec should build")
}

async fn service_with_mock(mock: MockTestReportServiceImpl) -> (TestReportService, JoinHandle<()>) {
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
        TestReportService::new(channel, RetryPolicy::default()),
        handle,
    )
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
        .count_test_steps("test_report_id == \"r1\" && status == TEST_STATUS_FAILED".to_string())
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

// --- spec::build (pure validation + mapping) ---

#[test]
fn build_computes_step_paths_and_parent_links() {
    let built = built_from_json(
        r#"{
            "name": "r", "test_system_name": "rig", "test_case": "tc",
            "steps": [
                { "name": "first" },
                { "name": "second", "steps": [ { "name": "child" } ] }
            ]
        }"#,
    );

    let paths: Vec<(&str, Option<&str>)> = built
        .steps
        .iter()
        .map(|s| (s.step_path.as_str(), s.parent_path.as_deref()))
        .collect();
    // Pre-order, with computed paths and parent links.
    assert_eq!(paths, vec![("1", None), ("2", None), ("2.1", Some("2"))]);
    assert_eq!(built.steps[2].step.step_path, "2.1");
}

#[test]
fn build_defaults_status_and_step_type() {
    let built = built_from_json(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s" } ] }"#,
    );
    assert_eq!(built.request.status, TestStatus::Passed as i32);
    assert_eq!(built.steps[0].step.status, TestStatus::Passed as i32);
    assert_eq!(built.steps[0].step.step_type, TestStepType::Action as i32);
}

#[test]
fn build_computes_passed_from_numeric_bounds() {
    let built = built_from_json(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s", "measurements": [
                { "name": "in",  "numeric_value": 3.3, "numeric_bounds": { "min": 3.0, "max": 3.6 } },
                { "name": "out", "numeric_value": 9.9, "numeric_bounds": { "min": 3.0, "max": 3.6 } }
             ] } ] }"#,
    );
    let ms = &built.steps[0].measurements;
    assert!(ms[0].passed, "in-bounds should pass");
    assert!(!ms[1].passed, "out-of-bounds should fail");
    assert_eq!(ms[0].measurement_type, TestMeasurementType::Double as i32);
    match &ms[0].value {
        Some(test_measurement::Value::NumericValue(v)) => assert_eq!(*v, 3.3),
        _ => panic!("expected numeric value"),
    }
}

#[test]
fn build_explicit_passed_overrides_bounds() {
    let built = built_from_json(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s", "measurements": [
                { "name": "m", "numeric_value": 9.9, "numeric_bounds": { "max": 1.0 }, "passed": true }
             ] } ] }"#,
    );
    assert!(built.steps[0].measurements[0].passed);
}

#[test]
fn build_derives_string_and_boolean_types() {
    let built = built_from_json(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s", "measurements": [
                { "name": "str", "string_value": "ok", "string_expected": "ok" },
                { "name": "bool", "boolean_value": true }
             ] } ] }"#,
    );
    let ms = &built.steps[0].measurements;
    assert_eq!(ms[0].measurement_type, TestMeasurementType::String as i32);
    assert!(ms[0].passed, "string equal to expected should pass");
    assert_eq!(ms[1].measurement_type, TestMeasurementType::Boolean as i32);
}

#[test]
fn build_rejects_wrong_value_count() {
    let zero = serde_json::from_str(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s", "measurements": [ { "name": "m" } ] } ] }"#,
    )
    .unwrap();
    assert!(spec::build(zero).is_err(), "zero values must be rejected");

    let two = serde_json::from_str(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s", "measurements": [
                { "name": "m", "numeric_value": 1.0, "boolean_value": true } ] } ] }"#,
    )
    .unwrap();
    assert!(spec::build(two).is_err(), "two values must be rejected");
}

#[test]
fn build_rejects_unknown_enum_and_bad_timestamp() {
    let bad_status = serde_json::from_str(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc", "status": "NOPE" }"#,
    )
    .unwrap();
    assert!(spec::build(bad_status).is_err());

    let bad_ts = serde_json::from_str(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc", "start_time": "not-a-time" }"#,
    )
    .unwrap();
    assert!(spec::build(bad_ts).is_err());
}

#[test]
fn build_rejects_empty_required_field() {
    let empty_name =
        serde_json::from_str(r#"{ "name": "", "test_system_name": "rig", "test_case": "tc" }"#)
            .unwrap();
    assert!(spec::build(empty_name).is_err());
}

// --- create_test_report (composite orchestration against the mock) ---

#[tokio::test]
async fn create_test_report_creates_tree_in_order() {
    let mut mock = MockTestReportServiceImpl::new();

    mock.expect_create_test_report().times(1).returning(|_| {
        Ok(Response::new(CreateTestReportResponse {
            test_report: Some(TestReport {
                test_report_id: "r1".into(),
                ..Default::default()
            }),
        }))
    });

    // Echo back an id derived from step_path, and assert report id + parent linkage.
    mock.expect_create_test_step().times(3).returning(|req| {
        let step = req.into_inner().test_step.expect("step present");
        assert_eq!(step.test_report_id, "r1");
        if step.step_path == "2.1" {
            assert_eq!(
                step.parent_step_id, "id-2",
                "child links to parent's real id"
            );
        } else {
            assert_eq!(step.parent_step_id, "", "roots have no parent");
        }
        let test_step_id = format!("id-{}", step.step_path);
        Ok(Response::new(CreateTestStepResponse {
            test_step: Some(TestStep {
                test_step_id,
                ..step
            }),
        }))
    });

    mock.expect_create_test_measurements()
        .times(1)
        .returning(|req| {
            let measurements = req.into_inner().test_measurements;
            assert_eq!(measurements.len(), 1);
            assert_eq!(measurements[0].test_step_id, "id-1");
            assert_eq!(measurements[0].test_report_id, "r1");
            Ok(Response::new(CreateTestMeasurementsResponse {
                measurements_created_count: measurements.len() as i32,
                measurement_ids: vec!["m1".into()],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let built = built_from_json(
        r#"{
            "name": "r", "test_system_name": "rig", "test_case": "tc",
            "steps": [
                { "name": "first", "measurements": [ { "name": "v", "numeric_value": 1.0 } ] },
                { "name": "second", "steps": [ { "name": "child" } ] }
            ]
        }"#,
    );

    let created = service
        .create_test_report(built)
        .await
        .expect("create_test_report failed");

    assert_eq!(created.test_report_id, "r1");
    assert_eq!(created.steps_created, 3);
    assert_eq!(created.measurements_created, 1);
}

#[tokio::test]
async fn create_test_report_surfaces_report_id_on_step_failure() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_create_test_report().returning(|_| {
        Ok(Response::new(CreateTestReportResponse {
            test_report: Some(TestReport {
                test_report_id: "r1".into(),
                ..Default::default()
            }),
        }))
    });
    mock.expect_create_test_step()
        .returning(|_| Err(Status::internal("boom")));

    let (service, _h) = service_with_mock(mock).await;

    let built = built_from_json(
        r#"{ "name": "r", "test_system_name": "rig", "test_case": "tc",
             "steps": [ { "name": "s" } ] }"#,
    );

    let err = service
        .create_test_report(built)
        .await
        .expect_err("expected step failure");

    let msg = err.to_string();
    assert!(
        msg.contains("r1"),
        "error should name the created report id: {msg}"
    );
}

// --- append_test_measurements ---

#[tokio::test]
async fn append_test_measurements_sets_ids_and_batches() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_create_test_measurements()
        .times(1)
        .returning(|req| {
            let measurements = req.into_inner().test_measurements;
            assert_eq!(measurements.len(), 2);
            for m in &measurements {
                assert_eq!(m.test_report_id, "r9");
                assert_eq!(m.test_step_id, "s9");
            }
            Ok(Response::new(CreateTestMeasurementsResponse {
                measurements_created_count: measurements.len() as i32,
                measurement_ids: vec!["m1".into(), "m2".into()],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let measurements = spec::build_measurements(
        serde_json::from_str(
            r#"[
                { "name": "v", "numeric_value": 1.0 },
                { "name": "ok", "boolean_value": true }
            ]"#,
        )
        .unwrap(),
    )
    .expect("measurements should build");

    let created = service
        .append_test_measurements("r9".to_string(), "s9".to_string(), measurements)
        .await
        .expect("append_test_measurements failed");

    assert_eq!(created, 2);
}

#[tokio::test]
async fn append_test_measurements_propagates_grpc_error() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_create_test_measurements()
        .returning(|_| Err(Status::not_found("no such step")));

    let (service, _h) = service_with_mock(mock).await;

    let measurements = spec::build_measurements(
        serde_json::from_str(r#"[ { "name": "v", "numeric_value": 1.0 } ]"#).unwrap(),
    )
    .unwrap();

    let err = service
        .append_test_measurements("r9".to_string(), "missing".to_string(), measurements)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to append test measurements")
    );
}

#[test]
fn normalize_enum_filter_rewrites_canonical_names_to_short_form() {
    use super::normalize_enum_filter;

    assert_eq!(
        normalize_enum_filter("status == \"TEST_STATUS_FAILED\""),
        "status == \"failed\""
    );
    assert_eq!(
        normalize_enum_filter("status == \"TEST_STATUS_IN_PROGRESS\""),
        "status == \"in_progress\""
    );
    assert_eq!(
        normalize_enum_filter("step_type == \"TEST_STEP_TYPE_ACTION\""),
        "step_type == \"action\""
    );
    assert_eq!(
        normalize_enum_filter("measurement_type == \"TEST_MEASUREMENT_TYPE_DOUBLE\""),
        "measurement_type == \"double\""
    );
}

#[test]
fn normalize_enum_filter_handles_compound_and_already_short_filters() {
    use super::normalize_enum_filter;

    // Multiple enum values in one expression are each rewritten.
    assert_eq!(
        normalize_enum_filter(
            "test_report_id == \"r1\" && status == \"TEST_STATUS_FAILED\" \
             && step_type == \"TEST_STEP_TYPE_GROUP\""
        ),
        "test_report_id == \"r1\" && status == \"failed\" && step_type == \"group\""
    );

    // The short form the backend already accepts is left untouched.
    assert_eq!(
        normalize_enum_filter("status == \"failed\""),
        "status == \"failed\""
    );
}

#[test]
fn normalize_enum_filter_leaves_unrelated_values_intact() {
    use super::normalize_enum_filter;

    // Identifiers, operators, and ordinary string values are not enum values and stay verbatim.
    assert_eq!(
        normalize_enum_filter("name == \"nightly\" && passed == false"),
        "name == \"nightly\" && passed == false"
    );

    // A prefix with no suffix is not a valid enum value and is left alone.
    assert_eq!(
        normalize_enum_filter("name == \"TEST_STATUS_\""),
        "name == \"TEST_STATUS_\""
    );
}

#[tokio::test]
async fn count_test_steps_forwards_short_enum_form() {
    let mut mock = MockTestReportServiceImpl::new();
    mock.expect_count_test_steps()
        .withf(|req| req.get_ref().filter == "status == \"failed\"")
        .returning(|_| Ok(Response::new(CountTestStepsResponse { count: 7 })));

    let (service, _h) = service_with_mock(mock).await;

    let count = service
        .count_test_steps("status == \"TEST_STATUS_FAILED\"".to_string())
        .await
        .expect("count_test_steps failed");

    assert_eq!(count, 7);
}
