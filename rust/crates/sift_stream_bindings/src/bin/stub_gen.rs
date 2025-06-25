use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // Use the public wrapper function to access stub info
    let stub = sift_stream_bindings::stub_info()?;
    stub.generate()?;
    Ok(())
}
