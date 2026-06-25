use std::{collections::HashMap, fs::File, io::Seek, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use crossterm::style::Stylize;
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse,
        ParquetComplexTypesImportMode, ParquetConfig,
        data_import_service_client::DataImportServiceClient, parquet_config::Config,
    },
};

use crate::cmd::import::parquet::detect_parquet_schema::{
    TimeFormatSource, detect_flat_dataset_config,
};
use crate::cmd::import::parquet::proto_time_format_display;
use crate::{
    cli::{FlatDatasetArgs, channel::DataType},
    cmd::{
        Context,
        import::{
            TimePreview,
            parquet::FooterMetadata,
            preview_import_config,
            utils::{try_parse_bit_field_config, try_parse_enum_config, upload_gzipped_file},
            wait_for_job_completion,
        },
    },
    util::{
        api::create_grpc_channel,
        explore_url::{build_explore_url, pending_import_tip},
        tty::Output,
    },
};

pub async fn run(ctx: Context, args: FlatDatasetArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let mut file = File::open(&args.common.path).context("failed to open parquet file")?;
    let footer_md = FooterMetadata::try_from(&mut file)?;

    let (mut config, format_source) = {
        let (flat_dataset_config, format_source) =
            detect_flat_dataset_config(&file, &args).context("failed to detect parquet schema")?;
        (
            ParquetConfig {
                config: Some(Config::FlatDataset(flat_dataset_config)),
                ..Default::default()
            },
            format_source,
        )
    };
    update_config_with_overrides(&mut config, &args)?;
    let create_data_import_req = create_data_import_request(&args, config, footer_md)?;

    if args.common.preview {
        let parquet_conf = create_data_import_req.parquet_config.unwrap();
        let Config::FlatDataset(flatset_conf) = parquet_conf.config.unwrap() else {
            anyhow::bail!("expected flatdataset config for preview");
        };

        let channel_confs = flatset_conf
            .data_columns
            .iter()
            .filter_map(|col| col.channel_config.as_ref())
            .collect::<Vec<&ChannelConfig>>();

        let time_format_display = flatset_conf.time_column.as_ref().map(|tc| {
            let base = proto_time_format_display(tc.format);
            if format_source == TimeFormatSource::Defaulted {
                format!("{base} — defaulted; pass --time-format if incorrect")
            } else {
                base
            }
        });
        let time_preview = flatset_conf.time_column.as_ref().map(|tc| TimePreview {
            path: tc.path.as_str(),
            format: time_format_display.as_deref().unwrap_or("unspecified"),
        });

        preview_import_config(
            &parquet_conf.asset_name,
            if parquet_conf.run_id.is_empty() {
                parquet_conf.run_name.as_str()
            } else {
                parquet_conf.run_id.as_str()
            },
            time_preview,
            &channel_confs,
        );
        return Ok(ExitCode::SUCCESS);
    }

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import")?
        .into_inner();

    file.rewind()?;
    let job_id = upload_gzipped_file(
        &ctx,
        &upload_url,
        file,
        &args.common.path,
        "application/vnd.apache.parquet",
    )
    .await
    .context("failed to upload Parquet file")?;

    let run_identifier = args
        .common
        .run_id
        .as_deref()
        .or(args.common.run.as_deref());
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

fn update_config_with_overrides(
    parquet_config: &mut ParquetConfig,
    args: &FlatDatasetArgs,
) -> Result<()> {
    let Some(Config::FlatDataset(flat_dataset_conf)) = parquet_config.config.as_mut() else {
        return Err(anyhow!("unexpected missing Parquet file config"));
    };
    if flat_dataset_conf.data_columns.is_empty() {
        return Err(anyhow!(
            "failed to find any channel data columns in the provided Parquet file"
        ));
    }
    let num_overrides = args.channel_path.len();

    if ![
        args.data_type.len(),
        args.unit.len(),
        args.description.len(),
    ]
    .iter()
    .all(|n| *n == num_overrides)
    {
        return Err(anyhow!(
            "occurrences of --data-type, --units, and --descriptions must equal --channel-paths"
        ))
        .context("keep in mind that --units and --descriptions can be empty strings");
    }

    if num_overrides == 0 {
        return Ok(());
    }
    let path_index_lookup = {
        let mut lookup = HashMap::new();
        for (i, config) in flat_dataset_conf.data_columns.iter().enumerate() {
            lookup.insert(config.path.clone(), i);
        }
        lookup
    };

    let mut enum_configs_iter = {
        let mut parsed_enum_configs = Vec::with_capacity(args.enum_config.len());

        for config in &args.enum_config {
            let parsed = try_parse_enum_config(config)?;
            parsed_enum_configs.push(parsed);
        }
        parsed_enum_configs.into_iter()
    };

    let mut bit_field_configs_iter = {
        let mut parsed_bit_field_configs = Vec::with_capacity(args.bit_field_config.len());

        for config in &args.bit_field_config {
            let parsed = try_parse_bit_field_config(config)?;
            parsed_bit_field_configs.push(parsed);
        }
        parsed_bit_field_configs.into_iter()
    };

    for (i, channel) in args.channel_path.iter().enumerate() {
        let Some(idx) = path_index_lookup.get(channel) else {
            return Err(anyhow!(
                "override for {channel} was specified but it wasn't found in the Parquet file"
            ));
        };

        let dt = args.data_type.get(i).unwrap();
        let units = args.unit.get(i).unwrap();
        let description = args.description.get(i).unwrap();

        let mut updated_config = ChannelConfig {
            name: channel.clone(),
            units: units.clone(),
            description: description.clone(),
            ..Default::default()
        };

        match dt {
            DataType::Infer => {
                updated_config.data_type = flat_dataset_conf
                    .data_columns
                    .get(*idx)
                    .unwrap()
                    .channel_config
                    .as_ref()
                    .unwrap()
                    .data_type;
            }
            DataType::BitField => {
                let Some(bf_conf) = bit_field_configs_iter.next() else {
                    return Err(anyhow!(
                        "'{channel}' was declared as type bit-field but --bit-field-config was not specified"
                    ));
                };
                updated_config.data_type = ChannelDataType::BitField.into();
                updated_config.bit_field_elements = bf_conf;
            }
            DataType::Enum => {
                let Some(enum_conf) = enum_configs_iter.next() else {
                    return Err(anyhow!(
                        "'{channel}' was declared as type enum but --enum-config was not specified"
                    ));
                };
                updated_config.data_type = ChannelDataType::Enum.into();
                updated_config.enum_types = enum_conf;
            }
            _ => updated_config.data_type = ChannelDataType::from(dt.clone()).into(),
        }

        let target = flat_dataset_conf
            .data_columns
            .get_mut(*idx)
            .unwrap()
            .channel_config
            .as_mut()
            .unwrap();
        *target = updated_config;
    }
    Ok(())
}

fn create_data_import_request(
    args: &FlatDatasetArgs,
    config: ParquetConfig,
    footer_md: FooterMetadata,
) -> Result<CreateDataImportFromUploadRequest> {
    let req = CreateDataImportFromUploadRequest {
        parquet_config: Some(ParquetConfig {
            asset_name: args.common.asset.clone(),
            run_name: args.common.run.clone().unwrap_or_default(),
            run_id: args.common.run_id.clone().unwrap_or_default(),
            footer_offset: footer_md.offset,
            footer_length: u32::try_from(footer_md.length)
                .context("parquet footer length too large")?,
            complex_types_import_mode: ParquetComplexTypesImportMode::from(
                args.complex_types_mode.clone(),
            )
            .into(),
            config: config.config,
        }),
        ..Default::default()
    };
    Ok(req)
}
