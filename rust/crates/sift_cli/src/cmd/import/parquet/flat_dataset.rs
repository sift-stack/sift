use std::{collections::HashMap, fs::File, io::Seek, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::DateTime;
use crossterm::style::Stylize;
use pbjson_types::Timestamp;
use reqwest::header::{CONTENT_ENCODING, CONTENT_TYPE};
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse, DataTypeKey,
        DetectConfigRequest, ParquetComplexTypesImportMode, ParquetConfig, ParquetTimeColumn,
        TimeFormat, data_import_service_client::DataImportServiceClient, parquet_config::Config,
    },
};

use crate::{
    cli::{FlatDatasetArgs, channel::DataType},
    cmd::{
        Context,
        import::{
            parquet::{FooterMetadata, get_footer},
            preview_import_config,
            utils::{
                gzip_file, try_parse_bit_field_config, try_parse_enum_config, validate_time_format,
            },
            wait_for_job_completion,
        },
    },
    util::{
        api::{create_grpc_channel, create_rest_client},
        tty::Output,
    },
};

pub async fn run(ctx: Context, args: FlatDatasetArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let mut file = File::open(&args.path).context("failed to open Parquet file")?;
    let footer_md = FooterMetadata::try_from(&mut file)?;

    let mut config = {
        let footer = get_footer(&mut file, footer_md)?;
        let resp = data_imports_client
            .detect_config(DetectConfigRequest {
                data: footer,
                r#type: DataTypeKey::ParquetFlatdataset.into(),
            })
            .await
            .context("failed to parse Parquet schema")?
            .into_inner();

        resp.parquet_config
            .ok_or(anyhow!("unexpected empty Parquet config"))?
    };

    update_config_with_overrides(&mut config, &args)?;
    let create_data_import_req = create_data_import_request(&args, config, footer_md)?;

    if args.preview {
        let parquet_conf = create_data_import_req.parquet_config.unwrap();
        let Config::FlatDataset(flatset_conf) = parquet_conf.config.unwrap();

        let channel_confs = flatset_conf
            .data_columns
            .iter()
            .filter_map(|col| col.channel_config.as_ref())
            .collect::<Vec<&ChannelConfig>>();

        preview_import_config(
            &parquet_conf.asset_name,
            if parquet_conf.run_id.is_empty() {
                parquet_conf.run_name.as_str()
            } else {
                parquet_conf.run_id.as_str()
            },
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
    let compressed_data = gzip_file(file)?;

    let rest_client = create_rest_client(&ctx)?;
    let res = rest_client
        .post(upload_url)
        .header(CONTENT_ENCODING, "gzip")
        .header(CONTENT_TYPE, "application/vnd.apache.parquet")
        .body(compressed_data)
        .send()
        .await
        .context("failed to upload Parquet file")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| "<failed to read body>".into());
        return Err(anyhow!(
            "failed to upload Parquet with http status {status}: {text}"
        ));
    }

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
    validate_time_format(args.time_format, &args.relative_start_time)?;

    let relative_start_time = match &args.relative_start_time {
        Some(start) => {
            let rs = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            let utc = rs.to_utc();
            Some(Timestamp::from(utc))
        }
        None => None,
    };

    flat_dataset_conf.time_column = Some(ParquetTimeColumn {
        relative_start_time,
        path: args.time_path.clone(),
        format: TimeFormat::from(args.time_format).into(),
    });

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
            asset_name: args.asset.clone(),
            run_name: args.run.clone().unwrap_or_default(),
            footer_offset: footer_md.offset,
            footer_length: u32::try_from(footer_md.length)
                .context("parquet footer length too large")?,
            complex_types_import_mode: ParquetComplexTypesImportMode::from(
                args.complex_types_mode.clone(),
            )
            .into(),
            config: config.config,
            ..Default::default()
        }),
        ..Default::default()
    };
    Ok(req)
}
