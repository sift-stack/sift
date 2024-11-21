use chrono::Utc;
use pbjson_types::Timestamp;
use sift_rs::{
    gen::sift::{
        common::r#type::v1::ChannelDataType,
        ingest::v1::{
            ingest_service_client::IngestServiceClient,
            ingest_with_config_data_channel_value::Type, IngestWithConfigDataChannelValue,
            IngestWithConfigDataStreamRequest,
        },
        ingestion_configs::v1::{
            ingestion_config_service_client::IngestionConfigServiceClient, ChannelConfig,
            CreateIngestionConfigRequest, FlowConfig, IngestionConfig, ListIngestionConfigsRequest,
        },
        runs::v2::{run_service_client::RunServiceClient, CreateRunRequest, Run},
    },
    grpc::{use_sift_channel, SiftChannel, SiftChannelConfig},
};
use std::{env, error::Error};

/// Simulates a data source
mod data;
use data::data_source;

/// Name of the asset that we want to ingest data for.
pub const ASSET_NAME: &str = "LV-426";

/// Unique client-chosen identifier used to identify an ingestion config.
pub const CLIENT_KEY: &str = "lv-426-v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to Sift
    let grpc_channel = use_sift_channel(SiftChannelConfig {
        uri: env::var("SIFT_URI")?,
        apikey: env::var("SIFT_API_KEY")?,
    })?;

    // Create your ingestion config which defines the schema of your telemetry.
    let ingestion_config =
        get_or_create_ingestion_config(grpc_channel.clone(), ASSET_NAME, CLIENT_KEY).await?;
    println!(
        "initialized ingestion config {}",
        ingestion_config.client_key
    );

    // Create a run to group all the data ingested during this period.
    let run = create_run(grpc_channel.clone(), ASSET_NAME).await?;
    println!("initialized run {}", &run.name);

    let mut ingestion_service = IngestServiceClient::new(grpc_channel);
    let data_stream = data_source();

    // Stream data to Sift from a data source.
    while let Ok((timestamp, velocity)) = data_stream.recv() {
        let req = IngestWithConfigDataStreamRequest {
            run_id: run.run_id.clone(),
            ingestion_config_id: String::from(&ingestion_config.ingestion_config_id),
            flow: String::from("velocity_reading"),
            timestamp: Some(Timestamp::from(timestamp)),
            channel_values: vec![IngestWithConfigDataChannelValue {
                r#type: Some(Type::Double(velocity)),
            }],
            // Set this flag to `true` only for debugging purposes to get real-time data validation from
            // the Sift API. Do not use in production as it will hurt performance.
            end_stream_on_validation_error: false,
            ..Default::default()
        };
        ingestion_service
            .ingest_with_config_data_stream(tokio_stream::once(req))
            .await?;
        println!("ingested a velocity_reading flow");
    }

    println!("done.");
    Ok(())
}

/// Channel and flow configuration used to create an ingestion config.
pub fn channel_configs() -> Vec<FlowConfig> {
    vec![FlowConfig {
        name: String::from("velocity_reading"),
        channels: vec![ChannelConfig {
            name: String::from("velocity"),
            component: String::from("mainmotor"),
            unit: String::from("km/hr"),
            description: String::from("vehicle speed"),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }]
}

/// Retrieves an existing ingestion config or create it.
async fn get_or_create_ingestion_config(
    grpc_channel: SiftChannel,
    asset_name: &str,
    client_key: &str,
) -> Result<IngestionConfig, Box<dyn Error>> {
    let mut svc = IngestionConfigServiceClient::new(grpc_channel);

    let list_res = svc
        .list_ingestion_configs(ListIngestionConfigsRequest {
            filter: format!("client_key == '{client_key}'"),
            ..Default::default()
        })
        .await?;

    if let Some(ingestion_config) = list_res.into_inner().ingestion_configs.first().cloned() {
        return Ok(ingestion_config);
    }

    let req = CreateIngestionConfigRequest {
        asset_name: asset_name.to_string(),
        client_key: client_key.to_string(),
        flows: channel_configs(),
        ..Default::default()
    };

    let create_res = svc.create_ingestion_config(req).await?;
    let ingestion_conf = create_res
        .into_inner()
        .ingestion_config
        .ok_or("expected ingestion config")?;
    Ok(ingestion_conf)
}

/// Create a run to use to group all the data ingested during this period.
async fn create_run(grpc_channel: SiftChannel, run_name: &str) -> Result<Run, Box<dyn Error>> {
    let mut svc = RunServiceClient::new(grpc_channel);
    let ts = Utc::now();

    let create_req = CreateRunRequest {
        name: format!("[{}].{}", run_name.to_string(), ts.timestamp()),
        start_time: Some(Timestamp::from(ts)),
        ..Default::default()
    };
    let create_res = svc.create_run(create_req).await?;
    let run = create_res.into_inner().run.expect("expected run");
    Ok(run)
}
