use crate::cmd::import::utils::validate_time_format;
use anyhow::{Context, Result};
use arrow_schema::DataType;
use chrono::DateTime;
use parquet::arrow::parquet_to_arrow_schema;
use parquet::file::metadata::ParquetMetaDataReader;
use parquet::file::reader::ChunkReader;
use pbjson_types::Timestamp;
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        ParquetDataColumn, ParquetFlatDatasetConfig, ParquetTimeColumn, TimeFormat,
    },
};

use crate::cli::FlatDatasetArgs;

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

    validate_time_format(args.time_format, &args.relative_start_time)
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

    let mut time_column = None;
    let mut data_columns = Vec::new();

    for field in arrow_schema.fields() {
        if field.name() == &args.time_path {
            time_column = Some(ParquetTimeColumn {
                relative_start_time: relative_start_time_input,
                path: args.time_path.clone(),
                format: TimeFormat::from(args.time_format).into(),
            });
        } else if let Some(channel_type) = arrow_type_to_channel_data_type(field.data_type()) {
            data_columns.push(ParquetDataColumn {
                path: field.name().to_string(),
                channel_config: Some(ChannelConfig {
                    name: field.name().to_string(),
                    data_type: channel_type.into(),
                    ..Default::default()
                }),
            });
        } else {
            anyhow::bail!("unsupported column type for '{}'", field.name());
        }
    }

    if time_column.is_none() {
        anyhow::bail!(
            "time column '{}' not found in parquet schema",
            args.time_path
        );
    }

    Ok(ParquetFlatDatasetConfig {
        time_column,
        data_columns,
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
