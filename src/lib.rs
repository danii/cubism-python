use cubism::core::version;
use pyo3::prelude::*;

#[pymodule]
fn idk(py: Python, m: &PyModule) -> PyResult<()> {
	println!("{:?}", version());
	Ok(())
}
