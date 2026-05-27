use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    channels::v3::{
        Channel, ListChannelsRequest, ListChannelsResponse,
        channel_service_client::ChannelServiceClient,
    },
};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct ChannelService {
    channel: SiftChannel,
}

impl ChannelService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_channels(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Channel>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = ChannelServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_channels(ListChannelsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query channels")?;

            let ListChannelsResponse {
                channels,
                next_page_token,
            } = resp.into_inner();
            if channels.is_empty() {
                break;
            }
            results.extend(channels);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
