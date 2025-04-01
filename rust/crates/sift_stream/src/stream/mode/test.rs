use super::super::SiftStream;
use super::ingestion_config::{Flow, IngestionConfigMode};
use crate::{ChannelValue, TimeValue};
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::ingest_with_config_data_channel_value::Type,
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};

#[test]
fn validate_handling_empty_values() {
    let flow_configs = vec![FlowConfig {
        name: String::from("foo"),
        channels: vec![
            ChannelConfig {
                name: String::from("bar"),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("baz"),
                data_type: ChannelDataType::Int32.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("qux"),
                data_type: ChannelDataType::Int64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("quux"),
                data_type: ChannelDataType::Uint32.into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }];

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("quux", 12_u32),
        ],
    );

    let req = SiftStream::<IngestionConfigMode>::message_to_ingest_req(
        &flow,
        "ingestion-config-id",
        None,
        &flow_configs,
    )
    .expect("request should have been generated");

    assert!(
        req.channel_values.len() == 4,
        "should have 4 channel values since flow has 4 channel configs"
    );

    let mut channel_values = req.channel_values.into_iter();
    assert_eq!(
        Some(ChannelValue::empty_pb()),
        channel_values.next().unwrap().r#type
    );
    assert_eq!(Some(Type::Int32(10)), channel_values.next().unwrap().r#type);
    assert_eq!(
        Some(ChannelValue::empty_pb()),
        channel_values.next().unwrap().r#type
    );
    assert_eq!(
        Some(Type::Uint32(12)),
        channel_values.next().unwrap().r#type
    );
}

#[test]
fn validate_handling_no_matches_based_on_name() {
    let flow_configs = vec![FlowConfig {
        name: String::from("foo"),
        channels: vec![
            ChannelConfig {
                name: String::from("bar"),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("baz"),
                data_type: ChannelDataType::Int32.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("qux"),
                data_type: ChannelDataType::Int64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("quux"),
                data_type: ChannelDataType::Uint32.into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }];

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("unknown_channel", 12_u32),
        ],
    );

    let req = SiftStream::<IngestionConfigMode>::message_to_ingest_req(
        &flow,
        "ingestion-config-id",
        None,
        &flow_configs,
    );

    assert!(
        req.is_none(),
        "request should be none because no flows match"
    );
}

#[test]
fn validate_handling_no_matches_based_on_type() {
    let flow_configs = vec![FlowConfig {
        name: String::from("foo"),
        channels: vec![
            ChannelConfig {
                name: String::from("bar"),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("baz"),
                data_type: ChannelDataType::Int32.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("qux"),
                data_type: ChannelDataType::Int64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: String::from("quux"),
                data_type: ChannelDataType::Uint32.into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }];

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("quux", 12.0_f32), // type is wrong
        ],
    );

    let req = SiftStream::<IngestionConfigMode>::message_to_ingest_req(
        &flow,
        "ingestion-config-id",
        None,
        &flow_configs,
    );

    assert!(
        req.is_none(),
        "request should be none because no flows match"
    );
}

#[test]
fn validate_handling_message_against_multiple_flows_with_same_name_with_atleast_one_match() {
    let flow_configs = vec![
        FlowConfig {
            name: String::from("foo"),
            channels: vec![
                ChannelConfig {
                    name: String::from("bar"),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("baz"),
                    data_type: ChannelDataType::Int32.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("qux"),
                    data_type: ChannelDataType::Int64.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("quux"),
                    data_type: ChannelDataType::Uint32.into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        FlowConfig {
            name: String::from("foo"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    data_type: ChannelDataType::Int32.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("qux"),
                    data_type: ChannelDataType::Int64.into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ];

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("quux", 12_u32),
            ChannelValue::new("qux", 15_i64),
        ],
    );

    let req = SiftStream::<IngestionConfigMode>::message_to_ingest_req(
        &flow,
        "ingestion-config-id",
        None,
        &flow_configs,
    )
    .expect("expected request to be generated because there is a matching flow");

    assert!(
        req.channel_values.len() == 4,
        "should have 4 channel values since one of the 'foo' flows has 4 channel configs"
    );

    let mut channel_values = req.channel_values.into_iter();
    assert_eq!(
        Some(ChannelValue::empty_pb()),
        channel_values.next().unwrap().r#type,
        "bar should be empty"
    );
    assert_eq!(
        Some(Type::Int32(10)),
        channel_values.next().unwrap().r#type,
        "baz should be 10_i32"
    );
    assert_eq!(
        Some(Type::Int64(15)),
        channel_values.next().unwrap().r#type,
        "qux should be 15_i64"
    );
    assert_eq!(
        Some(Type::Uint32(12)),
        channel_values.next().unwrap().r#type,
        "quuz should be 12_u32"
    );
}

#[test]
fn validate_handling_message_against_multiple_flows_with_same_name_with_no_match() {
    let flow_configs = vec![
        FlowConfig {
            name: String::from("foo"),
            channels: vec![
                ChannelConfig {
                    name: String::from("bar"),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("baz"),
                    data_type: ChannelDataType::Int32.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("qux"),
                    data_type: ChannelDataType::Int64.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("quux"),
                    data_type: ChannelDataType::Uint32.into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        FlowConfig {
            name: String::from("foo"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    data_type: ChannelDataType::Int32.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("qux"),
                    data_type: ChannelDataType::Int64.into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ];

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("quux", 12_u32),
            ChannelValue::new("qux", 15_i64),
            ChannelValue::new("foobar", 15_i64),
            ChannelValue::new("foobaz", 15_i64),
        ],
    );

    let req = SiftStream::<IngestionConfigMode>::message_to_ingest_req(
        &flow,
        "ingestion-config-id",
        None,
        &flow_configs,
    );

    assert!(
        req.is_none(),
        "should be None because there are no flows that contain all specified channels"
    );
}
