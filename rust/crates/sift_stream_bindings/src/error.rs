use pyo3::prelude::*;
use sift_error::Error as SiftError;

// Newtype wrapper for sift_error::Error
#[derive(Debug)]
pub struct SiftErrorWrapper(pub SiftError);

impl From<SiftError> for SiftErrorWrapper {
    fn from(err: SiftError) -> Self {
        Self(err)
    }
}

impl From<SiftErrorWrapper> for PyErr {
    fn from(err: SiftErrorWrapper) -> Self {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", err.0))
    }
}
