use std::path::PathBuf;

use crate::cli::tdms::TdmsFallbackMethod;
use crate::cli::time::TimeFormat;
use crate::cli::{CommonImportArgs, ImportTdmsArgs};
use crate::cmd::import::tdms::detect_tdms_config::{
    build_tdms_config, find_time_channel, is_waveform_channel, tdms_to_sift_data_type,
};
use chrono::DateTime;
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::data_imports::v2::{
    TdmsFallbackMethod as ProtoFallbackMethod, TimeFormat as ProtoTimeFormat,
};
use tdms::data_type::{TDMSValue, TdmsDataType};
use tdms::segment::{Channel, Endianness, MetadataProperty};

fn make_args() -> ImportTdmsArgs {
    ImportTdmsArgs {
        common: CommonImportArgs {
            path: PathBuf::from("test.tdms"),
            asset: "test-asset".into(),
            run: None,
            run_id: None,
            wait: false,
            preview: false,
        },
        start_time_override: None,
        fallback_method: TdmsFallbackMethod::FailOnError,
        time_format: None,
        relative_start_time: None,
        import_file_properties: false,
    }
}

fn make_channel(data_type: TdmsDataType, property_names: &[&str]) -> Channel {
    Channel {
        full_path: String::new(),
        group_path: String::new(),
        path: String::new(),
        data_type,
        raw_data_index: None,
        daqmx_data_index: None,
        properties: property_names
            .iter()
            .map(|n| MetadataProperty {
                name: n.to_string(),
                data_type: TdmsDataType::String,
                value: TDMSValue {
                    data_type: TdmsDataType::String,
                    endianness: Endianness::Little,
                    value: None,
                },
            })
            .collect(),
        chunk_positions: Vec::new(),
        string_offset_pos: None,
        interleaved_offset: 0,
    }
}

#[test]
fn build_tdms_config_defaults() {
    let args = make_args();
    let cfg = build_tdms_config(&args).expect("defaults should build");
    assert_eq!(cfg.asset_name, "test-asset");
    assert_eq!(cfg.run_name, "");
    assert_eq!(cfg.run_id, "");
    assert_eq!(cfg.fallback_method, ProtoFallbackMethod::FailOnError as i32);
    assert!(cfg.data.is_empty());
    assert!(cfg.start_time_override.is_none());
    assert!(cfg.relative_start_time.is_none());
    assert!(cfg.time_format.is_none());
    assert!(!cfg.import_file_properties);
}

#[test]
fn build_tdms_config_run_name_passes_through() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    let cfg = build_tdms_config(&args).expect("build");
    assert_eq!(cfg.run_name, "my-run");
    assert_eq!(cfg.run_id, "");
}

#[test]
fn build_tdms_config_run_id_passes_through() {
    let mut args = make_args();
    args.common.run_id = Some("run-abc-123".into());
    let cfg = build_tdms_config(&args).expect("build");
    assert_eq!(cfg.run_id, "run-abc-123");
    assert_eq!(cfg.run_name, "");
}

#[test]
fn build_tdms_config_fallback_method_ignore_error() {
    let mut args = make_args();
    args.fallback_method = TdmsFallbackMethod::IgnoreError;
    let cfg = build_tdms_config(&args).expect("build");
    assert_eq!(cfg.fallback_method, ProtoFallbackMethod::IgnoreError as i32);
}

#[test]
fn build_tdms_config_time_format_absolute_rfc3339() {
    let mut args = make_args();
    args.time_format = Some(TimeFormat::AbsoluteRfc3339);
    let cfg = build_tdms_config(&args).expect("build");
    assert_eq!(
        cfg.time_format,
        Some(ProtoTimeFormat::AbsoluteRfc3339 as i32)
    );
}

#[test]
fn build_tdms_config_relative_start_time_parses_rfc3339() {
    let mut args = make_args();
    args.time_format = Some(TimeFormat::RelativeSeconds);
    args.relative_start_time = Some("2026-05-06T12:00:00Z".into());
    let cfg = build_tdms_config(&args).expect("build");
    let ts = cfg.relative_start_time.expect("timestamp");
    let expected = DateTime::parse_from_rfc3339(args.relative_start_time.as_deref().unwrap())
        .unwrap()
        .timestamp();
    assert_eq!(ts.seconds, expected);
    assert_eq!(ts.nanos, 0);
}

#[test]
fn build_tdms_config_start_time_override_parses_rfc3339() {
    let mut args = make_args();
    args.start_time_override = Some("2026-01-01T00:00:00Z".into());
    let cfg = build_tdms_config(&args).expect("build");
    let ts = cfg.start_time_override.expect("timestamp");
    let expected = DateTime::parse_from_rfc3339(args.start_time_override.as_deref().unwrap())
        .unwrap()
        .timestamp();
    assert_eq!(ts.seconds, expected);
}

#[test]
fn build_tdms_config_import_file_properties_with_run() {
    let mut args = make_args();
    args.import_file_properties = true;
    args.common.run = Some("my-run".into());
    let cfg = build_tdms_config(&args).expect("build");
    assert!(cfg.import_file_properties);
}

#[test]
fn build_tdms_config_import_file_properties_requires_run() {
    let mut args = make_args();
    args.import_file_properties = true;
    let err = build_tdms_config(&args).unwrap_err();
    assert!(
        err.to_string().contains("import-file-properties"),
        "expected validation error, got: {err:#}"
    );
}

#[test]
fn build_tdms_config_invalid_relative_start_time_errors() {
    let mut args = make_args();
    args.relative_start_time = Some("not a date".into());
    let err = build_tdms_config(&args).unwrap_err();
    assert!(
        err.chain().any(|e| e.to_string().contains("RFC3339")),
        "expected RFC3339 error, got: {err:#}"
    );
}

#[test]
fn build_tdms_config_invalid_start_time_override_errors() {
    let mut args = make_args();
    args.start_time_override = Some("garbage".into());
    let err = build_tdms_config(&args).unwrap_err();
    assert!(
        err.chain().any(|e| e.to_string().contains("RFC3339")),
        "expected RFC3339 error, got: {err:#}"
    );
}

#[test]
fn test_tdms_to_sift_data_type() {
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::SingleFloat(4)),
        Some(ChannelDataType::Float)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::SingleFloatWithUnit(4)),
        Some(ChannelDataType::Float)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::DoubleFloat(4)),
        Some(ChannelDataType::Double)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::DoubleFloatWithUnit(4)),
        Some(ChannelDataType::Double)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::ExtendedFloat(4)),
        Some(ChannelDataType::Double)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::ExtendedFloatWithUnit(4)),
        Some(ChannelDataType::Double)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::I8(4)),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::I16(4)),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::I32(4)),
        Some(ChannelDataType::Int32)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::I64(4)),
        Some(ChannelDataType::Int64)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::U8(4)),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::U16(4)),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::U32(4)),
        Some(ChannelDataType::Uint32)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::U64(4)),
        Some(ChannelDataType::Uint64)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::Boolean(4)),
        Some(ChannelDataType::Bool)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::String),
        Some(ChannelDataType::String)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::TimeStamp(4)),
        Some(ChannelDataType::Int64)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::ComplexSingleFloat(4)),
        Some(ChannelDataType::Float)
    );
    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::ComplexDoubleFloat(4)),
        Some(ChannelDataType::Double)
    );

    assert_eq!(
        tdms_to_sift_data_type(TdmsDataType::FixedPoint(4)),
        Some(ChannelDataType::Double)
    );

    assert_eq!(tdms_to_sift_data_type(TdmsDataType::Void), None);
    assert_eq!(tdms_to_sift_data_type(TdmsDataType::DAQmxRawData), None);
}

#[test]
fn test_find_time_channel() {
    let time_ch = make_channel(TdmsDataType::TimeStamp(16), &[]);
    let data_ch = make_channel(TdmsDataType::DoubleFloat(8), &[]);

    let channels = vec![
        ("Voltage".to_string(), &data_ch),
        ("MyTimeChannel".to_string(), &time_ch),
    ];
    assert_eq!(
        find_time_channel(&channels),
        Some("MyTimeChannel".to_string())
    );

    let channels = vec![
        ("Voltage".to_string(), &data_ch),
        ("Time".to_string(), &data_ch),
    ];
    assert_eq!(find_time_channel(&channels), Some("Time".to_string()));

    let channels = vec![("TIMESTAMP".to_string(), &data_ch)];
    assert_eq!(find_time_channel(&channels), Some("TIMESTAMP".to_string()));

    let channels = vec![
        ("Voltage".to_string(), &data_ch),
        ("Pressure".to_string(), &data_ch),
    ];
    assert_eq!(find_time_channel(&channels), None);

    let channels: Vec<(String, &Channel)> = vec![];
    assert_eq!(find_time_channel(&channels), None);
}

#[test]
fn test_is_waveform_channel() {
    let ch = make_channel(
        TdmsDataType::DoubleFloat(8),
        &["wf_start_time", "wf_increment"],
    );
    assert!(is_waveform_channel(&ch));

    let ch = make_channel(TdmsDataType::DoubleFloat(8), &["wf_start_time"]);
    assert!(!is_waveform_channel(&ch));

    let ch = make_channel(TdmsDataType::DoubleFloat(8), &["wf_increment"]);
    assert!(!is_waveform_channel(&ch));

    let ch = make_channel(TdmsDataType::DoubleFloat(8), &[]);
    assert!(!is_waveform_channel(&ch));

    let ch = make_channel(
        TdmsDataType::DoubleFloat(8),
        &["unit_string", "description"],
    );
    assert!(!is_waveform_channel(&ch));

    let ch = make_channel(
        TdmsDataType::DoubleFloat(8),
        &["wf_start_time", "wf_increment", "unit_string"],
    );
    assert!(is_waveform_channel(&ch));
}
