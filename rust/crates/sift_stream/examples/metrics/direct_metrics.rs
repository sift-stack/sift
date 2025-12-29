//! Example demonstrating direct metrics access from SiftStream.
//!
//! This example shows how to:
//! - Access metrics using `get_metrics_snapshot()` on a SiftStream instance
//! - Display all available metric fields
//! - Perform periodic metrics polling
//! - Access checkpoint and backup metrics
//!
//! Run with: `cargo run --example direct-metrics --features metrics-unstable`
#[cfg(not(feature = "metrics-unstable"))]
compile_error!(
    "This example requires the 'metrics-unstable' feature to be enabled. Run with: cargo run --example direct-metrics --features metrics-unstable"
);

use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigForm, RecoveryStrategy, RunForm, SiftStreamBuilder, TimeValue,
};
use std::{env, error::Error, process::ExitCode, time::Duration};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").expect("SIFT_API_KEY environment variable must be set"),
        uri: env::var("SIFT_URI").expect("SIFT_URI environment variable must be set"),
    };

    // Define the schema of your telemetry
    let ingestion_config = IngestionConfigForm {
        asset_name: "MetricsExample".into(),
        client_key: "metrics-example-v1".into(),
        flows: vec![FlowConfig {
            name: "sensor-data".into(),
            channels: vec![ChannelConfig {
                name: "temperature".into(),
                description: "Temperature sensor reading".into(),
                data_type: ChannelDataType::Double.into(),
                unit: "Celsius".into(),
                ..Default::default()
            }],
        }],
    };

    // Define a run to group together data
    let run = RunForm {
        name: "Metrics Example Run".into(),
        client_key: "metrics-example-run".into(),
        description: Some("Example demonstrating metrics access".into()),
        tags: Some(vec!["metrics".into(), "example".into()]),
        metadata: None,
    };

    // Initialize your Sift Stream
    let mut sift_stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(ingestion_config)
        .recovery_strategy(RecoveryStrategy::default())
        .attach_run(run)
        .build()
        .await?;

    println!("Starting to send data and collect metrics...\n");

    // Send some data to generate metrics
    for i in 0..100 {
        let flow = Flow::new(
            "sensor-data",
            TimeValue::now(),
            &[ChannelValue::new("temperature", 20.0 + (i as f64) * 0.1)],
        );

        sift_stream.send(flow).await?;

        // Print metrics every 10 messages
        if i % 10 == 0 && i > 0 {
            let metrics = sift_stream.get_metrics_snapshot();
            print_metrics_snapshot(&metrics, i);
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Print final metrics
    println!("\n=== Final Metrics ===");
    let final_metrics = sift_stream.get_metrics_snapshot();
    print_metrics_snapshot(&final_metrics, 100);

    // Demonstrate accessing specific metric categories
    let metrics = sift_stream.get_metrics_snapshot();
    println!("\n=== Checkpoint Metrics ===");
    println!("Total checkpoints: {}", metrics.checkpoint.checkpoint_count);
    println!(
        "Failed checkpoints: {}",
        metrics.checkpoint.failed_checkpoint_count
    );
    println!(
        "Current checkpoint elapsed: {:.2}s",
        metrics.checkpoint.cur_elapsed_secs
    );
    println!(
        "Current checkpoint message rate: {:.2} msg/s",
        metrics.checkpoint.cur_message_rate
    );
    println!(
        "Current checkpoint byte rate: {:.2} bytes/s",
        metrics.checkpoint.cur_byte_rate
    );

    println!("\n=== Backup Metrics ===");
    println!("Total backup files: {}", metrics.backups.total_file_count);
    println!("Total backup bytes: {}", metrics.backups.total_bytes);
    println!("Total backup messages: {}", metrics.backups.total_messages);
    println!(
        "Files pending ingestion: {}",
        metrics.backups.files_pending_ingestion
    );
    println!("Files ingested: {}", metrics.backups.files_ingested);

    // Gracefully terminate your stream
    sift_stream
        .finish()
        .await
        .expect("failed to gracefully terminate Sift stream");

    Ok(())
}

fn print_metrics_snapshot(metrics: &sift_stream::SiftStreamMetricsSnapshot, message_count: u64) {
    println!(
        "=== Metrics Snapshot (after {} messages) ===",
        message_count
    );
    println!("Elapsed time: {:.2}s", metrics.elapsed_secs);
    println!("Loaded flows: {}", metrics.loaded_flows);
    println!("Unique flows received: {}", metrics.unique_flows_received);
    println!("Messages received: {}", metrics.messages_received);
    println!("Messages sent: {}", metrics.messages_sent);
    println!("Message rate: {:.2} messages/s", metrics.message_rate);
    println!("Bytes sent: {}", metrics.bytes_sent);
    println!("Byte rate: {:.2} bytes/s", metrics.byte_rate);
    println!("Current retry count: {}", metrics.cur_retry_count);
    println!(
        "Ingestion channel depth: {}",
        metrics.ingestion_channel_depth
    );
    println!("Backup channel depth: {}", metrics.backup_channel_depth);
    println!();
}
