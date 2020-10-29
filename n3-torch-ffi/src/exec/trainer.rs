use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Trainer {}

#[pymethods]
impl Trainer {
    #[new]
    fn new() -> Self {
        Self {}
    }

    fn train(&self) -> PyResult<()> {
        not_implemented("train")
    }

    fn eval(&self) -> PyResult<()> {
        not_implemented("eval")
    }
}

fn not_implemented<T>(method: &str) -> PyResult<T> {
    Err(pyo3::exceptions::PyNotImplementedError::new_err(format!(
        "Trainer::{} should be implemented.",
        method,
    )))
}
