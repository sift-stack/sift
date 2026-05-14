use anyhow::{Context, Result, anyhow};
use sift_rs::{
    SiftChannel,
    channels::v3::{
        Channel, ListChannelsRequest, ListChannelsResponse,
        channel_service_client::ChannelServiceClient,
    },
};

/// Resolves all channel CLI inputs (names, regex, and explicit IDs) into a combined list of
/// channel IDs, scoped to `asset_ids`. Explicit `ids` are included as-is; name and regex
/// lookups are filtered to channels belonging to the given assets.
pub async fn resolve_channel_ids(
    grpc_channel: SiftChannel,
    names: &[String],
    regex: Option<&str>,
    ids: Vec<String>,
    asset_ids: &[String],
) -> Result<Vec<String>> {
    let asset_ids_cel = asset_ids
        .iter()
        .map(|a| format!("'{a}'"))
        .collect::<Vec<_>>()
        .join(",");

    let mut channel_ids = ids;

    if !names.is_empty() {
        let names_cel = names
            .iter()
            .map(|c| format!("'{c}'"))
            .collect::<Vec<_>>()
            .join(",");
        let filter = format!("asset_id in [{asset_ids_cel}] && name in [{names_cel}]");
        let filtered_channels = filter_channels(grpc_channel.clone(), &filter).await?;

        if filtered_channels.is_empty() {
            return Err(anyhow!(
                "no channels matched the provided `channel` inputs."
            ));
        }

        channel_ids.extend(filtered_channels.iter().map(|c| c.channel_id.clone()));
    }

    if let Some(re) = regex {
        let filter = format!("asset_id in [{asset_ids_cel}] && name.matches(\"{re}\")");
        let filtered_channels = filter_channels(grpc_channel.clone(), &filter).await?;

        if filtered_channels.is_empty() {
            return Err(anyhow!(
                "no channels matched the provided `channel_regex` inputs."
            ));
        }

        channel_ids.extend(filtered_channels.iter().map(|c| c.channel_id.clone()));
    }

    Ok(channel_ids)
}

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

        query_result.extend(channels);

        if next_page_token.is_empty() {
            break;
        }
        page_token = next_page_token;
    }
    Ok(query_result)
}
