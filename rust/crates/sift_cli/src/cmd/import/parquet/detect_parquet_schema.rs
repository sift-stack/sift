use crate::cmd::import::utils::validate_time_format;
use anyhow::{Context, Result};
use arrow_schema::{DataType, Field, TimeUnit};
use chrono::DateTime;
use parquet::arrow::parquet_to_arrow_schema;
use parquet::file::metadata::ParquetMetaDataReader;
use parquet::file::reader::ChunkReader;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        ParquetColumn, ParquetDataColumn, ParquetFlatDatasetConfig,
        ParquetSingleChannelPerRowConfig, ParquetSingleChannelPerRowMultiChannelConfig,
        ParquetSingleChannelPerRowSingleChannelConfig, ParquetTimeColumn, TimeFormat,
        parquet_single_channel_per_row_config::Config as ChannelPerRowInnerConfig,
    },
};

use crate::cli::channel::DataType as CliDataType;
use crate::cli::parquet::ChannelMode;
use crate::cli::time::TimeFormat as CliTimeFormat;
use crate::cli::{ChannelPerRowArgs, FlatDatasetArgs};

pub fn detect_flat_dataset_config<R: ChunkReader>(
    file: &R,
    args: &FlatDatasetArgs,
) -> Result<ParquetFlatDatasetConfig> {
    let metadata = ParquetMetaDataReader::new().parse_and_finish(file)?;

    let arrow_schema = parquet_to_arrow_schema(
        metadata.file_metadata().schema_descr(),
        metadata.file_metadata().key_value_metadata(),
    )
    .context("detecting flat dataset config arrow schema")?;

    let time_field: &Field = match &args.time_path {
        Some(path) => arrow_schema
            .fields()
            .iter()
            .find(|f| f.name() == path)
            .map(|f| &**f)
            .with_context(|| format!("time column '{path}' not found in parquet schema"))?,
        None => arrow_schema
            .fields()
            .iter()
            .find_map(|f| auto_detect_time_column_field(f))
            .context(
                "no time column auto-detected — pass --time-path explicitly (looked for time, timestamp, timestamps, ts)",
            )?,
    };
    let time_path = time_field.name().clone();

    let resolved_format = match args.time_format {
        Some(fmt) => fmt,
        None => infer_time_format_from_arrow(time_field.data_type()).with_context(|| {
            format!(
                "could not infer time format for column '{time_path}' (Arrow type {:?}) — pass --time-format explicitly",
                time_field.data_type()
            )
        })?,
    };

    validate_time_format(resolved_format, &args.relative_start_time)
        .context("validating time format")?;

    let relative_start_time_input = match &args.relative_start_time {
        Some(start) => {
            let rs = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            let utc = rs.to_utc();
            Some(Timestamp::from(utc))
        }
        None => None,
    };

    let time_column = Some(ParquetTimeColumn {
        relative_start_time: relative_start_time_input,
        path: time_path.clone(),
        format: TimeFormat::from(resolved_format).into(),
    });

    let mut data_columns = Vec::new();
    for field in arrow_schema.fields() {
        if field.name() == &time_path {
            continue;
        }
        let Some(channel_type) = arrow_type_to_channel_data_type(field.data_type()) else {
            anyhow::bail!("unsupported column type for '{}'", field.name());
        };
        data_columns.push(ParquetDataColumn {
            path: field.name().to_string(),
            channel_config: Some(ChannelConfig {
                name: field.name().to_string(),
                data_type: channel_type.into(),
                ..Default::default()
            }),
        });
    }

    Ok(ParquetFlatDatasetConfig {
        time_column,
        data_columns,
    })
}

pub(super) fn auto_detect_time_column_field(field: &Field) -> Option<&Field> {
    match field.name().as_str() {
        "time" | "timestamp" | "timestamps" | "ts" => Some(field),
        _ => None,
    }
}

pub(super) fn infer_time_format_from_arrow(dt: &DataType) -> Option<CliTimeFormat> {
    match dt {
        DataType::Timestamp(TimeUnit::Second, _) => Some(CliTimeFormat::AbsoluteUnixSeconds),
        DataType::Timestamp(TimeUnit::Millisecond, _) => {
            Some(CliTimeFormat::AbsoluteUnixMilliseconds)
        }
        DataType::Timestamp(TimeUnit::Microsecond, _) => {
            Some(CliTimeFormat::AbsoluteUnixMicroseconds)
        }
        DataType::Timestamp(TimeUnit::Nanosecond, _) => {
            Some(CliTimeFormat::AbsoluteUnixNanoseconds)
        }
        DataType::Int64 => Some(CliTimeFormat::AbsoluteUnixNanoseconds),
        DataType::Utf8 | DataType::LargeUtf8 => Some(CliTimeFormat::AbsoluteRfc3339),
        _ => None,
    }
}

pub fn detect_channel_per_row_config<R: ChunkReader>(
    file: &R,
    args: &ChannelPerRowArgs,
) -> Result<ParquetSingleChannelPerRowConfig> {
    let metadata = ParquetMetaDataReader::new().parse_and_finish(file)?;
    let arrow_schema = parquet_to_arrow_schema(
        metadata.file_metadata().schema_descr(),
        metadata.file_metadata().key_value_metadata(),
    )
    .context("detecting channel-per-row arrow schema")?;

    let time_field: &Field = match &args.time_path {
        Some(path) => arrow_schema
            .fields()
            .iter()
            .find(|f| f.name() == path)
            .map(|f| &**f)
            .with_context(|| format!("time column '{path}' not found in parquet schema"))?,
        None => arrow_schema
            .fields()
            .iter()
            .find_map(|f| auto_detect_time_column_field(f))
            .context(
                "no time column auto-detected — pass --time-path explicitly (looked for time, timestamp, timestamps, ts)",
            )?,
    };
    let time_path = time_field.name().clone();

    let resolved_format = match args.time_format {
        Some(fmt) => fmt,
        None => infer_time_format_from_arrow(time_field.data_type()).with_context(|| {
            format!(
                "could not infer time format for column '{time_path}' (Arrow type {:?}) — pass --time-format explicitly",
                time_field.data_type()
            )
        })?,
    };

    validate_time_format(resolved_format, &args.relative_start_time)
        .context("validating time format")?;

    let relative_start_time = match &args.relative_start_time {
        Some(start) => {
            let dt = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            Some(Timestamp::from(dt.to_utc()))
        }
        None => None,
    };

    let time_column = Some(ParquetTimeColumn {
        relative_start_time,
        path: time_path,
        format: TimeFormat::from(resolved_format).into(),
    });

    let data_field = arrow_schema
        .fields()
        .iter()
        .find(|field| field.name() == &args.data_path)
        .with_context(|| {
            format!(
                "data column '{}' not found in parquet schema",
                args.data_path
            )
        })?;
    let data_channel_type = arrow_type_to_channel_data_type(data_field.data_type())
        .with_context(|| format!("unsupported data type for column '{}'", args.data_path))?;

    let mut columns = vec![ParquetColumn {
        path: args.data_path.clone(),
        column_config: Some(ChannelConfig {
            data_type: data_channel_type.into(),
            ..Default::default()
        }),
    }];

    let inner_config = match args.mode {
        ChannelMode::Single => {
            let channel_name = args
                .channel_name
                .as_ref()
                .expect("clap enforces --channel-name for --mode single");

            let resolved_type = match args.data_type {
                None | Some(CliDataType::Infer) => data_channel_type,
                Some(ref dt) => ChannelDataType::from(dt.clone()),
            };

            ChannelPerRowInnerConfig::SingleChannel(ParquetSingleChannelPerRowSingleChannelConfig {
                data_path: args.data_path.clone(),
                channel: Some(ChannelConfig {
                    name: channel_name.clone(),
                    data_type: resolved_type.into(),
                    units: args.unit.clone().unwrap_or_default(),
                    description: args.description.clone().unwrap_or_default(),
                    ..Default::default()
                }),
            })
        }
        ChannelMode::Multi => {
            let name_path = args
                .name_path
                .as_ref()
                .expect("clap enforces --name-path for --mode multi");

            let name_field = arrow_schema
                .fields()
                .iter()
                .find(|field| field.name() == name_path)
                .with_context(|| {
                    format!("name column '{name_path}' not found in parquet schema")
                })?;
            let name_channel_type = arrow_type_to_channel_data_type(name_field.data_type())
                .with_context(|| format!("unsupported data type for name column '{name_path}'"))?;

            columns.push(ParquetColumn {
                path: name_path.clone(),
                column_config: Some(ChannelConfig {
                    data_type: name_channel_type.into(),
                    ..Default::default()
                }),
            });

            ChannelPerRowInnerConfig::MultiChannel(ParquetSingleChannelPerRowMultiChannelConfig {
                name_path: name_path.clone(),
                data_path: args.data_path.clone(),
            })
        }
    };

    Ok(ParquetSingleChannelPerRowConfig {
        time_column,
        columns,
        config: Some(inner_config),
    })
}

pub(super) fn arrow_type_to_channel_data_type(dt: &DataType) -> Option<ChannelDataType> {
    match dt {
        DataType::Boolean => Some(ChannelDataType::Bool),
        DataType::Float16 | DataType::Float32 => Some(ChannelDataType::Float),
        DataType::Float64 => Some(ChannelDataType::Double),
        DataType::Int8 | DataType::Int16 | DataType::Int32 => Some(ChannelDataType::Int32),
        DataType::Int64 => Some(ChannelDataType::Int64),
        DataType::UInt8 | DataType::UInt16 | DataType::UInt32 => Some(ChannelDataType::Uint32),
        DataType::UInt64 => Some(ChannelDataType::Uint64),
        DataType::Utf8 | DataType::LargeUtf8 => Some(ChannelDataType::String),
        DataType::Binary | DataType::LargeBinary | DataType::FixedSizeBinary(_) => {
            Some(ChannelDataType::Bytes)
        }
        DataType::Timestamp(_, _)
        | DataType::Date32
        | DataType::Date64
        | DataType::Time32(_)
        | DataType::Time64(_)
        | DataType::Duration(_) => Some(ChannelDataType::Int64),
        DataType::List(_) | DataType::Map(_, _) => Some(ChannelDataType::Bytes),
        _ => None,
    }
}
