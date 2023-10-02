pub mod alife;
pub mod constants;
pub mod heuristic;
pub mod map;

use pyo3::prelude::*;
use pyo3::{
    exceptions::PyIndexError,
    pymodule,
    types::{PyDict, PyModule},
    FromPyObject, PyAny, PyObject, Python,
};

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    println!("Debugging disabled");

    Ok((a + b).to_string())
}

#[pymodule]
fn libcmput651py<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
