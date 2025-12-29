//! Example demonstrating the HTTP metrics server for SiftStream.
//!
//! This example shows how to:
//! - Start the metrics HTTP server using `MetricsServerBuilder`
//! - Query the `/` and `/metrics` endpoints
//! - Parse and display the JSON metrics response
//! - Query the server from external tools (demonstrated with curl command)
//!
//! Run with: `cargo run --example http_server --features metrics-unstable`
//!
//! Once running, you can query metrics using:
//! ```bash
//! curl http://127.0.0.1:8080/metrics
//! ```
//!
//! To pretty-print the metrics, you can also use the `jq` command:
//! ```bash
//! curl http://127.0.0.1:8080/metrics | jq .
//! ```
#[cfg(not(feature = "metrics-unstable"))]
compile_error!(
    "This example requires the 'metrics-unstable' feature to be enabled. Run with: cargo run --example http-server-metrics --features metrics-unstable"
);

use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigForm, MetricsServerBuilder, RecoveryStrategy, RunForm, SiftStreamBuilder,
    TimeValue,
};
use std::{env, error::Error, net::SocketAddr, process::ExitCode, time::Duration};
use tokio::signal;
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
        asset_name: "MetricsServerExample".into(),
        client_key: "metrics-server-example-v1".into(),
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
        name: "Metrics Server Example Run".into(),
        client_key: "metrics-server-example-run".into(),
        description: Some("Example demonstrating HTTP metrics server".into()),
        tags: Some(vec!["metrics".into(), "http-server".into()]),
        metadata: None,
    };

    // Start the metrics HTTP server
    // Defaults to 127.0.0.1:8080, but can be customized
    let socket_addr: SocketAddr = "127.0.0.1:8080".parse().expect("Invalid socket address");

    println!("Starting metrics HTTP server on {}...", socket_addr);
    MetricsServerBuilder::new()
        .socket(socket_addr)
        .start_metrics_server()
        .await?;

    println!("Metrics server started! Metrics are available at:");
    println!("  - http://{}/", socket_addr);
    println!("  - http://{}/metrics", socket_addr);
    println!("\nYou can query metrics using:");
    println!("  curl http://{}/metrics", socket_addr);
    println!("\nStarting to send data continuously...");
    println!("Press Ctrl+C to stop.\n");

    // Initialize your Sift Stream
    // The metrics from this stream will be automatically registered with the HTTP server
    let mut sift_stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(ingestion_config)
        .recovery_strategy(RecoveryStrategy::default())
        .attach_run(run)
        .build()
        .await?;

    println!("\n=== Example: Querying Metrics Server ===");
    println!("You can query the metrics server from your application or external tools.");
    println!("The server returns JSON with metrics organized by sift_stream_id.");
    println!("\nExample curl command:");
    println!("  curl http://{}/metrics", socket_addr);
    println!("\nExample Python code to query metrics:");
    println!(
        r#"  import requests
  import json
  
  response = requests.get("http://127.0.0.1:8080/metrics")
  metrics = json.loads(response.text)
  print(json.dumps(metrics, indent=2))"#
    );
    println!("\nSending data continuously. Press Ctrl+C to stop...\n");

    // Continuously send data until Ctrl-C is pressed
    // We use tokio::select! to race between sending data and waiting for Ctrl-C
    let mut counter = 0u64;

    let mut signal = Box::pin(signal::ctrl_c());

    loop {
        tokio::select! {
            _ = &mut signal => {
                println!("\nCtrl+C received. Shutting down gracefully...");
                break;
            }
            result = async {
                let flow = Flow::new(
                    "sensor-data",
                    TimeValue::now(),
                    &[ChannelValue::new("temperature", 20.0 + (counter as f64) * 0.1)],
                );
                sift_stream.send(flow).await
            } => {
                match result {
                    Ok(_) => {
                        counter += 1;

                        // Print status every 10 messages
                        if counter % 10 == 0 {
                            println!("Sent {} messages.", counter);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error sending flow: {}", e);
                        break;
                    }
                }

                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    }

    // Gracefully terminate your stream
    sift_stream
        .finish()
        .await
        .expect("failed to gracefully terminate Sift stream");

    println!("Stream finished gracefully. Exiting...");

    Ok(())
}
