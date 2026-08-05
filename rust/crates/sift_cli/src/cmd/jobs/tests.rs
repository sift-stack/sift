use super::*;

#[test]
fn build_list_filter_produces_empty_string_when_nothing_selected() {
    assert_eq!(build_list_filter(None, None), "");
}

#[test]
fn build_list_filter_produces_type_only() {
    assert_eq!(
        build_list_filter(Some(&JobTypeArg::DataImport), None),
        "job_type == \"JOB_TYPE_DATA_IMPORT\""
    );
}

#[test]
fn build_list_filter_produces_status_only() {
    assert_eq!(
        build_list_filter(None, Some(&JobStatusArg::Failed)),
        "job_status == \"JOB_STATUS_FAILED\""
    );
}

#[test]
fn build_list_filter_ands_type_and_status() {
    assert_eq!(
        build_list_filter(
            Some(&JobTypeArg::RuleEvaluation),
            Some(&JobStatusArg::Running)
        ),
        "job_type == \"JOB_TYPE_RULE_EVALUATION\" && job_status == \"JOB_STATUS_RUNNING\""
    );
}

#[test]
fn exit_code_for_status_matches_documented_semantics() {
    let cases = [
        (JobStatus::Finished, ExitCode::SUCCESS),
        (JobStatus::Failed, ExitCode::from(EXIT_JOB_FAILED)),
        (JobStatus::Cancelled, ExitCode::from(EXIT_JOB_CANCELLED)),
        (
            JobStatus::CancelRequested,
            ExitCode::from(EXIT_JOB_CANCELLED),
        ),
        (JobStatus::Created, ExitCode::from(EXIT_JOB_RUNNING)),
        (JobStatus::Running, ExitCode::from(EXIT_JOB_RUNNING)),
    ];
    for (status, expected) in cases {
        assert_eq!(
            format!("{:?}", exit_code_for_status(status)),
            format!("{expected:?}"),
            "status: {status:?}"
        );
    }
}

#[test]
fn status_label_covers_every_variant() {
    assert_eq!(status_label(JobStatus::Created), "created");
    assert_eq!(status_label(JobStatus::Running), "running");
    assert_eq!(status_label(JobStatus::Finished), "finished");
    assert_eq!(status_label(JobStatus::Failed), "failed");
    assert_eq!(status_label(JobStatus::Cancelled), "cancelled");
    assert_eq!(status_label(JobStatus::CancelRequested), "cancel-requested");
}

#[test]
fn type_label_covers_every_variant() {
    assert_eq!(type_label(JobType::DataImport), "data-import");
    assert_eq!(type_label(JobType::DataExport), "data-export");
    assert_eq!(type_label(JobType::RuleEvaluation), "rule-evaluation");
    assert_eq!(type_label(JobType::Unspecified), "unspecified");
}

#[test]
fn format_timestamp_handles_missing_and_present() {
    assert_eq!(format_timestamp(None), "-");

    let ts = Timestamp {
        seconds: 1_767_225_600,
        nanos: 0,
    };
    assert_eq!(format_timestamp(Some(&ts)), "2026-01-01T00:00:00Z");
}
