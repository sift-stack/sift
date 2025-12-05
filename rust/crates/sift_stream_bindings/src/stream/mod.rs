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
use sift_stream::{Flow, FlowConfig, IngestionConfigEncoder, LiveStreaming, SiftStream};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
pub struct SiftStreamPy {
    inner: Arc<Mutex<Option<SiftStream<IngestionConfigEncoder, LiveStreaming>>>>,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct FlowPy {
    inner: Flow,
}

// Trait Implementations
impl From<SiftStream<IngestionConfigEncoder, LiveStreaming>> for SiftStreamPy {
    fn from(stream: SiftStream<IngestionConfigEncoder, LiveStreaming>) -> Self {
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

            let flow_rs: Flow = flow.into();
            match stream.send(flow_rs).await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;

        Ok(awaitable.into())
    }

    // Function to take in a python iterable of PyFlow and call send on each item
    // Can allow more performant sending of data from python to SiftStream
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
            let stream = guard.as_mut().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream has been consumed by finish()",
                )
            })?;

            for flow in flows_vec {
                let flow_rs: Flow = flow.into();
                match stream.send(flow_rs).await {
                    Ok(_) => (),
                    Err(e) => return Err(SiftErrorWrapper(e).into()),
                }
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

    pub fn send_requests_nonblocking(&self, flows: &Bound<'_, PyAny>) -> PyResult<()> {
        let flow_iter = PyIterator::from_object(flows)?;
        let mut flows_vec: Vec<IngestWithConfigDataStreamRequest> = Vec::new();
        for item in flow_iter {
            let request = item?.extract::<IngestWithConfigDataStreamRequestWrapperPy>()?;
            flows_vec.push(request.into());
        }

        let mut inner_guard = self.inner.blocking_lock();
        let stream = inner_guard.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream has been consumed by finish()",
            )
        })?;

        stream
            .send_requests_nonblocking(flows_vec)
            .map_err(|e| SiftErrorWrapper(e).into())
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

    pub fn get_flow_descriptor(&self, flow_name: &str) -> PyResult<FlowDescriptorPy> {
        let inner_guard = self.inner.blocking_lock();
        let stream = inner_guard.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream has been consumed by finish()",
            )
        })?;
        match stream.get_flow_descriptor(flow_name) {
            Ok(descriptor) => Ok(FlowDescriptorPy::from(descriptor)),
            Err(e) => Err(SiftErrorWrapper(e).into()),
        }
    }

    pub fn get_flows(&self) -> PyResult<HashMap<String, FlowDescriptorPy>> {
        let inner_guard = self.inner.blocking_lock();
        let sift_stream = inner_guard.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream has been consumed by finish()",
            )
        })?;
        Ok(sift_stream
            .get_flows()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect())
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
