use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    campaigns::v1::{
        Campaign, CampaignReport, GetCampaignReportSummariesRequest,
        GetCampaignReportSummariesResponse, ListCampaignsRequest, ListCampaignsResponse,
        campaign_service_client::CampaignServiceClient,
    },
};
use std::collections::HashMap;

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct CampaignService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl CampaignService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    /// Lists campaigns. Always sets `skip_report_summaries = true` on the
    /// outgoing request so listing stays cheap; the per-report `num_*` fields
    /// are aggregated from every report in a campaign, which is expensive to
    /// compute for campaigns with many reports. Use `review_campaigns` to
    /// fetch those counts explicitly.
    pub async fn list_campaigns(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        include_archived: Option<bool>,
        organization_id: Option<String>,
    ) -> Result<Vec<Campaign>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();
        let include_archived = include_archived.unwrap_or_default();
        let organization_id = organization_id.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let organization_id = organization_id.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let organization_id = organization_id.clone();
                let token = token.clone();
                async move {
                    let mut client = CampaignServiceClient::new(channel);
                    client
                        .list_campaigns(ListCampaignsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            organization_id,
                            include_archived,
                            order_by,
                            skip_report_summaries: true,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query campaigns")?;

            let ListCampaignsResponse {
                campaigns,
                next_page_token,
            } = resp;
            if campaigns.is_empty() {
                break;
            }
            results.extend(campaigns);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    /// Fetches report summaries (annotation totals and rule pass/fail
    /// classifications) for one or more campaigns in bulk. Unlike
    /// `list_campaigns`, this does not skip summaries -- computing them is the
    /// entire point of the call, so only fetch what is actually needed.
    ///
    /// The wire response wraps each campaign's reports in a `CampaignReports`
    /// message (`repeated` fields can't be map values directly in proto3);
    /// that wrapper is unwrapped here since it carries no meaning of its own.
    pub async fn review_campaigns(
        &self,
        campaign_ids: Vec<String>,
        organization_id: Option<String>,
    ) -> Result<HashMap<String, Vec<CampaignReport>>> {
        let channel = self.channel.clone();
        let organization_id = organization_id.unwrap_or_default();

        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let campaign_ids = campaign_ids.clone();
            let organization_id = organization_id.clone();
            async move {
                let mut client = CampaignServiceClient::new(channel);
                client
                    .get_campaign_report_summaries(GetCampaignReportSummariesRequest {
                        campaign_ids,
                        organization_id,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to fetch campaign report summaries")?;

        let GetCampaignReportSummariesResponse {
            summaries_by_campaign_id,
        } = resp;

        Ok(summaries_by_campaign_id
            .into_iter()
            .map(|(campaign_id, reports)| (campaign_id, reports.reports))
            .collect())
    }
}
