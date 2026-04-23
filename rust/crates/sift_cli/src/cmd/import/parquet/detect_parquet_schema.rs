use std::fs::File;

use anyhow::Result;
use arrow_schema::{DataType, TimeUnit};
use parquet::arrow::parquet_to_arrow_schema;
use parquet::file::metadata::ParquetMetaDataReader;
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        ParquetDataColumn, ParquetFlatDatasetConfig, ParquetTimeColumn, TimeFormat,
    },
};

pub fn detect_flat_dataset_config(file: &File) -> Result<ParquetFlatDatasetConfig> {
    let metadata = ParquetMetaDataReader::new().parse_and_finish(file)?;

    let arrow_schema = parquet_to_arrow_schema(
        metadata.file_metadata().schema_descr(),
        metadata.file_metadata().key_value_metadata(),
    )?;

    let mut time_column = None;
    let mut data_columns = Vec::new();

    for field in arrow_schema.fields() {
        if time_column.is_none()
            && let Some(format) = detect_time_format(field.data_type())
        {
            time_column = Some(ParquetTimeColumn {
                path: field.name().to_string(),
                format: format.into(),
                relative_start_time: None,
            });
        }
        if let Some(channel_type) = arrow_type_to_channel_data_type(field.data_type()) {
            data_columns.push(ParquetDataColumn {
                path: field.name().to_string(),
                channel_config: Some(ChannelConfig {
                    name: field.name().to_string(),
                    data_type: channel_type.into(),
                    ..Default::default()
                }),
            });
        }
    }

    if time_column.is_none() {
        anyhow::bail!("no valid time column detected in parquet schema");
    }

    Ok(ParquetFlatDatasetConfig {
        time_column,
        data_columns,
    })
}

fn arrow_type_to_channel_data_type(dt: &DataType) -> Option<ChannelDataType> {
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

fn detect_time_format(dt: &DataType) -> Option<TimeFormat> {
    match dt {
        DataType::Timestamp(TimeUnit::Second, _) => Some(TimeFormat::AbsoluteUnixSeconds),
        DataType::Timestamp(TimeUnit::Millisecond, _) => Some(TimeFormat::AbsoluteUnixMilliseconds),
        DataType::Timestamp(TimeUnit::Microsecond, _) => Some(TimeFormat::AbsoluteUnixMicroseconds),
        DataType::Timestamp(TimeUnit::Nanosecond, _) => Some(TimeFormat::AbsoluteUnixNanoseconds),
        DataType::Time32(TimeUnit::Second) => Some(TimeFormat::RelativeSeconds),
        DataType::Time32(TimeUnit::Millisecond) => Some(TimeFormat::RelativeMilliseconds),
        DataType::Time64(TimeUnit::Microsecond) => Some(TimeFormat::RelativeMicroseconds),
        DataType::Time64(TimeUnit::Nanosecond) => Some(TimeFormat::RelativeNanoseconds),
        DataType::Duration(TimeUnit::Second) => Some(TimeFormat::RelativeSeconds),
        DataType::Duration(TimeUnit::Millisecond) => Some(TimeFormat::RelativeMilliseconds),
        DataType::Duration(TimeUnit::Microsecond) => Some(TimeFormat::RelativeMicroseconds),
        DataType::Duration(TimeUnit::Nanosecond) => Some(TimeFormat::RelativeNanoseconds),
        DataType::Date32 | DataType::Date64 => Some(TimeFormat::AbsoluteUnixNanoseconds),
        _ => None,
    }
}
