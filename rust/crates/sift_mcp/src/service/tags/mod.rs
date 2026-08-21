use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    tags::v2::{ListTagsRequest, ListTagsResponse, Tag, tag_service_client::TagServiceClient},
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct TagService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl TagService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_tags(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Tag>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

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
                    let mut client = TagServiceClient::new(channel);
                    client
                        .list_tags(ListTagsRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                            // Not exposed as a tool parameter: 0 is TAG_TYPE_UNSPECIFIED,
                            // which matches tags of every type.
                            tag_type: 0,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query tags")?;

            let ListTagsResponse {
                tags,
                next_page_token,
            } = resp;
            if tags.is_empty() {
                break;
            }
            results.extend(tags);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
