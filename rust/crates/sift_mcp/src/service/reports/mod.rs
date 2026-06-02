use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    reports::v1::{
        ListReportsRequest, ListReportsResponse, Report, report_service_client::ReportServiceClient,
    },
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct ReportService {
    channel: SiftChannel,
}

impl ReportService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_reports(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        organization_id: Option<String>,
    ) -> Result<Vec<Report>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = ReportServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_reports(ListReportsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                    organization_id: organization_id.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query reports")?;

            let ListReportsResponse {
                reports,
                next_page_token,
            } = resp.into_inner();
            if reports.is_empty() {
                break;
            }
            results.extend(reports);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
