use pyo3::prelude::*;

mod data;
mod grpc;

#[pymodule]
fn sift_pyrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<data::DataService>()?;
    Ok(())
}
