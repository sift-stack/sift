use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    runs::v2::{ListRunsRequest, ListRunsResponse, Run, run_service_client::RunServiceClient},
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct RunService {
    channel: SiftChannel,
}

impl RunService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_runs(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Run>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = RunServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_runs(ListRunsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query runs")?;

            let ListRunsResponse {
                runs,
                next_page_token,
            } = resp.into_inner();
            if runs.is_empty() {
                break;
            }
            results.extend(runs);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
