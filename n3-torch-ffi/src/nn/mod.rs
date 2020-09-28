mod ext;
mod tensor_graph;

pub use self::ext::ExternNode;
pub use self::tensor_graph::TensorGraph;

use std::collections::BTreeMap;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList};

use crate::tensor::*;

#[pyclass(subclass)]
pub struct Node {
    #[pyo3(get)]
    node_input: Py<PyDict>,
    #[pyo3(get)]
    node_output: Py<PyDict>,
    tensor_graph: TensorGraph,
}

#[pymethods]
impl Node {
    #[new]
    pub fn new(py: Python) -> PyResult<Self> {
        Ok(Self {
            node_input: PyDict::new(py).into_py(py),
            node_output: PyDict::new(py).into_py(py),
            tensor_graph: TensorGraph::empty(py)?,
        })
    }
}

impl Node {
    pub fn init_node(
        &mut self,
        py: Python,
        node_input: Py<PyDict>,
        node_output: Py<PyDict>,
        tensor_graph: Vec<PyObject>,
    ) -> PyResult<()> {
        self.node_input = node_input;
        self.node_output = node_output;
        self.tensor_graph = TensorGraph::new(py, tensor_graph)?;
        Ok(())
    }
}

#[pymethods]
impl Node {
    #[call]
    #[args(kwargs = "**")]
    fn __call__(&self, py: Python, kwargs: Option<&PyDict>) -> PyResult<TensorOutput> {
        let kwargs = TensorInput::new(match kwargs {
            Some(kwargs) => kwargs.into_py(py),
            None => PyDict::new(py).into_py(py),
        });
        self.forward(py, &kwargs)
    }

    fn forward(&self, py: Python, input: &TensorInput) -> PyResult<TensorOutput> {
        let output = input.to_output(py)?;
        let output = output.as_ref(py);

        let mut x_final = None;

        let mut nodes = self.tensor_graph.as_ref(py).iter()?;
        while let Some(node) = nodes.next() {
            let node = node?;

            let x = PyDict::new(py);
            for (k, n) in node
                .getattr("node_input")?
                .extract::<BTreeMap<String, _>>()?
            {
                let idx = index(py, &output, n)?;
                x.set_item(k, idx)?;
            }

            let mut x = node.call((), Some(x))?;
            if !x.get_type().is_subclass::<pyo3::types::PyDict>()? {
                x = [("x", x)].into_py_dict(py);
            }

            for (k, n) in node
                .getattr("node_output")?
                .extract::<BTreeMap<String, PyObject>>()?
            {
                let (k, n) = (n, x.get_item(k).unwrap());
                output.set_item(k, n)?;
            }

            x_final = Some(x);
        }

        let x = x_final
            .unwrap()
            .into_py(py)
            .extract::<BTreeMap<Out, PyObject>>(py)?
            .into_iter()
            .map(|(k, v)| (k.name, v))
            .into_py_dict(py)
            .into_py(py);
        Ok(TensorOutput::new(x))
    }

    fn children(&self, py: Python) -> PyResult<PyObject> {
        self.tensor_graph.children(py)
    }

    fn parameters(&self, py: Python) -> PyResult<PyObject> {
        self.tensor_graph.parameters(py)
    }

    fn train(&self, py: Python, mode: bool) -> PyResult<PyObject> {
        self.tensor_graph.train(py, mode)
    }

    fn eval(&self, py: Python) -> PyResult<PyObject> {
        self.tensor_graph.eval(py)
    }

    fn to(&self, py: Python, device: Option<PyObject>) -> PyResult<PyObject> {
        self.tensor_graph.to(py, device)
    }
}

fn index(py: Python, data: &PyDict, key: PyObject) -> PyResult<PyObject> {
    let key = key.as_ref(py);
    if key.get_type().is_subclass::<pyo3::types::PyList>()? {
        let list = PyList::empty(py);

        for key in key.extract::<Vec<PyObject>>()? {
            list.append(index(py, data, key)?)?;
        }
        Ok(list.into_py(py))
    } else {
        Ok(data.get_item(key).unwrap().into_py(py))
    }
}
