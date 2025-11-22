use super::validate_flows;
use super::{FlowBuilder, FlowDescriptor, FlowDescriptorBuilder};
use crate::TimeValue;
use crate::stream::channel::ChannelEnum;
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::ingest::v1::ingest_with_config_data_channel_value::Type;
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};

#[test]
fn test_validate_flows_compatible_flows() {
    let user_flows = vec![
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
    ];

    // Contains all the flows `user_flows` expects but with two `wheel` flows and different order.
    let sift_flows = vec![
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![ChannelConfig {
                name: String::from("foo_v2"),
                description: String::from("foo"),
                ..Default::default()
            }],
        },
    ];

    assert!(
        validate_flows(&user_flows, &sift_flows).is_ok(),
        "validation should have succeeded"
    );
}

#[test]
fn test_validate_flows_incompatible_flows() {
    let user_flows = vec![
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
    ];

    // 'wheel' flow in Sift is different
    let sift_flows = vec![
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![ChannelConfig {
                name: String::from("foo_v2"),
                description: String::from("foo"),
                ..Default::default()
            }],
        },
    ];

    assert!(
        validate_flows(&user_flows, &sift_flows).is_err(),
        "validation should have failed because 'wheel' flows differ."
    );
}

#[test]
fn test_validate_flows_missing_flows() {
    let user_flows = vec![
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
    ];

    assert!(
        validate_flows(&user_flows, &Vec::new()).is_err(),
        "validation should fail because Sift is missing all specified flows",
    );
}

#[test]
fn test_flow_descriptor_builder_new() {
    let builder: FlowDescriptorBuilder<String> =
        FlowDescriptorBuilder::new("config_id_123", "my_flow");
    let descriptor = builder.build();

    // Verify the descriptor was created correctly by using public methods
    // We can't access private fields, so we verify by checking that get returns None for non-existent keys
    assert_eq!(descriptor.get("nonexistent"), None);
    assert_eq!(descriptor.mapping().len(), 0);
}

#[test]
fn test_flow_descriptor_builder_add_channels() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "sensor_flow");

    let temp_idx = builder.add("temperature", ChannelDataType::Double);
    let pressure_idx = builder.add("pressure", ChannelDataType::Float);
    let humidity_idx = builder.add("humidity", ChannelDataType::Double);

    let descriptor = builder.build();

    // Verify indices are sequential
    assert_eq!(temp_idx.0, 0);
    assert_eq!(pressure_idx.0, 1);
    assert_eq!(humidity_idx.0, 2);

    // Verify field types using public methods
    assert_eq!(descriptor.get("temperature"), Some(ChannelDataType::Double));
    assert_eq!(descriptor.get("pressure"), Some(ChannelDataType::Float));
    assert_eq!(descriptor.get("humidity"), Some(ChannelDataType::Double));

    // Verify index map - HashMap.get() accepts &str when key is String
    assert_eq!(descriptor.mapping().len(), 3);
    assert_eq!(
        descriptor.mapping().get("temperature").map(|idx| idx.0),
        Some(temp_idx.0)
    );
    assert_eq!(
        descriptor.mapping().get("pressure").map(|idx| idx.0),
        Some(pressure_idx.0)
    );
    assert_eq!(
        descriptor.mapping().get("humidity").map(|idx| idx.0),
        Some(humidity_idx.0)
    );
}

#[test]
fn test_flow_descriptor_builder_all_data_types() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "all_types_flow");

    builder.add("bool_field", ChannelDataType::Bool);
    builder.add("string_field", ChannelDataType::String);
    builder.add("float_field", ChannelDataType::Float);
    builder.add("double_field", ChannelDataType::Double);
    builder.add("int32_field", ChannelDataType::Int32);
    builder.add("int64_field", ChannelDataType::Int64);
    builder.add("uint32_field", ChannelDataType::Uint32);
    builder.add("uint64_field", ChannelDataType::Uint64);
    builder.add("enum_field", ChannelDataType::Enum);
    builder.add("bitfield_field", ChannelDataType::BitField);

    let descriptor = builder.build();

    // Verify all 10 channels were added by checking the mapping
    assert_eq!(descriptor.mapping().len(), 10);
    // Verify each channel type
    assert_eq!(descriptor.get("bool_field"), Some(ChannelDataType::Bool));
    assert_eq!(
        descriptor.get("string_field"),
        Some(ChannelDataType::String)
    );
    assert_eq!(descriptor.get("float_field"), Some(ChannelDataType::Float));
    assert_eq!(
        descriptor.get("double_field"),
        Some(ChannelDataType::Double)
    );
    assert_eq!(descriptor.get("int32_field"), Some(ChannelDataType::Int32));
    assert_eq!(descriptor.get("int64_field"), Some(ChannelDataType::Int64));
    assert_eq!(
        descriptor.get("uint32_field"),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        descriptor.get("uint64_field"),
        Some(ChannelDataType::Uint64)
    );
    assert_eq!(descriptor.get("enum_field"), Some(ChannelDataType::Enum));
    assert_eq!(
        descriptor.get("bitfield_field"),
        Some(ChannelDataType::BitField)
    );
}

#[test]
fn test_flow_descriptor_clone() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    builder.add("temperature", ChannelDataType::Double);
    builder.add("pressure", ChannelDataType::Float);

    let descriptor1 = builder.build();
    let descriptor2 = descriptor1.clone();

    // Verify both descriptors have the same content using public methods
    assert_eq!(descriptor1.mapping().len(), descriptor2.mapping().len());

    // Verify they can be used independently
    assert_eq!(
        descriptor1.get("temperature"),
        Some(ChannelDataType::Double)
    );
    assert_eq!(
        descriptor2.get("temperature"),
        Some(ChannelDataType::Double)
    );
}

#[test]
fn test_flow_builder_new() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    builder.add("temperature", ChannelDataType::Double);
    builder.add("pressure", ChannelDataType::Float);

    let descriptor = builder.build();
    let flow_builder = FlowBuilder::new(&descriptor);

    // Verify initial state.
    let now = TimeValue::now();
    let request = flow_builder.request(now.clone());
    assert_eq!(request.ingestion_config_id, "config_id");
    assert_eq!(request.flow, "test_flow");
    assert_eq!(request.run_id, "");
    assert_eq!(request.timestamp, Some(now.0));
    assert!(!request.end_stream_on_validation_error);
    assert_eq!(request.channel_values.len(), 2);

    // When no values are set, the channel values are empty.
    assert_eq!(
        request.channel_values[0].r#type,
        Some(Type::Empty(pbjson_types::Empty {}))
    );
    assert_eq!(
        request.channel_values[1].r#type,
        Some(Type::Empty(pbjson_types::Empty {}))
    );
}

#[test]
fn test_flow_builder_set_with_index() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    let temp_idx = builder.add("temperature", ChannelDataType::Double);
    let pressure_idx = builder.add("pressure", ChannelDataType::Float);
    let humidity_idx = builder.add("humidity", ChannelDataType::Double);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    // Set values using indices
    assert!(flow_builder.set(temp_idx, 23.5_f64).is_ok());
    assert!(flow_builder.set(pressure_idx, 1013.25_f32).is_ok());
    assert!(flow_builder.set(humidity_idx, 65.0_f64).is_ok());

    // Verify values were set correctly by building a request
    let now = TimeValue::now();
    let request = flow_builder.request(now);
    assert_eq!(request.channel_values.len(), 3);
    assert_eq!(
        request.channel_values[0].r#type,
        Some(Type::Double(23.5_f64))
    );
    assert_eq!(
        request.channel_values[1].r#type,
        Some(Type::Float(1013.25_f32))
    );
    assert_eq!(
        request.channel_values[2].r#type,
        Some(Type::Double(65.0_f64))
    );
}

#[test]
fn test_flow_builder_set_with_key() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    builder.add("temperature", ChannelDataType::Double);
    builder.add("pressure", ChannelDataType::Float);
    builder.add("humidity", ChannelDataType::Double);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    // Set values using keys
    assert!(flow_builder.set_with_key("temperature", 23.5_f64).is_ok());
    assert!(flow_builder.set_with_key("pressure", 1013.25_f32).is_ok());
    assert!(flow_builder.set_with_key("humidity", 65.0_f64).is_ok());

    // Verify values were set correctly by building a request
    let now = TimeValue::now();
    let request = flow_builder.request(now);
    assert_eq!(request.channel_values.len(), 3);
    assert_eq!(
        request.channel_values[0].r#type,
        Some(Type::Double(23.5_f64))
    );
    assert_eq!(
        request.channel_values[1].r#type,
        Some(Type::Float(1013.25_f32))
    );
    assert_eq!(
        request.channel_values[2].r#type,
        Some(Type::Double(65.0_f64))
    );
}

#[test]
fn test_flow_builder_set_with_key_invalid_key() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    builder.add("temperature", ChannelDataType::Double);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    // Try to set a value with an invalid key
    let result = flow_builder.set_with_key("nonexistent", 23.5_f64);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("not found in flow descriptor")
    );
}

#[test]
fn test_flow_builder_set_wrong_type() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    let temp_idx = builder.add("temperature", ChannelDataType::Double);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    // Try to set a value with wrong type
    let result = flow_builder.set(temp_idx, 23_i32);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("incorrect data type")
    );
}

#[test]
fn test_flow_builder_all_value_types() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "all_types_flow");
    let bool_idx = builder.add("bool_field", ChannelDataType::Bool);
    let string_idx = builder.add("string_field", ChannelDataType::String);
    let float_idx = builder.add("float_field", ChannelDataType::Float);
    let double_idx = builder.add("double_field", ChannelDataType::Double);
    let int32_idx = builder.add("int32_field", ChannelDataType::Int32);
    let int64_idx = builder.add("int64_field", ChannelDataType::Int64);
    let uint32_idx = builder.add("uint32_field", ChannelDataType::Uint32);
    let uint64_idx = builder.add("uint64_field", ChannelDataType::Uint64);
    let enum_idx = builder.add("enum_field", ChannelDataType::Enum);
    let bitfield_idx = builder.add("bitfield_field", ChannelDataType::BitField);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    flow_builder.set(bool_idx, true).unwrap();
    flow_builder.set(string_idx, "test".to_string()).unwrap();
    flow_builder.set(float_idx, 1.5_f32).unwrap();
    flow_builder.set(double_idx, 2.5_f64).unwrap();
    flow_builder.set(int32_idx, -10_i32).unwrap();
    flow_builder.set(int64_idx, -20_i64).unwrap();
    flow_builder.set(uint32_idx, 30_u32).unwrap();
    flow_builder.set(uint64_idx, 40_u64).unwrap();
    flow_builder.set(enum_idx, ChannelEnum(5)).unwrap();
    flow_builder.set(bitfield_idx, vec![1, 2, 3]).unwrap();

    // Verify all values were set by building a request
    let now = TimeValue::now();
    let request = flow_builder.request(now);
    assert_eq!(request.channel_values.len(), 10);
    for value in &request.channel_values {
        assert!(!matches!(
            value.r#type,
            Some(Type::Empty(pbjson_types::Empty {}))
        ));
    }
}

#[test]
fn test_flow_builder_attach_run_id() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "test_flow");
    builder.add("temperature", ChannelDataType::Double);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    flow_builder.attach_run_id("run_123");
    let now = TimeValue::now();
    let request = flow_builder.request(now);
    assert_eq!(request.run_id, "run_123");
}

#[test]
fn test_flow_config_to_flow_descriptor_owned() {
    let flow_config = FlowConfig {
        name: "sensor_flow".to_string(),
        channels: vec![
            ChannelConfig {
                name: "temperature".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "pressure".to_string(),
                data_type: ChannelDataType::Float.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "humidity".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
        ],
    };

    let descriptor: FlowDescriptor<String> =
        FlowDescriptor::try_from(("config_id_123", flow_config)).unwrap();

    // Verify descriptor by using public methods
    // Note: When FlowDescriptor<String> is created from FlowConfig, get() expects String
    assert_eq!(descriptor.mapping().len(), 3);
    assert_eq!(descriptor.get("temperature"), Some(ChannelDataType::Double));
    assert_eq!(descriptor.get("pressure"), Some(ChannelDataType::Float));
    assert_eq!(descriptor.get("humidity"), Some(ChannelDataType::Double));
}

#[test]
fn test_flow_config_to_flow_descriptor_borrowed() {
    let flow_config = FlowConfig {
        name: "sensor_flow".to_string(),
        channels: vec![
            ChannelConfig {
                name: "temperature".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "pressure".to_string(),
                data_type: ChannelDataType::Float.into(),
                ..Default::default()
            },
        ],
    };

    let descriptor: FlowDescriptor<String> =
        FlowDescriptor::try_from(("config_id_123", &flow_config)).unwrap();

    // Verify descriptor by using public methods
    assert_eq!(descriptor.mapping().len(), 2);

    // Verify flow_config is still usable (was borrowed)
    assert_eq!(flow_config.name, "sensor_flow");
}

#[test]
fn test_flow_config_to_flow_descriptor_all_types() {
    let flow_config = FlowConfig {
        name: "all_types_flow".to_string(),
        channels: vec![
            ChannelConfig {
                name: "bool_field".to_string(),
                data_type: ChannelDataType::Bool.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "string_field".to_string(),
                data_type: ChannelDataType::String.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "float_field".to_string(),
                data_type: ChannelDataType::Float.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "double_field".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "int32_field".to_string(),
                data_type: ChannelDataType::Int32.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "int64_field".to_string(),
                data_type: ChannelDataType::Int64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "uint32_field".to_string(),
                data_type: ChannelDataType::Uint32.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "uint64_field".to_string(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "enum_field".to_string(),
                data_type: ChannelDataType::Enum.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "bitfield_field".to_string(),
                data_type: ChannelDataType::BitField.into(),
                ..Default::default()
            },
        ],
    };

    let descriptor: FlowDescriptor<String> =
        FlowDescriptor::try_from(("config_id", flow_config)).unwrap();

    // Verify all 10 channels were added
    assert_eq!(descriptor.mapping().len(), 10);
    assert_eq!(descriptor.get("bool_field"), Some(ChannelDataType::Bool));
    assert_eq!(
        descriptor.get("string_field"),
        Some(ChannelDataType::String)
    );
    assert_eq!(descriptor.get("float_field"), Some(ChannelDataType::Float));
    assert_eq!(
        descriptor.get("double_field"),
        Some(ChannelDataType::Double)
    );
    assert_eq!(descriptor.get("int32_field"), Some(ChannelDataType::Int32));
    assert_eq!(descriptor.get("int64_field"), Some(ChannelDataType::Int64));
    assert_eq!(
        descriptor.get("uint32_field"),
        Some(ChannelDataType::Uint32)
    );
    assert_eq!(
        descriptor.get("uint64_field"),
        Some(ChannelDataType::Uint64)
    );
    assert_eq!(descriptor.get("enum_field"), Some(ChannelDataType::Enum));
    assert_eq!(
        descriptor.get("bitfield_field"),
        Some(ChannelDataType::BitField)
    );
}

#[test]
fn test_flow_config_to_flow_descriptor_empty_channels() {
    let flow_config = FlowConfig {
        name: "empty_flow".to_string(),
        channels: vec![],
    };

    let descriptor: FlowDescriptor<String> =
        FlowDescriptor::try_from(("config_id", flow_config)).unwrap();

    // Verify empty descriptor
    assert_eq!(descriptor.mapping().len(), 0);
    assert_eq!(descriptor.get("nonexistent"), None);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SensorField {
    Temperature,
    Pressure,
    Humidity,
    Time,
    State,
}

#[test]
fn test_flow_descriptor_with_enum_keys() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "sensor_flow");

    let temp_idx = builder.add(SensorField::Temperature, ChannelDataType::Double);
    let pressure_idx = builder.add(SensorField::Pressure, ChannelDataType::Float);
    let humidity_idx = builder.add(SensorField::Humidity, ChannelDataType::Double);
    let time_idx = builder.add(SensorField::Time, ChannelDataType::Int64);
    let state_idx = builder.add(SensorField::State, ChannelDataType::Enum);

    let descriptor = builder.build();

    // Verify indices
    assert_eq!(temp_idx.0, 0);
    assert_eq!(pressure_idx.0, 1);
    assert_eq!(humidity_idx.0, 2);
    assert_eq!(time_idx.0, 3);
    assert_eq!(state_idx.0, 4);

    // Verify field types
    assert_eq!(
        descriptor.get(&SensorField::Temperature),
        Some(ChannelDataType::Double)
    );
    assert_eq!(
        descriptor.get(&SensorField::Pressure),
        Some(ChannelDataType::Float)
    );
    assert_eq!(
        descriptor.get(&SensorField::Humidity),
        Some(ChannelDataType::Double)
    );
    assert_eq!(
        descriptor.get(&SensorField::Time),
        Some(ChannelDataType::Int64)
    );
    assert_eq!(
        descriptor.get(&SensorField::State),
        Some(ChannelDataType::Enum)
    );

    // Verify mapping
    let mapping = descriptor.mapping();
    assert_eq!(mapping.len(), 5);
    assert_eq!(
        mapping.get(&SensorField::Temperature).map(|idx| idx.0),
        Some(temp_idx.0)
    );
    assert_eq!(
        mapping.get(&SensorField::Pressure).map(|idx| idx.0),
        Some(pressure_idx.0)
    );
    assert_eq!(
        mapping.get(&SensorField::Humidity).map(|idx| idx.0),
        Some(humidity_idx.0)
    );
    assert_eq!(
        mapping.get(&SensorField::Time).map(|idx| idx.0),
        Some(time_idx.0)
    );
    assert_eq!(
        mapping.get(&SensorField::State).map(|idx| idx.0),
        Some(state_idx.0)
    );
}

#[test]
fn test_flow_builder_with_enum_keys() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "sensor_flow");

    let temp_idx = builder.add(SensorField::Temperature, ChannelDataType::Double);
    let pressure_idx = builder.add(SensorField::Pressure, ChannelDataType::Float);
    let humidity_idx = builder.add(SensorField::Humidity, ChannelDataType::Double);
    let time_idx = builder.add(SensorField::Time, ChannelDataType::Int64);
    let state_idx = builder.add(SensorField::State, ChannelDataType::Enum);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    // Set values using indices
    flow_builder.set(temp_idx, 23.5_f64).unwrap();
    flow_builder.set(pressure_idx, 1013.25_f32).unwrap();
    flow_builder.set(humidity_idx, 65.0_f64).unwrap();
    flow_builder.set(time_idx, 1234567890_i64).unwrap();
    flow_builder.set(state_idx, ChannelEnum(1)).unwrap();

    // Build request and verify values were set
    let now = TimeValue::now();
    let request = flow_builder.request(now);

    // Verify values were set
    assert_eq!(request.channel_values.len(), 5);
    for value in &request.channel_values {
        assert!(value.r#type.is_some());
    }

    assert_eq!(request.ingestion_config_id, "config_id");
    assert_eq!(request.flow, "sensor_flow");
    assert_eq!(request.channel_values.len(), 5);
}

#[test]
fn test_flow_builder_set_with_enum_key() {
    let mut builder = FlowDescriptorBuilder::new("config_id", "sensor_flow");

    builder.add(SensorField::Temperature, ChannelDataType::Double);
    builder.add(SensorField::Pressure, ChannelDataType::Float);
    builder.add(SensorField::Humidity, ChannelDataType::Double);

    let descriptor = builder.build();
    let mut flow_builder = FlowBuilder::new(&descriptor);

    // Set values using enum keys
    flow_builder
        .set_with_key(&SensorField::Temperature, 23.5_f64)
        .unwrap();
    flow_builder
        .set_with_key(&SensorField::Pressure, 1013.25_f32)
        .unwrap();
    flow_builder
        .set_with_key(&SensorField::Humidity, 65.0_f64)
        .unwrap();

    // Verify values were set by building a request
    let now = TimeValue::now();
    let request = flow_builder.request(now);
    assert_eq!(request.channel_values.len(), 3);
    for value in &request.channel_values {
        assert!(value.r#type.is_some());
    }
}
