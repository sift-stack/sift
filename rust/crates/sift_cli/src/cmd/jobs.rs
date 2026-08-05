use std::process::ExitCode;

use anyhow::{Context as AnyhowContext, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use crossterm::style::Stylize;
use pbjson_types::Timestamp;
use sift_rs::jobs::v1::{Job, JobStatus, JobType, ListJobsRequest};

use crate::cli::{GetJobArgs, GetJobsArgs, JobStatusArg, JobTypeArg, StatusJobArgs, WaitJobArgs};
use crate::util::{api::create_grpc_channel, job::JobServiceWrapper, tty::Output};

use super::Context;

const EXIT_JOB_FAILED: u8 = 1;
const EXIT_JOB_CANCELLED: u8 = 2;
const EXIT_JOB_RUNNING: u8 = 3;

pub async fn get_jobs(ctx: Context, args: GetJobsArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut job_service = JobServiceWrapper::new(grpc_channel);

    let filter = build_list_filter(args.job_type.as_ref(), args.status.as_ref());
    let jobs = job_service
        .list_jobs(ListJobsRequest {
            page_size: args.limit,
            filter,
            order_by: "created_date desc".to_string(),
            ..Default::default()
        })
        .await
        .context("failed to list jobs")?
        .into_inner()
        .jobs;

    if jobs.is_empty() {
        Output::new().line("No jobs matched").print();
        return Ok(ExitCode::SUCCESS);
    }

    print_jobs_table(&jobs);
    Ok(ExitCode::SUCCESS)
}

pub async fn get_job(ctx: Context, args: GetJobArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut job_service = JobServiceWrapper::new(grpc_channel);

    let Some(job) = job_service.get_job(&args.job_id).await? else {
        Output::new()
            .line(format!("Job `{}` not found", args.job_id))
            .eprint();
        return Ok(ExitCode::FAILURE);
    };

    print_job_details(&job);
    Ok(ExitCode::SUCCESS)
}

pub async fn status_job(ctx: Context, args: StatusJobArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut job_service = JobServiceWrapper::new(grpc_channel);

    let Some(job) = job_service.get_job(&args.job_id).await? else {
        Output::new()
            .line(format!("Job `{}` not found", args.job_id))
            .eprint();
        return Ok(ExitCode::FAILURE);
    };

    let status = job.job_status();
    println!("{}", status_label(status));
    Ok(exit_code_for_status(status))
}

pub async fn wait_job(ctx: Context, args: WaitJobArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;

    let mut handles = Vec::with_capacity(args.job_ids.len());
    for job_id in args.job_ids {
        let channel = grpc_channel.clone();
        handles.push(tokio::spawn(async move {
            let mut svc = JobServiceWrapper::new(channel);
            let outcome = svc.poll_until_terminal(&job_id, |_| {}).await;
            (job_id, outcome)
        }));
    }

    let mut any_failed = false;
    for handle in handles {
        let (job_id, outcome) = handle.await.context("wait task panicked")?;
        match outcome {
            Ok(Some(job)) => {
                let status = job.job_status();
                let line = format!("{}: {}", job_id.cyan(), status_label(status));
                match status {
                    JobStatus::Finished => Output::new().line(line).print(),
                    JobStatus::Cancelled => Output::new().line(line).print(),
                    _ => {
                        any_failed = true;
                        Output::new().line(line).eprint();
                    }
                }
            }
            Ok(None) => {
                any_failed = true;
                Output::new()
                    .line(format!("Job `{job_id}` disappeared before it finished"))
                    .eprint();
            }
            Err(err) => {
                any_failed = true;
                Output::new()
                    .line(format!("Job `{job_id}` failed to poll: {err:#}"))
                    .eprint();
            }
        }
    }

    if any_failed {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn build_list_filter(job_type: Option<&JobTypeArg>, status: Option<&JobStatusArg>) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(t) = job_type {
        parts.push(format!("job_type == \"{}\"", job_type_cel(t)));
    }
    if let Some(s) = status {
        parts.push(format!("job_status == \"{}\"", job_status_cel(s)));
    }
    parts.join(" && ")
}

fn job_type_cel(t: &JobTypeArg) -> &'static str {
    match t {
        JobTypeArg::DataImport => "JOB_TYPE_DATA_IMPORT",
        JobTypeArg::DataExport => "JOB_TYPE_DATA_EXPORT",
        JobTypeArg::RuleEvaluation => "JOB_TYPE_RULE_EVALUATION",
    }
}

fn job_status_cel(s: &JobStatusArg) -> &'static str {
    match s {
        JobStatusArg::Created => "JOB_STATUS_CREATED",
        JobStatusArg::Running => "JOB_STATUS_RUNNING",
        JobStatusArg::Finished => "JOB_STATUS_FINISHED",
        JobStatusArg::Failed => "JOB_STATUS_FAILED",
        JobStatusArg::Cancelled => "JOB_STATUS_CANCELLED",
        JobStatusArg::CancelRequested => "JOB_STATUS_CANCEL_REQUESTED",
    }
}

fn status_label(status: JobStatus) -> String {
    match status {
        JobStatus::Created => "created".to_string(),
        JobStatus::Running => "running".to_string(),
        JobStatus::Finished => "finished".to_string(),
        JobStatus::Failed => "failed".to_string(),
        JobStatus::Cancelled => "cancelled".to_string(),
        JobStatus::CancelRequested => "cancel-requested".to_string(),
        other => format!("{other:?}"),
    }
}

fn type_label(job_type: JobType) -> &'static str {
    match job_type {
        JobType::DataImport => "data-import",
        JobType::DataExport => "data-export",
        JobType::RuleEvaluation => "rule-evaluation",
        JobType::Unspecified => "unspecified",
    }
}

fn exit_code_for_status(status: JobStatus) -> ExitCode {
    match status {
        JobStatus::Finished => ExitCode::SUCCESS,
        JobStatus::Failed => ExitCode::from(EXIT_JOB_FAILED),
        JobStatus::Cancelled | JobStatus::CancelRequested => ExitCode::from(EXIT_JOB_CANCELLED),
        JobStatus::Created | JobStatus::Running => ExitCode::from(EXIT_JOB_RUNNING),
        _ => ExitCode::from(EXIT_JOB_RUNNING),
    }
}

fn print_jobs_table(jobs: &[Job]) {
    let mut out = Output::new();
    out.line(format!(
        "{:<38}  {:<15}  {:<17}  {}",
        "JOB ID", "TYPE", "STATUS", "CREATED",
    ));
    for job in jobs {
        out.line(format!(
            "{:<38}  {:<15}  {:<17}  {}",
            job.job_id,
            type_label(job.job_type()),
            status_label(job.job_status()),
            format_timestamp(job.created_date.as_ref()),
        ));
    }
    out.print();
}

fn format_timestamp(ts: Option<&Timestamp>) -> String {
    let Some(ts) = ts else {
        return "-".to_string();
    };
    DateTime::<Utc>::from_timestamp(ts.seconds, ts.nanos as u32)
        .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true))
        .unwrap_or_else(|| "-".to_string())
}

fn print_job_details(job: &Job) {
    let mut out = Output::new();
    out.line(format!("{}: {}", "Job ID".green(), job.job_id));
    out.line(format!(
        "{}: {}",
        "Type".green(),
        type_label(job.job_type())
    ));
    out.line(format!(
        "{}: {}",
        "Status".green(),
        status_label(job.job_status())
    ));
    if job.created_date.is_some() {
        out.line(format!(
            "{}: {}",
            "Created".green(),
            format_timestamp(job.created_date.as_ref())
        ));
    }
    if job.started_date.is_some() {
        out.line(format!(
            "{}: {}",
            "Started".green(),
            format_timestamp(job.started_date.as_ref())
        ));
    }
    if job.completed_date.is_some() {
        out.line(format!(
            "{}: {}",
            "Completed".green(),
            format_timestamp(job.completed_date.as_ref())
        ));
    }
    if job.job_status() == JobStatus::Failed {
        if let Some(details) = job.job_status_details.as_ref() {
            out.line(format!("{}: {:?}", "Failure details".red(), details));
        }
    }
    out.print();
}

#[cfg(test)]
mod tests {
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
}
