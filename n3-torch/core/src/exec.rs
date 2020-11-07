use pyo3::prelude::*;
use pyo3::types::PyCFunction;
use pyo3::wrap_pyfunction;

use n3_machine::{MachineId, Program, PORT};
use n3_torch_ffi::{pyo3, SignalHandler};

use crate::code::BuildCode;

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute)(py)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(self) fn n3_execute(
    py: Python,
    id_primary: MachineId,
    id_local: MachineId,
    id_world: MachineId,
    machine: &str,
    command: &str,
    program: &Program,
    handler: SignalHandler,
) -> PyResult<()> {
    let is_root = id_primary == 0;
    let is_distributed = id_world > 1;

    // Step 1. Load the program
    let mut program = n3_program::Program::load(program).unwrap();

    // Step 2. Attach variables
    if program.env.is_none() {
        program.env = Some(Default::default());
    }
    let env = program.env.as_mut().unwrap();
    env.insert("id".to_string(), Some(id_local.into()));
    env.insert("machine".to_string(), Some(machine.to_string().into()));

    env.insert("is root".to_string(), Some(is_root.into()));
    env.insert("is distributed".to_string(), Some(is_distributed.into()));

    let gpu_id = machine.split("cuda").nth(1).map(|x| {
        if x.is_empty() {
            0 as MachineId
        } else {
            x[1..].parse().unwrap()
        }
    });
    env.insert("gpu id".to_string(), gpu_id.map(|x| x.into()));

    // Step 3. Ready for DDP
    if is_distributed {
        let env = py.import("os")?.get("environ")?;
        env.set_item("MASTER_ADDR", "127.0.0.1")?; // TODO: to be implemented
        env.set_item("MASTER_PORT", format!("{}", PORT))?;

        env.set_item("RANK", format!("{}", id_primary))?;
        env.set_item("LOCAL_RANK", format!("{}", id_local))?;
        env.set_item("WORLD_SIZE", format!("{}", id_world))?;
    }

    // Step 4. Define the node in REPL
    let program = program.build(py, ())?.into_py(py);

    // Step 5. Do its own job
    handler.run(py, move |handler| {
        pyo3::Python::with_gil(|py| {
            // execute the command
            program.call_method1(py, command, (handler,))?;
            // finalize
            program.call_method0(py, "close")?;
            Ok(())
        })
    })
}
