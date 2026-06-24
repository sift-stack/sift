use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    test_reports::v1::{
        CountTestMeasurementsRequest, CountTestStepsRequest, ListTestMeasurementsRequest,
        ListTestMeasurementsResponse, ListTestReportsRequest, ListTestReportsResponse,
        ListTestStepsRequest, ListTestStepsResponse, TestMeasurement, TestReport, TestStep,
        test_report_service_client::TestReportServiceClient,
    },
};

#[cfg(test)]
mod test;

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
}
