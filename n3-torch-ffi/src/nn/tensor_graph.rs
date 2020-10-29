use std::ops::{Deref, DerefMut};

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::torch::Torch;

pub struct TensorGraph(PyObject);

impl TensorGraph {
    pub fn new(py: Python, nodes: Vec<PyObject>) -> PyResult<Self> {
        Ok(Self {
            0: Torch(py).nn("ModuleList")?.call1((nodes,))?.into_py(py),
        })
    }

    pub fn empty(py: Python) -> PyResult<Self> {
        Self::new(py, vec![])
    }

    pub fn children(&self, py: Python) -> PyResult<PyObject> {
        self.0.call_method0(py, "children")
    }

    pub fn parameters(&self, py: Python) -> PyResult<PyObject> {
        self.0.call_method0(py, "parameters")
    }

    pub fn train(&self, py: Python, mode: bool) -> PyResult<PyObject> {
        self.0.call_method1(py, "train", (mode,))
    }

    pub fn eval(&self, py: Python) -> PyResult<PyObject> {
        self.train(py, false)
    }

    pub fn to(&self, py: Python, device: Option<PyObject>) -> PyResult<PyObject> {
        let kwargs = PyDict::new(py);
        if let Some(device) = device {
            kwargs.set_item("device", device)?;
        }

        self.0.call_method(py, "to", (), Some(kwargs))
    }
}

impl Deref for TensorGraph {
    type Target = PyObject;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TensorGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use pyo3::types::IntoPyDict;
    use pyo3::*;

    use super::*;
    use crate::machine::*;

    #[test]
    fn test_linear() -> std::result::Result<(), ()> {
        fn linear(py: Python, input_channels: usize, output_channels: usize) -> PyResult<PyObject> {
            Ok(Torch(py)
                .nn("Linear")?
                .call(
                    (),
                    Some(
                        [
                            ("in_features", input_channels),
                            ("out_features", output_channels),
                        ]
                        .into_py_dict(py),
                    ),
                )?
                .into_py(py))
        }

        #[pyfunction]
        fn test_tg_linear(py: Python) -> PyResult<()> {
            let mut machine = PyMachine::new(py);
            let torch = Torch(py);

            // get a sample tensor graph
            let tensor_graph = TensorGraph::new(
                py,
                vec![
                    linear(py, 16, 32)?,
                    linear(py, 32, 64)?,
                    linear(py, 64, 10)?,
                ],
            )?;

            // get a sample 3x16 tensor
            let mut output = torch.this("zeros")?.call1((3, 16))?;

            // propagate (16 -> 32 -> 64 -> 10)
            for node in tensor_graph.as_ref(py).iter()? {
                let node = node?;
                output = node.call_method1("forward", (output,))?;
            }

            // test output shape
            assert_eq!(output.getattr("shape")?.extract::<(_, _)>()?, (3, 10));

            machine.py_terminate()
        }

        Python::with_gil(|py| {
            let mut process = pyo3_mp::Process::new(py)?;
            process.spawn(wrap_pyfunction!(test_tg_linear)(py)?, (), None)?;
            process.join()
        })
        .map_err(|e: PyErr| Python::with_gil(|py| e.print_and_set_sys_last_vars(py)))
    }
}
