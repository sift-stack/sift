use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_stream::stream::time::TimeValue;

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct TimeValuePy {
    pub inner: TimeValue,
}

// Trait Implementations
impl Default for TimeValuePy {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TimeValuePy> for TimeValue {
    fn from(time: TimeValuePy) -> Self {
        time.inner
    }
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl TimeValuePy {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: TimeValue::now(),
        }
    }

    #[staticmethod]
    pub fn from_timestamp(secs: i64, nsecs: u32) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_timestamp(secs, nsecs)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?,
        })
    }

    #[staticmethod]
    pub fn from_timestamp_millis(millis: i64) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_timestamp_millis(millis)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?,
        })
    }

    #[staticmethod]
    pub fn from_timestamp_micros(micros: i64) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_timestamp_micros(micros)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?,
        })
    }

    #[staticmethod]
    pub fn from_timestamp_nanos(nanos: i64) -> Self {
        Self {
            inner: TimeValue::from_timestamp_nanos(nanos),
        }
    }

    #[staticmethod]
    pub fn from_rfc3339(val: &str) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_rfc3339(val)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?,
        })
    }
}
