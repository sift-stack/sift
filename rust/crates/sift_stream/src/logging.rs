/// Log level for captured tracing events.
///
/// Decoupled from `tracing::Level` so that core logic does not require the `tracing` feature.
/// Ordering: higher discriminant = more verbose. A filter of `Info` passes events where
/// `event_level <= LogLevel::Info`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(u8)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    #[default]
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl LogLevel {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }
}

#[cfg(feature = "tracing")]
impl From<&tracing::Level> for LogLevel {
    fn from(l: &tracing::Level) -> Self {
        match *l {
            tracing::Level::ERROR => Self::Error,
            tracing::Level::WARN => Self::Warn,
            tracing::Level::INFO => Self::Info,
            tracing::Level::DEBUG => Self::Debug,
            tracing::Level::TRACE => Self::Trace,
        }
    }
}

/// A captured log event from the scoped tracing dispatch.
///
/// `file` and `line` come from tracing callsite metadata — compile-time constants that
/// are guaranteed to point to the original call-site. Used for deduplication only.
pub(crate) struct LogEvent {
    pub level: LogLevel,
    /// The target module path of the event (e.g. `sift_stream::stream::tasks::ingestion`).
    pub target: &'static str,
    /// Source file name from the callsite (empty string if unavailable).
    pub file: &'static str,
    /// Source line number from the callsite (0 if unavailable).
    pub line: u32,
    /// The formatted event message.
    pub message: String,
    /// Additional structured key-value fields beyond `message`.
    pub fields: Vec<(String, String)>,
    /// Wall-clock timestamp when the event was captured.
    /// Stored for potential future use (e.g. as a flow timestamp channel).
    #[allow(dead_code)]
    pub timestamp: std::time::SystemTime,
}

/// Output from [`LogDeduplicator::process`].
pub(crate) enum DeduplicatorOutput {
    /// Event is a repeat of the previous one; suppressed.
    Suppress,
    /// A single event to emit.
    Emit(LogEvent),
    /// Emit the pending repeat-count summary first, then the new event.
    EmitSummaryThenEmit(LogEvent, LogEvent),
}

/// Tracks consecutive repeated log events (same file + line) and collapses them.
///
/// Lives as local mutable state inside `MetricsStreamingTask::run()` — single-threaded,
/// no mutex required.
#[derive(Default)]
pub(crate) struct LogDeduplicator {
    last_file: &'static str,
    last_line: u32,
    last_level: LogLevel,
    last_target: &'static str,
    repeat_count: u64,
}

impl LogDeduplicator {
    /// Process an incoming event. Returns what should be forwarded to Sift.
    ///
    /// Pointer equality is valid for `&'static str` dedup: strings from the same callsite
    /// in the same binary share their static storage address.
    pub(crate) fn process(&mut self, event: LogEvent) -> DeduplicatorOutput {
        let is_repeat = std::ptr::eq(event.file, self.last_file) && event.line == self.last_line;

        if is_repeat {
            self.repeat_count += 1;
            return DeduplicatorOutput::Suppress;
        }

        let summary = (self.repeat_count > 0).then(|| LogEvent {
            level: self.last_level,
            target: self.last_target,
            file: self.last_file,
            line: self.last_line,
            message: format!(
                "(previous message repeated {} additional time(s))",
                self.repeat_count
            ),
            fields: vec![],
            timestamp: std::time::SystemTime::now(),
        });

        self.last_file = event.file;
        self.last_line = event.line;
        self.last_level = event.level;
        self.last_target = event.target;
        self.repeat_count = 0;

        match summary {
            Some(s) => DeduplicatorOutput::EmitSummaryThenEmit(s, event),
            None => DeduplicatorOutput::Emit(event),
        }
    }

    /// Flush any pending repeat-count summary (call at shutdown).
    pub(crate) fn flush(&mut self) -> Option<LogEvent> {
        (self.repeat_count > 0).then(|| {
            let e = LogEvent {
                level: self.last_level,
                target: self.last_target,
                file: self.last_file,
                line: self.last_line,
                message: format!(
                    "(previous message repeated {} additional time(s))",
                    self.repeat_count
                ),
                fields: vec![],
                timestamp: std::time::SystemTime::now(),
            };
            self.repeat_count = 0;
            e
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Two distinct static string addresses to simulate different source file paths.
    // Using clearly different content ensures the compiler cannot share their storage.
    static FILE_A: &str = "crate/src/tasks/module_a.rs";
    static FILE_B: &str = "crate/src/tasks/module_b.rs";

    fn event(file: &'static str, line: u32, level: LogLevel, msg: &str) -> LogEvent {
        LogEvent {
            level,
            target: "sift_stream::test",
            file,
            line,
            message: msg.to_owned(),
            fields: vec![],
            timestamp: std::time::SystemTime::now(),
        }
    }

    #[test]
    fn log_level_ordering() {
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Trace);
    }

    #[test]
    fn log_level_as_str() {
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Trace.as_str(), "TRACE");
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn log_level_from_tracing_level() {
        assert_eq!(LogLevel::from(&tracing::Level::ERROR), LogLevel::Error);
        assert_eq!(LogLevel::from(&tracing::Level::WARN), LogLevel::Warn);
        assert_eq!(LogLevel::from(&tracing::Level::INFO), LogLevel::Info);
        assert_eq!(LogLevel::from(&tracing::Level::DEBUG), LogLevel::Debug);
        assert_eq!(LogLevel::from(&tracing::Level::TRACE), LogLevel::Trace);
    }

    #[test]
    fn dedup_first_event_is_emitted() {
        let mut d = LogDeduplicator::default();
        assert!(matches!(
            d.process(event(FILE_A, 10, LogLevel::Info, "msg")),
            DeduplicatorOutput::Emit(_)
        ));
    }

    #[test]
    fn dedup_repeat_at_same_callsite_is_suppressed() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        assert!(matches!(
            d.process(event(FILE_A, 10, LogLevel::Info, "msg")),
            DeduplicatorOutput::Suppress
        ));
    }

    #[test]
    fn dedup_multiple_consecutive_repeats_are_all_suppressed() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        for _ in 0..5 {
            assert!(matches!(
                d.process(event(FILE_A, 10, LogLevel::Info, "msg")),
                DeduplicatorOutput::Suppress
            ));
        }
    }

    #[test]
    fn dedup_different_line_emits_without_summary() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "first"));
        // No repeats accumulated, so no summary is needed.
        assert!(matches!(
            d.process(event(FILE_A, 20, LogLevel::Info, "second")),
            DeduplicatorOutput::Emit(_)
        ));
    }

    #[test]
    fn dedup_different_file_emits_without_summary() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "first"));
        assert!(matches!(
            d.process(event(FILE_B, 10, LogLevel::Info, "second")),
            DeduplicatorOutput::Emit(_)
        ));
    }

    #[test]
    fn dedup_new_callsite_after_repeats_emits_summary_then_event() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg")); // suppressed ×1
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg")); // suppressed ×2
        assert!(matches!(
            d.process(event(FILE_B, 5, LogLevel::Warn, "new")),
            DeduplicatorOutput::EmitSummaryThenEmit(_, _)
        ));
    }

    #[test]
    fn dedup_summary_mentions_correct_repeat_count() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg")); // +1
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg")); // +2
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg")); // +3
        match d.process(event(FILE_B, 5, LogLevel::Info, "new")) {
            DeduplicatorOutput::EmitSummaryThenEmit(summary, new_event) => {
                assert!(
                    summary.message.contains("3"),
                    "summary should mention 3 repeats, got: {:?}",
                    summary.message
                );
                assert_eq!(new_event.message, "new");
            }
            _ => panic!("expected EmitSummaryThenEmit, got a different variant"),
        }
    }

    #[test]
    fn dedup_flush_on_fresh_state_returns_none() {
        let mut d = LogDeduplicator::default();
        assert!(d.flush().is_none());
    }

    #[test]
    fn dedup_flush_after_single_emit_returns_none() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        // One event emitted, no pending repeats.
        assert!(d.flush().is_none());
    }

    #[test]
    fn dedup_flush_with_pending_repeats_returns_summary() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg")); // suppressed
        let summary = d.flush().expect("expected a summary");
        assert!(
            summary.message.contains("1"),
            "summary should mention 1 repeat, got: {:?}",
            summary.message
        );
    }

    #[test]
    fn dedup_flush_resets_repeat_count() {
        let mut d = LogDeduplicator::default();
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        let _ = d.process(event(FILE_A, 10, LogLevel::Info, "msg"));
        let _ = d.flush();
        // Second flush should find nothing pending.
        assert!(d.flush().is_none());
    }
}
