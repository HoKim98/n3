use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use super::base::{BuildArgs, BuildCode};
use super::graph::Args;

impl<'a> BuildCode<'a> for n3_program::Program {
    type Args = ();
    type Output = &'a PyAny;

    fn build(&'a self, py: Python<'a>, (): Self::Args) -> PyResult<Self::Output> {
        // Step 1. Convert the args
        let args = Args(&self.graph).into_py_dict(py);
        if let Some(env) = &self.env {
            let env = Args(env).into_py_dict(py);
            args.set_item("env", env)?;
        }

        let build_args = BuildArgs {
            args: &args,
            scripts: &self.scripts,
        };

        // Step 2. Build the nodes
        let nodes: Vec<_> = self
            .nodes
            .iter()
            .map(|(k, v)| Ok((k, v.build(py, &build_args)?)))
            .collect::<PyResult<_>>()?;
        let nodes = nodes.into_iter().into_py_dict(py);

        // Step 3. Get the main program
        let main = &self.scripts["__main__"];

        // Step 4. Define the node in REPL
        py.run(&main.source, None, None)?;

        // Step 5. Instantiate
        py.eval(
            &format!("{name}(args, nodes)", name = &main.name),
            None,
            Some([("args", args), ("nodes", nodes)].into_py_dict(py)),
        )
    }
}
