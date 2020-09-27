use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass(subclass)]
pub struct Node {}

#[pymethods]
impl Node {
    #[classmethod]
    pub fn name(cls: &PyType) -> String {
        cls.name().to_string()
    }
}

#[cfg(test)]
mod tests {
    use pyo3::{py_run, wrap_pymodule};

    use super::*;
    use crate::PyInit_n3;

    #[test]
    fn test_dummy_node() {
        Python::with_gil(|py| {
            let n3 = wrap_pymodule!(n3)(py);

            py_run!(py, n3 n3, r#"
class CustomNode(n3.Node):
    pass

assert CustomNode.name() == 'CustomNode'
            "#);
        })
    }
}
