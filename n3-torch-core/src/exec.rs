use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyCFunction};
use pyo3::wrap_pyfunction;

use n3_machine::Program;
use n3_torch_ffi::pyo3;

use crate::graph::Args;

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute)(py)
}

#[pyfunction]
pub(self) fn n3_execute(
    py: Python,
    id: usize,
    machine: &str,
    command: &str,
    program: &Program,
) -> PyResult<()> {
    let program = n3_builder::Program::load(program).unwrap();
    dbg!(id, machine);

    let args = Args(&program.graph).into_py_dict(py);
    if let Some(env) = &program.env {
        let env = Args(env).into_py_dict(py);
        args.set_item("env", env)?;
    }

    let nodes = (&[] as &[(&str, PyObject)]).into_py_dict(py);

    let model = program.nodes["model"].as_node();
    let model_n1 = model.tensor_graph[1].as_node();
    let model_n1_conv = model_n1.tensor_graph[0].as_extern();
    dbg!(model_n1_conv);

    let main = &program.scripts["__main__"];

    // Step 1. Define the Node
    py.run(&main.source, None, None)?;
    // Step 2. Instantiate
    let trainer = py.eval(
        &format!("{name}(args, nodes)", name = &main.name),
        None,
        Some([("args", args), ("nodes", nodes)].into_py_dict(py)),
    )?;
    // Step 3. Do its own job
    trainer.call_method0(command)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use n3_builder::*;

    use super::*;

    fn load_n3(py: Python) -> PyResult<()> {
        let sys = py.import("sys")?;
        sys.get("path")?
            .call_method1("insert", (0, "../n3-torch-ffi-py"))?;
        Ok(())
    }

    #[test]
    fn test_exec() -> std::result::Result<(), ()> {
        Python::with_gil(|py| {
            let envs = GlobalVars::default();
            envs.set("root", "../n3-builder/tests/data/").unwrap();
            let mut root = ExecRoot::try_new(envs).unwrap();

            let args = root.get("DummyImageClassification").unwrap();
            args.set("data", "Mnist").unwrap();
            args.set("model", "LeNet5").unwrap();
            args.set("epoch", "1").unwrap();
            args.set("batch size", "10").unwrap();

            let program = args.build_with_env().unwrap();

            load_n3(py)
                .and_then(|()| n3_execute(py, 0, "cuda:0", "train", &program))
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);
                })
        })
    }
}
