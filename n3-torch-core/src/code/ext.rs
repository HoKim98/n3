use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use n3_program::ast;
use n3_torch_ffi::pyo3;

use super::base::{BuildArgs, BuildCode};
use super::graph::Args;
use super::node::NodeBuilder;
use super::out::OutsExtern;

impl<'a> BuildCode<'a> for n3_program::ExternCode {
    type Args = &'a BuildArgs<'a>;
    type Output = &'a PyAny;

    fn build(&'a self, py: Python<'a>, args: Self::Args) -> PyResult<Self::Output> {
        // Step 1. Build the data
        let input = OutsExtern(&self.data.input).into_py_dict(py);
        let output = OutsExtern(&self.data.output).into_py_dict(py);
        let values = Args(&self.data.graph).into_py_dict(py);

        // Step 2. Get the source
        let main = &args.scripts[&self.data.name];

        // Step 3. Define the node in REPL
        py.run(&main.source, None, None)?;

        // Step 4. Instantiate extern node
        let node = py.eval(
            &format!(
                "{name}(args=args, input=input, output=output, values=values)",
                name = &main.name
            ),
            None,
            Some(
                [
                    ("args", args.args),
                    ("input", input),
                    ("output", output),
                    ("values", values),
                ]
                .into_py_dict(py),
            ),
        )?;

        // Step 5. Instantiate
        match self.ty {
            ast::ExternNodeType::Default => NodeBuilder {
                data: &self.data,
                tensor_graph: &[node],
            }
            .build(py),
            // do not instantiate the special nodes into NodeExecutable
            ast::ExternNodeType::Data | ast::ExternNodeType::Optim => Ok(node),
        }
    }
}
