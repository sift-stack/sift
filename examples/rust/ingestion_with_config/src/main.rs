use anyhow::Result;
use chrono::Utc;
use dotenv::dotenv;
use std::{env, process::ExitCode, time::Duration};

/// Concerned with loading the telemetry config yaml.
mod config;
use config::{CONFIG_DIR_NAME, CONFIG_ENV_VAR, TelemetryConfig};

/// Concerned with ingestion.
mod ingestion;
use ingestion::{ChannelValue, Flow, SiftIngestionService};

/// gRPC utilities to create transport channels and such.
mod grpc;

#[tokio::main]
async fn main() -> ExitCode {
    match simulate().await {
        Ok(_) => {
            println!("Done.");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn simulate() -> Result<()> {
    let _ = dotenv()?;
    let current_dir = env::current_dir()?;
    let config_name = env::var(CONFIG_ENV_VAR).map(|c| current_dir.join(CONFIG_DIR_NAME).join(c))?;
    let telemetry_config = TelemetryConfig::from_config(config_name)?;
    let asset_name = telemetry_config.asset_name.clone();

    let channel = grpc::use_channel()?;
    let mut ingestion_service =
        SiftIngestionService::from_config(channel.clone(), telemetry_config).await?;
    ingestion_service.end_stream_on_error();

    let mut current_time = Utc::now();

    // Start a run
    let run_name = format!(
        "{}.{}",
        asset_name.to_ascii_lowercase(),
        current_time.timestamp()
    );
    let _ = ingestion_service
        .start_run(
            channel.clone(),
            &run_name,
            Some(current_time),
            None,
            None,
            None,
            None,
        )
        .await?;

    let mut flows = Vec::new();

    for i in 0..100 {
        let flow = Flow {
            // Will be validated downstream... value is from configs/lunar_rover0.yml.
            name: "readings".to_string(),
            timestamp: current_time,

            // Order based on configs/lunar_rover0.yml and thus the flow
            channel_values: vec![
                // Velocity channel
                ChannelValue::Double(f64::from(60 * i).sin()),
                // voltage channel
                ChannelValue::Int32(i),
            ],
        };
        current_time += Duration::from_millis(5);
        flows.push(flow);
    }

    println!("Ingestion data for asset \"{asset_name}\" for run \"{run_name}\".");

    // Ingest data
    ingestion_service.send_values(flows).await?;

    Ok(())
}
