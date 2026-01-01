use super::Context;
use anyhow::{Ok, anyhow};
use crossterm::style::Stylize;
use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigEncoder, IngestionConfigForm, RecoveryStrategy, RetryPolicy, SiftStream,
    SiftStreamBuilder, TimeValue,
};

/// Streams metrics to Sift.
pub struct MetricsStreamingClient {
    ctx: Context,
    asset_name: String,
    sift_stream: Option<SiftStream<IngestionConfigEncoder>>,
}

impl MetricsStreamingClient {
    pub fn build(
        ctx: Context,
        stream_metrics: &Option<bool>,
        asset_name: &Option<String>,
    ) -> Result<Option<MetricsStreamingClient>, anyhow::Error> {
        if !stream_metrics.unwrap_or(false) {
            return Ok(None);
        }

        let Some(asset_name) = asset_name else {
            return Err(anyhow!(
                "must specify {} with streaming enabled",
                "--metrics_asset_name".cyan()
            ));
        };

        Ok(Some(MetricsStreamingClient {
            ctx,
            asset_name: asset_name.clone(),
            sift_stream: None,
        }))
    }

    /// Initialize SiftStream and create ingestion config.
    pub async fn initialize(&mut self) -> Result<(), anyhow::Error> {
        let credentials = Credentials::Config {
            apikey: self.ctx.api_key.clone(),
            uri: self.ctx.grpc_uri.clone(),
        };

        let ingestion_config = IngestionConfigForm {
            asset_name: self.asset_name.to_string(),
            client_key: "stress-test-ingestion-config-test".into(),
            flows: vec![FlowConfig {
                name: "metrics".into(),
                channels: vec![
                    ChannelConfig {
                        name: "total_num_streams".into(),
                        description: "Total number of streams created".into(),
                        data_type: ChannelDataType::Uint32.into(),
                        ..Default::default()
                    },
                    ChannelConfig {
                        name: "total_num_bytes_read".into(),
                        description: "Total number of bytes read".into(),
                        unit: "B".into(),
                        data_type: ChannelDataType::Uint64.into(),
                        ..Default::default()
                    },
                    ChannelConfig {
                        name: "total_num_messages".into(),
                        description: "Total number of messages received".into(),
                        unit: "message".into(),
                        data_type: ChannelDataType::Uint64.into(),
                        ..Default::default()
                    },
                    ChannelConfig {
                        name: "bytes_per_s".into(),
                        description: "Number of bytes received per second".into(),
                        data_type: ChannelDataType::Double.into(),
                        unit: "B/s".into(),
                        ..Default::default()
                    },
                    ChannelConfig {
                        name: "messages_per_s".into(),
                        description: "Number of messages received per second".into(),
                        unit: "message/s".into(),
                        data_type: ChannelDataType::Double.into(),
                        ..Default::default()
                    },
                ],
            }],
        };

        let sift_stream = SiftStreamBuilder::new(credentials)
            .ingestion_config(ingestion_config)
            .recovery_strategy(RecoveryStrategy::RetryOnly(RetryPolicy::default()))
            .build()
            .await?;

        self.sift_stream = Some(sift_stream);

        Ok(())
    }

    /// Send metrics to Sift.
    pub async fn ingest(&mut self, metrics: Metrics) {
        let flow = Flow::new(
            "metrics",
            TimeValue::now(),
            &[
                ChannelValue::new("total_num_streams", metrics.total_num_streams),
                ChannelValue::new("total_num_bytes_read", metrics.total_num_bytes_read),
                ChannelValue::new("total_num_messages", metrics.total_num_messages),
                ChannelValue::new("bytes_per_s", metrics.bytes_per_s),
                ChannelValue::new("messages_per_s", metrics.messages_per_s),
            ],
        );

        self.sift_stream.as_mut().unwrap().send(flow).await.unwrap();
    }
}

pub struct Metrics {
    pub total_num_streams: u32,
    pub total_num_bytes_read: u64,
    pub total_num_messages: u64,
    pub bytes_per_s: f64,
    pub messages_per_s: f64,
}
