use anyhow::{Context, Result, anyhow};
use sift_rs::{
    SiftChannel,
    calculated_channels::v2::{
        CalculatedChannel, ListCalculatedChannelsRequest, ListCalculatedChannelsResponse,
        calculated_channel_query_configuration::Query,
        calculated_channel_service_client::CalculatedChannelServiceClient,
    },
    exports::v1::CalculatedChannelConfig,
};

pub async fn filter_calculated_channels(
    grpc_channel: SiftChannel,
    filter: &str,
) -> Result<Vec<CalculatedChannel>> {
    let mut service = CalculatedChannelServiceClient::new(grpc_channel);
    let mut page_token = String::new();
    let mut query_result = Vec::new();

    loop {
        let ListCalculatedChannelsResponse {
            calculated_channels,
            next_page_token,
            ..
        } = service
            .list_calculated_channels(ListCalculatedChannelsRequest {
                page_token,
                filter: filter.to_string(),
                page_size: 1000,
                ..Default::default()
            })
            .await
            .context("failed to query calculated channels")?
            .into_inner();

        query_result.extend(calculated_channels.into_iter());

        if next_page_token.is_empty() {
            break;
        }
        page_token = next_page_token;
    }
    Ok(query_result)
}

pub fn channel_applies_to_assets(channel: &CalculatedChannel, asset_ids: &[String]) -> bool {
    use sift_rs::calculated_channels::v2::calculated_channel_asset_configuration::AssetScope;

    let Some(config) = &channel.calculated_channel_configuration else {
        return true;
    };
    let Some(asset_config) = &config.asset_configuration else {
        return true;
    };
    match &asset_config.asset_scope {
        None => true,
        Some(AssetScope::AllAssets(_)) => true,
        Some(AssetScope::Selection(selection)) => {
            selection.asset_ids.iter().any(|id| asset_ids.contains(id))
        }
    }
}

pub fn to_calculated_channel_config(channel: CalculatedChannel) -> Result<CalculatedChannelConfig> {
    let name = channel.name.clone();
    let units = channel.units.clone();

    let config = channel
        .calculated_channel_configuration
        .ok_or_else(|| anyhow!("calculated channel '{name}' has no configuration"))?;

    let query_config = config
        .query_configuration
        .ok_or_else(|| anyhow!("calculated channel '{name}' has no query configuration"))?;

    let sel = match query_config.query {
        Some(Query::Sel(sel)) => sel,
        None => return Err(anyhow!("calculated channel '{name}' has no query")),
    };

    Ok(CalculatedChannelConfig {
        name,
        expression: sel.expression,
        channel_references: sel.expression_channel_references,
        units,
    })
}
