use crate::stream::SiftStreamPy;
use crate::stream::config::{IngestionConfigFormPy, RunFormPy};
use crate::stream::retry::{DurationPy, RecoveryStrategyPy};
use pyo3::prelude::*;
use sift_stream::{Credentials, IngestionConfigMode, SiftStreamBuilder};
use std::sync::Arc;
use std::sync::Mutex;

// Type Definitions
#[pyclass]
pub struct SiftStreamBuilderPy {
    inner: Arc<Mutex<Option<SiftStreamBuilder<IngestionConfigMode>>>>,
    #[pyo3(get, set)]
    uri: String,
    #[pyo3(get, set)]
    apikey: String,
    #[pyo3(get, set)]
    enable_tls: bool,
    #[pyo3(get, set)]
    ingestion_config: Option<IngestionConfigFormPy>,
    #[pyo3(get, set)]
    recovery_strategy: Option<RecoveryStrategyPy>,
    #[pyo3(get, set)]
    checkpoint_interval: DurationPy,
    #[pyo3(get, set)]
    run: Option<RunFormPy>,
    #[pyo3(get, set)]
    run_id: Option<String>,
}

// PyO3 Method Implementations
#[pymethods]
impl SiftStreamBuilderPy {
    #[new]
    pub fn new(uri: &str, apikey: &str) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(SiftStreamBuilder::new(
                Credentials::Config {
                    uri: uri.into(),
                    apikey: apikey.into(),
                },
            )))),
            uri: uri.into(),
            apikey: apikey.into(),
            enable_tls: true,
            ingestion_config: None,
            recovery_strategy: None,
            checkpoint_interval: DurationPy::new(60, 0),
            run: None,
            run_id: None,
        }
    }

    pub fn build(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let mut inner = self.inner.lock().unwrap().take().unwrap();
        if let Some(config) = self.ingestion_config.as_ref() {
            inner = inner.ingestion_config(config.clone().into());
        }

        if !self.enable_tls {
            inner = inner.disable_tls();
        }

        if let Some(strategy) = self.recovery_strategy.as_ref() {
            inner = inner.recovery_strategy(strategy.clone().into());
        }

        inner = inner.checkpoint_interval(self.checkpoint_interval.into());

        if let Some(run) = self.run.as_ref() {
            inner = inner.attach_run(run.clone().into());
        }

        if let Some(run_id) = self.run_id.as_ref() {
            inner = inner.attach_run_id(run_id);
        }

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.build().await {
                Ok(stream) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(stream))),
                }),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }
}
