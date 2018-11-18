#[macro_use]
extern crate pyo3;

use pyo3::exceptions;
use pyo3::prelude::*;

#[pymodinit]
fn _lenrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(len))?;

    Ok(())
}

#[pyfunction]
fn len(py: Python, obj: PyObject) -> PyResult<PyObject> {
    if let Ok(s) = obj.extract::<String>(py) {
        return Ok(s.len().to_object(py));
    }
    if let Ok(v) = obj.extract::<Vec<String>>(py) {
        return Ok(v.len().to_object(py));
    }
    Err(PyErr::new::<exceptions::TypeError, _>("Type not supported"))
}
