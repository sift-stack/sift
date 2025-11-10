use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct MetadataPy {
    #[pyo3(get, set)]
    key: String,
    #[pyo3(get, set)]
    value: MetadataValuePy,
}

#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug, Clone)]
pub enum MetadataValuePy {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[gen_stub_pymethods]
#[pymethods]
impl MetadataValuePy {
    #[new]
    fn new(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        if obj.is_instance_of::<pyo3::types::PyString>() {
            let s: String = obj.extract()?;
            Ok(MetadataValuePy::String(s))
        } else if obj.is_instance_of::<pyo3::types::PyInt>() {
            let i: i64 = obj.extract()?;
            Ok(MetadataValuePy::Number(i as f64))
        } else if obj.is_instance_of::<pyo3::types::PyFloat>() {
            let f: f64 = obj.extract()?;
            Ok(MetadataValuePy::Number(f))
        } else if obj.is_instance_of::<pyo3::types::PyBool>() {
            let b: bool = obj.extract()?;
            Ok(MetadataValuePy::Boolean(b))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected a string, integer, float, or boolean",
            ))
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn is_string(&self) -> bool {
        matches!(self, MetadataValuePy::String(_))
    }

    fn is_number(&self) -> bool {
        matches!(self, MetadataValuePy::Number(_))
    }

    fn is_boolean(&self) -> bool {
        matches!(self, MetadataValuePy::Boolean(_))
    }
}

// Convert MetadataPy to sift_rs::metadata::v1::MetadataValue
impl From<MetadataPy> for sift_rs::metadata::v1::MetadataValue {
    fn from(metadata: MetadataPy) -> Self {
        use sift_rs::wrappers::metadata::MetadataEnumValue;

        let value: MetadataEnumValue = match metadata.value {
            MetadataValuePy::String(s) => s.into(),
            MetadataValuePy::Number(n) => n.into(),
            MetadataValuePy::Boolean(b) => b.into(),
        };
        (metadata.key.as_str(), value).into()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl MetadataPy {
    #[new]
    fn new(key: String, value: MetadataValuePy) -> MetadataPy {
        MetadataPy { key, value }
    }
}
