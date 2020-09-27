use pyo3::prelude::*;

use super::tensor_graph::TensorGraph;
use super::NodeBase;
use crate::pybox::PyBox;

#[pyclass(subclass, extends=NodeBase)]
pub struct Node {
    name: String,
    node_input: n3_builder::Outs,
    node_output: n3_builder::Outs,
    tensor_graph: PyBox<TensorGraph>,
}

impl Node {
    pub fn new(
        py: Python,
        name: String,
        node_input: n3_builder::Outs,
        node_output: n3_builder::Outs,
        tensor_graph: Vec<PyObject>,
    ) -> PyResult<Self> {
        Ok(Self {
            name,
            node_input,
            node_output,
            tensor_graph: PyBox::new(py, TensorGraph::new(py, tensor_graph)?)?,
        })
    }
}
