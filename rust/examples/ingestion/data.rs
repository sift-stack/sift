use chrono::Utc;
use pbjson_types::Timestamp;
use sift_rs::{
    ingest::v1::{
        ingest_with_config_data_channel_value::Type, IngestWithConfigDataChannelValue,
        IngestWithConfigDataStreamRequest,
    },
    ingestion_configs::v1::IngestionConfig,
    runs::v2::Run,
};
use std::time::{Duration, Instant};
use tokio::{
    sync::mpsc::unbounded_channel,
    task::{self, JoinHandle},
    time,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub fn init_data_source(
    ingestion_config: IngestionConfig,
    run: Run,
) -> (
    JoinHandle<()>,
    UnboundedReceiverStream<IngestWithConfigDataStreamRequest>,
) {
    let (data_tx, data_rx) = unbounded_channel();

    let data_source_handle = task::spawn(async move {
        let duration = Duration::from_secs(60);
        let start = Instant::now();

        // Generating a nice sine wave
        let mut t: f64 = 0.0;
        let amplitude = 5.0;
        let offset = 5.0;
        let frequency = 0.1;

        while Instant::now().duration_since(start) < duration {
            let value = amplitude * (t * frequency).sin() + offset;
            let req = IngestWithConfigDataStreamRequest {
                run_id: run.run_id.clone(),
                ingestion_config_id: String::from(&ingestion_config.ingestion_config_id),
                flow: String::from("velocity_reading"),
                timestamp: Some(Timestamp::from(Utc::now())),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Double(value)),
                }],
                // Set this flag to `true` only for debugging purposes to get real-time data validation from
                // the Sift API. Do not use in production as it will hurt performance.
                end_stream_on_validation_error: false,
                ..Default::default()
            };
            println!("Emitting value for velocity_reading");
            data_tx.send(req).unwrap();
            t += 1.0;
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    (data_source_handle, UnboundedReceiverStream::new(data_rx))
}
