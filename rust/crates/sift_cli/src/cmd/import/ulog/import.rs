use std::{fs::File, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result};
use chrono::DateTime;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::ChannelConfig,
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse, UlogConfig,
        UlogParseErrorPolicy as ProtoUlogParseErrorPolicy,
        data_import_service_client::DataImportServiceClient,
    },
};

use crate::{
    cli::ImportUlogArgs,
    cmd::{
        Context,
        import::{
            finish_import, preview_import_config, ulog::detect_ulog_config::detect_config,
            utils::upload_gzipped_file,
        },
    },
    util::{api::create_grpc_channel, tty::Output},
};

pub async fn run(ctx: Context, args: ImportUlogArgs) -> Result<ExitCode> {
    let grpc_channel =
        create_grpc_channel(&ctx).context("failed to create grpc channel for ulog import")?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let ulog_config = build_ulog_config(&args).context("failed to build ulog config")?;

    if args.common.preview {
        let run_label = if ulog_config.run_id.is_empty() {
            ulog_config.run_name.as_str()
        } else {
            ulog_config.run_id.as_str()
        };

        match detect_config(&args.common.path) {
            Ok(channel_configs) => {
                let refs: Vec<&ChannelConfig> = channel_configs.iter().collect();
                preview_import_config(&args.common.asset, run_label, None, &refs);
            }
            Err(e) => {
                preview_import_config(&args.common.asset, run_label, None, &[]);
                Output::new()
                    .line(format!("client-side preview parse failed: {e:#}"))
                    .tip("the server-side parser may still ingest this file correctly")
                    .eprint();
            }
        }
        return Ok(ExitCode::SUCCESS);
    }

    let file = File::open(&args.common.path).context("failed to open ulog file")?;

    let create_data_import_req = create_data_import_request(ulog_config);

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import for ulog")?
        .into_inner();

    let job_id = upload_gzipped_file(
        &ctx,
        &upload_url,
        file,
        &args.common.path,
        "application/octet-stream",
    )
    .await
    .context("failed to upload ulog file")?;

    finish_import(
        &ctx,
        grpc_channel,
        job_id,
        &args.common.asset,
        args.common.run.as_deref(),
        args.common.run_id.as_deref(),
        args.common.wait,
    )
    .await
}

pub fn build_ulog_config(args: &ImportUlogArgs) -> Result<UlogConfig> {
    if (!args.info_key.is_empty() || !args.param_key.is_empty())
        && args.common.run.is_none()
        && args.common.run_id.is_none()
    {
        anyhow::bail!("--info-key and --param-key require --run or --run-id");
    }

    let relative_start_time = match &args.relative_start_time {
        Some(start) => {
            let dt = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            Some(Timestamp::from(dt.to_utc()))
        }
        None => None,
    };

    // Send only one run identifier; --run-id takes precedence over --run.
    let run_name = if args.common.run_id.is_some() {
        String::new()
    } else {
        args.common.run.clone().unwrap_or_default()
    };

    Ok(UlogConfig {
        asset_name: args.common.asset.clone(),
        run_name,
        run_id: args.common.run_id.clone().unwrap_or_default(),
        relative_start_time,
        info_keys: args.info_key.clone(),
        param_keys: args.param_key.clone(),
        parse_error_policy: ProtoUlogParseErrorPolicy::from(args.parse_error_policy).into(),
        // Empty `data` tells the server to import every detected channel.
        ..Default::default()
    })
}

fn create_data_import_request(config: UlogConfig) -> CreateDataImportFromUploadRequest {
    CreateDataImportFromUploadRequest {
        ulog_config: Some(config),
        ..Default::default()
    }
}
