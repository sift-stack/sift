use super::{FlowPy, SiftStreamInner, SiftStreamPy, stream_consumed_err};
use crate::error::SiftErrorWrapper;
use crate::metrics::SiftStreamMetricsSnapshotPy;
use crate::stream::config::{FlowConfigPy, FlowDescriptorPy, RunSelectorPy};
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::*;
use sift_stream::{
    AutoRegisterStream, FileBackup, Flow, LiveStreamingOnly, LiveStreamingWithBackups,
    SiftStreamAutoRegister,
};
use std::sync::Arc;
use tokio::sync::Mutex;

enum SiftStreamAutoRegisterInner {
    LiveOnly(SiftStreamAutoRegister<LiveStreamingOnly>),
    LiveWithBackups(SiftStreamAutoRegister<LiveStreamingWithBackups>),
    FileBackup(SiftStreamAutoRegister<FileBackup>),
}

macro_rules! dispatch_auto_mut {
    ($opt:expr, |$s:ident| $body:expr) => {
        match $opt.as_mut().ok_or_else(stream_consumed_err)? {
            SiftStreamAutoRegisterInner::LiveOnly($s) => $body,
            SiftStreamAutoRegisterInner::LiveWithBackups($s) => $body,
            SiftStreamAutoRegisterInner::FileBackup($s) => $body,
        }
    };
}

macro_rules! dispatch_auto_ref {
    ($opt:expr, |$s:ident| $body:expr) => {
        match $opt.as_ref().ok_or_else(stream_consumed_err)? {
            SiftStreamAutoRegisterInner::LiveOnly($s) => $body,
            SiftStreamAutoRegisterInner::LiveWithBackups($s) => $body,
            SiftStreamAutoRegisterInner::FileBackup($s) => $body,
        }
    };
}

fn auto_register_err(e: impl std::fmt::Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}"))
}

/// Python binding for [`SiftStreamAutoRegister`](sift_stream::SiftStreamAutoRegister).
///
/// Convenience wrapper that auto-registers flows on first `send`.
///
/// Construct via `SiftStreamAutoRegisterPy.from_stream(stream)`.
#[gen_stub_pyclass]
#[pyclass]
pub struct SiftStreamAutoRegisterPy {
    inner: Arc<Mutex<Option<SiftStreamAutoRegisterInner>>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl SiftStreamAutoRegisterPy {
    /// Construct from an existing `SiftStreamPy`, consuming it.
    ///
    /// After this call, the original `stream` object is in a consumed state and
    /// calling any method on it will raise `RuntimeError`.
    ///
    /// `staged_configs` is an optional list of `FlowConfig` objects. When a staged
    /// config exists for a flow, it is used for registration instead of a minimal
    /// derived config, then removed after successful registration.
    #[staticmethod]
    #[pyo3(signature = (stream, staged_configs = None))]
    pub fn from_stream(
        py: Python,
        stream: &SiftStreamPy,
        staged_configs: Option<Vec<FlowConfigPy>>,
    ) -> PyResult<Py<PyAny>> {
        let stream_inner = stream.inner.clone();
        let staged: Vec<sift_stream::FlowConfig> = staged_configs
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        let awaitable = future_into_py(py, async move {
            let mut guard = stream_inner.lock().await;
            let inner = guard.take().ok_or_else(stream_consumed_err)?;

            let auto_inner = match inner {
                SiftStreamInner::LiveOnly(s) => {
                    SiftStreamAutoRegisterInner::LiveOnly(SiftStreamAutoRegister::new(s, staged))
                }
                SiftStreamInner::LiveWithBackups(s) => {
                    SiftStreamAutoRegisterInner::LiveWithBackups(SiftStreamAutoRegister::new(
                        s, staged,
                    ))
                }
                SiftStreamInner::FileBackup(s) => {
                    SiftStreamAutoRegisterInner::FileBackup(SiftStreamAutoRegister::new(s, staged))
                }
            };

            Ok(SiftStreamAutoRegisterPy {
                inner: Arc::new(Mutex::new(Some(auto_inner))),
            })
        })?;

        Ok(awaitable.into())
    }

    /// Send a flow, auto-registering it with Sift if not already cached.
    ///
    /// On the first call for a given flow name, a `FlowConfig` is derived from the `Flow`
    /// using each channel's name and data type. Subsequent sends for the same flow are
    /// cache-hits with no overhead.
    ///
    /// Raises `RuntimeError` if flow registration or the underlying send fails.
    pub fn send(&self, py: Python, flow: FlowPy) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();
        let flow_rs: Flow = flow.into();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            dispatch_auto_mut!(guard, |s| {
                s.send(flow_rs).await.map_err(auto_register_err)
            })
        })?;

        Ok(awaitable.into())
    }

    /// Drain remaining data and shut down the stream.
    ///
    /// Must be called when ingestion is complete. After this call, all other methods
    /// will raise `RuntimeError`.
    pub fn finish(&self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.take().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has already been finished",
                )
            })?;

            let result = match stream {
                SiftStreamAutoRegisterInner::LiveOnly(s) => s.finish().await,
                SiftStreamAutoRegisterInner::LiveWithBackups(s) => s.finish().await,
                SiftStreamAutoRegisterInner::FileBackup(s) => s.finish().await,
            };

            result.map_err(auto_register_err)
        })?;

        Ok(awaitable.into())
    }

    /// Retrieve the flow descriptor for a given flow name from the local cache.
    ///
    /// Raises `RuntimeError` if the flow has not been registered yet.
    pub fn get_flow_descriptor(&self, flow_name: &str) -> PyResult<FlowDescriptorPy> {
        let inner_guard = self.inner.blocking_lock();
        dispatch_auto_ref!(inner_guard, |s| {
            s.get_flow_descriptor(flow_name)
                .map(FlowDescriptorPy::from)
                .map_err(|e| SiftErrorWrapper(e).into())
        })
    }

    /// Attach a run to the stream.
    ///
    /// Data sent after this call will be associated with the specified run.
    pub fn attach_run(&self, py: Python, run_selector: RunSelectorPy) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            dispatch_auto_mut!(guard, |s| {
                s.attach_run(run_selector.into())
                    .await
                    .map_err(|e| SiftErrorWrapper(e).into())
            })
        })?;

        Ok(awaitable.into())
    }

    /// Detach the run, if any, currently associated with the stream.
    pub fn detach_run(&self) -> PyResult<()> {
        let mut inner_guard = self.inner.blocking_lock();
        dispatch_auto_mut!(inner_guard, |s| s.detach_run());
        Ok(())
    }

    /// Return the ID of the attached run, or `None` if no run is attached.
    pub fn run(&self) -> PyResult<Option<String>> {
        let inner_guard = self.inner.blocking_lock();
        Ok(dispatch_auto_ref!(inner_guard, |s| {
            s.run().map(|r| r.run_id.clone())
        }))
    }

    /// Retrieve a snapshot of the current stream metrics.
    pub fn get_metrics_snapshot(&self) -> PyResult<SiftStreamMetricsSnapshotPy> {
        let inner_guard = self.inner.blocking_lock();
        Ok(dispatch_auto_ref!(inner_guard, |s| {
            s.get_metrics_snapshot().into()
        }))
    }
}
