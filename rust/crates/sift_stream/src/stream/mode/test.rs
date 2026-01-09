use super::ingestion_config::Flow;
use crate::stream::flow::FlowDescriptor;
use crate::stream::helpers;
use crate::{ChannelValue, TimeValue};
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::ingest_with_config_data_channel_value::Type,
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};

#[test]
fn validate_handling_empty_values() {
    let flow_config = FlowConfig {
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
    };
    let flow_descriptor = FlowDescriptor::try_from(("ingestion_config_id", flow_config))
        .expect("flow descriptor should be generated");

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("quux", 12_u32),
        ],
    );

    let req = helpers::message_to_ingest_req(&flow, None, &flow_descriptor)
        .expect("request should have been generated");

    assert!(
        req.channel_values.len() == 4,
        "should have 4 channel values since flow has 4 channel configs"
    );

    let mut channel_values = req.channel_values.into_iter();
    assert_eq!(
        Some(Type::Empty(pbjson_types::Empty {})),
        channel_values.next().unwrap().r#type
    );
    assert_eq!(Some(Type::Int32(10)), channel_values.next().unwrap().r#type);
    assert_eq!(
        Some(Type::Empty(pbjson_types::Empty {})),
        channel_values.next().unwrap().r#type
    );
    assert_eq!(
        Some(Type::Uint32(12)),
        channel_values.next().unwrap().r#type
    );
}

#[test]
fn validate_handling_no_matches_based_on_name() {
    let flow_config = FlowConfig {
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
    };
    let flow_descriptor = FlowDescriptor::try_from(("ingestion_config_id", flow_config))
        .expect("flow descriptor should be generated");

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("unknown_channel", 12_u32),
        ],
    );

    let req = helpers::message_to_ingest_req(&flow, None, &flow_descriptor);

    assert!(
        req.is_none(),
        "request should be none because no flows match"
    );
}

#[test]
fn validate_handling_no_matches_based_on_type() {
    let flow_config = FlowConfig {
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
    };
    let flow_descriptor = FlowDescriptor::try_from(("ingestion_config_id", flow_config))
        .expect("flow descriptor should be generated");

    let flow = Flow::new(
        "foo",
        TimeValue::default(),
        &[
            ChannelValue::new("baz", 10_i32),
            ChannelValue::new("quux", 12.0_f32), // type is wrong
        ],
    );

    let req = helpers::message_to_ingest_req(&flow, None, &flow_descriptor);

    assert!(
        req.is_none(),
        "request should be none because no flows match"
    );
}
