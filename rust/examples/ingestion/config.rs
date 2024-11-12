use sift_rs::gen::sift::{
    common::r#type::v1::ChannelDataType,
    ingestion_configs::v1::{ChannelConfig, FlowConfig},
};

/// Name of the asset that we want to ingest dats for.
pub const ASSET_NAME: &str = "LV426";

/// Unique client-chosen identifier used to identify an ingestion config.
pub const CLIENT_KEY: &str = "lv426-v1";

/// Channel and flow configuration used to create an ingestion config.
pub fn channel_configs() -> Vec<FlowConfig> {
    return vec![
        FlowConfig {
            name: String::from("reading"),
            channels: vec![
                ChannelConfig {
                    name: "velocity".to_string(),
                    component: "mainmotor".to_string(),
                    unit: "km/hr".to_string(),
                    description: "vehicle speed".to_string(),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: "voltage".to_string(),
                    unit: "kV".to_string(),
                    description: "potential difference".to_string(),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("log"),
            channels: vec![ChannelConfig {
                name: "log".to_string(),
                data_type: ChannelDataType::String.into(),
                ..Default::default()
            }],
        },
    ];
}
