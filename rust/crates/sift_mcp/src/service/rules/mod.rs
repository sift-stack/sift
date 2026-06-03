use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    rules::v1::{
        ListRulesRequest, ListRulesResponse, Rule, rule_service_client::RuleServiceClient,
    },
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct RuleService {
    channel: SiftChannel,
}

impl RuleService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_rules(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Rule>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = RuleServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_rules(ListRulesRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query rules")?;

            let ListRulesResponse {
                rules,
                next_page_token,
            } = resp.into_inner();
            if rules.is_empty() {
                break;
            }
            results.extend(rules);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
