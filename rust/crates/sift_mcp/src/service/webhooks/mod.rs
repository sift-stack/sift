use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, bail};
use reqwest::Url;
use sift_rs::{
    SiftChannel,
    webhooks::v1::{
        ListWebhooksRequest, ListWebhooksResponse, Webhook,
        webhook_service_client::WebhookServiceClient,
    },
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct WebhookService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl WebhookService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    /// Lists webhooks with their secrets withheld; see [`redact`].
    pub async fn list_webhooks(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<common::Page<Webhook>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();
        let mut has_more = false;

        let order_by = order_by.unwrap_or_default();

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
                    let mut client = WebhookServiceClient::new(channel);
                    client
                        .list_webhooks(ListWebhooksRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query webhooks")?;

            let ListWebhooksResponse {
                webhooks,
                next_page_token,
            } = resp;
            if webhooks.is_empty() {
                break;
            }
            results.extend(webhooks.into_iter().map(redact));

            if results.len() >= record_limit {
                // The cap, not the end of the data: report that more exist so the
                // caller does not read this page's size as the match total.
                has_more = results.len() > record_limit || !next_page_token.is_empty();
                break;
            }
            if next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(common::Page {
            items: results,
            has_more,
        })
    }

    /// Fetches the webhooks with the given ids, archived ones included. An id
    /// with no webhook is absent from the result, so the caller can tell a
    /// missing webhook from an archived one.
    pub async fn get_webhooks(&self, webhook_ids: &[String]) -> Result<Vec<Webhook>> {
        if webhook_ids.is_empty() {
            return Ok(Vec::new());
        }

        let ids = webhook_ids
            .iter()
            .map(|id| format!("\"{}\"", common::cel_escape(id)))
            .collect::<Vec<_>>()
            .join(", ");

        let page = self
            .list_webhooks(
                format!("webhook_id in [{ids}]"),
                None,
                Some(common::PAGE_SIZE),
            )
            .await?;

        if page.has_more {
            bail!(
                "webhook lookup matched more than {} webhooks and is incomplete; \
                 pass fewer webhook ids",
                common::PAGE_SIZE,
            );
        }
        Ok(page.items)
    }
}

/// Withholds the credentials a webhook carries. `ListWebhooks` returns them in
/// plaintext: header values such as `Authorization`, target URLs that are
/// themselves the secret (a Slack or Teams path, a Pipedream or Lambda
/// function URL subdomain), and body templates that embed a key (PagerDuty's
/// `routing_key`). Headers keep only their names, the body template is
/// dropped, and the target is cut by [`redact_target`].
fn redact(mut webhook: Webhook) -> Webhook {
    webhook.target_url = redact_target(&webhook.target_url);
    for header in &mut webhook.http_headers {
        header.value.clear();
    }
    webhook.payload = None;
    webhook
}

/// Cuts a target URL to its scheme and the last two labels of its host, such
/// as `https://slack.com`: enough to tell where a webhook sends, with no path,
/// query, port, user info, or per-endpoint subdomain left to replay. An IP
/// address host is kept whole. Empty when the URL has no host.
fn redact_target(target_url: &str) -> String {
    let Ok(url) = Url::parse(target_url) else {
        return String::new();
    };
    let host = match url.domain() {
        Some(domain) => {
            let labels: Vec<&str> = domain.split('.').collect();
            labels[labels.len().saturating_sub(2)..].join(".")
        }
        None => match url.host_str() {
            Some(ip) => ip.to_string(),
            None => return String::new(),
        },
    };
    format!("{}://{host}", url.scheme())
}
