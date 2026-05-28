use crate::cli::{FlatDatasetArgs, parquet::ComplexTypesMode, time::TimeFormat};
use crate::cmd::import::parquet::detect_parquet_schema::{self, arrow_type_to_channel_data_type};
use arrow_array::{
    BooleanArray, Float32Array, Float64Array, Int32Array, Int64Array, RecordBatch, StringArray,
    TimestampMicrosecondArray, TimestampMillisecondArray, TimestampNanosecondArray,
    TimestampSecondArray, UInt32Array, UInt64Array,
};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use bytes::Bytes;
use parquet::arrow::arrow_writer::ArrowWriter;
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::data_imports::v2::TimeFormat as ProtoTimeFormat;
use std::path::PathBuf;
use std::sync::Arc;

fn make_test_args(time_path: Option<&str>, time_format: Option<TimeFormat>) -> FlatDatasetArgs {
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
        time_path: time_path.map(|s| s.to_string()),
        time_format,
        relative_start_time: None,
        complex_types_mode: ComplexTypesMode::default(),
        wait: false,
        preview: false,
    }
}

fn create_test_batch() -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![
        Field::new("time", DataType::Timestamp(TimeUnit::Second, None), false),
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Float64, true),
        Field::new("c", DataType::Utf8, false),
    ]));

    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(TimestampSecondArray::from(vec![1, 2, 3])),
            Arc::new(Int32Array::from(vec![1, 2, 3])),
            Arc::new(Float64Array::from(vec![Some(4.0), None, Some(5.0)])),
            Arc::new(StringArray::from(vec!["alpha", "beta", "gamma"])),
        ],
    )
    .expect("failed to create test record batch")
}

fn write_to_parquet_bytes(batch: &RecordBatch) -> Bytes {
    let mut buffer = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None)
        .expect("failed to create parquet writer");
    writer
        .write(batch)
        .expect("failed to write batch to parquet");
    writer.close().expect("failed to close parquet writer");
    Bytes::from(buffer)
}

#[test]
fn test_detect_parquet_on_import() {
    let batch = create_test_batch();
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(Some("time"), Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to detect flat dataset config");

    let time_col = config
        .time_column
        .expect("expected time column to be detected");

    assert_eq!(time_col.path, "time");

    assert_eq!(config.data_columns.len(), 3);
    assert_eq!(config.data_columns[0].path, "a");
    assert_eq!(config.data_columns[1].path, "b");
    assert_eq!(config.data_columns[2].path, "c");
}

#[test]
fn test_time_column_excluded_from_data_columns() {
    let batch = create_test_batch();
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(Some("time"), Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to detect flat dataset config");

    for col in &config.data_columns {
        assert_ne!(
            col.path, "time",
            "time column should not be in data_columns"
        );
    }
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
fn test_detect_config_assigns_correct_data_types_for_varied_columns() {
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
    )
    .expect("failed to create varied columns record batch");

    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(Some("time"), Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to detect flat dataset config");

    assert_eq!(config.data_columns.len(), 8);

    let expected = [
        ("bool_col", i32::from(ChannelDataType::Bool)),
        ("float32_col", i32::from(ChannelDataType::Float)),
        ("float64_col", i32::from(ChannelDataType::Double)),
        ("int32_col", i32::from(ChannelDataType::Int32)),
        ("int64_col", i32::from(ChannelDataType::Int64)),
        ("uint32_col", i32::from(ChannelDataType::Uint32)),
        ("uint64_col", i32::from(ChannelDataType::Uint64)),
        ("string_col", i32::from(ChannelDataType::String)),
    ];

    for (col, (expected_name, expected_type)) in config.data_columns.iter().zip(expected.iter()) {
        assert_eq!(&col.path, expected_name);
        assert_eq!(
            col.channel_config.as_ref().unwrap().data_type,
            *expected_type,
            "wrong data type for column {expected_name}"
        );
    }
}

#[test]
fn test_relative_time_format_without_start_time_returns_error() {
    let batch = create_test_batch();
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(Some("time"), Some(TimeFormat::RelativeNanoseconds));

    let result = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args);
    assert!(
        result.is_err(),
        "should error when relative time format has no start time"
    );
}

#[test]
fn test_invalid_rfc3339_relative_start_time_returns_error() {
    let batch = create_test_batch();
    let bytes = write_to_parquet_bytes(&batch);
    let mut args = make_test_args(Some("time"), Some(TimeFormat::RelativeNanoseconds));
    args.relative_start_time = Some("not-a-valid-timestamp".to_string());

    let result = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args);
    assert!(
        result.is_err(),
        "should error on invalid RFC3339 start time"
    );
}

#[test]
fn test_time_path_not_in_parquet_returns_error() {
    let batch = create_test_batch();
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(Some("nonexistent_column"), Some(TimeFormat::AbsoluteUnixSeconds));

    let result = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args);
    assert!(
        result.is_err(),
        "should error when time path is not found in parquet schema"
    );
}

fn make_batch_with_time_col_named(name: &str) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![
        Field::new(name, DataType::Timestamp(TimeUnit::Second, None), false),
        Field::new("a", DataType::Int32, false),
    ]));

    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(TimestampSecondArray::from(vec![1, 2, 3])),
            Arc::new(Int32Array::from(vec![1, 2, 3])),
        ],
    )
    .expect("failed to create test record batch")
}

#[test]
fn test_auto_detect_time_column_named_time() {
    let batch = make_batch_with_time_col_named("time");
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to auto-detect time column 'time'");

    assert_eq!(config.time_column.expect("time column").path, "time");
    assert_eq!(config.data_columns.len(), 1);
    assert_eq!(config.data_columns[0].path, "a");
}

#[test]
fn test_auto_detect_time_column_named_timestamp() {
    let batch = make_batch_with_time_col_named("timestamp");
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to auto-detect time column 'timestamp'");

    assert_eq!(config.time_column.expect("time column").path, "timestamp");
    assert_eq!(config.data_columns.len(), 1);
    assert_eq!(config.data_columns[0].path, "a");
}

#[test]
fn test_auto_detect_time_column_named_ts() {
    let batch = make_batch_with_time_col_named("ts");
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to auto-detect time column 'ts'");

    assert_eq!(config.time_column.expect("time column").path, "ts");
    assert_eq!(config.data_columns.len(), 1);
    assert_eq!(config.data_columns[0].path, "a");
}

#[test]
fn test_auto_detect_ignores_partial_match_event_time() {
    let batch = make_batch_with_time_col_named("event_time");
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixSeconds));

    let result = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args);
    assert!(
        result.is_err(),
        "auto-detect should only match exact names (time/timestamp/ts), not 'event_time'"
    );
}

#[test]
fn test_auto_detect_errors_when_no_candidate_column() {
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Float64, false),
    ]));
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int32Array::from(vec![1, 2, 3])),
            Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0])),
        ],
    )
    .expect("failed to create batch with no time-like column");
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixSeconds));

    let result = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args);
    assert!(
        result.is_err(),
        "should error when no time/timestamp/ts column exists and --time-path is missing"
    );
}

#[test]
fn test_auto_detect_excludes_time_column_from_data_columns() {
    let batch = make_batch_with_time_col_named("timestamp");
    let bytes = write_to_parquet_bytes(&batch);
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixSeconds));

    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to auto-detect");

    for col in &config.data_columns {
        assert_ne!(
            col.path, "timestamp",
            "auto-detected time column should not appear in data_columns"
        );
    }
}

fn build_parquet_with_time_field(time_field: Field, time_array: Arc<dyn arrow_array::Array>) -> Bytes {
    let schema = Arc::new(Schema::new(vec![
        time_field,
        Field::new("a", DataType::Int32, false),
    ]));
    let batch = RecordBatch::try_new(
        schema,
        vec![time_array, Arc::new(Int32Array::from(vec![1, 2, 3]))],
    )
    .expect("failed to create record batch");
    write_to_parquet_bytes(&batch)
}

#[test]
fn test_infer_format_timestamp_second() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Timestamp(TimeUnit::Second, None), false),
        Arc::new(TimestampSecondArray::from(vec![1, 2, 3])),
    );
    let args = make_test_args(None, None);
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to infer format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteUnixSeconds as i32,
    );
}

#[test]
fn test_infer_format_timestamp_millisecond() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Timestamp(TimeUnit::Millisecond, None), false),
        Arc::new(TimestampMillisecondArray::from(vec![1, 2, 3])),
    );
    let args = make_test_args(None, None);
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to infer format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteUnixMilliseconds as i32,
    );
}

#[test]
fn test_infer_format_timestamp_microsecond() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Timestamp(TimeUnit::Microsecond, None), false),
        Arc::new(TimestampMicrosecondArray::from(vec![1, 2, 3])),
    );
    let args = make_test_args(None, None);
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to infer format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteUnixMicroseconds as i32,
    );
}

#[test]
fn test_infer_format_timestamp_nanosecond() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Timestamp(TimeUnit::Nanosecond, None), false),
        Arc::new(TimestampNanosecondArray::from(vec![1, 2, 3])),
    );
    let args = make_test_args(None, None);
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to infer format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteUnixNanoseconds as i32,
    );
}

#[test]
fn test_infer_format_int64_defaults_to_nanoseconds() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Int64, false),
        Arc::new(Int64Array::from(vec![1i64, 2, 3])),
    );
    let args = make_test_args(None, None);
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to infer format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteUnixNanoseconds as i32,
    );
}

#[test]
fn test_infer_format_utf8_defaults_to_rfc3339() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Utf8, false),
        Arc::new(StringArray::from(vec!["2024-01-01T00:00:00Z", "2024-01-02T00:00:00Z", "2024-01-03T00:00:00Z"])),
    );
    let args = make_test_args(None, None);
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed to infer format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteRfc3339 as i32,
    );
}

#[test]
fn test_infer_format_errors_for_float64_time_column() {
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Float64, false),
        Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0])),
    );
    let args = make_test_args(None, None);
    let result = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args);
    assert!(
        result.is_err(),
        "should error when time column has a type we can't map to a format"
    );
}

#[test]
fn test_explicit_format_overrides_inference() {
    // Time column is Int64 — would infer nanoseconds. Explicit format should win.
    let bytes = build_parquet_with_time_field(
        Field::new("time", DataType::Int64, false),
        Arc::new(Int64Array::from(vec![1i64, 2, 3])),
    );
    let args = make_test_args(None, Some(TimeFormat::AbsoluteUnixMilliseconds));
    let config = detect_parquet_schema::detect_flat_dataset_config(&bytes, &args)
        .expect("failed with explicit format");
    assert_eq!(
        config.time_column.expect("time column").format,
        ProtoTimeFormat::AbsoluteUnixMilliseconds as i32,
    );
}
