use crate::{
    logging::{LogEvent, LogLevel},
    metrics::SiftStreamMetrics,
};
use std::sync::Arc;

/// Extracts the event message and structured key-value fields from a tracing event.
#[derive(Default)]
pub(crate) struct TelemetryVisitor {
    pub message: String,
    pub fields: Vec<(String, String)>,
}

impl tracing::field::Visit for TelemetryVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_owned();
        } else {
            self.fields
                .push((field.name().to_owned(), value.to_owned()));
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let s = format!("{value:?}");
        if field.name() == "message" {
            self.message = s;
        } else {
            self.fields.push((field.name().to_owned(), s));
        }
    }
}

/// A `tracing_subscriber` layer that captures events originating from `sift_stream::*` and
/// forwards them to a bounded channel for later forwarding to Sift.
///
/// Installed as part of a scoped dispatch on each background task future so that events
/// are captured without touching the global subscriber.
pub(crate) struct SiftTelemetryLayer {
    tx: async_channel::Sender<LogEvent>,
    level_filter: LogLevel,
    metrics: Arc<SiftStreamMetrics>,
}

impl SiftTelemetryLayer {
    pub(crate) fn new(
        tx: async_channel::Sender<LogEvent>,
        level_filter: LogLevel,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Self {
        Self {
            tx,
            level_filter,
            metrics,
        }
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::layer::Layer<S> for SiftTelemetryLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = event.metadata();

        // Only capture events originating from this crate.
        if !metadata.target().starts_with("sift_stream") {
            return;
        }

        // Level filter: plain comparison, no lock.
        let level = LogLevel::from(metadata.level());
        if level > self.level_filter {
            return;
        }

        let mut visitor = TelemetryVisitor::default();
        event.record(&mut visitor);

        let log_event = LogEvent {
            level,
            target: metadata.target(),
            file: metadata.file().unwrap_or(""),
            line: metadata.line().unwrap_or(0),
            message: visitor.message,
            fields: visitor.fields,
            timestamp: std::time::SystemTime::now(),
        };

        if self.tx.try_send(log_event).is_err() {
            self.metrics.logs_dropped_channel_full.increment();
        }
    }
}

/// Forwards `sift_stream::*` events from the scoped dispatch to a captured
/// `tracing::Dispatch`, typically the global subscriber at stream construction time.
///
/// This ensures that sift_stream's own tracing events from background tasks still reach
/// the user's subscriber (e.g. a console logger) even when those tasks run under a scoped
/// dispatch.
///
/// Only events with a `sift_stream` target prefix are forwarded. Third-party library events
/// (h2, hyper, tonic, etc.) emitted from within the tasks are intentionally not forwarded to
/// avoid unintended side effects such as flooding subscribers with low-level TRACE spam.
///
/// **Limitation**: span lifecycle is not forwarded. This is acceptable for sift_stream's
/// background tasks which do not create spans.
pub(crate) struct DispatchForwardingLayer(pub tracing::Dispatch);

impl<S: tracing::Subscriber> tracing_subscriber::layer::Layer<S> for DispatchForwardingLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Only forward events originating from this crate to the base subscriber.
        if event.metadata().target().starts_with("sift_stream") && self.0.enabled(event.metadata())
        {
            self.0.event(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::SiftStreamMetrics;
    use std::sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    };
    use tracing_subscriber::layer::SubscriberExt;

    fn make_layer(
        capacity: usize,
        level: LogLevel,
    ) -> (
        SiftTelemetryLayer,
        async_channel::Receiver<LogEvent>,
        Arc<SiftStreamMetrics>,
    ) {
        let (tx, rx) = async_channel::bounded(capacity);
        let metrics = Arc::new(SiftStreamMetrics::new());
        let layer = SiftTelemetryLayer::new(tx, level, metrics.clone());
        (layer, rx, metrics)
    }

    fn with_layer<F: FnOnce()>(layer: SiftTelemetryLayer, f: F) {
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::dispatcher::with_default(&tracing::Dispatch::new(subscriber), f);
    }

    /// Minimal subscriber that counts received events; used as the base for
    /// `DispatchForwardingLayer` tests.
    struct EventCounter(Arc<AtomicU32>);

    impl tracing::Subscriber for EventCounter {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
        fn event(&self, _: &tracing::Event<'_>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
        fn enter(&self, _: &tracing::span::Id) {}
        fn exit(&self, _: &tracing::span::Id) {}
    }

    #[test]
    fn layer_captures_sift_stream_event() {
        let (layer, rx, _) = make_layer(8, LogLevel::Info);
        with_layer(layer, || {
            tracing::event!(
                target: "sift_stream::tasks::ingestion",
                tracing::Level::INFO,
                "hello from sift"
            );
        });
        assert_eq!(rx.len(), 1);
        let ev = rx.try_recv().unwrap();
        assert_eq!(ev.message, "hello from sift");
        assert_eq!(ev.level, LogLevel::Info);
        assert_eq!(ev.target, "sift_stream::tasks::ingestion");
    }

    #[test]
    fn layer_ignores_non_sift_stream_target() {
        let (layer, rx, _) = make_layer(8, LogLevel::Info);
        with_layer(layer, || {
            tracing::event!(target: "h2::proto::connection", tracing::Level::INFO, "h2 noise");
            tracing::event!(target: "tonic::transport", tracing::Level::WARN, "tonic noise");
            tracing::event!(target: "hyper::client", tracing::Level::ERROR, "hyper noise");
        });
        assert_eq!(rx.len(), 0, "non-sift_stream events must not be captured");
    }

    #[test]
    fn layer_filters_events_below_level_threshold() {
        // With an INFO filter, DEBUG and TRACE are too verbose and must be dropped.
        let (layer, rx, _) = make_layer(8, LogLevel::Info);
        with_layer(layer, || {
            tracing::event!(
                target: "sift_stream::tasks",
                tracing::Level::DEBUG,
                "debug noise"
            );
            tracing::event!(
                target: "sift_stream::tasks",
                tracing::Level::TRACE,
                "trace noise"
            );
        });
        assert_eq!(
            rx.len(),
            0,
            "DEBUG/TRACE must be dropped with an INFO filter"
        );
    }

    #[test]
    fn layer_passes_events_at_and_above_level_threshold() {
        let (layer, rx, _) = make_layer(8, LogLevel::Info);
        with_layer(layer, || {
            tracing::event!(target: "sift_stream::tasks", tracing::Level::INFO, "info");
            tracing::event!(target: "sift_stream::tasks", tracing::Level::WARN, "warn");
            tracing::event!(target: "sift_stream::tasks", tracing::Level::ERROR, "error");
        });
        assert_eq!(
            rx.len(),
            3,
            "INFO, WARN, ERROR must all pass an INFO filter"
        );
    }

    #[test]
    fn layer_captures_structured_fields() {
        let (layer, rx, _) = make_layer(8, LogLevel::Info);
        with_layer(layer, || {
            tracing::event!(
                target: "sift_stream::tasks",
                tracing::Level::INFO,
                user = "alice",
                request_id = "req-42",
                "user login"
            );
        });
        let ev = rx.try_recv().unwrap();
        assert_eq!(ev.message, "user login");
        assert!(
            ev.fields.iter().any(|(k, v)| k == "user" && v == "alice"),
            "user field not captured: {:?}",
            ev.fields
        );
        assert!(
            ev.fields
                .iter()
                .any(|(k, v)| k == "request_id" && v == "req-42"),
            "request_id field not captured: {:?}",
            ev.fields
        );
    }

    #[test]
    fn layer_increments_dropped_metric_on_full_channel() {
        // Capacity 1: the first event fills the channel; the second must be dropped.
        let (layer, rx, metrics) = make_layer(1, LogLevel::Info);
        with_layer(layer, || {
            tracing::event!(target: "sift_stream::tasks", tracing::Level::INFO, "first");
            tracing::event!(target: "sift_stream::tasks", tracing::Level::INFO, "overflow");
        });
        assert_eq!(rx.len(), 1, "only the first event should be in the channel");
        assert_eq!(
            metrics.logs_dropped_channel_full.get(),
            1,
            "exactly one drop should be counted"
        );
    }

    #[test]
    fn forwarding_layer_forwards_sift_stream_events_to_base() {
        let count = Arc::new(AtomicU32::new(0));
        let base = tracing::Dispatch::new(EventCounter(count.clone()));
        let layer = DispatchForwardingLayer(base);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::dispatcher::with_default(&tracing::Dispatch::new(subscriber), || {
            tracing::event!(
                target: "sift_stream::tasks::ingestion",
                tracing::Level::INFO,
                "sift event"
            );
            tracing::event!(
                target: "sift_stream::stream::builder",
                tracing::Level::WARN,
                "another sift event"
            );
        });
        assert_eq!(
            count.load(Ordering::Relaxed),
            2,
            "both sift_stream events must reach the base dispatch"
        );
    }

    #[test]
    fn forwarding_layer_does_not_forward_non_sift_stream_events() {
        let count = Arc::new(AtomicU32::new(0));
        let base = tracing::Dispatch::new(EventCounter(count.clone()));
        let layer = DispatchForwardingLayer(base);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::dispatcher::with_default(&tracing::Dispatch::new(subscriber), || {
            tracing::event!(
                target: "h2::proto::connection",
                tracing::Level::TRACE,
                "h2 spam"
            );
            tracing::event!(target: "tonic::transport", tracing::Level::DEBUG, "tonic noise");
            tracing::event!(target: "hyper::client", tracing::Level::INFO, "hyper log");
        });
        assert_eq!(
            count.load(Ordering::Relaxed),
            0,
            "third-party library events must not reach the base dispatch"
        );
    }
}
