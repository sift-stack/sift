use crate::cli::{FlatDatasetArgs, parquet::ComplexTypesMode, time::TimeFormat};
use crate::cmd::import::parquet::detect_parquet_schema::{self, arrow_type_to_channel_data_type};
use anyhow::Context;
use arrow_array::{
    BooleanArray, Float32Array, Float64Array, Int32Array, Int64Array, RecordBatch, StringArray,
    TimestampSecondArray, UInt32Array, UInt64Array,
};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use parquet::arrow::arrow_writer::ArrowWriter;
use sift_rs::common::r#type::v1::ChannelDataType;
use std::io::{Seek, Write};
use std::path::PathBuf;
use std::sync::Arc;

fn make_test_args(time_path: &str, time_format: TimeFormat) -> FlatDatasetArgs {
    FlatDatasetArgs {
        path: PathBuf::from("test.parquet"),
        asset: String::from("test-asset"),
        run: None,
        channel_path: vec![],
        data_type: vec![],
        unit: vec![],
        description: vec![],
        enum_config: vec![],
        bit_field_config: vec![],
        time_path: time_path.to_string(),
        time_format,
        relative_start_time: None,
        complex_types_mode: ComplexTypesMode::default(),
        wait: false,
        preview: false,
    }
}

fn create_test_batch() -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("time", DataType::Timestamp(TimeUnit::Second, None), false),
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Float64, true),
        Field::new("c", DataType::Utf8, false),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(TimestampSecondArray::from(vec![1, 2, 3])),
            Arc::new(Int32Array::from(vec![1, 2, 3])),
            Arc::new(Float64Array::from(vec![Some(4.0), None, Some(5.0)])),
            Arc::new(StringArray::from(vec!["alpha", "beta", "gamma"])),
        ],
    )?;
    Ok(batch)
}

fn write_to_parquet_memory(batch: &RecordBatch) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None)?;
    writer.write(batch)?;
    writer.close()?;
    Ok(buffer)
}

#[test]
fn test_detect_parquet_on_import() -> Result<(), Box<dyn std::error::Error>> {
    let batch = create_test_batch()?;
    let parquet_bytes = write_to_parquet_memory(&batch)?;
    let args = make_test_args("time", TimeFormat::AbsoluteUnixSeconds);

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let config = detect_parquet_schema::detect_flat_dataset_config(&file, &args)
        .context("Detecting parquet schema test failure")?;

    let time_col = match config.time_column {
        Some(col) => col,
        None => return Err("no time column detected".into()),
    };

    assert_eq!(time_col.path, "time");

    assert_eq!(config.data_columns.len(), 3);
    assert_eq!(config.data_columns[0].path, "a");
    assert_eq!(config.data_columns[1].path, "b");
    assert_eq!(config.data_columns[2].path, "c");

    Ok(())
}

#[test]
fn test_time_column_excluded_from_data_columns() -> Result<(), Box<dyn std::error::Error>> {
    let batch = create_test_batch()?;
    let parquet_bytes = write_to_parquet_memory(&batch)?;
    let args = make_test_args("time", TimeFormat::AbsoluteUnixSeconds);

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let config = detect_parquet_schema::detect_flat_dataset_config(&file, &args)?;

    for col in &config.data_columns {
        assert_ne!(
            col.path, "time",
            "time column should not be in data_columns"
        );
    }

    Ok(())
}

#[test]
fn test_arrow_boolean_maps_to_channel_bool() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Boolean),
        Some(ChannelDataType::Bool)
    );
}

#[test]
fn test_arrow_float16_and_float32_map_to_channel_float() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Float16),
        Some(ChannelDataType::Float)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Float32),
        Some(ChannelDataType::Float)
    );
}

#[test]
fn test_arrow_float64_maps_to_channel_double() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Float64),
        Some(ChannelDataType::Double)
    );
}

#[test]
fn test_arrow_int8_int16_int32_map_to_channel_int32() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Int8),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Int16),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Int32),
        Some(ChannelDataType::Int32)
    );
}

#[test]
fn test_arrow_int64_maps_to_channel_int64() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Int64),
        Some(ChannelDataType::Int64)
    );
}

#[test]
fn test_arrow_uint8_uint16_uint32_map_to_channel_uint32() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::UInt8),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::UInt16),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::UInt32),
        Some(ChannelDataType::Uint32)
    );
}

#[test]
fn test_arrow_uint64_maps_to_channel_uint64() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::UInt64),
        Some(ChannelDataType::Uint64)
    );
}

#[test]
fn test_arrow_utf8_and_large_utf8_map_to_channel_string() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Utf8),
        Some(ChannelDataType::String)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::LargeUtf8),
        Some(ChannelDataType::String)
    );
}

#[test]
fn test_arrow_binary_types_map_to_channel_bytes() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Binary),
        Some(ChannelDataType::Bytes)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::LargeBinary),
        Some(ChannelDataType::Bytes)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::FixedSizeBinary(16)),
        Some(ChannelDataType::Bytes)
    );
}

#[test]
fn test_arrow_timestamp_and_date_types_map_to_channel_int64() {
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Timestamp(TimeUnit::Second, None)),
        Some(ChannelDataType::Int64)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Timestamp(TimeUnit::Nanosecond, None)),
        Some(ChannelDataType::Int64)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Date32),
        Some(ChannelDataType::Int64)
    );
    assert_eq!(
        arrow_type_to_channel_data_type(&DataType::Date64),
        Some(ChannelDataType::Int64)
    );
}

#[test]
fn test_arrow_unsupported_type_returns_none() {
    assert_eq!(arrow_type_to_channel_data_type(&DataType::Null), None);
}

#[test]
fn test_detect_config_assigns_correct_data_types_for_varied_columns()
-> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("time", DataType::Timestamp(TimeUnit::Second, None), false),
        Field::new("bool_col", DataType::Boolean, false),
        Field::new("float32_col", DataType::Float32, false),
        Field::new("float64_col", DataType::Float64, false),
        Field::new("int32_col", DataType::Int32, false),
        Field::new("int64_col", DataType::Int64, false),
        Field::new("uint32_col", DataType::UInt32, false),
        Field::new("uint64_col", DataType::UInt64, false),
        Field::new("string_col", DataType::Utf8, false),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(TimestampSecondArray::from(vec![1, 2, 3])),
            Arc::new(BooleanArray::from(vec![true, false, true])),
            Arc::new(Float32Array::from(vec![1.0f32, 2.0, 3.0])),
            Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0])),
            Arc::new(Int32Array::from(vec![1, 2, 3])),
            Arc::new(Int64Array::from(vec![1i64, 2, 3])),
            Arc::new(UInt32Array::from(vec![1u32, 2, 3])),
            Arc::new(UInt64Array::from(vec![1u64, 2, 3])),
            Arc::new(StringArray::from(vec!["a", "b", "c"])),
        ],
    )?;

    let parquet_bytes = write_to_parquet_memory(&batch)?;
    let args = make_test_args("time", TimeFormat::AbsoluteUnixSeconds);

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let config = detect_parquet_schema::detect_flat_dataset_config(&file, &args)
        .context("detect_flat_dataset failed, not test correct data columns")?;

    assert_eq!(config.data_columns.len(), 8);

    let expected: Vec<(&str, i32)> = vec![
        ("bool_col", ChannelDataType::Bool.into()),
        ("float32_col", ChannelDataType::Float.into()),
        ("float64_col", ChannelDataType::Double.into()),
        ("int32_col", ChannelDataType::Int32.into()),
        ("int64_col", ChannelDataType::Int64.into()),
        ("uint32_col", ChannelDataType::Uint32.into()),
        ("uint64_col", ChannelDataType::Uint64.into()),
        ("string_col", ChannelDataType::String.into()),
    ];

    for (col, (expected_name, expected_type)) in config.data_columns.iter().zip(expected.iter()) {
        assert_eq!(&col.path, expected_name);
        assert_eq!(
            col.channel_config.as_ref().unwrap().data_type,
            *expected_type,
            "wrong data type for column {expected_name}"
        );
    }

    Ok(())
}

#[test]
fn test_relative_time_format_without_start_time_returns_error()
-> Result<(), Box<dyn std::error::Error>> {
    let batch = create_test_batch()?;
    let parquet_bytes = write_to_parquet_memory(&batch)?;
    let args = make_test_args("time", TimeFormat::RelativeNanoseconds);

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let result = detect_parquet_schema::detect_flat_dataset_config(&file, &args);
    assert!(
        result.is_err(),
        "should error when relative time format has no start time"
    );

    Ok(())
}

#[test]
fn test_invalid_rfc3339_relative_start_time_returns_error() -> Result<(), Box<dyn std::error::Error>>
{
    let batch = create_test_batch()?;
    let parquet_bytes = write_to_parquet_memory(&batch)?;
    let mut args = make_test_args("time", TimeFormat::RelativeNanoseconds);
    args.relative_start_time = Some("not-a-valid-timestamp".to_string());

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let result = detect_parquet_schema::detect_flat_dataset_config(&file, &args);
    assert!(
        result.is_err(),
        "should error on invalid RFC3339 start time"
    );

    Ok(())
}

#[test]
fn test_time_path_not_in_parquet_still_returns_config() -> Result<(), Box<dyn std::error::Error>> {
    let batch = create_test_batch()?;
    let parquet_bytes = write_to_parquet_memory(&batch)?;
    let args = make_test_args("nonexistent_column", TimeFormat::AbsoluteUnixSeconds);

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let config = detect_parquet_schema::detect_flat_dataset_config(&file, &args)?;

    assert_eq!(config.time_column.unwrap().path, "nonexistent_column");
    assert_eq!(config.data_columns.len(), 4);

    Ok(())
}
