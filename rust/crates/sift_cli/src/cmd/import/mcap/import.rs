use std::{fs::File, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result};
use chrono::DateTime;
use crossterm::style::Stylize;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse,
        McapComplexTypesImportMode as ProtoMcapComplexTypesImportMode, McapConfig,
        McapParseErrorPolicy as ProtoMcapParseErrorPolicy,
        data_import_service_client::DataImportServiceClient,
    },
};

use crate::{
    cli::{ImportMcapArgs, mcap::McapComplexTypesMode},
    cmd::{
        Context,
        import::{
            finish_import,
            mcap::detect_mcap_config::{DetectedChannel, detect_config},
            preview_import_config,
            utils::upload_gzipped_file,
        },
    },
    util::{api::create_grpc_channel, tty::Output},
};

/// Suffix given to the JSON channel of a variable-cardinality field.
const JSON_CHANNEL_SUFFIX: &str = ".json";

pub async fn run(ctx: Context, args: ImportMcapArgs) -> Result<ExitCode> {
    let grpc_channel =
        create_grpc_channel(&ctx).context("failed to create grpc channel for mcap import")?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let mcap_config = build_mcap_config(&args).context("failed to build mcap config")?;

    if args.common.preview {
        let run_label = if mcap_config.run_id.is_empty() {
            mcap_config.run_name.as_str()
        } else {
            mcap_config.run_id.as_str()
        };

        match detect_config(&args.common.path) {
            Ok(detection) => {
                let channel_configs =
                    channel_configs(&detection.channels, args.complex_types_import_mode);
                let refs: Vec<&ChannelConfig> = channel_configs.iter().collect();
                preview_import_config(&args.common.asset, run_label, None, &refs);

                if !detection.warnings.is_empty() {
                    let mut out = Output::new();
                    for warning in &detection.warnings {
                        out.line(format!("{}: {warning}", "warning".yellow()));
                    }
                    out.print();
                }
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

    let file = File::open(&args.common.path).context("failed to open mcap file")?;

    let create_data_import_req = create_data_import_request(mcap_config);

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import for mcap")?
        .into_inner();

    let job_id = upload_gzipped_file(
        &ctx,
        &upload_url,
        file,
        &args.common.path,
        "application/octet-stream",
    )
    .await
    .context("failed to upload mcap file")?;

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

pub fn build_mcap_config(args: &ImportMcapArgs) -> Result<McapConfig> {
    if !args.metadata_record.is_empty() && args.common.run.is_none() && args.common.run_id.is_none()
    {
        anyhow::bail!("--metadata-record requires --run or --run-id");
    }

    let relative_start_time = match &args.relative_start_time {
        Some(start) => {
            let dt = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            Some(Timestamp::from(dt.to_utc()))
        }
        None => None,
    };

    Ok(McapConfig {
        asset_name: args.common.asset.clone(),
        run_name: args.common.run.clone().unwrap_or_default(),
        run_id: args.common.run_id.clone().unwrap_or_default(),
        relative_start_time,
        metadata_records: args.metadata_record.clone(),
        parse_error_policy: ProtoMcapParseErrorPolicy::from(args.parse_error_policy).into(),
        complex_types_import_mode: ProtoMcapComplexTypesImportMode::from(
            args.complex_types_import_mode,
        )
        .into(),
        ..Default::default()
    })
}

/// Expands detected channels into the Sift channels the import creates.
///
/// A variable-cardinality field is one detected channel, but the complex-types
/// mode decides which channels it becomes: Arrow IPC bytes under the field's
/// name, a JSON string under `<name>.json`, both, or neither.
pub fn channel_configs(
    detected: &[DetectedChannel],
    mode: McapComplexTypesMode,
) -> Vec<ChannelConfig> {
    let mut configs = Vec::new();

    for channel in detected {
        if !channel.complex {
            configs.push(ChannelConfig {
                name: channel.name.clone(),
                data_type: channel.data_type.into(),
                ..Default::default()
            });
            continue;
        }
        if mode == McapComplexTypesMode::Ignore {
            continue;
        }
        if matches!(
            mode,
            McapComplexTypesMode::Bytes | McapComplexTypesMode::Both
        ) {
            configs.push(ChannelConfig {
                name: channel.name.clone(),
                data_type: ChannelDataType::Bytes.into(),
                ..Default::default()
            });
        }
        if matches!(
            mode,
            McapComplexTypesMode::String | McapComplexTypesMode::Both
        ) {
            configs.push(ChannelConfig {
                name: format!("{}{JSON_CHANNEL_SUFFIX}", channel.name),
                data_type: ChannelDataType::String.into(),
                ..Default::default()
            });
        }
    }
    configs
}

fn create_data_import_request(config: McapConfig) -> CreateDataImportFromUploadRequest {
    CreateDataImportFromUploadRequest {
        mcap_config: Some(config),
        ..Default::default()
    }
}
