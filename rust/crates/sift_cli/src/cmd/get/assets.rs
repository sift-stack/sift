use std::process::ExitCode;

use anyhow::{Context as AnyhowContext, Result};
use sift_rs::assets::v1::{
    ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient,
};

use crate::{
    BIN_NAME,
    cli::{GetAssetArgs, OutputFormats},
    cmd::Context,
    util::{
        api::create_grpc_channel,
        app_uri::normalize_app_uri,
        explore_url::build_explore_url,
        tty::{Output, hyperlink, link_style, stdout_is_tty},
    },
};

const ASSET_ID_CHAR_LENGTH_WHITESPACE_LENGTH: usize = 38;

pub async fn run(ctx: Context, args: GetAssetArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;

    let ListAssetsResponse { assets, .. } = AssetServiceClient::new(grpc_channel)
        .list_assets(ListAssetsRequest {
            filter: args.common.filter.unwrap_or_default(),
            order_by: args.common.order_by,
            page_size: args.common.limit,
            ..Default::default()
        })
        .await
        .context("failed to list assets")?
        .into_inner();
    let app_uri = ctx.app_uri.as_deref().and_then(normalize_app_uri);
    let mut output = Output::new();
    match args.common.output_format {
        Some(OutputFormats::Json) => {
            output.line(serde_json::to_string_pretty(&assets).context("failed to encode assets")?);
        }
        Some(OutputFormats::Text) | None => {
            if assets.is_empty() {
                output.line("no assets found");
            } else {
                let linkify = stdout_is_tty();

                output.line(format_args!(
                    "{:<width$}{}",
                    "ID",
                    "Name",
                    width = ASSET_ID_CHAR_LENGTH_WHITESPACE_LENGTH,
                ));
                for asset in &assets {
                    let name = match build_explore_url(app_uri, &asset.name, None) {
                        Some(url) if linkify => hyperlink(&link_style(&asset.name), &url),
                        _ => asset.name.clone(),
                    };
                    output.line(format_args!(
                        "{:<width$}{name}",
                        asset.asset_id,
                        width = ASSET_ID_CHAR_LENGTH_WHITESPACE_LENGTH,
                    ));
                }
                output.tip(match app_uri {
                    Some(host) => format!(
                        "View an asset in Explore at {host}/explore?method=single&assets=<NAME>"
                    ),
                    None => format!(
                        "Run `{BIN_NAME} config update --app-uri <SIFT_WEB_ORIGIN>` for Explore \
                         links."
                    ),
                });
            }
        }
    }
    output.print();

    Ok(ExitCode::SUCCESS)
}
