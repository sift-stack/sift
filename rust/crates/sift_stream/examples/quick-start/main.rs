use sift_rs::metadata;
use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigForm, RecoveryStrategy, RunForm, SiftStreamBuilder, TimeValue,
};
use std::{
    env,
    error::Error,
    process::ExitCode,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
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
        apikey: env::var("SIFT_API_KEY").unwrap(),
        uri: env::var("SIFT_URI").unwrap(),
    };

    // Define the schema of your telemetry
    let ingestion_config = IngestionConfigForm {
        asset_name: "MarsRover0".into(),
        client_key: "mars-rover0-ingestion-config-v1".into(),
        flows: vec![FlowConfig {
            name: "robotic-arm".into(),
            channels: vec![ChannelConfig {
                name: "joint-angle-encoder".into(),
                description: "measures the angular position of the arm’s joints".into(),
                data_type: ChannelDataType::Double.into(),
                unit: "degrees".into(),
                ..Default::default()
            }],
        }],
    };

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap();

    // Create metadata using the metadata macro
    let metadata = metadata![
        ("test_number", 5.0),
        ("is_simulation", true),
        ("location", "SiftHQ"),
    ];

    // Define an optional run to group together data for this period of telemetry ingestion.
    let run = RunForm {
        name: format!("[MarsRover0].{ts}"),
        client_key: format!("mars-rover-sim-{ts}"),
        description: Some("simulation run".into()),
        tags: Some(vec!["simulation".into()]),
        metadata: Some(metadata),
    };

    // Initialize your Sift Stream
    let mut sift_stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(ingestion_config)
        .recovery_strategy(RecoveryStrategy::default())
        .attach_run(run)
        .build()
        .await?;

    // Stream telemetry to Sift
    for i in 0..360 {
        let flow = Flow::new(
            "robotic-arm",
            TimeValue::now(),
            &[ChannelValue::new("joint-angle-encoder", f64::from(i).sin())],
        );

        // Send telemetry to Sift
        sift_stream.send(flow).await.unwrap();

        // For demonstrative purposes, adding a contrived wait to get 10Hz data.
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Gracefully terminate your stream
    sift_stream
        .finish()
        .await
        .expect("failed to gracefully terminate Sift stream");

    Ok(())
}
