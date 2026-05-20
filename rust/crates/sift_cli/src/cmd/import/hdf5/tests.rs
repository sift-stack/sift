use std::path::PathBuf;

use chrono::DateTime;
use hdf5::types::{EnumMember, EnumType, FloatSize, IntSize, TypeDescriptor};
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::data_imports::v2::TimeFormat as ProtoTimeFormat;

use crate::cli::hdf5::Hdf5Schema;
use crate::cli::time::TimeFormat;
use crate::cli::{CommonImportArgs, ImportHdf5Args};
use crate::cmd::import::hdf5::detect_hdf5_schema::{
    basename, enum_types_for, hdf5_to_sift_data_type, is_time_dataset_name, parent_path,
};
use crate::cmd::import::hdf5::import::build_hdf5_config;
use crate::cmd::import::utils::group_path_to_channel_name;

fn make_args() -> ImportHdf5Args {
    ImportHdf5Args {
        common: CommonImportArgs {
            path: PathBuf::from("test.h5"),
            asset: "test-asset".into(),
            run: None,
            run_id: None,
            wait: false,
            preview: false,
        },
        schema: Hdf5Schema::OneD,
        time_format: Some(TimeFormat::AbsoluteRfc3339),
        relative_start_time: None,
        time_index: None,
        time_field: None,
    }
}

#[test]
fn build_hdf5_config_defaults() {
    let args = make_args();
    let cfg = build_hdf5_config(&args).expect("defaults should build");
    assert_eq!(cfg.asset_name, "test-asset");
    assert_eq!(cfg.run_name, "");
    assert_eq!(cfg.run_id, "");
    assert!(cfg.data.is_empty());
    assert!(cfg.relative_start_time.is_none());
    assert_eq!(cfg.time_format, ProtoTimeFormat::AbsoluteRfc3339 as i32);
}

#[test]
fn build_hdf5_config_missing_time_format_errors() {
    let mut args = make_args();
    args.time_format = None;
    let err = build_hdf5_config(&args).unwrap_err();
    assert!(
        err.to_string().contains("--time-format"),
        "expected --time-format error, got: {err:#}"
    );
}

#[test]
fn build_hdf5_config_run_name_passes_through() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    let cfg = build_hdf5_config(&args).expect("build");
    assert_eq!(cfg.run_name, "my-run");
    assert_eq!(cfg.run_id, "");
}

#[test]
fn build_hdf5_config_run_id_passes_through() {
    let mut args = make_args();
    args.common.run_id = Some("run-abc-123".into());
    let cfg = build_hdf5_config(&args).expect("build");
    assert_eq!(cfg.run_id, "run-abc-123");
    assert_eq!(cfg.run_name, "");
}

#[test]
fn build_hdf5_config_time_format_passes_through() {
    let mut args = make_args();
    args.time_format = Some(TimeFormat::AbsoluteRfc3339);
    let cfg = build_hdf5_config(&args).expect("build");
    assert_eq!(cfg.time_format, ProtoTimeFormat::AbsoluteRfc3339 as i32);
}

#[test]
fn build_hdf5_config_relative_start_time_parses_rfc3339() {
    let mut args = make_args();
    args.time_format = Some(TimeFormat::RelativeSeconds);
    args.relative_start_time = Some("2026-05-13T12:00:00Z".into());
    let cfg = build_hdf5_config(&args).expect("build");
    let ts = cfg.relative_start_time.expect("timestamp");
    let expected = DateTime::parse_from_rfc3339(args.relative_start_time.as_deref().unwrap())
        .unwrap()
        .timestamp();
    assert_eq!(ts.seconds, expected);
    assert_eq!(ts.nanos, 0);
}

#[test]
fn build_hdf5_config_invalid_relative_start_time_errors() {
    let mut args = make_args();
    args.relative_start_time = Some("not a date".into());
    let err = build_hdf5_config(&args).unwrap_err();
    assert!(
        err.chain().any(|e| e.to_string().contains("RFC3339")),
        "expected RFC3339 error, got: {err:#}"
    );
}

#[test]
fn build_hdf5_config_relative_time_format_requires_start() {
    let mut args = make_args();
    args.time_format = Some(TimeFormat::RelativeSeconds);
    let err = build_hdf5_config(&args).unwrap_err();
    assert!(
        err.chain()
            .any(|e| e.to_string().contains("--relative-start-time")),
        "expected validation error, got: {err:#}"
    );
}

#[test]
fn is_time_dataset_name_recognizes_known_names() {
    assert!(is_time_dataset_name("time"));
    assert!(is_time_dataset_name("Time"));
    assert!(is_time_dataset_name("TIME"));
    assert!(is_time_dataset_name("timestamp"));
    assert!(is_time_dataset_name("Timestamp"));
    assert!(is_time_dataset_name("timestamps"));
    assert!(is_time_dataset_name("ts"));
    assert!(is_time_dataset_name("/time"));
    assert!(is_time_dataset_name("/Timestamp"));
}

#[test]
fn is_time_dataset_name_rejects_unrelated_names() {
    assert!(!is_time_dataset_name("temperature"));
    assert!(!is_time_dataset_name("voltage"));
    assert!(!is_time_dataset_name("time_series"));
    assert!(!is_time_dataset_name("my_time"));
    assert!(!is_time_dataset_name(""));
}

#[test]
fn is_time_dataset_name_recognizes_nested_paths() {
    assert!(is_time_dataset_name("/group1/time"));
    assert!(is_time_dataset_name("/a/b/c/Timestamp"));
    assert!(is_time_dataset_name("nested/ts"));
    assert!(!is_time_dataset_name("/group1/time_series"));
    assert!(!is_time_dataset_name("/time/sensor"));
}

#[test]
fn basename_returns_leaf() {
    assert_eq!(basename("/group/sub/leaf"), "leaf");
    assert_eq!(basename("/leaf"), "leaf");
    assert_eq!(basename("leaf"), "leaf");
    assert_eq!(basename("/"), "");
}

#[test]
fn parent_path_walks_up() {
    assert_eq!(parent_path("/a/b/c"), "/a/b");
    assert_eq!(parent_path("/a"), "/");
    assert_eq!(parent_path("/"), "/");
    assert_eq!(parent_path("leaf"), "/");
}

#[test]
fn hdf5_to_sift_data_type_maps_boolean() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Boolean),
        Some(ChannelDataType::Bool)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_integer_u1() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Integer(IntSize::U1)),
        Some(ChannelDataType::Int32)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_integer_u2() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Integer(IntSize::U2)),
        Some(ChannelDataType::Int32)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_integer_u4() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Integer(IntSize::U4)),
        Some(ChannelDataType::Int32)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_integer_u8() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Integer(IntSize::U8)),
        Some(ChannelDataType::Int64)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_unsigned_u1() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Unsigned(IntSize::U1)),
        Some(ChannelDataType::Uint32)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_unsigned_u4() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Unsigned(IntSize::U4)),
        Some(ChannelDataType::Uint32)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_unsigned_u8() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Unsigned(IntSize::U8)),
        Some(ChannelDataType::Uint64)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_float_u4() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Float(FloatSize::U4)),
        Some(ChannelDataType::Float)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_float_u8() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::Float(FloatSize::U8)),
        Some(ChannelDataType::Double)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_strings() {
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::VarLenUnicode),
        Some(ChannelDataType::String)
    );
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::VarLenAscii),
        Some(ChannelDataType::String)
    );
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::FixedAscii(16)),
        Some(ChannelDataType::String)
    );
    assert_eq!(
        hdf5_to_sift_data_type(&TypeDescriptor::FixedUnicode(16)),
        Some(ChannelDataType::String)
    );
}

#[test]
fn hdf5_to_sift_data_type_maps_enum() {
    let ty = TypeDescriptor::Enum(EnumType {
        size: IntSize::U4,
        signed: false,
        members: vec![EnumMember {
            name: "RED".into(),
            value: 0,
        }],
    });
    assert_eq!(hdf5_to_sift_data_type(&ty), Some(ChannelDataType::Enum));
}

#[test]
fn enum_types_for_extracts_members() {
    let ty = TypeDescriptor::Enum(EnumType {
        size: IntSize::U4,
        signed: true,
        members: vec![
            EnumMember {
                name: "OFF".into(),
                value: 0,
            },
            EnumMember {
                name: "ON".into(),
                value: 1,
            },
        ],
    });
    let mapped = enum_types_for(&ty).unwrap();
    assert_eq!(mapped.len(), 2);
    assert_eq!(mapped[0].name, "OFF");
    assert_eq!(mapped[0].key, 0);
    assert!(mapped[0].is_signed);
    assert_eq!(mapped[1].name, "ON");
    assert_eq!(mapped[1].key, 1);
}

#[test]
fn enum_types_for_returns_empty_for_non_enum() {
    assert!(enum_types_for(&TypeDescriptor::Boolean).unwrap().is_empty());
    assert!(
        enum_types_for(&TypeDescriptor::Integer(IntSize::U4))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn group_path_to_channel_name_root_dataset() {
    assert_eq!(group_path_to_channel_name("/cpu_usage"), "cpu_usage");
}

#[test]
fn group_path_to_channel_name_single_nested_group() {
    assert_eq!(
        group_path_to_channel_name("/group1/current"),
        "group1.current"
    );
}

#[test]
fn group_path_to_channel_name_deeply_nested() {
    assert_eq!(
        group_path_to_channel_name("/group2/group3/group4/cell_voltage"),
        "group2.group3.group4.cell_voltage"
    );
}

#[test]
fn group_path_to_channel_name_no_leading_slash() {
    assert_eq!(
        group_path_to_channel_name("group1/current"),
        "group1.current"
    );
}
