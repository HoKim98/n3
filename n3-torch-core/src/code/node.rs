use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

use n3_builder::CodeData;
use n3_torch_ffi::pyo3;

use super::base::{BuildArgs, BuildCode};
use super::out::Outs;

impl<'a> BuildCode<'a> for n3_builder::NodeCode {
    type Args = &'a BuildArgs<'a>;
    type Output = &'a PyAny;

    fn build(&'a self, py: Python<'a>, args: Self::Args) -> PyResult<Self::Output> {
        // Step 1. Build the tensor graph
        let tensor_graph: Vec<_> = self
            .tensor_graph
            .iter()
            .map(|x| x.build(py, &args))
            .collect::<PyResult<_>>()?;

        // Step 2. Instantiate
        NodeBuilder {
            data: &self.data,
            tensor_graph: &tensor_graph,
        }
        .build(py)
    }
}

pub struct NodeBuilder<'a, 'b>
where
    'a: 'b,
{
    pub data: &'a CodeData,
    pub tensor_graph: &'b [&'a PyAny],
}

impl<'a, 'b> NodeBuilder<'a, 'b>
where
    'a: 'b,
{
    pub fn build(self, py: Python<'a>) -> PyResult<&'a PyAny> {
        // Step 1. Build the data
        let name = self.data.name.as_str().into_py(py);
        let input = Outs(&self.data.input).into_py_dict(py);
        let output = Outs(&self.data.output).into_py_dict(py);

        // Step 2. Build the tensor graph
        let tensor_graph = PyList::new(py, self.tensor_graph);

        // Step 3. Instantiate
        py.import("n3")?.get("node")?.getattr("node")?.call_method(
            "NodeExecutable",
            (),
            Some(
                [
                    ("name", name.as_ref(py)),
                    ("input", input),
                    ("output", output),
                    ("tensor_graph", tensor_graph),
                ]
                .into_py_dict(py),
            ),
        )
    }
}
