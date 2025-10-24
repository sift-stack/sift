pub mod builder;
pub mod channel;
pub mod config;
pub mod request;
pub mod retry;
pub mod time;

use crate::stream::channel::ChannelValuePy;
use crate::stream::config::{FlowConfigPy, RunSelectorPy};
use crate::stream::time::TimeValuePy;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_stream::{Flow, IngestionConfigMode, SiftStream};
use std::sync::Arc;
use std::sync::Mutex;

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

impl From<SiftStreamPy> for SiftStream<IngestionConfigMode> {
    fn from(stream: SiftStreamPy) -> Self {
        match stream.inner.lock() {
            Ok(mut guard) => match guard.take() {
                Some(stream) => stream,
                None => panic!("SiftStreamPy inner was None"),
            },
            Err(_) => panic!("Failed to acquire lock on SiftStreamPy inner"),
        }
    }
}

impl From<FlowPy> for Flow {
    fn from(flow: FlowPy) -> Self {
        flow.inner
    }
}
// TODO: Fix impl
// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl SiftStreamPy {
    pub fn send(&mut self, py: Python, flow: FlowPy) -> PyResult<Py<PyAny>> {
        let mut inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        let mut inner = match inner_guard.take() {
            Some(stream) => stream,
            None => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream was already consumed",
                ));
            }
        };

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.send(flow.into()).await {
                Ok(_) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(inner))),
                }),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    pub fn send_requests(
        &mut self,
        py: Python,
        requests: Vec<request::IngestWithConfigDataStreamRequestPy>,
    ) -> PyResult<Py<PyAny>> {
        let mut inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        let mut inner = match inner_guard.take() {
            Some(stream) => stream,
            None => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream was already consumed",
                ));
            }
        };

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let requests: Vec<sift_rs::ingest::v1::IngestWithConfigDataStreamRequest> =
                requests.into_iter().map(|r| r.into()).collect();
            match inner.send_requests(requests).await {
                Ok(_) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(inner))),
                }),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    pub fn get_metrics_snapshot(&self) -> PyResult<crate::metrics::SiftStreamMetricsSnapshotPy> {
        let inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        match inner_guard.as_ref() {
            Some(stream) => Ok(stream.get_metrics_snapshot().into()),
            None => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream was already consumed",
            )),
        }
    }

    pub fn add_new_flows(&mut self, py: Python, flow_configs: Vec<FlowConfigPy>) -> PyResult<Py<PyAny>> {
        let mut inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        let mut inner = match inner_guard.take() {
            Some(stream) => stream,
            None => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream was already consumed",
                ));
            }
        };

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let configs: Vec<sift_rs::ingestion_configs::v2::FlowConfig> =
                flow_configs.into_iter().map(|f| f.into()).collect();
            match inner.add_new_flows(&configs).await {
                Ok(_) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(inner))),
                }),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    pub fn attach_run(&mut self, py: Python, run_selector: RunSelectorPy) -> PyResult<Py<PyAny>> {
        let mut inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        let mut inner = match inner_guard.take() {
            Some(stream) => stream,
            None => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream was already consumed",
                ));
            }
        };

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.attach_run(run_selector.into()).await {
                Ok(_) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(inner))),
                }),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    pub fn detach_run(&mut self) -> PyResult<()> {
        let mut inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        match inner_guard.as_mut() {
            Some(stream) => {
                stream.detach_run();
                Ok(())
            }
            None => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream was already consumed",
            )),
        }
    }

    pub fn run(&self) -> PyResult<Option<String>> {
        let inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        match inner_guard.as_ref() {
            Some(stream) => Ok(stream.run().map(|r| r.run_id.clone())),
            None => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream was already consumed",
            )),
        }
    }

    pub fn finish(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let mut inner_guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to acquire lock on stream",
                ));
            }
        };

        let inner = match inner_guard.take() {
            Some(stream) => stream,
            None => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream was already consumed",
                ));
            }
        };

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.finish().await {
                Ok(_) => Ok(()),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
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
