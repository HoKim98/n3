use pyo3::prelude::*;
use pyo3::types::PyCFunction;
use pyo3::wrap_pyfunction;

use n3_machine::Program;
use n3_torch_ffi::pyo3;

use crate::code::BuildCode;
use crate::handler::SignalHandler;

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute)(py)
}

#[pyfunction]
pub(self) fn n3_execute(
    py: Python,
    id: u64,
    machine: &str,
    command: &str,
    program: &Program,
) -> PyResult<()> {
    // Step 1. Load the program
    let program = n3_program::Program::load(program).unwrap();
    dbg!(id, machine);

    // Step 2. Define the node in REPL
    let program = program.build(py, ())?.into_py(py);

    // Step 3. Do its own job
    let command = command.to_string();
    SignalHandler::run(py, move |handler| {
        pyo3::Python::with_gil(|py| {
            program.call_method1(py, &command, (handler,))?;
            Ok(())
        })
    })
    .unwrap()
}

#[cfg(test)]
mod test {
    use n3_builder::*;

    use super::*;

    fn load_n3(py: Python) -> PyResult<()> {
        let sys = py.import("sys")?;
        sys.get("path")?
            .call_method1("insert", (0, "../n3-torch-ffi-python"))?;
        Ok(())
    }

    #[test]
    fn test_exec() -> std::result::Result<(), ()> {
        Python::with_gil(|py| {
            let envs = GlobalVars::default();
            envs.set(dirs::N3_ROOT, "../n3-builder/tests/data/")
                .unwrap();
            envs.set(dirs::N3_SOURCE_ROOT, "../n3-torch-ffi-python/n3")
                .unwrap();
            let mut root = ExecRoot::try_new(envs).unwrap();

            let args = root.get("DummyImageClassification").unwrap();
            args.set("data", "Mnist").unwrap();
            args.set("model", "LeNet5").unwrap();
            args.set("epoch", "1").unwrap();
            args.set("batch size", "10").unwrap();

            let program = args.build_with_env().unwrap();

            load_n3(py)
                .and_then(|()| n3_execute(py, 0, "cuda:0", "train", &program))
                .and_then(|()| py.run("exit(0)", None, None))
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);
                })
        })
    }
}
