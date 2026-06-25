use std::{fs::File, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::DateTime;
use crossterm::style::Stylize;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::ChannelConfig,
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse, Hdf5Config,
        TimeFormat as ProtoTimeFormat, data_import_service_client::DataImportServiceClient,
    },
};

use crate::{
    cli::ImportHdf5Args,
    cmd::{
        Context,
        import::{
            hdf5::detect_hdf5_schema::{SUPPORTED_TYPES_BLURB, SkippedDataset, detect_config},
            preview_import_config,
            utils::{upload_gzipped_file, validate_time_format},
            wait_for_job_completion,
        },
    },
    util::{
        api::create_grpc_channel,
        explore_url::{build_explore_url, pending_import_tip},
        tty::Output,
    },
};

fn print_skipped_block(skipped: &[SkippedDataset]) {
    if skipped.is_empty() {
        return;
    }
    let mut out = Output::new();
    out.line(format!("{}: {{", "Skipped".green()));
    for s in skipped {
        out.line(format!("  {}: {}", s.path.clone().yellow(), s.reason));
    }
    out.line("}");
    out.line(format!("Supported types: {SUPPORTED_TYPES_BLURB}"));
    out.print();
}

pub async fn run(ctx: Context, args: ImportHdf5Args) -> Result<ExitCode> {
    let grpc_channel =
        create_grpc_channel(&ctx).context("failed to create grpc channel for hdf5 import")?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let mut hdf5_config = build_hdf5_config(&args).context("failed to build hdf5 config")?;

    if args.common.preview {
        let run_label = if hdf5_config.run_id.is_empty() {
            hdf5_config.run_name.as_str()
        } else {
            hdf5_config.run_id.as_str()
        };

        match detect_config(
            &args.common.path,
            args.schema,
            args.time_index.unwrap_or(0),
            args.time_field.as_deref(),
            args.time_name.as_deref(),
        ) {
            Ok((_, channel_configs, skipped)) => {
                let refs: Vec<&ChannelConfig> = channel_configs.iter().collect();
                preview_import_config(&args.common.asset, run_label, None, &refs);
                print_skipped_block(&skipped);
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

    let (data_configs, _, skipped) = detect_config(
        &args.common.path,
        args.schema,
        args.time_index.unwrap_or(0),
        args.time_field.as_deref(),
        args.time_name.as_deref(),
    )
    .context("failed to parse hdf5 file")?;
    hdf5_config.data = data_configs;

    if !skipped.is_empty() {
        Output::new()
            .line(format!(
                "{}: {} dataset(s) skipped — run with --preview to see details",
                "warning".yellow(),
                skipped.len()
            ))
            .eprint();
    }

    let file = File::open(&args.common.path).context("failed to open hdf5 file")?;

    let create_data_import_req = create_data_import_request(hdf5_config);

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import for hdf5")?
        .into_inner();

    let job_id = upload_gzipped_file(
        &ctx,
        &upload_url,
        file,
        &args.common.path,
        "application/x-hdf5",
    )
    .await
    .context("failed to upload hdf5 file")?;

    let run_identifier = args.common.run_id.as_deref().or(args.common.run.as_deref());
    let explore_url = build_explore_url(ctx.app_uri.as_deref(), &args.common.asset, run_identifier);

    let location = run_identifier.map_or_else(
        || format!("asset '{}'", args.common.asset.cyan()),
        |r| format!("run '{}'", r.cyan()),
    );

    if !args.common.wait {
        Output::new()
            .line(format!("{} file for processing", "Uploaded".green()))
            .tip(pending_import_tip(&location, explore_url.as_deref()))
            .print();

        return Ok(ExitCode::SUCCESS);
    }

    wait_for_job_completion(grpc_channel, job_id, location, explore_url).await
}

pub fn build_hdf5_config(args: &ImportHdf5Args) -> Result<Hdf5Config> {
    let time_format = args
        .time_format
        .ok_or_else(|| anyhow!("--time-format is required for HDF5 imports"))?;
    validate_time_format(time_format, &args.relative_start_time)
        .context("validating time format for hdf5")?;

    let relative_start_time = match &args.relative_start_time {
        Some(start) => {
            let dt = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            Some(Timestamp::from(dt.to_utc()))
        }
        None => None,
    };

    Ok(Hdf5Config {
        asset_name: args.common.asset.clone(),
        run_name: args.common.run.clone().unwrap_or_default(),
        run_id: args.common.run_id.clone().unwrap_or_default(),
        data: Vec::new(),
        time_format: ProtoTimeFormat::from(time_format) as i32,
        relative_start_time,
    })
}

fn create_data_import_request(config: Hdf5Config) -> CreateDataImportFromUploadRequest {
    CreateDataImportFromUploadRequest {
        hdf5_config: Some(config),
        ..Default::default()
    }
}
