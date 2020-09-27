use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::tensor::*;

#[pyclass]
pub struct ExternNode {
    #[pyo3(get)]
    node_input: Py<PyDict>,
    #[pyo3(get)]
    node_output: Py<PyDict>,
}

/// The node doesn't need to be thread safe.
unsafe impl Send for ExternNode {}

impl ExternNode {
    pub fn new(node_input: Py<PyDict>, node_output: Py<PyDict>) -> Self {
        Self {
            node_input,
            node_output,
        }
    }
}

#[pymethods]
impl ExternNode {
    fn forward(&self, py: Python, input: &TensorInput) -> PyResult<TensorOutput> {
        todo!();
    }

    fn parameters(&self) -> PyResult<PyObject> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(()))
    }
}
