use std::path::PathBuf;

use chrono::DateTime;
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    data_imports::v2::{
        McapComplexTypesImportMode as ProtoMcapComplexTypesImportMode,
        McapParseErrorPolicy as ProtoMcapParseErrorPolicy,
    },
};

use crate::cli::mcap::{McapComplexTypesMode, McapParseErrorPolicy};
use crate::cli::{CommonImportArgs, ImportMcapArgs};
use crate::cmd::import::mcap::detect_mcap_config::DetectedChannel;
use crate::cmd::import::mcap::import::{build_mcap_config, channel_configs};
use crate::cmd::import::mcap::ros2_schema::{expand_message_fields, parse_schema_defs};

fn make_args() -> ImportMcapArgs {
    ImportMcapArgs {
        common: CommonImportArgs {
            path: PathBuf::from("test.mcap"),
            asset: "test-asset".into(),
            run: None,
            run_id: None,
            wait: false,
            preview: false,
        },
        relative_start_time: None,
        metadata_record: Vec::new(),
        parse_error_policy: McapParseErrorPolicy::FailOnError,
        complex_types_import_mode: McapComplexTypesMode::Both,
    }
}

/// Detects the channels of a `ros2msg` schema, as `(name, data type)` pairs.
fn detect(schema_name: &str, schema_text: &str) -> Vec<(String, ChannelDataType)> {
    let (root, msgdefs) = parse_schema_defs(schema_name, schema_text).expect("schema should parse");
    expand_message_fields(&root, &msgdefs)
        .expect("expansion should succeed")
        .into_iter()
        .map(|leaf| (leaf.field_path, leaf.data_type))
        .collect()
}

fn detect_err(schema_name: &str, schema_text: &str) -> String {
    let result = parse_schema_defs(schema_name, schema_text)
        .and_then(|(root, msgdefs)| expand_message_fields(&root, &msgdefs));
    match result {
        Ok(leaves) => panic!("expected an error, got {leaves:?}"),
        Err(e) => e.to_string(),
    }
}

fn names(schema_name: &str, schema_text: &str) -> Vec<String> {
    detect(schema_name, schema_text)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

#[test]
fn build_mcap_config_defaults() {
    let args = make_args();
    let cfg = build_mcap_config(&args).expect("defaults should build");
    assert_eq!(cfg.asset_name, "test-asset");
    assert_eq!(cfg.run_name, "");
    assert_eq!(cfg.run_id, "");
    // An empty channel list imports every channel in the file.
    assert!(cfg.data.is_empty());
    assert!(cfg.relative_start_time.is_none());
    assert!(cfg.metadata_records.is_empty());
    assert_eq!(
        cfg.parse_error_policy,
        ProtoMcapParseErrorPolicy::FailOnError as i32
    );
    assert_eq!(
        cfg.complex_types_import_mode,
        ProtoMcapComplexTypesImportMode::Both as i32
    );
}

#[test]
fn build_mcap_config_run_name_passes_through() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    let cfg = build_mcap_config(&args).expect("build");
    assert_eq!(cfg.run_name, "my-run");
    assert_eq!(cfg.run_id, "");
}

#[test]
fn build_mcap_config_run_id_passes_through() {
    let mut args = make_args();
    args.common.run_id = Some("run-abc-123".into());
    let cfg = build_mcap_config(&args).expect("build");
    assert_eq!(cfg.run_id, "run-abc-123");
    assert_eq!(cfg.run_name, "");
}

#[test]
fn build_mcap_config_parse_error_policy_ignore_error() {
    let mut args = make_args();
    args.parse_error_policy = McapParseErrorPolicy::IgnoreError;
    let cfg = build_mcap_config(&args).expect("build");
    assert_eq!(
        cfg.parse_error_policy,
        ProtoMcapParseErrorPolicy::IgnoreError as i32
    );
}

#[test]
fn build_mcap_config_complex_types_modes_pass_through() {
    for (mode, expected) in [
        (
            McapComplexTypesMode::Both,
            ProtoMcapComplexTypesImportMode::Both,
        ),
        (
            McapComplexTypesMode::String,
            ProtoMcapComplexTypesImportMode::String,
        ),
        (
            McapComplexTypesMode::Bytes,
            ProtoMcapComplexTypesImportMode::Bytes,
        ),
        (
            McapComplexTypesMode::Ignore,
            ProtoMcapComplexTypesImportMode::Ignore,
        ),
    ] {
        let mut args = make_args();
        args.complex_types_import_mode = mode;
        let cfg = build_mcap_config(&args).expect("build");
        assert_eq!(
            cfg.complex_types_import_mode, expected as i32,
            "mode {mode}"
        );
    }
}

#[test]
fn build_mcap_config_relative_start_time_parses_rfc3339() {
    let mut args = make_args();
    args.relative_start_time = Some("2026-05-06T12:00:00Z".into());
    let cfg = build_mcap_config(&args).expect("build");
    let ts = cfg.relative_start_time.expect("timestamp");
    let expected = DateTime::parse_from_rfc3339(args.relative_start_time.as_deref().unwrap())
        .unwrap()
        .timestamp();
    assert_eq!(ts.seconds, expected);
    assert_eq!(ts.nanos, 0);
}

#[test]
fn build_mcap_config_invalid_relative_start_time_errors() {
    let mut args = make_args();
    args.relative_start_time = Some("not a date".into());
    let err = build_mcap_config(&args).unwrap_err();
    assert!(
        err.chain().any(|e| e.to_string().contains("RFC3339")),
        "expected RFC3339 error, got: {err:#}"
    );
}

#[test]
fn build_mcap_config_metadata_records_require_run() {
    let mut args = make_args();
    args.metadata_record = vec!["hardware".into()];
    let err = build_mcap_config(&args).unwrap_err();
    assert!(
        err.to_string().contains("--run"),
        "expected run validation error, got: {err:#}"
    );
}

#[test]
fn build_mcap_config_metadata_records_pass_through_with_run() {
    let mut args = make_args();
    args.common.run = Some("my-run".into());
    args.metadata_record = vec!["hardware".into(), "software".into()];
    let cfg = build_mcap_config(&args).expect("build");
    assert_eq!(cfg.metadata_records, vec!["hardware", "software"]);
}

fn detected(name: &str, data_type: ChannelDataType, complex: bool) -> DetectedChannel {
    DetectedChannel {
        topic: "/t".into(),
        field_path: name.into(),
        name: name.into(),
        data_type,
        complex,
    }
}

#[test]
fn channel_configs_leaves_scalar_channels_alone_in_every_mode() {
    let scalars = vec![detected("x", ChannelDataType::Double, false)];
    for mode in [
        McapComplexTypesMode::Both,
        McapComplexTypesMode::String,
        McapComplexTypesMode::Bytes,
        McapComplexTypesMode::Ignore,
    ] {
        let configs = channel_configs(&scalars, mode);
        assert_eq!(configs.len(), 1, "mode {mode}");
        assert_eq!(configs[0].name, "x");
        assert_eq!(configs[0].data_type, ChannelDataType::Double as i32);
    }
}

#[test]
fn channel_configs_expands_complex_fields_per_mode() {
    let complex = vec![detected("samples", ChannelDataType::Bytes, true)];

    let both = channel_configs(&complex, McapComplexTypesMode::Both);
    assert_eq!(
        both.iter()
            .map(|c| (c.name.as_str(), c.data_type))
            .collect::<Vec<_>>(),
        vec![
            ("samples", ChannelDataType::Bytes as i32),
            ("samples.json", ChannelDataType::String as i32),
        ]
    );

    let bytes = channel_configs(&complex, McapComplexTypesMode::Bytes);
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0].name, "samples");
    assert_eq!(bytes[0].data_type, ChannelDataType::Bytes as i32);

    let string = channel_configs(&complex, McapComplexTypesMode::String);
    assert_eq!(string.len(), 1);
    assert_eq!(string[0].name, "samples.json");
    assert_eq!(string[0].data_type, ChannelDataType::String as i32);

    assert!(channel_configs(&complex, McapComplexTypesMode::Ignore).is_empty());
}

#[test]
fn detect_maps_ros2_scalar_types_to_sift_types() {
    let schema = "\
bool a
int8 b
int16 c
int32 d
int64 e
uint8 f
uint16 g
uint32 h
uint64 i
byte j
char k
float32 l
float64 m
string n
";
    assert_eq!(
        detect("pkg_msgs/msg/Scalars", schema),
        vec![
            ("a".to_string(), ChannelDataType::Bool),
            ("b".to_string(), ChannelDataType::Int32),
            ("c".to_string(), ChannelDataType::Int32),
            ("d".to_string(), ChannelDataType::Int32),
            ("e".to_string(), ChannelDataType::Int64),
            ("f".to_string(), ChannelDataType::Uint32),
            ("g".to_string(), ChannelDataType::Uint32),
            ("h".to_string(), ChannelDataType::Uint32),
            ("i".to_string(), ChannelDataType::Uint64),
            ("j".to_string(), ChannelDataType::Uint32),
            ("k".to_string(), ChannelDataType::Uint32),
            ("l".to_string(), ChannelDataType::Float),
            ("m".to_string(), ChannelDataType::Double),
            ("n".to_string(), ChannelDataType::String),
        ]
    );
}

#[test]
fn detect_flattens_nested_message_types() {
    let schema = "\
geometry_msgs/Vector3 angular_velocity
================================================================================
MSG: geometry_msgs/Vector3
float64 x
float64 y
float64 z
";
    assert_eq!(
        names("sensor_msgs/msg/Imu", schema),
        vec![
            "angular_velocity.x",
            "angular_velocity.y",
            "angular_velocity.z"
        ]
    );
}

#[test]
fn detect_expands_fixed_size_arrays_with_indexes() {
    let schema = "float64[3] covariance\n";
    assert_eq!(
        names("pkg_msgs/msg/Arr", schema),
        vec!["covariance[0]", "covariance[1]", "covariance[2]"]
    );
}

#[test]
fn detect_expands_fixed_size_arrays_of_messages() {
    let schema = "\
geometry_msgs/Vector3[2] pair
================================================================================
MSG: geometry_msgs/Vector3
float64 x
float64 y
";
    assert_eq!(
        names("pkg_msgs/msg/Pairs", schema),
        vec!["pair[0].x", "pair[0].y", "pair[1].x", "pair[1].y"]
    );
}

#[test]
fn detect_treats_dynamic_and_bounded_arrays_as_one_complex_channel() {
    let schema = "\
float64[] samples
uint8[<=4] flags
";
    let (root, msgdefs) = parse_schema_defs("pkg_msgs/msg/Var", schema).expect("parse");
    let leaves = expand_message_fields(&root, &msgdefs).expect("expand");
    assert_eq!(leaves.len(), 2);
    for leaf in &leaves {
        assert!(leaf.complex, "{} should be complex", leaf.field_path);
        assert_eq!(leaf.data_type, ChannelDataType::Bytes);
    }
    assert_eq!(
        leaves
            .iter()
            .map(|l| l.field_path.as_str())
            .collect::<Vec<_>>(),
        vec!["samples", "flags"]
    );
}

#[test]
fn detect_collapses_time_and_duration_to_one_int64_channel() {
    let schema = "\
builtin_interfaces/Time stamp
builtin_interfaces/Duration timeout
";
    assert_eq!(
        detect("pkg_msgs/msg/Stamped", schema),
        vec![
            ("stamp".to_string(), ChannelDataType::Int64),
            ("timeout".to_string(), ChannelDataType::Int64),
        ]
    );
}

#[test]
fn detect_reads_bounded_strings_as_string() {
    let schema = "\
string<=16 label
string<=8[2] tags
";
    assert_eq!(
        detect("pkg_msgs/msg/Bounded", schema),
        vec![
            ("label".to_string(), ChannelDataType::String),
            ("tags[0]".to_string(), ChannelDataType::String),
            ("tags[1]".to_string(), ChannelDataType::String),
        ]
    );
}

#[test]
fn detect_skips_constants_and_comments() {
    let schema = "\
# a file-level comment
uint8 STATUS_OK=0
uint8 STATUS_BAD=1
uint8 status
    # an indented comment
float64 value   # a trailing comment
";
    assert_eq!(
        names("pkg_msgs/msg/Consts", schema),
        vec!["status", "value"]
    );
}

#[test]
fn detect_resolves_unqualified_type_names_against_the_referencing_package() {
    // `Vector3` has no package prefix, so it resolves within geometry_msgs.
    let schema = "\
geometry_msgs/Twist twist
================================================================================
MSG: geometry_msgs/Twist
Vector3 linear
================================================================================
MSG: geometry_msgs/Vector3
float64 x
";
    assert_eq!(names("pkg_msgs/msg/Cmd", schema), vec!["twist.linear.x"]);
}

#[test]
fn detect_accepts_a_schema_named_with_the_msg_infix() {
    // "std_msgs/msg/String" is looked up as "std_msgs/String" too.
    let schema = "string data\n";
    assert_eq!(names("std_msgs/msg/String", schema), vec!["data"]);
}

#[test]
fn detect_errors_on_wstring_fields() {
    let err = detect_err("pkg_msgs/msg/W", "wstring bad\n");
    assert!(
        err.contains("wstring"),
        "expected wstring error, got: {err}"
    );
}

#[test]
fn detect_errors_on_unsupported_legacy_types() {
    let err = detect_err("pkg_msgs/msg/L", "duration legacy\n");
    assert!(
        err.contains("unsupported type 'duration'"),
        "expected unsupported type error, got: {err}"
    );
}

#[test]
fn detect_errors_on_unknown_nested_type() {
    let err = detect_err("pkg_msgs/msg/N", "some_pkg/Missing thing\n");
    assert!(
        err.contains("some_pkg/Missing"),
        "expected unknown type error, got: {err}"
    );
}

#[test]
fn detect_errors_on_undecodable_element_of_a_dynamic_array() {
    // The array imports whole, but its elements are still decoded.
    let schema = "\
pkg_msgs/Inner[] items
================================================================================
MSG: pkg_msgs/Inner
wstring bad
";
    let err = detect_err("pkg_msgs/msg/Outer", schema);
    assert!(
        err.contains("wstring"),
        "expected wstring error, got: {err}"
    );
}

#[test]
fn detect_errors_on_cyclic_fixed_nesting() {
    let schema = "\
a_pkg/Inner inner
================================================================================
MSG: a_pkg/Inner
a_pkg/Outer outer
================================================================================
MSG: a_pkg/Outer
a_pkg/Inner inner
";
    let err = detect_err("a_pkg/msg/Deep", schema);
    assert!(
        err.contains("nests deeper than"),
        "expected nesting depth error, got: {err}"
    );
}

#[test]
fn detect_errors_when_the_schema_does_not_define_the_root_message() {
    let schema = "\
MSG: other_pkg/Unrelated
float64 x
";
    let result = parse_schema_defs("pkg_msgs/msg/Missing", schema);
    let err = result.expect_err("expected an error").to_string();
    assert!(
        err.contains("root message"),
        "expected root message error, got: {err}"
    );
}

#[test]
fn detect_errors_on_a_field_line_without_a_name() {
    let err = detect_err("pkg_msgs/msg/Bad", "float64\n");
    assert!(
        err.contains("field definition"),
        "expected field definition error, got: {err}"
    );
}

#[test]
fn detect_errors_on_a_zero_length_fixed_array() {
    let err = detect_err("pkg_msgs/msg/Bad", "float64[0] x\n");
    assert!(
        err.contains("integer > 0"),
        "expected array size error, got: {err}"
    );
}

#[test]
fn detect_errors_on_an_invalid_message_name() {
    // ROS 2 message names are CamelCase; `lowercase` is not a valid one.
    let err = detect_err("pkg_msgs/msg/Bad", "some_pkg/lowercase thing\n");
    assert!(
        err.contains("valid message name"),
        "expected message name error, got: {err}"
    );
}

#[test]
fn detect_ignores_blank_lines_and_separators_of_varying_length() {
    // A separator is three or more '=' characters on a line of their own.
    let schema = "\
geometry_msgs/Vector3 v

===
MSG: geometry_msgs/Vector3

float64 x
";
    assert_eq!(names("pkg_msgs/msg/Sep", schema), vec!["v.x"]);
}

/// Builds an MCAP file on disk and detects its channels.
mod files {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::{Path, PathBuf};

    use mcap::{Compression, WriteOptions, records::MessageHeader};
    use tempdir::TempDir;

    use crate::cmd::import::mcap::detect_mcap_config::{Detection, detect_config};

    const VECTOR3: &str = "\
geometry_msgs/Vector3 velocity
float64[] samples
================================================================================
MSG: geometry_msgs/Vector3
float64 x
float64 y
float64 z
";

    struct TopicSpec {
        topic: &'static str,
        schema: Option<(&'static str, &'static str, &'static str)>,
        message_encoding: &'static str,
        messages: usize,
    }

    fn imu() -> TopicSpec {
        TopicSpec {
            topic: "/imu",
            schema: Some(("sensor_msgs/msg/Imu", "ros2msg", VECTOR3)),
            message_encoding: "cdr",
            messages: 5,
        }
    }

    /// Writes an MCAP file into `dir` and returns its path.
    fn write_mcap(
        dir: &Path,
        name: &str,
        opts: WriteOptions,
        topics: &[TopicSpec],
        metadata: &[&str],
    ) -> PathBuf {
        let path = dir.join(name);
        let file = BufWriter::new(File::create(&path).expect("create mcap file"));
        let mut writer = opts.create(file).expect("create mcap writer");

        for spec in topics {
            let schema_id = match spec.schema {
                Some((name, encoding, text)) => writer
                    .add_schema(name, encoding, text.as_bytes())
                    .expect("add schema"),
                None => 0,
            };
            let channel_id = writer
                .add_channel(
                    schema_id,
                    spec.topic,
                    spec.message_encoding,
                    &BTreeMap::new(),
                )
                .expect("add channel");
            for sequence in 0..spec.messages {
                writer
                    .write_to_known_channel(
                        &MessageHeader {
                            channel_id,
                            sequence: sequence as u32,
                            log_time: sequence as u64 * 1_000,
                            publish_time: sequence as u64 * 1_000,
                        },
                        &[0u8; 8],
                    )
                    .expect("write message");
            }
        }

        for name in metadata {
            writer
                .write_metadata(&mcap::records::Metadata {
                    name: (*name).to_string(),
                    metadata: BTreeMap::from([("k".to_string(), "v".to_string())]),
                })
                .expect("write metadata");
        }

        writer.finish().expect("finish mcap");
        path
    }

    fn detect(dir: &Path, name: &str, opts: WriteOptions, topics: &[TopicSpec]) -> Detection {
        let path = write_mcap(dir, name, opts, topics, &[]);
        detect_config(&path).expect("detection should succeed")
    }

    fn channel_names(detection: &Detection) -> Vec<&str> {
        detection.channels.iter().map(|c| c.name.as_str()).collect()
    }

    /// The channels detected from `VECTOR3` on topic `/imu`.
    fn expected_imu_channels() -> Vec<&'static str> {
        vec![
            "/imu.velocity.x",
            "/imu.velocity.y",
            "/imu.velocity.z",
            "/imu.samples",
        ]
    }

    #[test]
    fn detects_channels_from_the_summary_section() {
        let dir = TempDir::new("mcap").expect("tempdir");
        let detection = detect(dir.path(), "summary.mcap", WriteOptions::new(), &[imu()]);
        assert_eq!(channel_names(&detection), expected_imu_channels());
        assert!(
            detection.warnings.is_empty(),
            "unexpected warnings: {:?}",
            detection.warnings
        );
    }

    #[test]
    fn detects_channels_written_only_inside_chunks() {
        // Without summary repeats, the schema and channel records live only
        // inside the compressed chunks, so the data section must be scanned.
        let dir = TempDir::new("mcap").expect("tempdir");
        let opts = WriteOptions::new()
            .repeat_channels(false)
            .repeat_schemas(false)
            .emit_statistics(false);
        let detection = detect(dir.path(), "chunked.mcap", opts, &[imu()]);
        assert_eq!(channel_names(&detection), expected_imu_channels());
    }

    #[test]
    fn detects_channels_in_an_unchunked_file() {
        let dir = TempDir::new("mcap").expect("tempdir");
        let opts = WriteOptions::new().use_chunks(false);
        let detection = detect(dir.path(), "unchunked.mcap", opts, &[imu()]);
        assert_eq!(channel_names(&detection), expected_imu_channels());
    }

    #[test]
    fn detects_channels_in_lz4_and_zstd_chunks() {
        let dir = TempDir::new("mcap").expect("tempdir");
        for (name, compression) in [
            ("lz4.mcap", Some(Compression::Lz4)),
            ("zstd.mcap", Some(Compression::Zstd)),
            ("none.mcap", None),
        ] {
            let opts = WriteOptions::new()
                .compression(compression)
                .repeat_channels(false)
                .repeat_schemas(false)
                .emit_statistics(false);
            let detection = detect(dir.path(), name, opts, &[imu()]);
            assert_eq!(channel_names(&detection), expected_imu_channels(), "{name}");
        }
    }

    #[test]
    fn detects_a_topic_that_logged_no_messages() {
        // Only schema and channel records are read, so an empty topic still
        // lists its channels.
        let dir = TempDir::new("mcap").expect("tempdir");
        let mut spec = imu();
        spec.messages = 0;
        let detection = detect(dir.path(), "empty.mcap", WriteOptions::new(), &[spec]);
        assert_eq!(channel_names(&detection), expected_imu_channels());
    }

    #[test]
    fn skips_topics_it_cannot_decode_and_keeps_the_rest() {
        let dir = TempDir::new("mcap").expect("tempdir");
        let topics = vec![
            imu(),
            TopicSpec {
                topic: "/protobuf",
                schema: Some(("some.Proto", "protobuf", "not ros")),
                message_encoding: "protobuf",
                messages: 2,
            },
            TopicSpec {
                topic: "/noschema",
                schema: None,
                message_encoding: "cdr",
                messages: 2,
            },
        ];
        let detection = detect(dir.path(), "mixed.mcap", WriteOptions::new(), &topics);
        assert_eq!(channel_names(&detection), expected_imu_channels());

        let warning = detection.warnings.join(" ");
        assert!(
            warning.contains("'/protobuf': its message encoding is 'protobuf'"),
            "expected a message-encoding warning, got: {warning}"
        );
        assert!(
            warning.contains("'/noschema': it has no schema"),
            "expected a missing-schema warning, got: {warning}"
        );
    }

    #[test]
    fn warns_and_keeps_what_it_read_from_a_truncated_file() {
        let dir = TempDir::new("mcap").expect("tempdir");
        let path = write_mcap(
            dir.path(),
            "truncated.mcap",
            WriteOptions::new().use_chunks(false),
            &[imu()],
            &[],
        );

        // Cut the file off partway so the summary section is gone.
        let bytes = std::fs::read(&path).expect("read mcap");
        let truncated = dir.path().join("cut.mcap");
        File::create(&truncated)
            .expect("create")
            .write_all(&bytes[..bytes.len() / 2])
            .expect("write");

        let detection = detect_config(&truncated).expect("detection should still succeed");
        assert_eq!(channel_names(&detection), expected_imu_channels());
        assert!(
            detection
                .warnings
                .iter()
                .any(|w| w.contains("may be incomplete")),
            "expected an incomplete-read warning, got: {:?}",
            detection.warnings
        );
    }

    #[test]
    fn errors_on_a_file_that_is_not_mcap() {
        let dir = TempDir::new("mcap").expect("tempdir");
        let path = dir.path().join("bogus.mcap");
        std::fs::write(&path, b"this is not an mcap file").expect("write");

        let err = detect_config(&path).expect_err("expected an error");
        assert!(
            format!("{err:#}").contains("bad magic bytes"),
            "expected a magic bytes error, got: {err:#}"
        );
    }

    #[test]
    fn detects_channels_alongside_metadata_records() {
        let dir = TempDir::new("mcap").expect("tempdir");
        let path = write_mcap(
            dir.path(),
            "metadata.mcap",
            WriteOptions::new(),
            &[imu()],
            &["hardware", "software"],
        );
        let detection = detect_config(&path).expect("detection should succeed");
        assert_eq!(channel_names(&detection), expected_imu_channels());
        assert!(
            detection.warnings.is_empty(),
            "unexpected warnings: {:?}",
            detection.warnings
        );
    }
}
