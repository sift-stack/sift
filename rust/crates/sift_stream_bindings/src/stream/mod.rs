pub mod builder;
pub mod channel;
pub mod config;
pub mod request;
pub mod retry;
pub mod time;

use crate::error::SiftErrorWrapper;
use crate::metrics::SiftStreamMetricsSnapshotPy;
use crate::stream::channel::ChannelValuePy;
use crate::stream::config::{FlowConfigPy, FlowDescriptorPy, RunSelectorPy};
use crate::stream::request::IngestWithConfigDataStreamRequestWrapperPy;
use crate::stream::time::TimeValuePy;
use pyo3::{prelude::*, types::PyIterator};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::*;
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use sift_stream::{
    FileBackup, Flow, FlowConfig, IngestionConfigEncoder, LiveStreamingOnly,
    LiveStreamingWithBackups, SiftStream,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// ──────────────────────────────────────────────────────────────────────────────
// Inner enum — holds any of the three concrete SiftStream transport types
// ──────────────────────────────────────────────────────────────────────────────

enum SiftStreamInner {
    LiveOnly(SiftStream<IngestionConfigEncoder, LiveStreamingOnly>),
    LiveWithBackups(SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>),
    FileBackup(SiftStream<IngestionConfigEncoder, FileBackup>),
}

// Dispatch over a shared (immutable) reference to the inner stream.
macro_rules! dispatch_ref {
    ($opt:expr, |$s:ident| $body:expr) => {
        match $opt.as_ref().ok_or_else(stream_consumed_err)? {
            SiftStreamInner::LiveOnly($s) => $body,
            SiftStreamInner::LiveWithBackups($s) => $body,
            SiftStreamInner::FileBackup($s) => $body,
        }
    };
}

// Dispatch over a mutable reference to the inner stream.
macro_rules! dispatch_mut {
    ($opt:expr, |$s:ident| $body:expr) => {
        match $opt.as_mut().ok_or_else(stream_consumed_err)? {
            SiftStreamInner::LiveOnly($s) => $body,
            SiftStreamInner::LiveWithBackups($s) => $body,
            SiftStreamInner::FileBackup($s) => $body,
        }
    };
}

fn stream_consumed_err() -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Stream has been consumed by finish()")
}

fn stream_finished_err() -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Stream has already been finished")
}

// ──────────────────────────────────────────────────────────────────────────────
// Type Definitions
// ──────────────────────────────────────────────────────────────────────────────

/// Python binding for [`SiftStream`](sift_stream::SiftStream).
///
/// This is a thin wrapper around the Rust `SiftStream` type. For detailed documentation,
/// see [`SiftStream`](sift_stream::SiftStream).
///
/// The Python binding provides the same functionality as the Rust type, with methods
/// adapted for Python's async/await syntax.
#[gen_stub_pyclass]
#[pyclass]
pub struct SiftStreamPy {
    inner: Arc<Mutex<Option<SiftStreamInner>>>,
}

/// Python binding for [`Flow`](sift_stream::Flow).
///
/// This is a thin wrapper around the Rust `Flow` type. For detailed documentation,
/// see [`Flow`](sift_stream::Flow).
///
/// A `Flow` represents a single telemetry message containing channel values that share
/// a common timestamp.
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone, Debug)]
pub struct FlowPy {
    inner: Flow,
}

// ──────────────────────────────────────────────────────────────────────────────
// Trait Implementations
// ──────────────────────────────────────────────────────────────────────────────

impl From<SiftStream<IngestionConfigEncoder, LiveStreamingOnly>> for SiftStreamPy {
    fn from(stream: SiftStream<IngestionConfigEncoder, LiveStreamingOnly>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(SiftStreamInner::LiveOnly(stream)))),
        }
    }
}

impl From<SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>> for SiftStreamPy {
    fn from(stream: SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(SiftStreamInner::LiveWithBackups(stream)))),
        }
    }
}

impl From<SiftStream<IngestionConfigEncoder, FileBackup>> for SiftStreamPy {
    fn from(stream: SiftStream<IngestionConfigEncoder, FileBackup>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(SiftStreamInner::FileBackup(stream)))),
        }
    }
}

impl From<FlowPy> for Flow {
    fn from(flow: FlowPy) -> Self {
        flow.inner
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Helper
// ──────────────────────────────────────────────────────────────────────────────

// All sift_stream send error types (SiftStreamSendError, SendError, TrySendError,
// SiftStreamTrySendError) intentionally omit the inner T from their Display output, so
// format!("{e}") is safe to use regardless of how large the undelivered message(s) may be.
fn py_err(e: impl std::fmt::Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}"))
}

// ──────────────────────────────────────────────────────────────────────────────
// PyO3 Method Implementations
// ──────────────────────────────────────────────────────────────────────────────

#[gen_stub_pymethods]
#[pymethods]
impl SiftStreamPy {
    pub fn send(&self, py: Python, flow: FlowPy) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let flow_rs: Flow = flow.into();
            dispatch_mut!(guard, |s| s.send(flow_rs).await.map_err(py_err))
        })?;

        Ok(awaitable.into())
    }

    // Accepts a Python iterable of FlowPy for more performant sending from Python.
    pub fn batch_send<'py>(
        &self,
        py: Python<'py>,
        flows: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        let flow_iter = PyIterator::from_object(flows)?;
        let mut flows_vec = Vec::new();
        for item in flow_iter {
            flows_vec.push(item?.extract::<FlowPy>()?);
        }

        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            for flow in flows_vec {
                let flow_rs: Flow = flow.into();
                dispatch_mut!(guard, |s| s.send(flow_rs).await.map_err(py_err))?;
            }
            Ok(())
        })?;

        Ok(awaitable.into())
    }

    pub fn send_requests(
        &self,
        py: Python,
        requests: Vec<request::IngestWithConfigDataStreamRequestPy>,
    ) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();
        let requests_rs: Vec<IngestWithConfigDataStreamRequest> =
            requests.into_iter().map(|r| r.into()).collect();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            dispatch_mut!(guard, |s| s
                .send_requests(requests_rs)
                .await
                .map_err(py_err))
        })?;

        Ok(awaitable.into())
    }

    pub fn try_send_requests(&self, flows: &Bound<'_, PyAny>) -> PyResult<()> {
        let flow_iter = PyIterator::from_object(flows)?;
        let mut flows_vec: Vec<IngestWithConfigDataStreamRequest> = Vec::new();
        for item in flow_iter {
            let request = item?.extract::<IngestWithConfigDataStreamRequestWrapperPy>()?;
            flows_vec.push(request.into());
        }

        let mut inner_guard = self.inner.blocking_lock();
        dispatch_mut!(inner_guard, |s| s
            .try_send_requests(flows_vec)
            .map_err(py_err))
    }

    pub fn get_metrics_snapshot(&self) -> PyResult<SiftStreamMetricsSnapshotPy> {
        let inner_guard = self.inner.blocking_lock();
        Ok(dispatch_ref!(inner_guard, |s| s
            .get_metrics_snapshot()
            .into()))
    }

    pub fn add_new_flows(
        &self,
        py: Python,
        flow_configs: Vec<FlowConfigPy>,
    ) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();
        let flow_configs_rs: Vec<FlowConfig> =
            flow_configs.into_iter().map(|cfg| cfg.into()).collect();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            dispatch_mut!(guard, |s| {
                s.add_new_flows(&flow_configs_rs)
                    .await
                    .map_err(|e| SiftErrorWrapper(e).into())
            })
        })?;

        Ok(awaitable.into())
    }

    pub fn get_flow_descriptor(&self, flow_name: &str) -> PyResult<FlowDescriptorPy> {
        let inner_guard = self.inner.blocking_lock();
        dispatch_ref!(inner_guard, |s| {
            s.get_flow_descriptor(flow_name)
                .map(FlowDescriptorPy::from)
                .map_err(|e| SiftErrorWrapper(e).into())
        })
    }

    pub fn get_flows(&self) -> PyResult<HashMap<String, FlowDescriptorPy>> {
        let inner_guard = self.inner.blocking_lock();
        Ok(dispatch_ref!(inner_guard, |s| {
            s.get_flows()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        }))
    }

    pub fn attach_run(&self, py: Python, run_selector: RunSelectorPy) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            dispatch_mut!(guard, |s| {
                s.attach_run(run_selector.into())
                    .await
                    .map_err(|e| SiftErrorWrapper(e).into())
            })
        })?;

        Ok(awaitable.into())
    }

    pub fn detach_run(&self) -> PyResult<()> {
        let mut inner_guard = self.inner.blocking_lock();
        dispatch_mut!(inner_guard, |s| s.detach_run());
        Ok(())
    }

    pub fn run(&self) -> PyResult<Option<String>> {
        let inner_guard = self.inner.blocking_lock();
        Ok(dispatch_ref!(inner_guard, |s| {
            s.run().map(|r| r.run_id.clone())
        }))
    }

    pub fn finish(&self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.take().ok_or_else(stream_finished_err)?;

            let result = match stream {
                SiftStreamInner::LiveOnly(s) => s.finish().await,
                SiftStreamInner::LiveWithBackups(s) => s.finish().await,
                SiftStreamInner::FileBackup(s) => s.finish().await,
            };

            result.map_err(|e| SiftErrorWrapper(e).into())
        })?;

        Ok(awaitable.into())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl FlowPy {
    #[new]
    pub fn new(flow_name: &str, timestamp: TimeValuePy, values: Vec<ChannelValuePy>) -> Self {
        Self {
            inner: Flow::new(
                flow_name,
                timestamp.into(),
                &values.into_iter().map(|v| v.into()).collect::<Vec<_>>(),
            ),
        }
    }
}
