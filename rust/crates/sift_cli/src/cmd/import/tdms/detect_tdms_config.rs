use std::{fs::File, path::Path, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::DateTime;
use crossterm::style::Stylize;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse, TdmsConfig,
        TdmsFallbackMethod as ProtoFallbackMethod, TimeFormat as ProtoTimeFormat,
        data_import_service_client::DataImportServiceClient,
    },
};
use tdms::TDMSFile;
use tdms::data_type::TdmsDataType;
use tdms::segment::Channel;

use crate::{
    cli::{ImportTdmsArgs, tdms::TdmsFallbackMethod},
    cmd::{
        Context,
        import::{
            preview_import_config,
            utils::{upload_gzipped_file, validate_time_format},
            wait_for_job_completion,
        },
    },
    util::{api::create_grpc_channel, tty::Output},
};

pub async fn run(ctx: Context, args: ImportTdmsArgs) -> Result<ExitCode> {
    let grpc_channel =
        create_grpc_channel(&ctx).context("failed to create grpc channel for tdms import")?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let tdms_config = build_tdms_config(&args).context("failed to build tdms config")?;

    if args.preview {
        let run_label = if tdms_config.run_id.is_empty() {
            tdms_config.run_name.as_str()
        } else {
            tdms_config.run_id.as_str()
        };

        match detect_config(&args.path, args.fallback_method) {
            Ok(channel_configs) => {
                let refs: Vec<&ChannelConfig> = channel_configs.iter().collect();
                preview_import_config(&args.asset, run_label, &refs);
            }
            Err(e) => {
                preview_import_config(&args.asset, run_label, &[]);
                Output::new()
                    .line(format!("client-side preview parse failed: {e:#}"))
                    .tip("the server-side parser may still ingest this file correctly")
                    .eprint();
            }
        }
        return Ok(ExitCode::SUCCESS);
    }

    let file = File::open(&args.path).context("failed to open tdms file")?;

    let create_data_import_req =
        create_data_import_request(tdms_config).context("failed to create data import req")?;

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import for tdms")?
        .into_inner();

    upload_gzipped_file(&ctx, &upload_url, file, "application/octet-stream")
        .await
        .context("failed to upload tdms file")?;

    let location = args.run.as_ref().map_or_else(
        || format!("asset '{}'", args.asset.cyan()),
        |r| format!("run '{}'", r.clone().cyan()),
    );

    if !args.wait {
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

pub fn build_tdms_config(args: &ImportTdmsArgs) -> Result<TdmsConfig> {
    if args.import_file_properties && args.run.is_none() && args.run_id.is_none() {
        anyhow::bail!("--import-file-properties requires --run or --run-id");
    }

    validate_time_format(
        args.time_format.unwrap_or_default(),
        &args.relative_start_time,
    )
    .context("validating time format for tdms")?;

    let relative_start_time_input = match &args.relative_start_time {
        Some(start) => {
            let rs = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            let utc = rs.to_utc();
            Some(Timestamp::from(utc))
        }
        None => None,
    };

    let start_time_override_input = match &args.start_time_override {
        Some(override_wf) => {
            let rs = DateTime::parse_from_rfc3339(override_wf)
                .context("--start-time-override is not valid RFC3339")?;
            let utc = rs.to_utc();
            Some(Timestamp::from(utc))
        }
        None => None,
    };

    Ok(TdmsConfig {
        asset_name: args.asset.clone(),
        run_name: args.run.clone().unwrap_or_default(),
        start_time_override: start_time_override_input,
        run_id: args.run_id.clone().unwrap_or_default(),
        fallback_method: ProtoFallbackMethod::from(args.fallback_method).into(),
        time_format: args.time_format.map(|tf| ProtoTimeFormat::from(tf).into()),
        relative_start_time: relative_start_time_input,
        import_file_properties: args.import_file_properties,
        ..Default::default()
    })
}

fn create_data_import_request(config: TdmsConfig) -> Result<CreateDataImportFromUploadRequest> {
    let req = CreateDataImportFromUploadRequest {
        tdms_config: Some(config),
        ..Default::default()
    };
    Ok(req)
}

pub(super) fn tdms_to_sift_data_type(tdms_type: TdmsDataType) -> Option<ChannelDataType> {
    match tdms_type {
        TdmsDataType::SingleFloat(_) | TdmsDataType::SingleFloatWithUnit(_) => {
            Some(ChannelDataType::Float)
        }
        TdmsDataType::DoubleFloat(_)
        | TdmsDataType::DoubleFloatWithUnit(_)
        | TdmsDataType::ExtendedFloat(_)
        | TdmsDataType::ExtendedFloatWithUnit(_) => Some(ChannelDataType::Double),

        TdmsDataType::I8(_) | TdmsDataType::I16(_) | TdmsDataType::I32(_) => {
            Some(ChannelDataType::Int32)
        }
        TdmsDataType::I64(_) => Some(ChannelDataType::Int64),

        TdmsDataType::U8(_) | TdmsDataType::U16(_) | TdmsDataType::U32(_) => {
            Some(ChannelDataType::Uint32)
        }
        TdmsDataType::U64(_) => Some(ChannelDataType::Uint64),

        TdmsDataType::Boolean(_) => Some(ChannelDataType::Bool),
        TdmsDataType::String => Some(ChannelDataType::String),
        TdmsDataType::TimeStamp(_) => Some(ChannelDataType::Int64),

        TdmsDataType::ComplexSingleFloat(_) => Some(ChannelDataType::Float),
        TdmsDataType::ComplexDoubleFloat(_) => Some(ChannelDataType::Double),

        TdmsDataType::FixedPoint(_) => Some(ChannelDataType::Double),

        TdmsDataType::Void | TdmsDataType::DAQmxRawData => None,
    }
}

pub(super) fn find_time_channel(channels: &[(String, &Channel)]) -> Option<String> {
    channels
        .iter()
        .find(|(name, ch)| {
            matches!(ch.data_type, TdmsDataType::TimeStamp(_))
                || name.eq_ignore_ascii_case("Time")
                || name.eq_ignore_ascii_case("Timestamp")
        })
        .map(|(name, _)| name.clone())
}

pub(super) fn is_waveform_channel(channel: &Channel) -> bool {
    let has_prop = |name: &str| channel.properties.iter().any(|p| p.name == name);
    has_prop("wf_start_time") && has_prop("wf_increment")
}

fn get_string_property(channel: &Channel, name: &str) -> String {
    channel
        .properties
        .iter()
        .find(|p| p.name == name)
        .and_then(|p| p.value.value.as_ref())
        .and_then(|bytes| String::from_utf8(bytes.clone()).ok())
        .unwrap_or_default()
}

fn detect_config(path: &Path, fallback_method: TdmsFallbackMethod) -> Result<Vec<ChannelConfig>> {
    let file =
        TDMSFile::from_path(path).map_err(|e| anyhow!("failed to open tdms for preview: {e}"))?;
    let mut channels_vec: Vec<ChannelConfig> = vec![];

    for group in file.groups() {
        let channels: Vec<(String, &Channel)> = file.channels(&group).into_iter().collect();
        let time_channel_name = find_time_channel(&channels);

        for (channel_name, channel) in &channels {
            if Some(channel_name) == time_channel_name.as_ref() {
                continue;
            }

            let Some(data_type) = tdms_to_sift_data_type(channel.data_type) else {
                continue;
            };

            let has_timing = is_waveform_channel(channel) || time_channel_name.is_some();
            if !has_timing {
                match fallback_method {
                    TdmsFallbackMethod::IgnoreError => continue,
                    TdmsFallbackMethod::FailOnError => {
                        return Err(anyhow!(
                            "no timing information for {}.{}",
                            group,
                            channel_name
                        ));
                    }
                }
            }

            channels_vec.push(ChannelConfig {
                name: format!("{}.{}", group, channel_name),
                units: get_string_property(channel, "unit_string"),
                description: get_string_property(channel, "description"),
                data_type: data_type as i32,
                ..Default::default()
            });
        }
    }
    Ok(channels_vec)
}
