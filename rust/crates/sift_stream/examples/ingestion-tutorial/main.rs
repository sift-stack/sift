// Streams simulated vehicle velocity and temperature telemetry, generated using random values to mimic onboard vehicle sensors, to Sift for up to 10 minutes.
//
// This example demonstrates the complete streaming ingestion lifecycle:
// - Authenticate with Sift
// - Define a telemetry schema (Flow + Channels)
// - Create an Asset and Run
// - Open a streaming ingestion session
// - Send timestamped flows in real time
//
// The program runs for INGEST_DURATION

use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigForm, RecoveryStrategy, RunForm, SiftStreamBuilder, TimeValue,
};
use std::{
    env,
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// Define configuration constants
// ---------------------------------------------------------------------
/// FLOW_NAME identifies the telemetry schema of a flow inside Sift.
const FLOW_NAME: &str = "vehicle_metrics";
/// SEND_INTERVAL controls sampling frequency.
const SEND_INTERVAL: Duration = Duration::from_millis(500);
/// INGEST_DURATION controls how long we send data before exiting
const INGEST_DURATION: Duration = Duration::from_secs(10 * 60);

/// Helper function to generate unique names
/// ---------------------------------------------------------------------
/// Sift Assets and Runs should have unique names.
/// This helper creates a timestamp + short random suffix to prevent collisions.
fn make_unique_suffix() -> String {
    format!(
        "{}_{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before UNIX EPOCH")
            .as_secs(),
        rand::random::<u32>()
    )
}

/// Main function
/// ---------------------------------------------------------------------
/// All ingestion logic lives inside this async function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Sift stream uses the tracing crate for logging, which we can enable
    // to see internal sift stream logs
    tracing_subscriber::fmt().init();

    tracing::info!("Starting streaming session.");

    // Create unique Asset and Run names
    // -----------------------------------------------------------------
    // An Asset represents the telemetry-producing system.
    // A Run represents a single data collection session for that Asset.
    let suffix = make_unique_suffix();
    let asset_name = format!("robot_vehicle_{suffix}");
    let run_name = format!("{asset_name}_run");

    // Load authentication from .env
    // -----------------------------------------------------------------
    // We load credentials from a .env file instead of hardcoding them.
    // These values are required to establish authenticated communication
    // with the gRPC endpoint of your Sift environment.
    dotenvy::dotenv()?;
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").unwrap(),
        uri: env::var("SIFT_GRPC_URL").unwrap(),
    };

    // Define telemetry signals (Channels) within a Flow
    // -----------------------------------------------------------------
    // A FlowConfig defines the telemetry schema.
    // Each ChannelConfig defines:
    //   - name (signal identifier)
    //   - unit (measurement unit)
    //   - data_type (numeric, string, etc.)
    //   - description (a human-readable explanation of what the Channel (signal) represents and how it should be interpreted)
    //
    // All telemetry sent to Sift must conform to this schema.
    let flow_config = FlowConfig {
        name: FLOW_NAME.into(),
        channels: vec![
            ChannelConfig {
                name: "velocity".into(),
                unit: "m/s".into(),
                data_type: ChannelDataType::Double.into(),
                description: "The velocity Channel streams real-time speed measurements of the vehicle in meters per second (m/s) as double-precision numeric values.".into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "temperature".into(),
                unit: "C".into(),
                data_type: ChannelDataType::Double.into(),
                description: "The temperature Channel streams real-time temperature readings of the vehicle in degrees Celsius (°C) as double-precision numeric values.".into(),
                ..Default::default()
            },
        ]
    };

    // Create ingestion configuration
    // -----------------------------------------------------------------
    // IngestionConfigForm requires:
    //   - An Asset
    //   - A client key identifier expected to be unique across the user's organization
    //   - One or more Flow configs
    let ingestion_client_key = format!("{asset_name}_v1");
    let ingestion_config = IngestionConfigForm {
        asset_name: asset_name.clone(),
        client_key: ingestion_client_key,
        flows: vec![flow_config],
    };

    // Create Run
    // -----------------------------------------------------------------
    // RunForm defines the session that will group all incoming flows.
    // While not strictly necessary for ingestion, Runs are useful for organizing
    // data from one or more Assets for a given period of time (such as a specific test,
    // or daily ops)
    // Requires a unique client_key, which we'll set to the same as the run name in this case
    let run = RunForm {
        name: run_name.clone(),
        client_key: run_name,
        ..Default::default()
    };

    // Initialize Sift Stream
    // -----------------------------------------------------------------
    // SiftStream is built using SiftStreamBuilder, which much be supplied with the user credentials
    // We will also provide the following:
    //   - The ingestion config defining the telemetry schema we plan to send
    //   - A default recovery strategy (Retry only, with no file backups of ingested data)
    //   - A run to attach incoming data to
    let mut sift_stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(ingestion_config)
        .recovery_strategy(RecoveryStrategy::default())
        .attach_run(run)
        .build()
        .await?;

    // Stream telemetry to Sift using the SiftStream::send method for INGEST_DURATION
    // -----------------------------------------------------------------
    // NOTE: This approach uses `Flow` and `SiftStream::send()` for ease of use, and will
    // provide acceptable performance for most users
    // In cases where additional performance is required, a separate, more performant method
    // is also available that uses `FlowBuilder` and `SiftStream::try_send_requests`
    // See `examples/quick-start/` for an example using this alternate approach
    let start = std::time::Instant::now();
    while start.elapsed() < INGEST_DURATION {
        // Generate mock telemetry values
        // ---------------------------------------------------------
        // In a real system, these would come from sensors,
        // hardware interfaces, or production metrics.
        let velocity = rand::random::<f64>() * 10.0;
        let temperature = rand::random::<f64>() * 20.0 + 20.0;

        // Create a Flow object that matches the FlowConfig schema
        // ---------------------------------------------------------
        // Requires the flow name, a timestamp, and channel values
        let flow = Flow::new(
            FLOW_NAME,
            TimeValue::now(),
            &[
                ChannelValue::new("velocity", velocity),
                ChannelValue::new("temperature", temperature),
            ],
        );

        // Send telemetry to Sift using Sift Stream
        // ---------------------------------------------------------
        // Each call sends the flow to a queue within Sift Stream for transmission to Sift.
        // Sift Stream uses a checkpoint system to verify this data reaches Sift, retrying if necessary.
        sift_stream.send(flow).await?;

        // For demonstrative purposes, add a wait to send data at approximately the SEND_INTERVAL
        tokio::time::sleep(SEND_INTERVAL).await;
    }

    // Calling finish() on sift stream ensures we properly send any queued data before gracefully closing the gRPC stream
    sift_stream.finish().await?;

    tracing::info!("Streaming session complete.");

    Ok(())
}
