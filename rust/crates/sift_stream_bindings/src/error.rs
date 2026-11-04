use crate::stream::retry::DurationPy;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use pyo3_stub_gen::type_info::{MemberInfo, PyClassInfo};
use pyo3_stub_gen::{PyStubType, TypeInfo};
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
        sift_error_to_py(&err.0, format!("{}", err.0))
    }
}

/// Raises [`RateLimitedErrorPy`] with `message` if `err` is a rate-limit error, and
/// `RuntimeError` with `message` otherwise.
pub(crate) fn sift_error_to_py(err: &SiftError, message: String) -> PyErr {
    if err.is_rate_limited() {
        PyErr::new::<RateLimitedErrorPy, _>((message, err.retry_delay().map(DurationPy::from)))
    } else {
        PyRuntimeError::new_err(message)
    }
}

/// Raised when Sift rate limits a request and the rate-limit policy does not retry it, or the
/// retries run out.
///
/// Subclasses `RuntimeError`, so existing `except RuntimeError` handlers still catch it. The
/// message starts with `[RateLimitedError]` and includes the delay Sift requested, when provided.
// Not `#[gen_stub_pyclass]`: pyo3-stub-gen cannot derive a stub for a native exception base, so
// the stub info is registered by hand below.
#[pyclass(extends = PyRuntimeError)]
pub struct RateLimitedErrorPy {
    message: String,
    /// The minimum wait Sift requested before retrying, or `None` if Sift did not provide one.
    #[pyo3(get)]
    retry_delay: Option<DurationPy>,
}

impl PyStubType for RateLimitedErrorPy {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("RateLimitedErrorPy", Default::default())
    }
}

pyo3_stub_gen::inventory::submit! {
    PyClassInfo {
        pyclass_name: "RateLimitedErrorPy",
        struct_id: std::any::TypeId::of::<RateLimitedErrorPy>,
        members: &[MemberInfo {
            name: "retry_delay",
            r#type: <Option<DurationPy> as PyStubType>::type_output,
            doc: "The minimum wait Sift requested before retrying, or `None` if Sift did not provide one.",
        }],
        module: None,
        doc: "Raised when Sift rate limits a request and the rate-limit policy does not retry it, or \
              the retries run out.\n\nSubclasses `RuntimeError`, so existing `except RuntimeError` \
              handlers still catch it. The message starts with `[RateLimitedError]` and includes \
              the delay Sift requested, when provided.",
        bases: &[|| TypeInfo::builtin("RuntimeError")],
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl RateLimitedErrorPy {
    #[new]
    #[pyo3(signature = (message, retry_delay = None))]
    pub fn new(message: String, retry_delay: Option<DurationPy>) -> Self {
        Self {
            message,
            retry_delay,
        }
    }

    // The exception args hold both constructor arguments, so without this `str()` would show
    // the args tuple rather than the message.
    pub fn __str__(&self) -> &str {
        &self.message
    }
}
