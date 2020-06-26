use cubism::core::version;
use pyo3::{
	prelude::{PyModule, PyResult, Python, pyfunction, pymodule},
	wrap_pyfunction
};

#[pymodule(cubism)]
pub fn init(_: Python, module: &PyModule) -> PyResult<()> {
	println!("Initialized!");
	module.add_wrapped(wrap_pyfunction!(get_version))?;
	Ok(())
}

#[pyfunction]
pub fn get_version() -> PyResult<(u8, u8, u16)> {
	Ok(version())
}
