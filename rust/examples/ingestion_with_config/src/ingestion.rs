use crate::{config::TelemetryConfig, grpc::SiftChannel};
use anyhow::{format_err, Result};
use chrono::{DateTime, Utc};
use pbjson_types::Timestamp;
use sift::gen::sift::{
    ingest::v1::{
        ingest_service_client::IngestServiceClient,
        ingest_with_config_data_channel_value::Type as GenChannelValue,
        IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
    },
    ingestion_configs::v1::{
        ingestion_config_service_client::IngestionConfigServiceClient,
        CreateIngestionConfigRequest, CreateIngestionConfigResponse, FlowConfig, IngestionConfig,
    },
    runs::v2::{
        run_service_client::RunServiceClient, CreateRunRequest, CreateRunResponse, Run,
    },
};
use std::collections::HashMap;

// Re-export for convenience.
pub type ChannelValue = GenChannelValue;

pub struct SiftIngestionService {
    client: IngestServiceClient<SiftChannel>,
    organization_id: Option<String>,
    ingestion_config: IngestionConfig,

    // Flow config by name
    flows: HashMap<String, FlowConfig>,

    // The current active run for this stream.
    run: Option<Run>,

    // Immediately terminate the stream if an error occurs during ingestion.
    end_stream_on_error: bool,
}

/// A unit of ingestion
pub struct Flow {
    pub name: String,
    pub channel_values: Vec<ChannelValue>,
    pub timestamp: DateTime<Utc>,
}

impl SiftIngestionService {
    pub async fn from_config(channel: SiftChannel, config: TelemetryConfig) -> Result<Self> {
        let ingestion_config = Self::create_ingestion_config(channel.clone(), &config).await?;
        let mut flows = HashMap::<String, FlowConfig>::new();

        for flow_config in config.flows {
            flows.insert(flow_config.name.clone(), flow_config);
        }

        let client = IngestServiceClient::new(channel.clone());

        Ok(Self {
            client,
            ingestion_config,
            flows,
            organization_id: config.organization_id,
            run: None,
            end_stream_on_error: false,
        })
    }

    /// Configure service to end stream if there is an error that occurs in the Sift API during
    /// ingestion.
    pub fn end_stream_on_error(&mut self) {
        self.end_stream_on_error = true;
    }

    /// Ingest multiple values at once.
    pub async fn send_values(&mut self, flows: Vec<Flow>) -> Result<()> {
        let mut requests = Vec::new();

        // Turn our flows into request objects
        for flow in flows {
            let channel_values = flow
                .channel_values
                .into_iter()
                .map(|v| IngestWithConfigDataChannelValue { r#type: Some(v) })
                .collect::<Vec<_>>();

            if !self.flows.contains_key(&flow.name) {
                return Err(format_err!("Couldn't find flow with name {}", flow.name));
            }

            let request = IngestWithConfigDataStreamRequest {
                channel_values,
                timestamp: Some(Timestamp::from(flow.timestamp)),
                flow: flow.name,
                ingestion_config_id: self.ingestion_config.ingestion_config_id.clone(),
                organization_id: self.organization_id.clone().unwrap_or_default(),
                end_stream_on_validation_error: self.end_stream_on_error,
                run_id: self
                    .run
                    .as_ref()
                    .map_or_else(String::new, |r| r.run_id.clone()),
            };

            requests.push(request);
        }

        // Stream requests and ignest data
        let stream = tokio_stream::iter(requests);
        let _ = self.client.ingest_with_config_data_stream(stream).await;
        Ok(())
    }

    #[allow(dead_code)]
    /// Ingest single value.
    pub async fn send_value(&mut self, flow: Flow) -> Result<()> {
        if !self.flows.contains_key(&flow.name) {
            return Err(format_err!("Couldn't find flow with name {}", flow.name));
        }

        let channel_values = flow
            .channel_values
            .into_iter()
            .map(|v| IngestWithConfigDataChannelValue { r#type: Some(v) })
            .collect::<Vec<_>>();

        let request = IngestWithConfigDataStreamRequest {
            channel_values,
            timestamp: Some(Timestamp::from(flow.timestamp)),
            flow: flow.name,
            run_id: self
                .run
                .as_ref()
                .map_or_else(String::new, |r| r.run_id.clone()),
            ingestion_config_id: self.ingestion_config.ingestion_config_id.clone(),
            organization_id: self.organization_id.clone().unwrap_or_default(),
            end_stream_on_validation_error: self.end_stream_on_error,
        };
        let value = tokio_stream::once(request);
        let _ = self.client.ingest_with_config_data_stream(value).await;
        Ok(())
    }

    /// Optionally start a run. If no start time is provided then it will default to now.
    pub async fn start_run(
        &mut self,
        channel: SiftChannel,
        name: &str,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        description: Option<&str>,
        organization_id: Option<&str>,
        tags: Option<Vec<String>>,
    ) -> Result<Run> {
        let mut client = RunServiceClient::new(channel);
        let request = CreateRunRequest {
            name: name.to_string(),
            start_time: start.or_else(|| Some(Utc::now())).map(Timestamp::from),
            stop_time: end.or_else(|| Some(Utc::now())).map(Timestamp::from),
            description: description.map(String::from).unwrap_or_default(),
            organization_id: organization_id.map(String::from).unwrap_or_default(),
            tags: tags.unwrap_or_default(),
        };
        let response = client.create_run(request).await?;
        let CreateRunResponse { run } = response.into_inner();
        self.run.clone_from(&run);
        run.ok_or_else(|| format_err!("Expected a run to be in the response"))
    }

    /// Creates the ingestion config with the flows that will be used for ingestion.
    async fn create_ingestion_config(
        channel: SiftChannel,
        config: &TelemetryConfig,
    ) -> Result<IngestionConfig> {
        let mut client = IngestionConfigServiceClient::new(channel);
        let request = CreateIngestionConfigRequest {
            asset_name: config.asset_name.clone(),
            flows: config.flows.clone(),
            organization_id: config.organization_id.clone().unwrap_or_default(),
            client_key: config.client_key.clone().unwrap_or_default(),
        };
        let response = client.create_ingestion_config(request).await?;
        let CreateIngestionConfigResponse { ingestion_config } = response.into_inner();

        ingestion_config
            .ok_or_else(|| format_err!("Expected an ingestion config to be in the response"))
    }
}
