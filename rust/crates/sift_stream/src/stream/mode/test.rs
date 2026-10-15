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

// A rate-limited flow is reported to the caller with its retry delay, while flows that were
// created in the same call are still registered.
#[tokio::test]
async fn add_new_flows_returns_rate_limit_error_and_registers_successful_flows() {
    use super::ingestion_config::IngestionConfigEncoder;
    use crate::metrics::SiftStreamMetrics;
    use crate::test::{
        MockIngestionConfigService, create_mock_grpc_channel_with_ingestion_service,
        rate_limited_status,
    };
    use sift_error::ErrorKind;
    use sift_rs::{ingestion_configs::v2::IngestionConfig, retry::RateLimitPolicy};
    use std::{collections::HashMap, sync::Arc, time::Duration};

    let delay = Duration::from_secs(2);
    let service = MockIngestionConfigService::with_flow_create_statuses(vec![(
        "rate_limited_flow",
        rate_limited_status(delay),
        usize::MAX,
    )]);
    let (grpc_channel, _) = create_mock_grpc_channel_with_ingestion_service(service).await;

    let mut encoder = IngestionConfigEncoder {
        grpc_channel,
        flows_by_name: HashMap::new(),
        ingestion_config: IngestionConfig {
            ingestion_config_id: "ingestion_config_id".to_string(),
            ..Default::default()
        },
        metrics: Arc::new(SiftStreamMetrics::default()),
        rate_limit_policy: RateLimitPolicy::ReturnError,
    };

    let new_flow = |name: &str| FlowConfig {
        name: name.to_string(),
        channels: vec![ChannelConfig {
            name: "channel".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    let err = encoder
        .add_new_flows(&[new_flow("rate_limited_flow"), new_flow("created_flow")])
        .await
        .unwrap_err();

    assert_eq!(
        err.kind(),
        ErrorKind::RateLimitedError {
            retry_delay: Some(delay)
        }
    );
    assert!(encoder.flows_by_name.contains_key("created_flow"));
    assert!(!encoder.flows_by_name.contains_key("rate_limited_flow"));
}
