use chrono::Utc;
use pbjson_types::Timestamp;
use rand::Rng;
use sift_rs::{
    gen::sift::{
        ingest::v1::{
            ingest_service_client::IngestServiceClient,
            ingest_with_config_data_channel_value::Type, IngestWithConfigDataChannelValue,
            IngestWithConfigDataStreamRequest,
        },
        ingestion_configs::v1::{
            ingestion_config_service_client::IngestionConfigServiceClient,
            CreateIngestionConfigRequest, IngestionConfig, ListIngestionConfigsRequest,
        },
        runs::v2::{run_service_client::RunServiceClient, CreateRunRequest, Run},
    },
    grpc::{use_sift_channel, SiftChannel, SiftChannelConfig},
};
use std::{
    env,
    error::Error,
    time::{Duration, Instant},
};

/// Contains our asset and channel configurations
pub mod config;
use config::{channel_configs, ASSET_NAME, CLIENT_KEY};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = env::var("SIFT_URI")?;
    let apikey = env::var("SIFT_API_KEY")?;
    let grpc_channel = use_sift_channel(SiftChannelConfig { uri, apikey })?;

    let ingestion_config =
        get_or_create_ingestion_config(grpc_channel.clone(), ASSET_NAME, CLIENT_KEY).await?;
    println!(
        "initialized ingestion config {}",
        ingestion_config.client_key
    );

    let run = create_run(grpc_channel.clone(), ASSET_NAME).await?;
    println!("initialized run {}", &run.name);

    let mut ingestion_service = IngestServiceClient::new(grpc_channel);

    // RNG to simulate real data
    let mut rng = rand::thread_rng();

    let start = Instant::now();
    let duration = Duration::from_secs(60);

    // Frequency to send data for each flow
    let readings_frequency_hz = 1.5;
    let logs_frequency_hz = 2.0;
    let readings_interval = Duration::from_secs_f64(1.0 / readings_frequency_hz);
    let logs_interval = Duration::from_secs_f64(1.0 / logs_frequency_hz);

    let mut last_reading = Instant::now();
    let mut last_log = Instant::now();

    // Buffer our requests rather than sending them 1 by 1 for better performance
    let mut buffer = Vec::new();
    let send_threshold = 5;

    while Instant::now().duration_since(start) < duration {
        let current = Instant::now();

        if current.duration_since(last_reading) >= readings_interval {
            buffer.push(IngestWithConfigDataStreamRequest {
                ingestion_config_id: String::from(&ingestion_config.ingestion_config_id),
                run_id: String::from(&run.run_id),
                flow: String::from("reading"),
                timestamp: Some(Timestamp::from(Utc::now())),
                channel_values: vec![
                    // velocity channel
                    IngestWithConfigDataChannelValue {
                        r#type: Some(Type::Double(rng.gen_range(1.0..10.0))),
                    },
                    // voltage channel
                    IngestWithConfigDataChannelValue {
                        r#type: Some(Type::Double(rng.gen_range(1.0..10.0))),
                    },
                ],
                // Use this flag only for debugging purposes to get real-time data validation from
                // the Sift API. Do not use in production as it will hurt performance.
                end_stream_on_validation_error: true,
                ..Default::default()
            });
            last_reading = current;
        }

        if current.duration_since(last_log) >= logs_interval {
            buffer.push(IngestWithConfigDataStreamRequest {
                ingestion_config_id: String::from(&ingestion_config.ingestion_config_id),
                run_id: String::from(&run.run_id),
                flow: String::from("log"),
                timestamp: Some(Timestamp::from(Utc::now())),
                channel_values: vec![
                    // log channel
                    IngestWithConfigDataChannelValue {
                        r#type: Some(Type::String("test log emission".to_string())),
                    },
                ],
                // Use this flag only for debugging purposes to get real-time data validation from
                // the Sift API. Do not use in production as it will hurt performance.
                end_stream_on_validation_error: true,
                ..Default::default()
            });
            last_log = current;
        }

        // Send data once buffer is full and re-init buffer
        if buffer.len() > send_threshold {
            println!("ingestion {} flows", buffer.len());
            let stream = tokio_stream::iter(buffer);
            ingestion_service
                .ingest_with_config_data_stream(stream)
                .await?;
            buffer = Vec::new();
        }
    }

    Ok(())
}

/// Retrieves an existing ingestion config or creates it
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
