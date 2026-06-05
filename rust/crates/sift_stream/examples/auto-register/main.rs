// Demonstrates SiftStreamAutoRegister: inline flow registration without pre-declaring schemas.
//
// The stream is initialized with an empty flow list. On the first send for any flow name,
// SiftStreamAutoRegister derives a FlowConfig from the Flow's channel values and registers
// it with Sift automatically. Subsequent sends for the same flow name are cache-hits and
// have no extra overhead.
//
// This pattern is well-suited for:
//   - Rapid prototyping where the full schema is not known ahead of time.
//   - Dynamic telemetry where flow names are determined at runtime.
//
// If your schema is fully known at build time, pre-registering flows via IngestionConfigForm
// avoids the registration round-trip on first send.

use sift_stream::{
    ChannelValue, Credentials, Flow, IngestionConfigForm, RunForm, SiftStreamAutoRegister,
    SiftStreamBuilder, TimeValue,
};
use std::{
    env,
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const SEND_INTERVAL: Duration = Duration::from_millis(200);
const NUM_SAMPLES: u32 = 25;

fn timestamp_suffix() -> String {
    format!(
        "{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    dotenvy::dotenv()?;
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY")?,
        uri: env::var("SIFT_GRPC_URL")?,
    };

    let suffix = timestamp_suffix();
    let asset_name = format!("demo_asset_{suffix}");
    let run_name = format!("{asset_name}_run");

    // Start with no pre-declared flows — SiftStreamAutoRegister will register them on demand.
    let stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(IngestionConfigForm {
            asset_name: asset_name.clone(),
            client_key: format!("{asset_name}_v1"),
            flows: vec![],
        })
        .attach_run(RunForm {
            name: run_name.clone(),
            client_key: run_name,
            ..Default::default()
        })
        .live_with_backups()
        .build()
        .await?;

    let mut auto = SiftStreamAutoRegister::new(stream);

    for i in 0..NUM_SAMPLES {
        let t = i as f64;

        // "vehicle-dynamics" is unknown to Sift on the first iteration.
        // SiftStreamAutoRegister derives { name: "vehicle-dynamics", channels: [velocity(f64),
        // heading(f32)] } and registers it before sending. All subsequent iterations skip
        // registration and deliver directly.
        auto.send(Flow::new(
            "vehicle-dynamics",
            TimeValue::now(),
            &[
                ChannelValue::new("velocity", t * 0.5_f64),
                ChannelValue::new("heading", (t * 3.6) as f32),
            ],
        ))
        .await?;

        // "engine-telemetry" is a second, independent flow — also auto-registered on its
        // first send and cached for all subsequent sends.
        auto.send(Flow::new(
            "engine-telemetry",
            TimeValue::now(),
            &[
                ChannelValue::new("rpm", (3000.0 + t * 10.0) as f32),
                ChannelValue::new("oil-temp", 85.0 + t * 0.1_f64),
            ],
        ))
        .await?;

        tokio::time::sleep(SEND_INTERVAL).await;
    }

    auto.finish().await?;
    tracing::info!(
        "done — {} samples sent for 2 auto-registered flows",
        NUM_SAMPLES
    );
    Ok(())
}
