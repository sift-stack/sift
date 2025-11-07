use crate::sift::metadata::MetadataPy;
use crate::stream::SiftStreamPy;
use crate::stream::config::{IngestionConfigFormPy, RunFormPy};
use crate::stream::retry::{DurationPy, RecoveryStrategyPy};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_stream::{Credentials, SiftStreamBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
pub struct SiftStreamBuilderPy {
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
    checkpoint_interval: Option<DurationPy>,
    #[pyo3(get, set)]
    run: Option<RunFormPy>,
    #[pyo3(get, set)]
    run_id: Option<String>,
    #[pyo3(get, set)]
    asset_tags: Option<Vec<String>>,
    #[pyo3(get, set)]
    metadata: Option<Vec<MetadataPy>>,
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl SiftStreamBuilderPy {
    #[new]
    pub fn new(uri: &str, apikey: &str) -> Self {
        Self {
            uri: uri.into(),
            apikey: apikey.into(),
            enable_tls: true,
            ingestion_config: None,
            recovery_strategy: None,
            checkpoint_interval: None,
            run: None,
            run_id: None,
            asset_tags: None,
            metadata: None,
        }
    }

    pub fn build(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let mut inner = SiftStreamBuilder::new(Credentials::Config {
            uri: self.uri.clone(),
            apikey: self.apikey.clone(),
        });

        if let Some(config) = self.ingestion_config.as_ref() {
            inner = inner.ingestion_config(config.clone().into());
        }

        if !self.enable_tls {
            inner = inner.disable_tls();
        }

        if let Some(strategy) = self.recovery_strategy.as_ref() {
            inner = inner.recovery_strategy(strategy.clone().into());
        }

        if let Some(checkpoint_interval) = self.checkpoint_interval.as_ref() {
            inner = inner.checkpoint_interval((*checkpoint_interval).into())
        }

        if let Some(run) = self.run.as_ref() {
            inner = inner.attach_run(run.clone().into());
        }

        if let Some(run_id) = self.run_id.as_ref() {
            inner = inner.attach_run_id(run_id);
        }

        inner = inner.add_asset_tags(self.asset_tags.clone());

        let metadata = self.metadata.clone().map(|v| {
            v.into_iter()
                .map(|m| m.into())
                .collect::<Vec<sift_rs::metadata::v1::MetadataValue>>()
        });
        inner = inner.add_asset_metadata(metadata);

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
