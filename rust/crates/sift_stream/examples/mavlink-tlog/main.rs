// Replays a drone telemetry log (.tlog) into Sift at the times it was recorded.
//
// QGroundControl, Mission Planner and MAVProxy save a vehicle's MAVLink link as a .tlog: every
// frame behind the microsecond time the ground station received it. This example reads the frames
// with `mavlink-tlog`, decodes them with `mavlink`, and sends position, attitude and battery flows
// with each frame's original timestamp. A `link` flow adds the packet loss and duplicate counts
// of the vehicle's link, worked out from the MAVLink sequence numbers.
//
//   SIFT_API_KEY=... SIFT_URI=... cargo run --example mavlink-tlog -- flight.tlog
//
// MAVLINK_SYSTEM_ID picks the vehicle (default 1, the ArduPilot and PX4 default).
// SIFT_ASSET_NAME names the Sift asset (default "mavlink-vehicle").

use mavlink::dialects::common::MavMessage;
use mavlink_tlog::{LinkStats, TlogReader};
use sift_stream::{
    ChannelConfig, ChannelDataType, ChannelValue, Credentials, Flow, FlowConfig,
    IngestionConfigEncoder, IngestionConfigForm, LiveStreamingWithBackups, RunForm, SiftStream,
    SiftStreamBuilder, TimeValue,
};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing_subscriber::filter::EnvFilter;

/// How often, in log time, to send the `link` flow.
const LINK_INTERVAL_US: u64 = 1_000_000;

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

fn channel(name: &str, data_type: ChannelDataType, unit: &str, description: &str) -> ChannelConfig {
    ChannelConfig {
        name: name.into(),
        data_type: data_type.into(),
        unit: unit.into(),
        description: description.into(),
        ..Default::default()
    }
}

fn flows() -> Vec<FlowConfig> {
    use ChannelDataType::{Double, Uint64};
    vec![
        FlowConfig {
            name: "GLOBAL_POSITION_INT".into(),
            channels: vec![
                channel("lat", Double, "deg", "latitude, WGS84"),
                channel("lon", Double, "deg", "longitude, WGS84"),
                channel("alt", Double, "m", "altitude above mean sea level"),
                channel("relative_alt", Double, "m", "altitude above home"),
                channel("vx", Double, "m/s", "ground speed north"),
                channel("vy", Double, "m/s", "ground speed east"),
                channel("vz", Double, "m/s", "ground speed down"),
                channel("hdg", Double, "deg", "heading, empty when unknown"),
            ],
        },
        FlowConfig {
            name: "ATTITUDE".into(),
            channels: vec![
                channel("roll", Double, "rad", "roll angle"),
                channel("pitch", Double, "rad", "pitch angle"),
                channel("yaw", Double, "rad", "yaw angle"),
                channel("rollspeed", Double, "rad/s", "roll rate"),
                channel("pitchspeed", Double, "rad/s", "pitch rate"),
                channel("yawspeed", Double, "rad/s", "yaw rate"),
            ],
        },
        FlowConfig {
            name: "SYS_STATUS".into(),
            channels: vec![
                channel("voltage_battery", Double, "V", "battery voltage"),
                channel("current_battery", Double, "A", "battery current"),
                channel("battery_remaining", Double, "%", "battery charge left"),
            ],
        },
        FlowConfig {
            name: "link".into(),
            channels: vec![
                channel(
                    "received",
                    Uint64,
                    "",
                    "frames received from the vehicle so far",
                ),
                channel("lost", Uint64, "", "frames lost so far, from sequence gaps"),
                channel("duplicates", Uint64, "", "frames received twice so far"),
                channel("loss", Double, "%", "lost / (received + lost)"),
            ],
        },
    ]
}

/// Converts one decoded message to a flow. Values are converted to degrees, metres, m/s, volts,
/// amps and percent. Fields the vehicle marks as unknown are left empty.
fn to_flow(message: &MavMessage, time: TimeValue) -> Option<Flow> {
    let flow = match message {
        MavMessage::GLOBAL_POSITION_INT(m) => {
            let mut values = vec![
                ChannelValue::new("lat", f64::from(m.lat) / 1e7),
                ChannelValue::new("lon", f64::from(m.lon) / 1e7),
                ChannelValue::new("alt", f64::from(m.alt) / 1e3),
                ChannelValue::new("relative_alt", f64::from(m.relative_alt) / 1e3),
                ChannelValue::new("vx", f64::from(m.vx) / 100.0),
                ChannelValue::new("vy", f64::from(m.vy) / 100.0),
                ChannelValue::new("vz", f64::from(m.vz) / 100.0),
            ];
            if m.hdg != u16::MAX {
                values.push(ChannelValue::new("hdg", f64::from(m.hdg) / 100.0));
            }
            Flow::new("GLOBAL_POSITION_INT", time, &values)
        }
        MavMessage::ATTITUDE(m) => Flow::new(
            "ATTITUDE",
            time,
            &[
                ChannelValue::new("roll", f64::from(m.roll)),
                ChannelValue::new("pitch", f64::from(m.pitch)),
                ChannelValue::new("yaw", f64::from(m.yaw)),
                ChannelValue::new("rollspeed", f64::from(m.rollspeed)),
                ChannelValue::new("pitchspeed", f64::from(m.pitchspeed)),
                ChannelValue::new("yawspeed", f64::from(m.yawspeed)),
            ],
        ),
        MavMessage::SYS_STATUS(m) => {
            let mut values = Vec::new();
            if m.voltage_battery != u16::MAX {
                values.push(ChannelValue::new(
                    "voltage_battery",
                    f64::from(m.voltage_battery) / 1e3,
                ));
            }
            if m.current_battery != -1 {
                values.push(ChannelValue::new(
                    "current_battery",
                    f64::from(m.current_battery) / 100.0,
                ));
            }
            if m.battery_remaining != -1 {
                values.push(ChannelValue::new(
                    "battery_remaining",
                    f64::from(m.battery_remaining),
                ));
            }
            Flow::new("SYS_STATUS", time, &values)
        }
        _ => return None,
    };
    Some(flow)
}

/// Loss and duplicate counts over every component of the vehicle.
fn link_flow(stats: &LinkStats, system_id: u8, time: TimeValue) -> Flow {
    let (mut received, mut lost, mut duplicates) = (0u64, 0u64, 0u64);
    for ((sys, _), s) in stats.sources() {
        if sys == system_id {
            received += s.received;
            lost += s.lost;
            duplicates += s.duplicates;
        }
    }
    let loss = if received + lost == 0 {
        0.0
    } else {
        100.0 * lost as f64 / (received + lost) as f64
    };
    Flow::new(
        "link",
        time,
        &[
            ChannelValue::new("received", received),
            ChannelValue::new("lost", lost),
            ChannelValue::new("duplicates", duplicates),
            ChannelValue::new("loss", loss),
        ],
    )
}

async fn run() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: cargo run --example mavlink-tlog -- <file.tlog>")?;
    let system_id: u8 = env::var("MAVLINK_SYSTEM_ID").map_or(Ok(1), |id| id.parse())?;
    let asset_name = env::var("SIFT_ASSET_NAME").unwrap_or_else(|_| "mavlink-vehicle".into());

    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").map_err(|_| "SIFT_API_KEY is not set")?,
        uri: env::var("SIFT_URI").map_err(|_| "SIFT_URI is not set")?,
    };

    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let file_name = Path::new(&path)
        .file_name()
        .map_or(path.clone(), |f| f.to_string_lossy().into_owned());

    let mut sift_stream = SiftStreamBuilder::new(credentials)
        .ingestion_config(IngestionConfigForm {
            asset_name: asset_name.clone(),
            client_key: format!("{asset_name}-mavlink-tlog-v1"),
            flows: flows(),
        })
        .attach_run(RunForm {
            name: format!("[{asset_name}].{file_name}.{ts}"),
            client_key: format!("{asset_name}-tlog-{ts}"),
            description: Some(format!("replay of {file_name}")),
            tags: Some(vec!["mavlink".into(), "tlog".into()]),
            metadata: None,
        })
        .live_with_backups()
        .build()
        .await?;

    let file = BufReader::new(File::open(&path)?);
    let (sent, stats) = replay(&mut sift_stream, file, system_id).await?;

    println!("{file_name}: {sent} flows sent");
    for ((sys, comp), s) in stats.sources() {
        println!(
            "  sender {sys}/{comp}: {} received, {} lost ({:.2}%), {} duplicates",
            s.received,
            s.lost,
            s.loss_ratio() * 100.0,
            s.duplicates
        );
    }

    // Gracefully terminate your stream
    sift_stream.finish().await?;
    Ok(())
}

/// Sends every frame of `system_id` in the log and returns the number of flows sent and the link
/// statistics.
async fn replay<R: Read>(
    sift_stream: &mut SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>,
    tlog: R,
    system_id: u8,
) -> Result<(u64, LinkStats), Box<dyn Error>> {
    let mut log = TlogReader::new(tlog);
    let mut stats = LinkStats::default();
    let mut next_link_us = 0;
    let mut last_time = None;
    let mut sent = 0;

    while let Some(record) = log.next_record()? {
        if record.frame.sys_id() != system_id {
            continue;
        }
        stats.observe(record.timestamp_us, &record.frame);

        // The time the ground station received the frame.
        let time = TimeValue::try_from_timestamp_micros(i64::try_from(record.timestamp_us)?)?;

        if record.timestamp_us >= next_link_us {
            sift_stream
                .send(link_flow(&stats, system_id, time.clone()))
                .await?;
            sent += 1;
            next_link_us = record.timestamp_us + LINK_INTERVAL_US;
        }
        last_time = Some(time.clone());

        // Frames from messages outside the common dialect (such as ardupilotmega) are skipped.
        let Ok((_, message)) =
            mavlink::MavlinkReader::new(record.frame.as_bytes()).read_any_message::<MavMessage>()
        else {
            continue;
        };
        if let Some(flow) = to_flow(&message, time) {
            sift_stream.send(flow).await?;
            sent += 1;
        }
    }

    // Final totals at the time of the last frame.
    if let Some(time) = last_time {
        sift_stream.send(link_flow(&stats, system_id, time)).await?;
        sent += 1;
    }

    let read = log.stats();
    if read.skipped_bytes > 0 || read.bad_crc > 0 {
        eprintln!(
            "damaged log: {} bytes skipped, {} frames with bad checksums",
            read.skipped_bytes, read.bad_crc
        );
    }
    Ok((sent, stats))
}
