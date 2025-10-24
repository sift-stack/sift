pub mod builder;
pub mod channel;
pub mod config;
pub mod request;
pub mod retry;
pub mod time;

use crate::error::SiftErrorWrapper;
use crate::metrics::SiftStreamMetricsSnapshotPy;
use crate::stream::channel::ChannelValuePy;
use crate::stream::config::{FlowConfigPy, RunSelectorPy};
use crate::stream::time::TimeValuePy;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::*;
use sift_stream::{Flow, FlowConfig, IngestionConfigMode, SiftStream};
use std::sync::Arc;
use tokio::sync::Mutex;

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
pub struct SiftStreamPy {
    inner: Arc<Mutex<Option<SiftStream<IngestionConfigMode>>>>,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct FlowPy {
    inner: Flow,
}

// Trait Implementations
impl From<SiftStream<IngestionConfigMode>> for SiftStreamPy {
    fn from(stream: SiftStream<IngestionConfigMode>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(stream))),
        }
    }
}

impl From<FlowPy> for Flow {
    fn from(flow: FlowPy) -> Self {
        flow.inner
    }
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl SiftStreamPy {
    pub fn send(&self, py: Python, flow: FlowPy) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.as_mut().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has been consumed by finish()",
                )
            })?;

            match stream.send(flow.into()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;

        Ok(awaitable.into())
    }

    pub fn send_requests(
        &self,
        py: Python,
        requests: Vec<request::IngestWithConfigDataStreamRequestPy>,
    ) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let requests = requests.into_iter().map(|req| req.into());

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.as_mut().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has been consumed by finish()",
                )
            })?;

            match stream.send_requests(requests).await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;

        Ok(awaitable.into())
    }

    pub fn get_metrics_snapshot(&self) -> PyResult<SiftStreamMetricsSnapshotPy> {
        let inner_guard = self.inner.blocking_lock();
        let stream = inner_guard.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream has been consumed by finish()",
            )
        })?;
        Ok(stream.get_metrics_snapshot().into())
    }

    pub fn add_new_flows(
        &self,
        py: Python,
        flow_configs: Vec<FlowConfigPy>,
    ) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let flow_configs = flow_configs
            .into_iter()
            .map(|cfg| cfg.into())
            .collect::<Vec<FlowConfig>>();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.as_mut().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has been consumed by finish()",
                )
            })?;

            match stream.add_new_flows(&flow_configs).await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;

        Ok(awaitable.into())
    }

    pub fn attach_run(&self, py: Python, run_selector: RunSelectorPy) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.as_mut().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has been consumed by finish()",
                )
            })?;

            match stream.attach_run(run_selector.into()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;

        Ok(awaitable.into())
    }

    pub fn detach_run(&self) -> PyResult<()> {
        let mut inner_guard = self.inner.blocking_lock();
        let stream = inner_guard.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream has been consumed by finish()",
            )
        })?;
        stream.detach_run();
        Ok(())
    }

    pub fn run(&self) -> PyResult<Option<String>> {
        let inner_guard = self.inner.blocking_lock();
        let stream = inner_guard.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream has been consumed by finish()",
            )
        })?;
        Ok(stream.run().map(|r| r.run_id.clone()))
    }

    pub fn finish(&self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = self.inner.clone();

        let awaitable = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let stream = guard.take().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has already been finished",
                )
            })?;

            match stream.finish().await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
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
