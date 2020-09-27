mod tensor_graph;

pub use self::tensor_graph::TensorGraph;

use std::collections::BTreeMap;

use pyo3::prelude::*;

use crate::builder;
use crate::machine::Machine;
use crate::tensor::*;

#[pyclass(subclass)]
pub struct Node {
    name: String,
    node_input: builder::Outs,
    node_output: builder::Outs,
    tensor_graph: TensorGraph,
}

impl Node {
    pub fn new(
        machine: &Machine,
        name: String,
        node_input: builder::Outs,
        node_output: builder::Outs,
        tensor_graph: Vec<PyObject>,
    ) -> PyResult<Self> {
        Ok(Self {
            name,
            node_input,
            node_output,
            tensor_graph: TensorGraph::new(machine, tensor_graph)?,
        })
    }
}

impl NodeExecutable for Node {
    fn forward(&self, kwargs: impl IntoIterator<Item = (String, Tensor)>) -> TensorOutput {
        let output: BTreeMap<_, _> = kwargs
            .into_iter()
            .map(|(k, v)| (Out::new(0, k), v))
            .collect();

        // for node in (*self.tensor_graph).iter() {

        // }

        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use pyo3::{py_run, wrap_pymodule};

//     use super::*;
//     use crate::PyInit_n3;

//     #[test]
//     fn test_subclass() {
//         Python::with_gil(|py| {
//             let n3 = wrap_pymodule!(n3)(py);

//             py_run!(py, n3 n3, r#"
// class CustomNode(n3.NodeBase):
//     pass

// assert CustomNode.name() == 'CustomNode'
//             "#);
//         })
//     }
// }
