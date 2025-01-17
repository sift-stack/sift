use chrono::{DateTime, Utc};
use pbjson_types::Timestamp;
use rand::Rng;
use sift_rs::{
    ingest::v1::{
        ingest_with_config_data_channel_value::Type, IngestWithConfigDataChannelValue,
        IngestWithConfigDataStreamRequest,
    },
    ingestion_configs::v1::IngestionConfig,
    runs::v2::Run,
};
use std::{
    iter::Iterator,
    ops::Drop,
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct DataSource {
    ingestion_config: IngestionConfig,
    run: Run,
    source: Option<JoinHandle<()>>,
    data_rx: Receiver<(DateTime<Utc>, f64)>,
}

impl DataSource {
    pub fn new(ingestion_config: IngestionConfig, run: Run) -> Self {
        let (data_tx, data_rx) = channel();
        let thread_handler = thread::spawn(move || {
            let duration = Duration::from_secs(60);
            let start = Instant::now();
            let mut rng = rand::thread_rng();

            while Instant::now().duration_since(start) < duration {
                data_tx
                    .send((Utc::now(), rng.gen_range(0.0..100.0)))
                    .unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });
        let source = Some(thread_handler);

        Self {
            ingestion_config,
            run,
            source,
            data_rx,
        }
    }
}

impl Iterator for DataSource {
    type Item = IngestWithConfigDataStreamRequest;

    fn next(&mut self) -> Option<Self::Item> {
        let Ok((timestamp, value)) = self.data_rx.recv() else {
            return None;
        };

        println!("ingestion velocity_reading");

        let req = IngestWithConfigDataStreamRequest {
            run_id: self.run.run_id.clone(),
            ingestion_config_id: String::from(&self.ingestion_config.ingestion_config_id),
            flow: String::from("velocity_reading"),
            timestamp: Some(Timestamp::from(timestamp)),
            channel_values: vec![IngestWithConfigDataChannelValue {
                r#type: Some(Type::Double(value)),
            }],
            // Set this flag to `true` only for debugging purposes to get real-time data validation from
            // the Sift API. Do not use in production as it will hurt performance.
            end_stream_on_validation_error: false,
            ..Default::default()
        };
        Some(req)
    }
}

impl Drop for DataSource {
    fn drop(&mut self) {
        let _ = self.source.take().map(|h| h.join());
    }
}
