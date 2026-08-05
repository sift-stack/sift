use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    jobs::v1::{Job, JobStatus, ListJobsRequest, job_service_client::JobServiceClient},
};
use tokio::time::sleep;

const POLL_INTERVAL: Duration = Duration::from_secs(3);

pub fn is_terminal_status(status: JobStatus) -> bool {
    matches!(
        status,
        JobStatus::Finished | JobStatus::Failed | JobStatus::Cancelled
    )
}

pub struct JobServiceWrapper(JobServiceClient<SiftChannel>);

impl Deref for JobServiceWrapper {
    type Target = JobServiceClient<SiftChannel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JobServiceWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JobServiceWrapper {
    pub fn new(grpc_channel: SiftChannel) -> Self {
        let job_service = JobServiceClient::new(grpc_channel);
        JobServiceWrapper(job_service)
    }

    pub async fn get_job(&mut self, job_id: &str) -> Result<Option<Job>> {
        let res = self
            .list_jobs(ListJobsRequest {
                page_size: 1,
                filter: format!("job_id == '{job_id}'"),
                ..Default::default()
            })
            .await
            .context("failed to retrieve job by ID")?
            .into_inner();

        Ok(res.jobs.first().cloned())
    }

    pub async fn poll_until_terminal<F>(
        &mut self,
        job_id: &str,
        mut on_status_change: F,
    ) -> Result<Option<Job>>
    where
        F: FnMut(&Job),
    {
        let Some(mut job) = self.get_job(job_id).await? else {
            return Ok(None);
        };
        on_status_change(&job);
        let mut last_status = job.job_status();

        while !is_terminal_status(last_status) {
            sleep(POLL_INTERVAL).await;
            let Some(updated) = self.get_job(job_id).await? else {
                return Ok(None);
            };
            if updated.job_status() != last_status {
                last_status = updated.job_status();
                on_status_change(&updated);
            }
            job = updated;
        }

        Ok(Some(job))
    }
}

#[cfg(test)]
mod tests {
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
}
