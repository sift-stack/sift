use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sift::gen::sift::ingestion_configs::v1::FlowConfig;
use std::{convert::AsRef, fs, path::Path};

pub const CONFIG_ENV_VAR: &str = "TELEMETRY_CONFIG";
pub const CONFIG_DIR_NAME: &str = "configs";

#[derive(Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub asset_name: String,
    pub flows: Vec<FlowConfig>,
    pub organization_id: Option<String>,
    pub client_key: Option<String>,
}

impl TelemetryConfig {
    pub fn from_config<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let config_path = path.as_ref();
        let config_file_string = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to load {}", config_path.display()))?;
        let telemetry_config = serde_yaml::from_str::<Self>(&config_file_string)
            .with_context(|| format!("Failed to serialize {}", config_path.display()))?;
        Ok(telemetry_config)
    }
}
