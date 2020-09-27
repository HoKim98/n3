use pyo3::prelude::*;
use pyo3::PyIterProtocol;

use super::NodeBase;
use crate::torch::Torch;

#[pyclass(subclass, extends=NodeBase)]
pub struct TensorGraph {
    inner: PyObject,
}

impl TensorGraph {
    pub fn new(py: Python, nodes: Vec<PyObject>) -> PyResult<PyClassInitializer<Self>> {
        Ok(
            PyClassInitializer::from(NodeBase::default()).add_subclass(Self {
                inner: Torch(py).nn("TensorGraph")?.call1((nodes,))?.into_py(py),
            }),
        )
    }
}

#[pyproto]
impl PyIterProtocol for TensorGraph {
    fn __iter__(self_: PyRef<Self>) -> PyObject {
        self_.inner.clone()
    }
}
