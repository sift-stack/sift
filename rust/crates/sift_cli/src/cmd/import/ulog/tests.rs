use std::path::PathBuf;

use crate::cli::ulog::UlogParseErrorPolicy;
use crate::cli::{CommonImportArgs, ImportUlogArgs};
use crate::cmd::import::ulog::detect_ulog_config::{
    detect_ulog_channels, scan_ulog, ulog_to_sift_data_type,
};
use crate::cmd::import::ulog::import::build_ulog_config;
use chrono::DateTime;
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::data_imports::v2::UlogParseErrorPolicy as ProtoUlogParseErrorPolicy;

fn make_args() -> ImportUlogArgs {
    ImportUlogArgs {
        common: CommonImportArgs {
            path: PathBuf::from("test.ulg"),
            asset: "test-asset".into(),
            run: None,
            run_id: None,
            wait: false,
            preview: false,
        },
        relative_start_time: None,
        info_key: Vec::new(),
        param_key: Vec::new(),
        parse_error_policy: UlogParseErrorPolicy::FailOnError,
    }
}

/// Valid 16-byte ULog header fixture.
fn ulog_header() -> Vec<u8> {
    let mut data = vec![0x55, 0x4c, 0x6f, 0x67, 0x01, 0x12, 0x35, 0x01];
    data.extend_from_slice(&0u64.to_le_bytes());
    data
}

fn push_message(data: &mut Vec<u8>, msg_type: u8, payload: &[u8]) {
    data.extend_from_slice(&(payload.len() as u16).to_le_bytes());
    data.push(msg_type);
    data.extend_from_slice(payload);
}

fn push_format(data: &mut Vec<u8>, definition: &str) {
    push_message(data, b'F', definition.as_bytes());
}

fn push_subscription(data: &mut Vec<u8>, multi_id: u8, msg_id: u16, message_name: &str) {
    let mut payload = vec![multi_id];
    payload.extend_from_slice(&msg_id.to_le_bytes());
    payload.extend_from_slice(message_name.as_bytes());
    push_message(data, b'A', &payload);
}

fn push_untagged_log(data: &mut Vec<u8>) {
    let mut payload = vec![0u8];
    payload.extend_from_slice(&0u64.to_le_bytes());
    payload.extend_from_slice(b"a log line");
    push_message(data, b'L', &payload);
}

fn push_tagged_log(data: &mut Vec<u8>, tag: u16) {
    let mut payload = vec![0u8];
    payload.extend_from_slice(&tag.to_le_bytes());
    payload.extend_from_slice(&0u64.to_le_bytes());
    payload.extend_from_slice(b"a log line");
    push_message(data, b'C', &payload);
}

fn detect(data: &[u8]) -> Vec<(String, ChannelDataType)> {
    let scan = scan_ulog(data).expect("scan should succeed");
    detect_ulog_channels(&scan).expect("detection should succeed")
}

#[test]
fn build_ulog_config_defaults() {
    let args = make_args();
    let cfg = build_ulog_config(&args).expect("defaults should build");
    assert_eq!(cfg.asset_name, "test-asset");
    assert_eq!(cfg.run_name, "");
    assert_eq!(cfg.run_id, "");
    assert!(cfg.data.is_empty());
    assert!(cfg.relative_start_time.is_none());
    assert!(cfg.info_keys.is_empty());
    assert!(cfg.param_keys.is_empty());
    assert_eq!(
        cfg.parse_error_policy,
        ProtoUlogParseErrorPolicy::FailOnError as i32
    );
}

#[test]
fn build_ulog_config_run_name_passes_through() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    let cfg = build_ulog_config(&args).expect("build");
    assert_eq!(cfg.run_name, "my-run");
    assert_eq!(cfg.run_id, "");
}

#[test]
fn build_ulog_config_run_id_passes_through() {
    let mut args = make_args();
    args.common.run_id = Some("run-abc-123".into());
    let cfg = build_ulog_config(&args).expect("build");
    assert_eq!(cfg.run_id, "run-abc-123");
    assert_eq!(cfg.run_name, "");
}

#[test]
fn build_ulog_config_run_id_takes_precedence_over_run_name() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    args.common.run_id = Some("run-abc-123".into());
    let cfg = build_ulog_config(&args).expect("build");
    assert_eq!(cfg.run_id, "run-abc-123");
    assert_eq!(cfg.run_name, "");
}

#[test]
fn build_ulog_config_parse_error_policy_ignore_error() {
    let mut args = make_args();
    args.parse_error_policy = UlogParseErrorPolicy::IgnoreError;
    let cfg = build_ulog_config(&args).expect("build");
    assert_eq!(
        cfg.parse_error_policy,
        ProtoUlogParseErrorPolicy::IgnoreError as i32
    );
}

#[test]
fn build_ulog_config_relative_start_time_parses_rfc3339() {
    let mut args = make_args();
    args.relative_start_time = Some("2026-05-06T12:00:00Z".into());
    let cfg = build_ulog_config(&args).expect("build");
    let ts = cfg.relative_start_time.expect("timestamp");
    let expected = DateTime::parse_from_rfc3339(args.relative_start_time.as_deref().unwrap())
        .unwrap()
        .timestamp();
    assert_eq!(ts.seconds, expected);
    assert_eq!(ts.nanos, 0);
}

#[test]
fn build_ulog_config_invalid_relative_start_time_errors() {
    let mut args = make_args();
    args.relative_start_time = Some("not a date".into());
    let err = build_ulog_config(&args).unwrap_err();
    assert!(
        err.chain().any(|e| e.to_string().contains("RFC3339")),
        "expected RFC3339 error, got: {err:#}"
    );
}

#[test]
fn build_ulog_config_info_and_param_keys_require_run() {
    let mut args = make_args();
    args.info_key = vec!["sys_name".into()];
    let err = build_ulog_config(&args).unwrap_err();
    assert!(
        err.to_string().contains("--run"),
        "expected run validation error, got: {err:#}"
    );

    let mut args = make_args();
    args.param_key = vec!["MC_PITCH_P".into()];
    let err = build_ulog_config(&args).unwrap_err();
    assert!(
        err.to_string().contains("--run"),
        "expected run validation error, got: {err:#}"
    );
}

#[test]
fn build_ulog_config_info_and_param_keys_pass_through_with_run() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    args.info_key = vec!["sys_name".into(), "ver_hw".into()];
    args.param_key = vec!["MC_PITCH_P".into()];
    let cfg = build_ulog_config(&args).expect("build");
    assert_eq!(cfg.info_keys, vec!["sys_name", "ver_hw"]);
    assert_eq!(cfg.param_keys, vec!["MC_PITCH_P"]);
}

#[test]
fn test_ulog_to_sift_data_type() {
    assert_eq!(
        ulog_to_sift_data_type("int8_t"),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        ulog_to_sift_data_type("int16_t"),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        ulog_to_sift_data_type("int32_t"),
        Some(ChannelDataType::Int32)
    );
    assert_eq!(
        ulog_to_sift_data_type("int64_t"),
        Some(ChannelDataType::Int64)
    );
    assert_eq!(
        ulog_to_sift_data_type("uint8_t"),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        ulog_to_sift_data_type("uint16_t"),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        ulog_to_sift_data_type("uint32_t"),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        ulog_to_sift_data_type("uint64_t"),
        Some(ChannelDataType::Uint64)
    );
    assert_eq!(
        ulog_to_sift_data_type("float"),
        Some(ChannelDataType::Float)
    );
    assert_eq!(
        ulog_to_sift_data_type("double"),
        Some(ChannelDataType::Double)
    );
    assert_eq!(ulog_to_sift_data_type("bool"), Some(ChannelDataType::Bool));
    assert_eq!(
        ulog_to_sift_data_type("char"),
        Some(ChannelDataType::String)
    );
    assert_eq!(ulog_to_sift_data_type("vehicle_status"), None);
}

#[test]
fn detect_lists_subscribed_topic_channels() {
    let mut data = ulog_header();
    push_format(
        &mut data,
        "sensor_accel:uint64_t timestamp;float x;float y;",
    );
    push_subscription(&mut data, 0, 1, "sensor_accel");

    // The exact match also pins that the timestamp axis is not a channel.
    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![
            ("sensor_accel_0.x".to_string(), ChannelDataType::Float),
            ("sensor_accel_0.y".to_string(), ChannelDataType::Float),
        ]
    );
}

#[test]
fn detect_expands_scalar_arrays() {
    let mut data = ulog_header();
    push_format(
        &mut data,
        "actuator_outputs:uint64_t timestamp;float[3] output;",
    );
    push_subscription(&mut data, 0, 1, "actuator_outputs");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(
        names,
        vec![
            "actuator_outputs_0.output[0]",
            "actuator_outputs_0.output[1]",
            "actuator_outputs_0.output[2]",
        ]
    );
}

#[test]
fn detect_flattens_nested_message_types() {
    let mut data = ulog_header();
    push_format(&mut data, "esc_report:int32_t esc_rpm;bool failures;");
    push_format(
        &mut data,
        "esc_status:uint64_t timestamp;esc_report[2] esc;uint8_t count;",
    );
    push_subscription(&mut data, 0, 1, "esc_status");

    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![
            (
                "esc_status_0.esc[0].esc_rpm".to_string(),
                ChannelDataType::Int32
            ),
            (
                "esc_status_0.esc[0].failures".to_string(),
                ChannelDataType::Bool
            ),
            (
                "esc_status_0.esc[1].esc_rpm".to_string(),
                ChannelDataType::Int32
            ),
            (
                "esc_status_0.esc[1].failures".to_string(),
                ChannelDataType::Bool
            ),
            ("esc_status_0.count".to_string(), ChannelDataType::Uint32),
        ]
    );
}

#[test]
fn detect_collapses_char_arrays_to_one_string_channel() {
    let mut data = ulog_header();
    push_format(
        &mut data,
        "mission_result:uint64_t timestamp;char[10] name;char flag;",
    );
    push_subscription(&mut data, 0, 1, "mission_result");

    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![
            ("mission_result_0.name".to_string(), ChannelDataType::String),
            ("mission_result_0.flag".to_string(), ChannelDataType::String),
        ]
    );
}

#[test]
fn detect_skips_padding_fields() {
    let mut data = ulog_header();
    push_format(
        &mut data,
        "sensor_accel:uint64_t timestamp;float x;uint8_t[3] _padding0;",
    );
    push_subscription(&mut data, 0, 1, "sensor_accel");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["sensor_accel_0.x"]);
}

#[test]
fn detect_skips_topic_without_timestamp() {
    let mut data = ulog_header();
    push_format(&mut data, "no_time_topic:float x;float y;");
    push_subscription(&mut data, 0, 1, "no_time_topic");

    assert!(detect(&data).is_empty());
}

#[test]
fn detect_skips_subscription_without_format() {
    let mut data = ulog_header();
    push_subscription(&mut data, 0, 1, "unknown_topic");

    assert!(detect(&data).is_empty());
}

#[test]
fn detect_dedups_repeated_subscriptions() {
    let mut data = ulog_header();
    push_format(&mut data, "sensor_accel:uint64_t timestamp;float x;");
    push_subscription(&mut data, 0, 1, "sensor_accel");
    push_subscription(&mut data, 0, 2, "sensor_accel");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["sensor_accel_0.x"]);
}

#[test]
fn detect_multi_instance_topics_get_distinct_channels() {
    let mut data = ulog_header();
    push_format(&mut data, "sensor_accel:uint64_t timestamp;float x;");
    push_subscription(&mut data, 0, 1, "sensor_accel");
    push_subscription(&mut data, 1, 2, "sensor_accel");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["sensor_accel_0.x", "sensor_accel_1.x"]);
}

#[test]
fn detect_adds_untagged_log_messages_channel() {
    let mut data = ulog_header();
    push_untagged_log(&mut data);

    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![("log_messages".to_string(), ChannelDataType::String)]
    );
}

#[test]
fn detect_adds_tagged_log_channels_sorted_by_tag() {
    let mut data = ulog_header();
    push_tagged_log(&mut data, 7);
    push_tagged_log(&mut data, 3);
    push_tagged_log(&mut data, 7);

    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![
            ("log_messages_3".to_string(), ChannelDataType::String),
            ("log_messages_7".to_string(), ChannelDataType::String),
        ]
    );
}

#[test]
fn detect_ignores_formats_after_a_subscription() {
    let mut data = ulog_header();
    push_subscription(&mut data, 0, 1, "late_topic");
    push_format(&mut data, "late_topic:uint64_t timestamp;float x;");

    assert!(detect(&data).is_empty());
}

#[test]
fn detect_ignores_formats_after_logged_strings() {
    // Untagged 'L' and tagged 'C' log messages both end the definitions
    // section, so a format defined after either is never registered.
    let mut data = ulog_header();
    push_untagged_log(&mut data);
    push_format(&mut data, "late_topic:uint64_t timestamp;float x;");
    push_subscription(&mut data, 0, 1, "late_topic");
    assert_eq!(
        detect(&data),
        vec![("log_messages".to_string(), ChannelDataType::String)]
    );

    let mut data = ulog_header();
    push_tagged_log(&mut data, 1);
    push_format(&mut data, "late_topic:uint64_t timestamp;float x;");
    push_subscription(&mut data, 0, 1, "late_topic");
    assert_eq!(
        detect(&data),
        vec![("log_messages_1".to_string(), ChannelDataType::String)]
    );
}

#[test]
fn detect_keeps_formats_after_a_stray_data_record() {
    let mut data = ulog_header();
    push_format(&mut data, "first:uint64_t timestamp;float x;");
    push_message(&mut data, b'D', &[0x00, 0x00, 0x01, 0x02]);
    push_format(&mut data, "late:uint64_t timestamp;float y;");
    push_subscription(&mut data, 0, 0, "first");
    push_subscription(&mut data, 0, 1, "late");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["first_0.x", "late_0.y"]);
}

#[test]
fn detect_skips_unknown_message_types_in_definitions() {
    let mut data = ulog_header();
    push_format(&mut data, "first:uint64_t timestamp;float x;");
    push_message(&mut data, b'Z', &[0xaa, 0xbb]);
    push_format(&mut data, "late:uint64_t timestamp;float y;");
    data.extend_from_slice(&[
        0x08, 0x00, b'S', 0x2f, 0x73, 0x13, 0x20, 0x25, 0x0c, 0xbb, 0x12,
    ]);
    push_subscription(&mut data, 0, 0, "first");
    push_subscription(&mut data, 0, 1, "late");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["first_0.x", "late_0.y"]);
}

#[test]
fn detect_skips_malformed_formats_and_keeps_the_rest() {
    let mut data = ulog_header();
    push_format(&mut data, "no colon in this payload");
    push_format(&mut data, "fieldless:uint64_t;float x;");
    push_format(&mut data, "good:uint64_t timestamp;float x;");
    push_subscription(&mut data, 0, 1, "good");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["good_0.x"]);
}

#[test]
fn detect_drops_invalid_utf8_bytes_in_formats() {
    let mut data = ulog_header();
    let mut payload = b"m".to_vec();
    payload.push(0xff);
    payload.extend_from_slice(b"1:uint64_t timestamp;float x");
    payload.push(0xff);
    payload.extend_from_slice(b"y;");
    push_message(&mut data, b'F', &payload);
    push_subscription(&mut data, 0, 1, "m1");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["m1_0.xy"]);
}

#[test]
fn detect_reads_only_the_first_format_segment() {
    let mut data = ulog_header();
    push_format(&mut data, "m:uint64_t timestamp;float a:b;double y;");
    push_subscription(&mut data, 0, 1, "m");

    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![("m_0.a".to_string(), ChannelDataType::Float)]
    );
}

#[test]
fn detect_ignores_extra_field_tokens() {
    // Only the first two space-separated tokens type and name a field, so
    // neither an extra token nor a trailing space hides the timestamp axis.
    let mut data = ulog_header();
    push_format(&mut data, "m:uint64_t timestamp extra;float x;");
    push_subscription(&mut data, 0, 1, "m");
    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["m_0.x"]);

    let mut data = ulog_header();
    push_format(&mut data, "m:uint64_t timestamp ;float x;");
    push_subscription(&mut data, 0, 1, "m");
    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["m_0.x"]);
}

#[test]
fn detect_treats_negative_array_size_as_scalar() {
    let mut data = ulog_header();
    push_format(&mut data, "m:uint64_t timestamp;float[-1] x;");
    push_subscription(&mut data, 0, 1, "m");

    let channels = detect(&data);
    assert_eq!(
        channels,
        vec![("m_0.x".to_string(), ChannelDataType::Float)]
    );
}

#[test]
fn detect_resyncs_after_garbage_bytes() {
    let mut data = ulog_header();
    push_format(&mut data, "sensor_accel:uint64_t timestamp;float x;");
    data.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef, 0xff]);
    data.extend_from_slice(&[
        0x08, 0x00, b'S', 0x2f, 0x73, 0x13, 0x20, 0x25, 0x0c, 0xbb, 0x12,
    ]);
    push_subscription(&mut data, 0, 1, "sensor_accel");

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["sensor_accel_0.x"]);
}

#[test]
fn detect_stops_cleanly_on_truncated_final_record() {
    let mut data = ulog_header();
    push_format(&mut data, "sensor_accel:uint64_t timestamp;float x;");
    push_subscription(&mut data, 0, 1, "sensor_accel");
    data.extend_from_slice(&100u16.to_le_bytes());
    data.push(b'D');
    data.extend_from_slice(&[0x01, 0x02]);

    let names: Vec<String> = detect(&data).into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, vec!["sensor_accel_0.x"]);
}

#[test]
fn scan_rejects_bad_magic() {
    let mut data = ulog_header();
    data[0] = 0x00;
    let err = scan_ulog(&data).unwrap_err();
    assert!(
        err.to_string().contains("magic"),
        "expected magic error, got: {err:#}"
    );
}

#[test]
fn scan_rejects_file_shorter_than_header() {
    let err = scan_ulog(&[0x55, 0x4c, 0x6f, 0x67]).unwrap_err();
    assert!(
        err.to_string().contains("size"),
        "expected size error, got: {err:#}"
    );
}

#[test]
fn detect_errors_on_unknown_nested_type() {
    let mut data = ulog_header();
    push_format(
        &mut data,
        "esc_status:uint64_t timestamp;missing_report esc;",
    );
    push_subscription(&mut data, 0, 1, "esc_status");

    let scan = scan_ulog(&data).expect("scan should succeed");
    let err = detect_ulog_channels(&scan).unwrap_err();
    assert!(
        err.to_string().contains("missing_report"),
        "expected unknown type error, got: {err:#}"
    );
}

#[test]
fn detect_errors_on_cyclic_formats() {
    let mut data = ulog_header();
    push_format(&mut data, "a_topic:uint64_t timestamp;b_inner b;");
    push_format(&mut data, "b_inner:a_topic a;");
    push_subscription(&mut data, 0, 1, "a_topic");

    let scan = scan_ulog(&data).expect("scan should succeed");
    let err = detect_ulog_channels(&scan).unwrap_err();
    assert!(
        err.to_string().contains("cyclic"),
        "expected cyclic format error, got: {err:#}"
    );
}
