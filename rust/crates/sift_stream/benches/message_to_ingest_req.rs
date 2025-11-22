//! Benchmark for the `message_to_ingest_req` function.
//!
//! This benchmark tests the performance of the `message_to_ingest_req` function under different
//! scenarios:
//! 1. **Ordered channel values**: The channel values in the message are in the same order as
//!    the corresponding FlowConfig channels (optimal case).
//! 2. **Randomized channel values**: The channel values in the message are in random order
//!    (worst case for the matching algorithm).
//! 3. **Varying sizes**: Tests both scenarios with different numbers of channels (5, 10, 20, 50, 100).
//!
//! The benchmark creates a configurable number of FlowConfigs and tests the function's ability
//! to match a message against the appropriate flow configuration.
//!
//! **Note**: This benchmark requires the `unstable` feature to be enabled:
//! ```bash
//! cargo bench --bench message_to_ingest_req --features unstable
//! ```

// Ensure this benchmark only compiles when the unstable feature is enabled
#[cfg(not(feature = "unstable"))]
compile_error!(
    "This benchmark requires the 'unstable' feature to be enabled. Run with: cargo bench --bench message_to_ingest_req --features unstable"
);

use criterion::{Criterion, criterion_group, criterion_main};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::hint::black_box;

use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use sift_stream::stream::mode::ingestion_config::Flow;
use sift_stream::{
    ChannelDataType, ChannelValue, TimeValue, Value,
    stream::mode::bench::{message_to_ingest_req, message_to_ingest_req_direct},
};

/// Creates a FlowConfig with a given name and number of channels.
fn flow_config(name: &str, num_channels: usize) -> FlowConfig {
    let mut channels = Vec::with_capacity(num_channels);

    for i in 0..num_channels {
        let data_type = (i as i32 % 11) + 1; // Avoid 0 (Unspecified)
        let channel = ChannelConfig {
            name: format!("value_{i}"),
            unit: "unit".to_string(),
            description: format!("Channel {i} description"),
            data_type,
            enum_types: vec![],
            bit_field_elements: vec![],
        };
        channels.push(channel);
    }

    FlowConfig {
        name: name.to_string(),
        channels,
    }
}

/// Creates a flow with channel values in the same order as the FlowConfig.
fn flow_ordered(name: &str, flow_config: &FlowConfig) -> Flow {
    let ts = TimeValue::from_timestamp_nanos(0);
    let mut values = Vec::with_capacity(flow_config.channels.len());

    for (i, channel_config) in flow_config.channels.iter().enumerate() {
        let data_type = ChannelDataType::try_from(channel_config.data_type).unwrap();
        let value = match data_type {
            ChannelDataType::Double => Value::Double(i as f64),
            ChannelDataType::String => Value::String(format!("{i}")),
            ChannelDataType::Float => Value::Float(i as f32),
            ChannelDataType::Bool => Value::Bool(i % 2 == 0),
            ChannelDataType::Int32 => Value::Int32(i as i32),
            ChannelDataType::Int64 => Value::Int64(i as i64),
            ChannelDataType::Uint32 => Value::Uint32(i as u32),
            ChannelDataType::Uint64 => Value::Uint64(i as u64),
            ChannelDataType::Enum => Value::Enum(i as u32),
            ChannelDataType::BitField => Value::BitField(vec![i as u8]),
            ChannelDataType::Unspecified => Value::String(format!("{i}")),
            ChannelDataType::Bytes => Value::BitField(vec![i as u8]),
        };

        values.push(ChannelValue::new(&channel_config.name, value));
    }

    Flow::new(name, ts, &values)
}

/// Creates a flow with channel values in randomized order.
fn flow_randomized(name: &str, flow_config: &FlowConfig) -> Flow {
    let ts = TimeValue::from_timestamp_nanos(0);
    let mut values = Vec::with_capacity(flow_config.channels.len());

    for (i, channel_config) in flow_config.channels.iter().enumerate() {
        let data_type = ChannelDataType::try_from(channel_config.data_type).unwrap();
        let value = match data_type {
            ChannelDataType::Double => Value::Double(i as f64),
            ChannelDataType::String => Value::String(format!("{i}")),
            ChannelDataType::Float => Value::Float(i as f32),
            ChannelDataType::Bool => Value::Bool(i % 2 == 0),
            ChannelDataType::Int32 => Value::Int32(i as i32),
            ChannelDataType::Int64 => Value::Int64(i as i64),
            ChannelDataType::Uint32 => Value::Uint32(i as u32),
            ChannelDataType::Uint64 => Value::Uint64(i as u64),
            ChannelDataType::Enum => Value::Enum(i as u32),
            ChannelDataType::BitField => Value::BitField(vec![i as u8]),
            ChannelDataType::Unspecified => Value::String(format!("{i}")),
            ChannelDataType::Bytes => Value::BitField(vec![i as u8]),
        };

        values.push(ChannelValue::new(&channel_config.name, value));
    }

    // Randomize the order of values
    let mut rng = thread_rng();
    values.shuffle(&mut rng);

    Flow::new(name, ts, &values)
}

// Configuration constants - these can be adjusted to test different scenarios
const NUM_FLOWS: usize = 10; // Number of flow configs to create
const NUM_CHANNELS_PER_FLOW: usize = 2000; // Number of channels per flow
const INGESTION_CONFIG_ID: &str = "benchmark-config";
const RUN_ID: Option<String> = None;
const FLOW_TO_RANDOMIZE: usize = 8;

fn benchmark_message_to_ingest_req_direct(c: &mut Criterion) {
    // Create a flow with ordered channel values (matching the first flow config)
    let message = flow_ordered("flow_0", &flow_config("flow_0", NUM_CHANNELS_PER_FLOW));

    c.bench_function("message_to_ingest_req_direct", |b| {
        b.iter(|| {
            black_box(message_to_ingest_req_direct(
                &message,
                INGESTION_CONFIG_ID,
                RUN_ID.clone(),
            ))
        })
    });
}

fn benchmark_message_to_ingest_req_ordered(c: &mut Criterion) {
    // Create flow configs
    let mut flow_configs = Vec::with_capacity(NUM_FLOWS);
    for i in 0..NUM_FLOWS {
        flow_configs.push(flow_config(&format!("flow_{i}"), NUM_CHANNELS_PER_FLOW));
    }

    // Create a flow with ordered channel values (matching the first flow config)
    let message = flow_ordered("flow_0", &flow_configs[FLOW_TO_RANDOMIZE]);

    c.bench_function("message_to_ingest_req_ordered", |b| {
        b.iter(|| {
            black_box(message_to_ingest_req(
                &message,
                INGESTION_CONFIG_ID,
                RUN_ID.clone(),
                &flow_configs,
            ))
        })
    });
}

fn benchmark_message_to_ingest_req_randomized(c: &mut Criterion) {
    // Create flow configs
    let mut flow_configs = Vec::with_capacity(NUM_FLOWS);
    for i in 0..NUM_FLOWS {
        flow_configs.push(flow_config(&format!("flow_{i}"), NUM_CHANNELS_PER_FLOW));
    }

    // Create a flow with randomized channel values (matching the first flow config)
    let message = flow_randomized("flow_0", &flow_configs[FLOW_TO_RANDOMIZE]);

    c.bench_function("message_to_ingest_req_randomized", |b| {
        b.iter(|| {
            black_box(message_to_ingest_req(
                &message,
                INGESTION_CONFIG_ID,
                RUN_ID.clone(),
                &flow_configs,
            ))
        })
    });
}

fn benchmark_message_to_ingest_req_varying_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_to_ingest_req_varying_sizes");

    for &num_channels in &[5, 10, 100, 1000, 5000] {
        // Create flow configs with varying channel counts
        let mut flow_configs = Vec::with_capacity(NUM_FLOWS);
        for i in 0..NUM_FLOWS {
            flow_configs.push(flow_config(&format!("flow_{i}"), num_channels));
        }

        // Test direct scenario
        let message_ordered = flow_ordered("flow_0", &flow_configs[FLOW_TO_RANDOMIZE]);
        group.bench_function(&format!("direct_{num_channels}_channels"), |b| {
            b.iter(|| {
                black_box(message_to_ingest_req_direct(
                    &message_ordered,
                    INGESTION_CONFIG_ID,
                    RUN_ID.clone(),
                ))
            })
        });

        group.bench_function(&format!("ordered_{num_channels}_channels"), |b| {
            b.iter(|| {
                black_box(message_to_ingest_req(
                    &message_ordered,
                    INGESTION_CONFIG_ID,
                    RUN_ID.clone(),
                    &flow_configs,
                ))
            })
        });

        // Test randomized scenario
        let message_randomized = flow_randomized("flow_0", &flow_configs[FLOW_TO_RANDOMIZE]);
        group.bench_function(&format!("randomized_{num_channels}_channels"), |b| {
            b.iter(|| {
                black_box(message_to_ingest_req(
                    &message_randomized,
                    INGESTION_CONFIG_ID,
                    RUN_ID.clone(),
                    &flow_configs,
                ))
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_message_to_ingest_req_direct,
    benchmark_message_to_ingest_req_ordered,
    benchmark_message_to_ingest_req_randomized,
    benchmark_message_to_ingest_req_varying_sizes
);
criterion_main!(benches);
