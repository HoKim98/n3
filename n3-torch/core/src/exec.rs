use pyo3::prelude::*;
use pyo3::types::PyCFunction;
use pyo3::wrap_pyfunction;

use n3_machine::{MachineId, Program};
use n3_torch_ffi::{pyo3, SignalHandler};

use crate::code::BuildCode;
use crate::process::exit_python;

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute)(py)
}

#[pyfunction]
pub(self) fn n3_execute(
    py: Python,
    id: MachineId,
    machine: &str,
    command: &str,
    program: &Program,
    handler: SignalHandler,
) -> PyResult<()> {
    // Step 1. Load the program
    let mut program = n3_program::Program::load(program).unwrap();

    // Step 2. Attach id, machine
    if program.env.is_none() {
        program.env = Some(Default::default());
    }
    let env = program.env.as_mut().unwrap();
    env.insert("id".to_string(), Some(id.into()));
    env.insert("machine".to_string(), Some(machine.to_string().into()));

    // Step 3. Define the node in REPL
    let program = program.build(py, ())?.into_py(py);

    // Step 4. Do its own job
    let command = command.to_string();
    handler.run(py, move |handler| {
        pyo3::Python::with_gil::<_, PyResult<_>>(|py| {
            program.call_method1(py, &command, (handler,))?;
            Ok(())
        })
    })?;

    // Step 5. Exit interpreter
    unsafe {
        exit_python();
    }
    Ok(())
}
