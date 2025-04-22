use crate::{Filter, Pagination, AssetIdentifier};
use super::super::utils::pbts_to_rfc3339;
use crossterm::style::Stylize;
use comfy_table::{Attribute, Cell, Table, presets::NOTHING};
use sift_connect::SiftChannel;
use sift_rs::{
    assets::v1::{
        asset_service_client::AssetServiceClient,
        ListAssetsRequest,
    },
    channels::v3::{
        channel_service_client::ChannelServiceClient,
        Channel,
        GetChannelRequest,
        ListChannelsRequest,
        ListChannelsResponse,
    },
    common::r#type::v1::ChannelDataType,
};
use anyhow::{Result, Context, format_err};
use std::io::Write;

pub async fn run<W: Write>(
    mut out: W,
    asset: AssetIdentifier,
    filter: Filter,
    pagination: Pagination,
    channel: SiftChannel,
) -> Result<()> {
    let AssetIdentifier { asset_id, asset_name } = asset;
    let Filter { id, name, iname, cel } = filter;
    let Pagination { max_records } = pagination;

    let mut channel_svc = ChannelServiceClient::new(channel.clone());

    let mut channels = Vec::new();

    if let Some(channel_id) = id {
        let channel = channel_svc.get_channel(GetChannelRequest { channel_id })
            .await
            .map(|res| res.into_inner().channel)
            .context("failed to retrieve channel by ID")?
            .ok_or_else(|| format_err!("unexpected empty response"))?;

        channels.push(channel);
    } else if let Some(cel_filter) = cel {
        channels = list_channels(&mut channel_svc, cel_filter, max_records).await?;
    } else if iname.is_some() || name.is_some() {
        let parent_asset_id = match (asset_id, asset_name) {
            (None, None) => {
                return Err(format_err!("{} or {} must be provided when filtering by name", "--asset-id".bold().cyan(), "--asset-name".bold().cyan()));
            }
            (Some(id), _) => id,
            (None, Some(name)) => {
                let mut asset_svc = AssetServiceClient::new(channel);
                let assets = asset_svc
                    .list_assets(ListAssetsRequest {
                        filter: format!("name == '{name}'"),
                        ..Default::default()
                    })
                    .await
                    .map(|res| res.into_inner().assets)
                    .context("failed to retrieve asset by name")?;

                if assets.is_empty() {
                    return Err(format_err!("failed to find asset with name {}", name.bold().cyan()));
                }
                assets.first().unwrap().asset_id.clone()
            }
        };
        if let Some(channel_name) = name {
            channels = list_channels(
                &mut channel_svc,
                format!("asset_id == '{parent_asset_id}' && name.matches('{channel_name}')"),
                max_records,
            ).await?;
        } else if let Some(channel_name) = iname {
            channels = list_channels(
                &mut channel_svc,
                format!("asset_id == '{parent_asset_id}' && name.matches('(?i){channel_name}')"),
                max_records,
            ).await?;
        } else {
            channels = list_channels(
                &mut channel_svc,
                format!("asset_id == '{parent_asset_id}'"),
                max_records,
            ).await?;
        }
    } else {
        channels = list_channels(&mut channel_svc, String::new(), max_records).await?;
    }

    let mut output = Table::new();
    output.load_preset(NOTHING);
    output.set_header([
        Cell::new("NAME").add_attribute(Attribute::Bold),
        Cell::new("ID").add_attribute(Attribute::Bold),
        Cell::new("DATA_TYPE").add_attribute(Attribute::Bold),
        Cell::new("CREATED").add_attribute(Attribute::Bold),
    ]);
    output.add_rows(channels.into_iter().map(|c| [
        c.name,
        c.channel_id,
        ChannelDataType::try_from(c.data_type).map(|c| c.as_str_name().to_string()).unwrap_or_default(),
        pbts_to_rfc3339(c.created_date),
    ]));

    writeln!(out, "{output}").context("failed to write to output")
}

async fn list_channels(
    svc: &mut ChannelServiceClient<SiftChannel>,
    filter: String,
    max_records: u32,
) -> Result<Vec<Channel>> {
    let pages = (max_records as f32 / 1000.0).ceil() as usize;

    let mut channels_list = Vec::new();
    let mut page_token = String::new();

    for _ in 0..pages {
        let ListChannelsResponse { channels, next_page_token } = svc
            .list_channels(ListChannelsRequest {
                page_token,
                page_size: max_records,
                filter: filter.clone(),
                ..Default::default()
            })
            .await
            .map(|res| res.into_inner())
            .context("failed to filter channels")?;

        page_token = next_page_token;
        channels_list.extend_from_slice(&channels);

        if page_token.is_empty() {
            break;
        }
    }
    Ok(channels_list)
}
