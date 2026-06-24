use std::collections::HashMap;

use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use sift_rs::{
    SiftChannel,
    test_reports::v1::{
        CountTestMeasurementsRequest, CountTestStepsRequest, CreateTestMeasurementsRequest,
        CreateTestStepRequest, ListTestMeasurementsRequest, ListTestMeasurementsResponse,
        ListTestReportsRequest, ListTestReportsResponse, ListTestStepsRequest,
        ListTestStepsResponse, TestMeasurement, TestReport, TestStep,
        test_report_service_client::TestReportServiceClient,
    },
};

pub mod spec;
use spec::{BuiltReport, BuiltStep};

#[cfg(test)]
mod test;

/// Summary of a report created by [`TestReportService::create_test_report`].
#[derive(Debug)]
pub struct CreatedReport {
    pub test_report_id: String,
    pub steps_created: usize,
    pub measurements_created: usize,
}

#[derive(Clone)]
pub struct TestReportService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl TestReportService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_test_reports(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<TestReport>> {
        let (page_size, record_limit) = common::paging(limit);
        let order_by = order_by.unwrap_or_default();
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = TestReportServiceClient::new(channel);
                    client
                        .list_test_reports(ListTestReportsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query test reports")?;

            let ListTestReportsResponse {
                test_reports,
                next_page_token,
            } = resp;
            if test_reports.is_empty() {
                break;
            }
            results.extend(test_reports);
            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        Ok(results)
    }

    pub async fn list_test_steps(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<TestStep>> {
        let (page_size, record_limit) = common::paging(limit);
        let order_by = order_by.unwrap_or_default();
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = TestReportServiceClient::new(channel);
                    client
                        .list_test_steps(ListTestStepsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query test steps")?;

            let ListTestStepsResponse {
                test_steps,
                next_page_token,
            } = resp;
            if test_steps.is_empty() {
                break;
            }
            results.extend(test_steps);
            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        Ok(results)
    }

    pub async fn list_test_measurements(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<TestMeasurement>> {
        let (page_size, record_limit) = common::paging(limit);
        let order_by = order_by.unwrap_or_default();
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = TestReportServiceClient::new(channel);
                    client
                        .list_test_measurements(ListTestMeasurementsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query test measurements")?;

            let ListTestMeasurementsResponse {
                test_measurements,
                next_page_token,
            } = resp;
            if test_measurements.is_empty() {
                break;
            }
            results.extend(test_measurements);
            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        Ok(results)
    }

    pub async fn count_test_steps(&self, filter: String) -> Result<i64> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let filter = filter.clone();
            async move {
                let mut client = TestReportServiceClient::new(channel);
                client
                    .count_test_steps(CountTestStepsRequest { filter })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to count test steps")?;

        Ok(resp.count)
    }

    pub async fn count_test_measurements(&self, filter: String) -> Result<i64> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let filter = filter.clone();
            async move {
                let mut client = TestReportServiceClient::new(channel);
                client
                    .count_test_measurements(CountTestMeasurementsRequest { filter })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to count test measurements")?;

        Ok(resp.count)
    }

    /// Create a full report tree: the report, then each step (parent before child, linking
    /// `parent_step_id` from the server-assigned ids), then all measurements in one batch.
    ///
    /// These RPCs are not transactional. On a mid-sequence failure the report (and any steps
    /// already created) remain; the returned error names the created `test_report_id` so the
    /// caller can inspect or clean up.
    pub async fn create_test_report(&self, report: BuiltReport) -> Result<CreatedReport> {
        let BuiltReport { request, steps } = report;

        let channel = self.channel.clone();
        let report_resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let request = request.clone();
            async move {
                let mut client = TestReportServiceClient::new(channel);
                client
                    .create_test_report(request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create test report")?;

        let test_report_id = report_resp
            .test_report
            .ok_or_else(|| anyhow!("create test report response missing report"))?
            .test_report_id;

        let mut path_to_id: HashMap<String, String> = HashMap::new();
        let mut measurements: Vec<TestMeasurement> = Vec::new();
        let mut steps_created = 0usize;

        for built in steps {
            let BuiltStep {
                step_path,
                parent_path,
                mut step,
                measurements: step_measurements,
            } = built;

            step.test_report_id = test_report_id.clone();
            step.parent_step_id = match parent_path {
                Some(parent) => path_to_id
                    .get(&parent)
                    .cloned()
                    .ok_or_else(|| anyhow!("internal: parent path `{parent}` not created yet"))?,
                None => String::new(),
            };

            let channel = self.channel.clone();
            let report_id = test_report_id.clone();
            let path = step_path.clone();
            let step_resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let step = step.clone();
                async move {
                    let mut client = TestReportServiceClient::new(channel);
                    client
                        .create_test_step(CreateTestStepRequest {
                            test_step: Some(step),
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .with_context(|| {
                format!(
                    "failed to create test step `{path}` for report `{report_id}` \
                     ({steps_created} step(s) already created)"
                )
            })?;

            let step_id = step_resp
                .test_step
                .ok_or_else(|| anyhow!("create test step response missing step"))?
                .test_step_id;
            path_to_id.insert(step_path, step_id.clone());
            steps_created += 1;

            for mut measurement in step_measurements {
                measurement.test_step_id = step_id.clone();
                measurement.test_report_id = test_report_id.clone();
                measurements.push(measurement);
            }
        }

        let measurements_created = if measurements.is_empty() {
            0
        } else {
            let channel = self.channel.clone();
            let report_id = test_report_id.clone();
            let count = measurements.len();
            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let measurements = measurements.clone();
                async move {
                    let mut client = TestReportServiceClient::new(channel);
                    client
                        .create_test_measurements(CreateTestMeasurementsRequest {
                            test_measurements: measurements,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .with_context(|| {
                format!(
                    "failed to create {count} test measurement(s) for report `{report_id}` \
                     (report and all steps already created)"
                )
            })?;
            resp.measurements_created_count as usize
        };

        Ok(CreatedReport {
            test_report_id,
            steps_created,
            measurements_created,
        })
    }

    /// Append measurements to an existing step. Sets `test_report_id`/`test_step_id` on each and
    /// sends one batch. The report and step must already exist; the server rejects unknown ids
    /// (surfaced as `INVALID_PARAMS`/`RESOURCE_NOT_FOUND`). Returns the count created.
    pub async fn append_test_measurements(
        &self,
        test_report_id: String,
        test_step_id: String,
        mut measurements: Vec<TestMeasurement>,
    ) -> Result<usize> {
        for measurement in &mut measurements {
            measurement.test_report_id = test_report_id.clone();
            measurement.test_step_id = test_step_id.clone();
        }

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let measurements = measurements.clone();
            async move {
                let mut client = TestReportServiceClient::new(channel);
                client
                    .create_test_measurements(CreateTestMeasurementsRequest {
                        test_measurements: measurements,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to append test measurements")?;

        Ok(resp.measurements_created_count as usize)
    }
}
