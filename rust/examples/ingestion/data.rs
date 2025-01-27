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
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio::{
    sync::mpsc::{channel, Receiver},
    task::{self, JoinHandle},
};
use tokio_stream::Stream;

pub struct DataSource {
    ingestion_config: IngestionConfig,
    run: Run,
    source: Option<JoinHandle<()>>,
    data_rx: Receiver<(DateTime<Utc>, f64)>,
}

impl DataSource {
    pub fn new(ingestion_config: IngestionConfig, run: Run) -> Self {
        let (data_tx, data_rx) = channel(1);

        let task_handler = task::spawn_blocking(move || {
            let duration = Duration::from_secs(180);
            let start = Instant::now();
            let mut rng = rand::thread_rng();

            while Instant::now().duration_since(start) < duration {
                data_tx
                    .blocking_send((Utc::now(), rng.gen_range(0.0..100.0)))
                    .unwrap();
                std::thread::sleep(Duration::from_millis(500));
            }
        });

        let source = Some(task_handler);

        Self {
            ingestion_config,
            run,
            source,
            data_rx,
        }
    }
}

impl Stream for DataSource {
    type Item = IngestWithConfigDataStreamRequest;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.data_rx.poll_recv(ctx) {
            Poll::Ready(Some((ts, val))) => {
                let req = IngestWithConfigDataStreamRequest {
                    run_id: self.run.run_id.clone(),
                    ingestion_config_id: String::from(&self.ingestion_config.ingestion_config_id),
                    flow: String::from("velocity_reading"),
                    timestamp: Some(Timestamp::from(ts)),
                    channel_values: vec![IngestWithConfigDataChannelValue {
                        r#type: Some(Type::Double(val)),
                    }],
                    // Set this flag to `true` only for debugging purposes to get real-time data validation from
                    // the Sift API. Do not use in production as it will hurt performance.
                    end_stream_on_validation_error: false,
                    ..Default::default()
                };
                println!("Emitting value for velocity_reading");
                Poll::Ready(Some(req))
            }
            Poll::Ready(_) => Poll::Ready(None),
            _ => Poll::Pending,
        }
    }
}

impl Drop for DataSource {
    fn drop(&mut self) {
        if let Some(handle) = self.source.take() {
            handle.abort();
        }
    }
}
