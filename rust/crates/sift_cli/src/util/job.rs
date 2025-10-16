use std::ops::{Deref, DerefMut};

use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    jobs::v1::{Job, JobType, ListJobsRequest, job_service_client::JobServiceClient},
};

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

    pub async fn get_latest_job_for_user(
        &mut self,
        user_id: &str,
        job_type: JobType,
    ) -> Result<Option<Job>> {
        let jt = job_type.as_str_name();

        let res = self
            .list_jobs(ListJobsRequest {
                page_size: 1,
                filter: format!("job_type == '{jt}' && created_by_user_id == '{user_id}'"),
                order_by: "created_date desc".into(),
                ..Default::default()
            })
            .await
            .context("failed to retrieve latest user job")?
            .into_inner();

        Ok(res.jobs.first().cloned())
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
}
