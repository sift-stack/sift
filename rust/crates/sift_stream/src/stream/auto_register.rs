use crate::SiftStream;
use crate::stream::flow::FlowDescriptor;
use crate::stream::mode::ingestion_config::{Flow, IngestionConfigEncoder};
use crate::stream::run::RunSelector;
use crate::stream::send_error::SiftStreamSendError;
use crate::stream::{Encoder, MetricsSnapshot, Transport};
use async_trait::async_trait;
use sift_error::prelude::{Error as SiftError, Result};
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use sift_rs::runs::v2::Run;
use std::collections::HashMap;
use std::fmt;

fn flow_config_from_flow(flow: &Flow) -> FlowConfig {
    FlowConfig {
        name: flow.flow_name.clone(),
        channels: flow
            .values
            .iter()
            .map(|cv| ChannelConfig {
                name: cv.name.clone(),
                data_type: cv.value.pb_data_type().into(),
                ..Default::default()
            })
            .collect(),
    }
}

/// Returns `Err` with a human-readable description if the channel names or data types in
/// `config` do not match those declared by `flow`. Comparison is set-based (order-independent).
fn validate_staged_config(flow: &Flow, config: &FlowConfig) -> std::result::Result<(), String> {
    let flow_channels: HashMap<&str, i32> = flow
        .values
        .iter()
        .map(|cv| (cv.name.as_str(), cv.value.pb_data_type().into()))
        .collect();

    let config_channels: HashMap<&str, i32> = config
        .channels
        .iter()
        .map(|ch| (ch.name.as_str(), ch.data_type))
        .collect();

    if flow_channels == config_channels {
        return Ok(());
    }

    let mut problems: Vec<String> = Vec::new();

    for (name, flow_type) in &flow_channels {
        match config_channels.get(name) {
            None => problems.push(format!(
                "channel '{name}' present in flow but missing from staged config"
            )),
            Some(config_type) if config_type != flow_type => problems.push(format!(
                "channel '{name}' data type mismatch (flow: {flow_type}, staged: {config_type})"
            )),
            _ => {}
        }
    }
    for name in config_channels.keys() {
        if !flow_channels.contains_key(name) {
            problems.push(format!(
                "channel '{name}' present in staged config but missing from flow"
            ));
        }
    }

    Err(format!(
        "flow '{}': {}",
        flow.flow_name,
        problems.join("; ")
    ))
}

/// Returned by [`SiftStreamAutoRegister::send`] when delivery fails.
#[derive(Debug)]
pub enum AutoRegisterSendError<T> {
    /// Flow registration with Sift failed before the send was attempted.
    RegistrationFailed(SiftError),
    /// The underlying stream send failed after registration succeeded.
    StreamError(SiftStreamSendError<T>),
    /// A staged [`FlowConfig`] was found for the flow but its channels do not match.
    StagedConfigMismatch(String),
}

impl<T: fmt::Debug> fmt::Display for AutoRegisterSendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistrationFailed(e) => write!(f, "flow registration failed: {e}"),
            Self::StreamError(e) => write!(f, "{e}"),
            Self::StagedConfigMismatch(msg) => write!(f, "staged config mismatch: {msg}"),
        }
    }
}

impl<T: fmt::Debug> std::error::Error for AutoRegisterSendError<T> {}

/// Trait for an auto-registering Sift stream.
///
/// Implement this trait to create mock implementations for testing without a live connection to
/// Sift. [`SiftStreamAutoRegister`] is the production implementation.
///
/// `finish` carries a `where Self: Sized` bound, which means it cannot be called through a
/// `dyn AutoRegisterStream` trait object. Use generic bounds (`impl AutoRegisterStream` or
/// `T: AutoRegisterStream`) instead.
#[async_trait]
pub trait AutoRegisterStream {
    /// Error type returned by [`send`](Self::send).
    type SendError: std::error::Error + Send + Sync + 'static;

    /// Send a flow, auto-registering it with Sift if not already in the local cache.
    ///
    /// On the first call for a given `flow_name`, the flow is registered before the message is
    /// sent. Subsequent calls for the same flow skip registration entirely.
    ///
    /// If a staged [`FlowConfig`] was provided for this flow at construction time, it is used
    /// for registration (preserving descriptions, units, and other metadata). The staged config
    /// is validated against the flow's channel names and data types before use — if they do not
    /// match, [`AutoRegisterSendError::StagedConfigMismatch`] is returned and the staged config
    /// is retained for the next attempt. If no staged config exists, a minimal config is derived
    /// from the flow itself.
    ///
    /// # Errors
    ///
    /// - [`AutoRegisterSendError::StagedConfigMismatch`] — a staged config was found but its
    ///   channels do not match the provided flow. The flow was not sent.
    /// - [`AutoRegisterSendError::RegistrationFailed`] — the Sift API call to register the
    ///   flow failed. The flow was not sent.
    /// - [`AutoRegisterSendError::StreamError`] — registration succeeded but the underlying
    ///   channel send failed (encode error or channel closed).
    async fn send(&mut self, flow: Flow) -> std::result::Result<(), Self::SendError>;

    /// Drain remaining data and shut down the stream.
    ///
    /// Must be called when ingestion is complete to avoid data loss.
    async fn finish(self) -> Result<()>
    where
        Self: Sized;

    /// Get the flow descriptor for a given flow name from the local cache.
    ///
    /// Returns `Err` if the flow has not yet been registered (either via a prior `send` call or
    /// during stream initialization).
    fn get_flow_descriptor(&self, flow_name: &str) -> Result<FlowDescriptor<String>>;

    /// Attach a run to the stream. Data sent after this call will be associated with the run.
    async fn attach_run(&mut self, run_selector: RunSelector) -> Result<()>;

    /// Detach the run currently associated with the stream, if any.
    fn detach_run(&mut self);

    /// Return the attached run, if one exists.
    fn run(&self) -> Option<&Run>;
}

/// Convenience wrapper around [`SiftStream<IngestionConfigEncoder, T>`] that auto-registers
/// flows on first `send`.
///
/// The trade-off: `send` may incur one round-trip to Sift when it encounters a flow for the
/// first time. Subsequent sends for the same flow are cache-hits and have no extra overhead.
///
/// Staged [`FlowConfig`]s can be provided at construction time via [`Self::new`]. When a staged
/// config exists for a flow, it is used for registration (preserving descriptions, units, and
/// other metadata) instead of a minimal derived config. The staged config is consumed after
/// successful registration.
pub struct SiftStreamAutoRegister<T>
where
    T: Transport<Encoder = IngestionConfigEncoder>,
{
    inner: SiftStream<IngestionConfigEncoder, T>,
    staged_configs: HashMap<String, FlowConfig>,
}

impl<T> SiftStreamAutoRegister<T>
where
    T: Transport<Encoder = IngestionConfigEncoder, Message = IngestWithConfigDataStreamRequest>,
    IngestionConfigEncoder: Encoder<Message = T::Message> + MetricsSnapshot,
{
    /// Wrap an existing `SiftStream`, optionally providing pre-built [`FlowConfig`]s to use
    /// during registration.
    ///
    /// Each entry in `staged_configs` is keyed by its `name` field. When `send` encounters an
    /// unregistered flow whose name matches a staged config, that config is used for
    /// registration instead of a minimal derived one, and then removed from the staging cache.
    /// Pass an empty `Vec` when no staged configs are needed.
    pub fn new(
        stream: SiftStream<IngestionConfigEncoder, T>,
        staged_configs: Vec<FlowConfig>,
    ) -> Self {
        Self {
            inner: stream,
            staged_configs: staged_configs
                .into_iter()
                .map(|c| (c.name.clone(), c))
                .collect(),
        }
    }

    #[cfg(feature = "metrics-unstable")]
    /// Retrieve a snapshot of the current stream metrics.
    pub fn get_metrics_snapshot(&self) -> crate::metrics::SiftStreamMetricsSnapshot {
        self.inner.get_metrics_snapshot()
    }

    /// Consume the wrapper and return the inner [`SiftStream`].
    pub fn into_inner(self) -> SiftStream<IngestionConfigEncoder, T> {
        self.inner
    }
}

#[async_trait]
impl<T> AutoRegisterStream for SiftStreamAutoRegister<T>
where
    T: Transport<Encoder = IngestionConfigEncoder, Message = IngestWithConfigDataStreamRequest>
        + Send,
    IngestionConfigEncoder: Encoder<Message = T::Message> + MetricsSnapshot,
    T::Message: Send,
{
    type SendError = AutoRegisterSendError<T::Message>;

    async fn send(&mut self, flow: Flow) -> std::result::Result<(), Self::SendError> {
        if self.inner.get_flow_descriptor(&flow.flow_name).is_err() {
            // If there is a staged config, validate it against the flow and then
            // register it. Otherwise, create a minimal flow config and register that.
            let flow_config = if let Some(staged) = self.staged_configs.get(&flow.flow_name) {
                validate_staged_config(&flow, staged)
                    .map_err(AutoRegisterSendError::StagedConfigMismatch)?;
                std::slice::from_ref(staged)
            } else {
                &[flow_config_from_flow(&flow)]
            };

            self.inner
                .add_new_flows(flow_config)
                .await
                .map_err(AutoRegisterSendError::RegistrationFailed)?;

            // Remove staged config after successful registration.
            self.staged_configs.remove(&flow.flow_name);
        }
        self.inner
            .send(flow)
            .await
            .map_err(AutoRegisterSendError::StreamError)
    }

    async fn finish(self) -> Result<()>
    where
        Self: Sized,
    {
        self.inner.finish().await
    }

    fn get_flow_descriptor(&self, flow_name: &str) -> Result<FlowDescriptor<String>> {
        self.inner.get_flow_descriptor(flow_name)
    }

    async fn attach_run(&mut self, run_selector: RunSelector) -> Result<()> {
        self.inner.attach_run(run_selector).await
    }

    fn detach_run(&mut self) {
        self.inner.detach_run()
    }

    fn run(&self) -> Option<&Run> {
        self.inner.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::channel::ChannelValue;
    use crate::stream::time::TimeValue;
    use sift_error::prelude::{Error as SiftError, ErrorKind};
    use sift_rs::common::r#type::v1::ChannelDataType;

    #[test]
    fn flow_config_from_flow_derives_channel_names_and_types() {
        let flow = Flow::new(
            "sensor-data",
            TimeValue::default(),
            &[
                ChannelValue::new("velocity", 9.8_f64),
                ChannelValue::new("temp", 72.1_f32),
                ChannelValue::new("enabled", true),
                ChannelValue::new("count", 42_i32),
            ],
        );

        let config = flow_config_from_flow(&flow);

        assert_eq!(config.name, "sensor-data");
        assert_eq!(config.channels.len(), 4);

        let expected = [
            ("velocity", ChannelDataType::Double),
            ("temp", ChannelDataType::Float),
            ("enabled", ChannelDataType::Bool),
            ("count", ChannelDataType::Int32),
        ];
        for (ch, (name, dt)) in config.channels.iter().zip(expected.iter()) {
            assert_eq!(&ch.name, name);
            assert_eq!(ch.data_type, *dt as i32);
        }
    }

    #[test]
    fn flow_config_from_flow_all_value_types() {
        use crate::stream::channel::ChannelEnum;
        let flow = Flow::new(
            "all-types",
            TimeValue::default(),
            &[
                ChannelValue::new("f64", 1.0_f64),
                ChannelValue::new("f32", 1.0_f32),
                ChannelValue::new("i32", 1_i32),
                ChannelValue::new("i64", 1_i64),
                ChannelValue::new("u32", 1_u32),
                ChannelValue::new("u64", 1_u64),
                ChannelValue::new("bool", true),
                ChannelValue::new("str", "hello"),
                ChannelValue::new("enum", ChannelEnum(0)),
                ChannelValue::new("bits", vec![0u8, 1u8]),
            ],
        );

        let config = flow_config_from_flow(&flow);

        let expected_types = [
            ChannelDataType::Double,
            ChannelDataType::Float,
            ChannelDataType::Int32,
            ChannelDataType::Int64,
            ChannelDataType::Uint32,
            ChannelDataType::Uint64,
            ChannelDataType::Bool,
            ChannelDataType::String,
            ChannelDataType::Enum,
            ChannelDataType::BitField,
        ];
        for (ch, dt) in config.channels.iter().zip(expected_types.iter()) {
            assert_eq!(ch.data_type, *dt as i32, "mismatch for channel {}", ch.name);
        }
    }

    #[test]
    fn flow_config_from_flow_empty_values_produces_empty_channels() {
        let flow = Flow::new("empty", TimeValue::default(), &[]);
        let config = flow_config_from_flow(&flow);
        assert_eq!(config.name, "empty");
        assert!(config.channels.is_empty());
    }

    #[test]
    fn flow_config_from_flow_leaves_unit_and_description_empty() {
        let flow = Flow::new(
            "test",
            TimeValue::default(),
            &[ChannelValue::new("ch", 1.0_f64)],
        );
        let config = flow_config_from_flow(&flow);
        assert!(config.channels[0].unit.is_empty());
        assert!(config.channels[0].description.is_empty());
    }

    #[test]
    fn auto_register_send_error_registration_failed_display() {
        let err: AutoRegisterSendError<()> = AutoRegisterSendError::RegistrationFailed(
            SiftError::new_msg(ErrorKind::GeneralError, "network timeout"),
        );
        let msg = err.to_string();
        assert!(msg.contains("flow registration failed"), "got: {msg}");
        assert!(msg.contains("network timeout"), "got: {msg}");
    }

    #[test]
    fn auto_register_send_error_stream_error_display() {
        let err: AutoRegisterSendError<u32> =
            AutoRegisterSendError::StreamError(SiftStreamSendError::ChannelClosed(0));
        let msg = err.to_string();
        assert!(msg.contains("channel closed"), "got: {msg}");
    }

    #[test]
    fn auto_register_send_error_implements_std_error() {
        fn assert_std_error<E: std::error::Error>(_: &E) {}
        let err: AutoRegisterSendError<u32> = AutoRegisterSendError::RegistrationFailed(
            SiftError::new_msg(ErrorKind::GeneralError, "x"),
        );
        assert_std_error(&err);
    }

    #[test]
    fn auto_register_send_error_debug() {
        let err: AutoRegisterSendError<u32> = AutoRegisterSendError::RegistrationFailed(
            SiftError::new_msg(ErrorKind::GeneralError, "x"),
        );
        assert!(format!("{:?}", err).contains("RegistrationFailed"));

        let err2: AutoRegisterSendError<u32> =
            AutoRegisterSendError::StreamError(SiftStreamSendError::ChannelClosed(0));
        assert!(format!("{:?}", err2).contains("StreamError"));
    }
}
