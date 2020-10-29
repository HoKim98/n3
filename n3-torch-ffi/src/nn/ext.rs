use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(subclass)]
pub struct ExternNode {
    #[pyo3(get)]
    node_input: Py<PyDict>,
    #[pyo3(get)]
    node_output: Py<PyDict>,
}

#[pymethods]
impl ExternNode {
    #[new]
    pub fn new(py: Python) -> Self {
        Self {
            node_input: PyDict::new(py).into_py(py),
            node_output: PyDict::new(py).into_py(py),
        }
    }

    pub fn init_node(&mut self, node_input: Py<PyDict>, node_output: Py<PyDict>) {
        self.node_input = node_input;
        self.node_output = node_output;
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use pyo3::types::IntoPyDict;
    use pyo3::*;

    use n3_torch_machine::HostMachine;

    use super::*;
    use crate::torch::Torch;
    use crate::PyInit_n3;

    #[test]
    fn test_subclass() -> std::result::Result<(), ()> {
        #[pyfunction]
        fn test_ext_subclass(py: Python) -> PyResult<()> {
            let mut host = HostMachine::try_new().unwrap();
            let builtins = py.import("builtins")?.into_py(py);
            let torch = Torch(py);

            let n3 = wrap_pymodule!(n3)(py);
            let nn = torch.this("nn")?.into_py(py);
            let zeros = torch.this("zeros")?.into_py(py);

            py.run(
                r#"
class MyExternNode(n3.ExternNode):
    def __init__(self):
        super().__init__()
        self.inner1 = nn.Linear(32, 64)
        self.inner2 = nn.Linear(64, 10, bias=False)

    def forward(self, x):
        x = self.inner1(x)
        x = self.inner2(x)
        return x


node = MyExternNode()
node.init_node({}, {})

assert len(list(node.parameters())) == 3

x = zeros(3, 32)
y = node(x)
assert y.shape == (3, 10)

"#,
                Some(
                    [
                        ("__builtins__", builtins),
                        ("nn", nn),
                        ("n3", n3),
                        ("zeros", zeros),
                    ]
                    .into_py_dict(py),
                ),
                None,
            )?;

            host.py_terminate()
        }

        Python::with_gil(|py| {
            let mut process = pyo3_mp::Process::new(py)?;
            process.spawn(wrap_pyfunction!(test_ext_subclass)(py)?, (), None)?;
            process.join()
        })
        .map_err(|e| Python::with_gil(|py| e.print_and_set_sys_last_vars(py)))
    }
}
