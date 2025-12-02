use sift_rs::metadata;
use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, DiskBackupPolicy, Flow, FlowBuilder,
    FlowConfig, IngestionConfigForm, RecoveryStrategy, RetryPolicy, RunForm, SiftStreamBuilder,
    TimeValue,
};
use std::{
    env,
    error::Error,
    path::PathBuf,
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

    let recovery_strategy = RecoveryStrategy::RetryWithBackups {
        retry_policy: RetryPolicy::default(),
        disk_backup_policy: DiskBackupPolicy {
            backups_dir: Some(PathBuf::from("/tmp/sift_backup")),
            ..Default::default()
        },
    };

    // Initialize your Sift Stream
    let mut sift_stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(ingestion_config)
        .recovery_strategy(recovery_strategy)
        .attach_run(run)
        .build_file_backup()
        .await?;

    // Stream telemetry to backup files using the [`SiftStream::send`] method.
    for i in 0..360 {
        let flow = Flow::new(
            "robotic-arm",
            TimeValue::now(),
            &[ChannelValue::new("joint-angle-encoder", f64::from(i).sin())],
        );

        sift_stream.send(flow).await.unwrap();

        // For demonstrative purposes, adding a contrived wait to get 10Hz data.
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Next, stream telemetry to backup files using the [`SiftStream::send_requests_nonblocking`] method
    // and the [`FlowBuilder`] to build the flow.
    //
    // This approach is more performant, and also provides methods to set the channel value via
    // the channel index instead of the key, which further improves performance by avoiding
    // hashing operations on the channel key.
    //
    // However, this approach does require setting the run ID on the flow builder instead of
    // letting the [`SiftStream`] handle it. Though this can be useful if using a single [`SiftStream`]
    // to send data for multiple runs/assets at one time.
    let descriptor = sift_stream.get_flow_descriptor("robotic-arm").unwrap();
    let run_id = sift_stream.run().unwrap().run_id.clone();
    for i in 0..360 {
        // Build the flow using the [`FlowBuilder`] and send it to
        // Sift using the [`SiftStream::send_requests_nonblocking`] method.
        let mut flow_builder = FlowBuilder::new(&descriptor);
        flow_builder.attach_run_id(&run_id);
        flow_builder
            .set_with_key("joint-angle-encoder", f64::from(i).sin())
            .unwrap();

        sift_stream
            .send_requests_nonblocking(vec![flow_builder.request(TimeValue::now())])
            .unwrap();

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
