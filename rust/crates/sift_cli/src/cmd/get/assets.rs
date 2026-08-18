use std::process::ExitCode;

use anyhow::{Context as AnyhowContext, Result};
use sift_rs::assets::v1::{
    ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient,
};

use crate::{
    cli::GetAssetArgs,
    cmd::Context,
    util::{api::create_grpc_channel, tty::Output},
};

const ASSET_ID_CHAR_LENGTH_WHITESPACE_LENGTH: usize = 38;

pub async fn run(ctx: Context, args: GetAssetArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;

    let ListAssetsResponse { assets, .. } = AssetServiceClient::new(grpc_channel)
        .list_assets(ListAssetsRequest {
            filter: args.filter.unwrap_or_default(),
            order_by: "modified_date desc".to_string(),
            page_size: 50,
            ..Default::default()
        })
        .await
        .context("failed to list assets")?
        .into_inner();
    let mut output = Output::new();
    if assets.is_empty() {
        output.line("no assets found");
    } else {
        output.line(format_args!(
            "{:<width$}{}",
            "ID",
            "Name",
            width = ASSET_ID_CHAR_LENGTH_WHITESPACE_LENGTH,
        ));
        for asset in &assets {
            output.line(format_args!(
                "{:<width$}{}",
                asset.asset_id,
                asset.name,
                width = ASSET_ID_CHAR_LENGTH_WHITESPACE_LENGTH,
            ));
        }
    }
    output.print();

    Ok(ExitCode::SUCCESS)
}
