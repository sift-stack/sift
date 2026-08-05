use super::*;

#[test]
fn terminal_statuses_stop_polling() {
    assert!(is_terminal_status(JobStatus::Finished));
    assert!(is_terminal_status(JobStatus::Failed));
    assert!(is_terminal_status(JobStatus::Cancelled));
}

#[test]
fn non_terminal_statuses_keep_polling() {
    assert!(!is_terminal_status(JobStatus::Created));
    assert!(!is_terminal_status(JobStatus::Running));
    assert!(!is_terminal_status(JobStatus::CancelRequested));
}
