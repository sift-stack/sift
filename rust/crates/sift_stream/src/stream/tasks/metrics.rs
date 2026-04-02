use crate::{
    Flow, FlowConfig, IngestionConfigForm, LiveStreamingWithBackups, RetryPolicy, SiftStream,
    SiftStreamBuilder, TimeValue,
    metrics::{SiftStreamMetrics, SiftStreamMetricsSnapshot},
    stream::{mode::ingestion_config::IngestionConfigEncoder, tasks::ControlMessage},
};
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use std::{sync::Arc, time::Duration};
use tokio::{select, sync::broadcast};

/// The asset to stream metrics for.
const METRICS_STREAMING_INGESTION_CONFIG_ASSET_NAME: &str = "sift_app";

/// The client key used for sift_stream metrics ingestion config.
const METRICS_STREAMING_INGESTION_CONFIG_CLIENT_KEY: &str = "sift-stream-metrics";

/// The flow name used for sift_stream metrics flow config.
const METRICS_STREAMING_FLOW_NAME: &str = "sift-stream-metrics-flow";

pub(crate) struct MetricsStreamingTask {
    stream: SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>,
    control_rx: broadcast::Receiver<ControlMessage>,
    session_name: String,
    interval: Duration,
    metrics: Arc<SiftStreamMetrics>,
}

impl MetricsStreamingTask {
    pub(crate) async fn new(
        setup_channel: SiftChannel,
        control_rx: broadcast::Receiver<ControlMessage>,
        session_name: String,
        interval: Duration,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        use std::hash::{Hash, Hasher};

        let channels = SiftStreamMetricsSnapshot::channel_configs(&session_name);

        // Hash the channel names to create a unique client key for the ingestion config.
        //
        // Given the same "session_name", which influences the channel names, and the same metrics configuration,
        // the ingestion config client key should be the same and re-used.
        let mut hasher = std::hash::DefaultHasher::new();
        channels.iter().for_each(|channel| {
            channel.name.hash(&mut hasher);
        });
        let hash_key = hasher.finish();

        let client_key = format!(
            "{}-{}",
            METRICS_STREAMING_INGESTION_CONFIG_CLIENT_KEY, hash_key
        );

        let ingestion_config = IngestionConfigForm {
            asset_name: METRICS_STREAMING_INGESTION_CONFIG_ASSET_NAME.to_string(),
            client_key,
            flows: vec![FlowConfig {
                name: METRICS_STREAMING_FLOW_NAME.to_string(),
                channels: SiftStreamMetricsSnapshot::channel_configs(&session_name),
            }],
        };

        // Build a new [`SiftStream`] that is responsible for streaming metrics to Sift.
        //
        // Most builder parameters are carried over from the main stream being monitored, however,
        // the differences are noted below:
        //
        // - Channel capacities are substantially lower since this stream deals with less throughput.
        // - Metrics streaming interval is set to `None` to disable streaming.
        // - The `setup-channel` is used for all gRPC channels in this stream since they are less
        //   critical and thus can be multiplexed over a single connection.
        //
        // NOTE: The build future is boxed/pinned due to async recursion -- generally a sift-stream
        // instance will be spawning a second sift-stream instance for streaming it's own metrics though
        // the limit of recursion here is 2 since the metrics-streaming sift-stream doesn't itself
        // spawn another sift-stream instance. Since this is only done during initialization, it is fine.
        let stream_fut = Box::pin(
            SiftStreamBuilder::from_channel(setup_channel.clone())
                .ingestion_config(ingestion_config)
                .live_with_backups()
                .metrics_streaming_interval(None)
                .control_channel_capacity(100)
                .ingestion_data_channel_capacity(1000)
                .backup_data_channel_capacity(1000)
                .build(),
        );

        let stream = stream_fut.await?;

        Ok(Self {
            stream,
            control_rx,
            session_name,
            interval,
            metrics,
        })
    }

    pub(crate) async fn run(mut self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);

        loop {
            select! {
                _ = interval.tick() => {
                    let metrics = self.metrics.snapshot();
                    let values = metrics.channel_values(&self.session_name);
                    let flow = Flow::new(METRICS_STREAMING_FLOW_NAME, TimeValue::now(), &values);
                    self.stream.send(flow).await.map_err(|e| {
                        Error::new_msg(ErrorKind::StreamError, e.to_string())
                    })?;
                }
                ctrl_msg = self.control_rx.recv() => {
                    match ctrl_msg {
                        Ok(ControlMessage::Shutdown) => {
                            break;
                        }
                        Err(e) => {
                            #[cfg(feature = "tracing")]
                            tracing::error!(
                                error = %e,
                                "metrics streaming task received error on control channel"
                            );
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!("metrics streaming task shutting down");

        self.stream
            .finish()
            .await
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
    }
}
