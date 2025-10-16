use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    channels::v3::{
        Channel, ListChannelsRequest, ListChannelsResponse,
        channel_service_client::ChannelServiceClient,
    },
};

pub async fn filter_channels(grpc_channel: SiftChannel, filter: &str) -> Result<Vec<Channel>> {
    let mut channel_service = ChannelServiceClient::new(grpc_channel);
    let mut page_token = String::new();
    let mut query_result = Vec::new();

    loop {
        let ListChannelsResponse {
            channels,
            next_page_token,
            ..
        } = channel_service
            .list_channels(ListChannelsRequest {
                page_token,
                filter: filter.to_string(),
                page_size: 1000,
                ..Default::default()
            })
            .await
            .context("failed to query channels")?
            .into_inner();

        query_result.extend(channels.into_iter());

        if next_page_token.is_empty() {
            break;
        }
        page_token = next_page_token;
    }
    Ok(query_result)
}
