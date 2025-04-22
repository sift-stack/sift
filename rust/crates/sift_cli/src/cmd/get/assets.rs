use crate::{Filter, Pagination};
use super::super::utils::pbts_to_rfc3339;
use comfy_table::{Attribute, Cell, Table, presets::NOTHING};
use sift_connect::SiftChannel;
use sift_rs::assets::v1::{
    asset_service_client::AssetServiceClient,
    Asset,
    GetAssetRequest,
    ListAssetsRequest,
    ListAssetsResponse,
};
use anyhow::{Result, Context, format_err};
use std::io::Write;

pub async fn run<W: Write>(
    mut out: W,
    filter: Filter,
    pagination: Pagination,
    channel: SiftChannel,
) -> Result<()> {
    let Filter { id, name, iname, cel } = filter;
    let Pagination { max_records } = pagination;

    let mut asset_svc = AssetServiceClient::new(channel);

    let mut assets = Vec::new();

    if let Some(asset_id) = id {
        let asset = asset_svc.get_asset(GetAssetRequest { asset_id })
            .await
            .map(|res| res.into_inner().asset)
            .context("failed to retrieve asset by ID")?
            .ok_or_else(|| format_err!("unexpected empty response"))?;

        assets.push(asset);
    } else if let Some(asset_name) = name {
        assets = list_assets(&mut asset_svc, format!("name.matches('{asset_name}')"), max_records).await?;
    } else if let Some(asset_name) = iname {
        assets = list_assets(&mut asset_svc, format!("name.matches('(?i){asset_name}')"), max_records).await?;
    } else if let Some(cel_filter) = cel {
        assets = list_assets(&mut asset_svc, cel_filter, max_records).await?;
    } else {
        assets = list_assets(&mut asset_svc, String::new(), max_records).await?;
    }

    let mut output = Table::new();
    output.load_preset(NOTHING);
    output.set_header([
        Cell::new("NAME").add_attribute(Attribute::Bold),
        Cell::new("ID").add_attribute(Attribute::Bold),
        Cell::new("CREATED").add_attribute(Attribute::Bold),
        Cell::new("TAGS").add_attribute(Attribute::Bold),
    ]);
    output.add_rows(assets.into_iter().map(|a| [
        a.name,
        a.asset_id,
        pbts_to_rfc3339(a.created_date),
        a.tags.join(","),
    ]));

    writeln!(out, "{output}").context("failed to write to output")
}

async fn list_assets(
    svc: &mut AssetServiceClient<SiftChannel>,
    filter: String,
    max_records: u32,
) -> Result<Vec<Asset>> {
    let pages = (max_records as f32 / 1000.0).ceil() as usize;

    let mut assets_list = Vec::new();
    let mut page_token = String::new();

    for _ in 0..pages {
        let ListAssetsResponse { assets, next_page_token } = svc
            .list_assets(ListAssetsRequest {
                page_token,
                page_size: max_records,
                filter: filter.clone(),
                ..Default::default()
            })
            .await
            .map(|res| res.into_inner())
            .context("failed to filter assets")?;

        page_token = next_page_token;
        assets_list.extend_from_slice(&assets);

        if page_token.is_empty() {
            break;
        }
    }
    Ok(assets_list)
}
