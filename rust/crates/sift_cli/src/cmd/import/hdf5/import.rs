use std::{fs::File, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::DateTime;
use crossterm::style::Stylize;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::ChannelConfig,
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse, Hdf5Config,
        TimeFormat as ProtoTimeFormat,
        data_import_service_client::DataImportServiceClient,
    },
};

use crate::{
    cli::ImportHdf5Args,
    cmd::{
        Context,
        import::{
            hdf5::detect_hdf5_schema::detect_config,
            preview_import_config,
            utils::{upload_gzipped_file, validate_time_format},
            wait_for_job_completion,
        },
    },
    util::{api::create_grpc_channel, tty::Output},
};

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
            args.time_index,
            args.time_field.as_deref(),
        ) {
            Ok((_, channel_configs)) => {
                let refs: Vec<&ChannelConfig> = channel_configs.iter().collect();
                preview_import_config(&args.common.asset, run_label, &refs);
            }
            Err(e) => {
                preview_import_config(&args.common.asset, run_label, &[]);
                Output::new()
                    .line(format!("client-side preview parse failed: {e:#}"))
                    .tip("the server-side parser may still ingest this file correctly")
                    .eprint();
            }
        }
        return Ok(ExitCode::SUCCESS);
    }

    let (data_configs, _) = detect_config(
        &args.common.path,
        args.schema,
        args.time_index,
        args.time_field.as_deref(),
    )
    .context("failed to parse hdf5 file")?;
    hdf5_config.data = data_configs;

    let file = File::open(&args.common.path).context("failed to open hdf5 file")?;

    let create_data_import_req = create_data_import_request(hdf5_config);

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import for hdf5")?
        .into_inner();

    upload_gzipped_file(&ctx, &upload_url, file, "application/x-hdf5")
        .await
        .context("failed to upload hdf5 file")?;

    let location = args.common.run.as_ref().map_or_else(
        || format!("asset '{}'", args.common.asset.cyan()),
        |r| format!("run '{}'", r.clone().cyan()),
    );

    if !args.common.wait {
        Output::new()
            .line(format!("{} file for processing", "Uploaded".green()))
            .tip(format!(
                "Once processing is complete the data will be available on the {location}."
            ))
            .print();

        return Ok(ExitCode::SUCCESS);
    }

    wait_for_job_completion(grpc_channel, location).await
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
