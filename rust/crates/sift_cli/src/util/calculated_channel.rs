use anyhow::{Context, Result, anyhow};
use sift_rs::{
    SiftChannel,
    calculated_channels::v2::{
        CalculatedChannel, CalculatedChannelAbstractChannelReference,
        ListCalculatedChannelsRequest, ListCalculatedChannelsResponse,
        ResolveCalculatedChannelRequest, calculated_channel_asset_configuration::AssetScope,
        calculated_channel_service_client::CalculatedChannelServiceClient,
        resolve_calculated_channel_request::CalculatedChannel as RequestCalculatedChannel,
    },
    common::r#type::v1::{
        Ids, NamedResources, ResourceIdentifier, named_resources::Resources,
        resource_identifier::Identifier,
    },
    exports::v1::CalculatedChannelConfig,
};

pub enum ResolveScope<'a> {
    Run(&'a str),
    Assets(&'a [String]),
}

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

pub async fn resolve_to_calculated_channel_configs(
    grpc_channel: SiftChannel,
    channel: &CalculatedChannel,
    scope: &ResolveScope<'_>,
) -> Result<Vec<CalculatedChannelConfig>> {
    let mut service = CalculatedChannelServiceClient::new(grpc_channel);

    let (assets, run) = match scope {
        ResolveScope::Run(run_id) => (
            None,
            Some(ResourceIdentifier {
                identifier: Some(Identifier::Id(run_id.to_string())),
            }),
        ),
        ResolveScope::Assets(asset_ids) => (
            Some(NamedResources {
                resources: Some(Resources::Ids(Ids {
                    ids: asset_ids.to_vec(),
                })),
            }),
            None,
        ),
    };

    let response = service
        .resolve_calculated_channel(ResolveCalculatedChannelRequest {
            assets,
            run,
            calculated_channel: Some(RequestCalculatedChannel::Identifier(ResourceIdentifier {
                identifier: Some(Identifier::Id(channel.calculated_channel_id.clone())),
            })),
            ..Default::default()
        })
        .await
        .with_context(|| format!("failed to resolve calculated channel '{}'", channel.name))?
        .into_inner();

    if !response.unresolved.is_empty() {
        let assets: Vec<_> = response
            .unresolved
            .iter()
            .map(|u| format!("'{}': {}", u.asset_name, u.error_message))
            .collect();
        return Err(anyhow!(
            "calculated channel '{}' could not be resolved for the following assets:\n{}",
            channel.name,
            assets.join("\n")
        ));
    }

    response
        .resolved
        .into_iter()
        .map(|resolved| {
            let expr = resolved.expression_request.ok_or_else(|| {
                anyhow!(
                    "resolved calculated channel '{}' has no expression request",
                    channel.name
                )
            })?;

            let channel_references = expr
                .expression_channel_references
                .into_iter()
                .map(|r| CalculatedChannelAbstractChannelReference {
                    channel_reference: r.channel_reference,
                    channel_identifier: r.channel_id,
                })
                .collect();

            Ok(CalculatedChannelConfig {
                name: channel.name.clone(),
                expression: expr.expression,
                channel_references,
                units: channel.units.clone(),
            })
        })
        .collect()
}
