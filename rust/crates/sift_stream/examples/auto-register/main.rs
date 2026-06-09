// Demonstrates SiftStreamAutoRegister: inline flow registration without pre-declaring schemas.
//
// Three flows show the full registration spectrum:
//
//   1. Pre-registered — declared in IngestionConfigForm.flows before the stream is built.
//      Sift already knows this flow; no registration round-trip occurs at runtime.
//
//   2. Staged — not declared in IngestionConfigForm, but a FlowConfig with units and
//      descriptions is provided to SiftStreamAutoRegister before the first send. On first
//      send, the staged config is validated against the actual channel values and used for
//      registration, preserving the metadata. All subsequent sends are cache-hits.
//
//   3. Dynamic — not declared anywhere ahead of time. On first send, SiftStreamAutoRegister
//      derives a minimal FlowConfig directly from the channel values (names and data types
//      only) and registers it with Sift. No metadata can be attached this way, but it
//      requires zero setup.
//
// If your schema is fully known at build time, pre-registering flows via IngestionConfigForm
// avoids the registration round-trip on first send entirely.

use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigForm, RunForm, SiftStreamAutoRegister, SiftStreamBuilder, TimeValue,
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

    // "gps-position" is pre-registered here. Sift already knows this flow when the stream
    // is built, so SiftStreamAutoRegister will never issue a registration call for it.
    let stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(IngestionConfigForm {
            asset_name: asset_name.clone(),
            client_key: format!("{asset_name}_v1"),
            flows: vec![FlowConfig {
                name: "gps-position".to_string(),
                channels: vec![
                    ChannelConfig {
                        name: "latitude".to_string(),
                        data_type: ChannelDataType::Double.into(),
                        unit: "deg".to_string(),
                        description: "WGS-84 latitude".to_string(),
                        ..Default::default()
                    },
                    ChannelConfig {
                        name: "longitude".to_string(),
                        data_type: ChannelDataType::Double.into(),
                        unit: "deg".to_string(),
                        description: "WGS-84 longitude".to_string(),
                        ..Default::default()
                    },
                ],
            }],
        })
        .attach_run(RunForm {
            name: run_name.clone(),
            client_key: run_name,
            ..Default::default()
        })
        .live_with_backups()
        .build()
        .await?;

    // "vehicle-dynamics" is staged: not declared in IngestionConfigForm, but a FlowConfig
    // with units and descriptions is provided here. On first send, SiftStreamAutoRegister
    // validates the staged config against the actual channel values and uses it for
    // registration. Channel names and data types must match or StagedConfigMismatch is returned.
    let staged_configs = vec![FlowConfig {
        name: "vehicle-dynamics".to_string(),
        channels: vec![
            ChannelConfig {
                name: "velocity".to_string(),
                data_type: ChannelDataType::Double.into(),
                unit: "m/s".to_string(),
                description: "Longitudinal vehicle speed".to_string(),
                ..Default::default()
            },
            ChannelConfig {
                name: "heading".to_string(),
                data_type: ChannelDataType::Float.into(),
                unit: "deg".to_string(),
                description: "Vehicle heading angle (0-360, clockwise from north)".to_string(),
                ..Default::default()
            },
        ],
    }];

    let mut auto = SiftStreamAutoRegister::new(stream, staged_configs);

    for i in 0..NUM_SAMPLES {
        let t = i as f64;

        // Pre-registered flow — Sift already knows "gps-position", so this send goes
        // straight through with no registration overhead on any iteration.
        auto.send(Flow::new(
            "gps-position",
            TimeValue::now(),
            &[
                ChannelValue::new("latitude", 37.7749 + t * 0.0001_f64),
                ChannelValue::new("longitude", -122.4194 + t * 0.0001_f64),
            ],
        ))
        .await?;

        // Staged flow — on the first iteration, the staged FlowConfig is validated and used
        // to register "vehicle-dynamics" with Sift, preserving units and descriptions.
        // All subsequent iterations are cache-hits with no extra overhead.
        auto.send(Flow::new(
            "vehicle-dynamics",
            TimeValue::now(),
            &[
                ChannelValue::new("velocity", t * 0.5_f64),
                ChannelValue::new("heading", (t * 3.6) as f32),
            ],
        ))
        .await?;

        // Dynamic flow — "engine-telemetry" has no pre-declared or staged config. On the
        // first iteration, SiftStreamAutoRegister derives a minimal FlowConfig (channel names
        // and data types only) and registers it. No units or descriptions can be attached
        // this way, but it requires zero setup.
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
        "done — {} samples sent for 3 flows (pre-registered, staged, dynamic)",
        NUM_SAMPLES
    );
    Ok(())
}
