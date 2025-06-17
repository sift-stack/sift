pub mod builder;
pub mod channel;
pub mod config;
pub mod request;
pub mod retry;
pub mod time;

use crate::stream::channel::ChannelValuePy;
use crate::stream::time::TimeValuePy;
use pyo3::prelude::*;
use sift_stream::{Flow, IngestionConfigMode, SiftStream};
use std::sync::Arc;
use std::sync::Mutex;

// Type Definitions
#[pyclass]
pub struct SiftStreamPy {
    inner: Arc<Mutex<Option<SiftStream<IngestionConfigMode>>>>,
}

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
        stream.inner.lock().unwrap().take().unwrap()
    }
}

impl From<FlowPy> for Flow {
    fn from(flow: FlowPy) -> Self {
        flow.inner
    }
}

// PyO3 Method Implementations
#[pymethods]
impl SiftStreamPy {
    pub fn send(&mut self, py: Python, flow: FlowPy) -> PyResult<Py<PyAny>> {
        let mut inner = self.inner.lock().unwrap().take().unwrap();
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
        let mut inner = self.inner.lock().unwrap().take().unwrap();
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

    pub fn finish(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = self.inner.lock().unwrap().take().unwrap();
        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.finish().await {
                Ok(_) => Ok(()),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }
}

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
