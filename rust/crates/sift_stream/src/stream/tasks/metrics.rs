use crate::{
    FlowBuilder, FlowConfig, FlowDescriptor, IngestionConfigForm, LiveStreamingOnly, SiftStream,
    SiftStreamBuilder,
    logging::{DeduplicatorOutput, LogDeduplicator, LogEvent},
    metrics::{MetricsFlowIndices, SiftStreamMetrics, SiftStreamMetricsSnapshot},
    stream::{
        flow::ChannelIndex, mode::ingestion_config::IngestionConfigEncoder, tasks::ControlMessage,
    },
};
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{common::r#type::v1::ChannelDataType, ingestion_configs::v2::ChannelConfig};
use std::{sync::Arc, time::Duration};
use tokio::{select, sync::broadcast};

/// The asset to stream metrics for.
const METRICS_STREAMING_INGESTION_CONFIG_ASSET_NAME: &str = "sift_app";

/// The client key used for sift_stream metrics ingestion config.
const METRICS_STREAMING_INGESTION_CONFIG_CLIENT_KEY: &str = "sift-stream-metrics";

/// The flow name used for sift_stream metrics flow config.
const METRICS_STREAMING_FLOW_NAME: &str = "sift-stream-metrics-flow";

/// The flow name used for sift_stream log events.
const LOG_FLOW_NAME: &str = "sift-stream-logs";

/// Pre-resolved [`ChannelIndex`] values for the log event flow channels.
struct LogFlowIndices {
    level: ChannelIndex,
    target: ChannelIndex,
    message: ChannelIndex,
}

impl LogFlowIndices {
    fn new(descriptor: &FlowDescriptor<String>, channel_prefix: &str) -> Result<Self> {
        let m = descriptor.mapping();
        let get = |suffix: &str| -> Result<ChannelIndex> {
            let key = format!("{channel_prefix}.{suffix}");
            m.get(&key).copied().ok_or_else(|| {
                Error::new_msg(
                    ErrorKind::NotFoundError,
                    format!("log channel '{key}' not found in flow descriptor"),
                )
            })
        };
        Ok(Self {
            level: get("tracing_event.level")?,
            target: get("tracing_event.target")?,
            message: get("tracing_event.message")?,
        })
    }
}

/// Returns the channel configs for the log event flow, namespaced under `channel_prefix`.
///
/// Channel names follow the same `{prefix}.{field}` convention used by the metrics flow.
fn log_channel_configs(channel_prefix: &str) -> Vec<ChannelConfig> {
    vec![
        ChannelConfig {
            name: format!("{channel_prefix}.tracing_event.level"),
            description: "Log level (ERROR, WARN, INFO, DEBUG, TRACE)".into(),
            data_type: ChannelDataType::String.into(),
            ..Default::default()
        },
        ChannelConfig {
            name: format!("{channel_prefix}.tracing_event.target"),
            description: "Source module target of the log event".into(),
            data_type: ChannelDataType::String.into(),
            ..Default::default()
        },
        ChannelConfig {
            name: format!("{channel_prefix}.tracing_event.message"),
            description: "Log message, including any structured key-value fields".into(),
            data_type: ChannelDataType::String.into(),
            ..Default::default()
        },
    ]
}

/// Build a [`FlowBuilder`] for a single log event.
///
/// Structured key-value fields are appended to the message as `key=value` pairs.
fn encode_log_event<'a>(
    event: &LogEvent,
    indices: &LogFlowIndices,
    descriptor: &'a FlowDescriptor<String>,
) -> FlowBuilder<'a, String> {
    let mut builder = FlowBuilder::new(descriptor);
    let _ = builder.set(indices.level, event.level.as_str());
    let _ = builder.set(indices.target, event.target);

    let message = if event.fields.is_empty() {
        event.message.clone()
    } else {
        let fields_str = event
            .fields
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(" ");
        format!("{} {fields_str}", event.message)
    };
    let _ = builder.set(indices.message, message);
    builder
}

pub(crate) struct MetricsStreamingTask {
    stream: SiftStream<IngestionConfigEncoder, LiveStreamingOnly>,
    control_rx: broadcast::Receiver<ControlMessage>,
    interval: Duration,
    metrics: Arc<SiftStreamMetrics>,
    metrics_flow_descriptor: FlowDescriptor<String>,
    flow_indices: MetricsFlowIndices,
    log_rx: Option<async_channel::Receiver<LogEvent>>,
    log_flow_descriptor: Option<FlowDescriptor<String>>,
    log_flow_indices: Option<LogFlowIndices>,
}

impl MetricsStreamingTask {
    pub(crate) async fn new(
        setup_channel: SiftChannel,
        control_rx: broadcast::Receiver<ControlMessage>,
        session_name: String,
        interval: Duration,
        metrics: Arc<SiftStreamMetrics>,
        log_rx: Option<async_channel::Receiver<LogEvent>>,
    ) -> Result<Self> {
        use std::hash::{Hash, Hasher};

        let metrics_channels = SiftStreamMetricsSnapshot::channel_configs(&session_name);

        // Hash the channel names to create a unique client key for the ingestion config.
        //
        // When the log flow is present, its channel names are included so that an ingestion
        // config with log channels and one without produce distinct client keys.
        let mut hasher = std::hash::DefaultHasher::new();
        metrics_channels.iter().for_each(|c| {
            c.name.hash(&mut hasher);
        });
        if log_rx.is_some() {
            log_channel_configs(&session_name).iter().for_each(|c| {
                c.name.hash(&mut hasher);
            });
        }
        let hash_key = hasher.finish();

        let client_key = format!(
            "{}-{}",
            METRICS_STREAMING_INGESTION_CONFIG_CLIENT_KEY, hash_key
        );

        let mut flows = vec![FlowConfig {
            name: METRICS_STREAMING_FLOW_NAME.to_string(),
            channels: metrics_channels,
        }];

        if log_rx.is_some() {
            flows.push(FlowConfig {
                name: LOG_FLOW_NAME.to_string(),
                channels: log_channel_configs(&session_name),
            });
        }

        let ingestion_config = IngestionConfigForm {
            asset_name: METRICS_STREAMING_INGESTION_CONFIG_ASSET_NAME.to_string(),
            client_key,
            flows,
        };

        // Build a new [`SiftStream`] that is responsible for streaming metrics and logs to Sift.
        //
        // Differences from the main stream:
        // - `LiveStreamingOnly` mode: no disk backups or checkpointing, reducing task count and
        //   I/O paths since observability data is non-critical.
        // - Channel capacities are substantially lower.
        // - Metrics streaming interval is `None` to prevent a recursive sub-stream.
        // - `disable_scoped_dispatch()` prevents `SiftTelemetryLayer` from capturing events
        //   from this sub-stream's own tasks and causing a recursive capture loop.
        // - The setup channel is used for all gRPC channels since they are less critical.
        //
        // NOTE: The build future is boxed/pinned due to async recursion.
        let stream_fut = Box::pin(
            SiftStreamBuilder::from_channel(setup_channel.clone())
                .ingestion_config(ingestion_config)
                .disable_scoped_dispatch()
                .live_only()
                .metrics_streaming_interval(None)
                .control_channel_capacity(100)
                .ingestion_data_channel_capacity(1000)
                .build(),
        );

        let stream = stream_fut.await?;

        let metrics_flow_descriptor = stream.get_flow_descriptor(METRICS_STREAMING_FLOW_NAME)?;
        let flow_indices = MetricsFlowIndices::new(&metrics_flow_descriptor, &session_name)?;

        let (log_flow_descriptor, log_flow_indices, log_rx) = match log_rx {
            Some(rx) => {
                let desc = stream.get_flow_descriptor(LOG_FLOW_NAME)?;
                let indices = LogFlowIndices::new(&desc, &session_name)?;
                (Some(desc), Some(indices), Some(rx))
            }
            None => (None, None, None),
        };

        Ok(Self {
            stream,
            control_rx,
            interval,
            metrics,
            metrics_flow_descriptor,
            flow_indices,
            log_rx,
            log_flow_descriptor,
            log_flow_indices,
        })
    }

    pub(crate) async fn run(mut self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        let mut dedup = LogDeduplicator::default();

        loop {
            select! {
                _ = interval.tick() => {
                    // Update log channel depth metric before taking the snapshot.
                    if let Some(rx) = &self.log_rx {
                        self.metrics.log_channel_depth.set(rx.len() as u64);
                    }
                    let snapshot = self.metrics.snapshot();
                    let mut builder = FlowBuilder::new(&self.metrics_flow_descriptor);
                    let _ = snapshot.populate_flow(&self.flow_indices, &mut builder);
                    let _ = self.stream.try_send(builder);
                }

                log_event = async {
                    match &self.log_rx {
                        Some(rx) => rx.recv().await,
                        None => std::future::pending().await,
                    }
                }, if self.log_rx.is_some() => {
                    if let Ok(event) = log_event {
                        match dedup.process(event) {
                            DeduplicatorOutput::Suppress => {}
                            DeduplicatorOutput::Emit(e) => {
                                self.send_log_event(e);
                            }
                            DeduplicatorOutput::EmitSummaryThenEmit(summary, e) => {
                                self.send_log_event(summary);
                                self.send_log_event(e);
                            }
                        }
                    }
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

        // Flush pending dedup state and drain remaining log events before shutdown.
        if self.log_flow_descriptor.is_some() {
            if let Some(summary) = dedup.flush() {
                self.send_log_event(summary);
            }
            // Collect remaining events first to avoid holding a borrow on self.log_rx
            // while calling send_log_event (which needs &mut self).
            let remaining: Vec<LogEvent> = match &self.log_rx {
                Some(rx) => {
                    let mut events = Vec::new();
                    while let Ok(event) = rx.try_recv() {
                        events.push(event);
                    }
                    events
                }
                None => Vec::new(),
            };
            for event in remaining {
                match dedup.process(event) {
                    DeduplicatorOutput::Suppress => {}
                    DeduplicatorOutput::Emit(e) => self.send_log_event(e),
                    DeduplicatorOutput::EmitSummaryThenEmit(s, e) => {
                        self.send_log_event(s);
                        self.send_log_event(e);
                    }
                }
            }
            if let Some(summary) = dedup.flush() {
                self.send_log_event(summary);
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!("metrics streaming task shutting down");

        self.stream
            .finish()
            .await
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
    }

    /// Encode and non-blockingly send a single log event to the stream.
    /// Errors (e.g. channel full) are silently discarded — log streaming is best-effort.
    fn send_log_event(&mut self, event: LogEvent) {
        if let (Some(desc), Some(indices)) = (&self.log_flow_descriptor, &self.log_flow_indices) {
            let builder = encode_log_event(&event, indices, desc);
            let _ = self.stream.try_send(builder);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TimeValue, logging::LogLevel};
    use sift_rs::{
        common::r#type::v1::ChannelDataType,
        ingest::v1::ingest_with_config_data_channel_value::Type, ingestion_configs::v2::FlowConfig,
    };

    const TEST_PREFIX: &str = "stream.test_asset.test_key";

    fn make_log_descriptor() -> FlowDescriptor<String> {
        FlowDescriptor::try_from((
            "test-ingestion-config-id",
            FlowConfig {
                name: LOG_FLOW_NAME.to_string(),
                channels: log_channel_configs(TEST_PREFIX),
            },
        ))
        .unwrap()
    }

    fn make_event(message: &str, fields: Vec<(String, String)>) -> LogEvent {
        LogEvent {
            level: LogLevel::Warn,
            target: "sift_stream::tasks::ingestion",
            file: "ingestion.rs",
            line: 42,
            message: message.to_owned(),
            fields,
            timestamp: std::time::SystemTime::now(),
        }
    }

    #[test]
    fn log_channel_configs_returns_three_string_channels() {
        let configs = log_channel_configs(TEST_PREFIX);
        assert_eq!(configs.len(), 3);
        let names: Vec<&str> = configs.iter().map(|c| c.name.as_str()).collect();
        assert!(
            names.contains(&"stream.test_asset.test_key.tracing_event.level"),
            "missing prefixed 'level' channel; got: {names:?}"
        );
        assert!(
            names.contains(&"stream.test_asset.test_key.tracing_event.target"),
            "missing prefixed 'target' channel; got: {names:?}"
        );
        assert!(
            names.contains(&"stream.test_asset.test_key.tracing_event.message"),
            "missing prefixed 'message' channel; got: {names:?}"
        );
        for c in &configs {
            assert_eq!(
                c.data_type,
                ChannelDataType::String as i32,
                "channel '{}' must be String type",
                c.name
            );
        }
    }

    #[test]
    fn log_flow_indices_new_succeeds_with_complete_descriptor() {
        let descriptor = make_log_descriptor();
        assert!(LogFlowIndices::new(&descriptor, TEST_PREFIX).is_ok());
    }

    #[test]
    fn log_flow_indices_new_fails_when_channel_missing() {
        // Descriptor with only the prefixed "level" and "target" — "message" is absent.
        let partial_descriptor = FlowDescriptor::try_from((
            "test-ingestion-config-id",
            FlowConfig {
                name: LOG_FLOW_NAME.to_string(),
                channels: vec![
                    ChannelConfig {
                        name: format!("{TEST_PREFIX}.tracing_event.level"),
                        data_type: ChannelDataType::String as i32,
                        ..Default::default()
                    },
                    ChannelConfig {
                        name: format!("{TEST_PREFIX}.tracing_event.target"),
                        data_type: ChannelDataType::String as i32,
                        ..Default::default()
                    },
                ],
            },
        ))
        .unwrap();
        assert!(LogFlowIndices::new(&partial_descriptor, TEST_PREFIX).is_err());
    }

    #[test]
    fn encode_log_event_no_fields_leaves_message_unchanged() {
        let descriptor = make_log_descriptor();
        let indices = LogFlowIndices::new(&descriptor, TEST_PREFIX).unwrap();
        let event = make_event("something happened", vec![]);
        let request = encode_log_event(&event, &indices, &descriptor).request(TimeValue::default());

        assert_eq!(
            request.channel_values[indices.message.as_usize()].r#type,
            Some(Type::String("something happened".to_owned()))
        );
    }

    #[test]
    fn encode_log_event_appends_fields_as_key_value_pairs() {
        let descriptor = make_log_descriptor();
        let indices = LogFlowIndices::new(&descriptor, TEST_PREFIX).unwrap();
        let event = make_event(
            "base message",
            vec![
                ("key1".to_owned(), "val1".to_owned()),
                ("key2".to_owned(), "val2".to_owned()),
            ],
        );
        let request = encode_log_event(&event, &indices, &descriptor).request(TimeValue::default());

        assert_eq!(
            request.channel_values[indices.message.as_usize()].r#type,
            Some(Type::String("base message key1=val1 key2=val2".to_owned()))
        );
    }

    #[test]
    fn encode_log_event_sets_level_and_target_channels() {
        let descriptor = make_log_descriptor();
        let indices = LogFlowIndices::new(&descriptor, TEST_PREFIX).unwrap();
        let event = make_event("msg", vec![]);
        let request = encode_log_event(&event, &indices, &descriptor).request(TimeValue::default());

        assert_eq!(
            request.channel_values[indices.level.as_usize()].r#type,
            Some(Type::String("WARN".to_owned())),
            "level channel should contain the string representation"
        );
        assert_eq!(
            request.channel_values[indices.target.as_usize()].r#type,
            Some(Type::String("sift_stream::tasks::ingestion".to_owned())),
            "target channel should contain the module target"
        );
    }
}
