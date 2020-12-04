use pyo3::prelude::*;
use pyo3::types::PyCFunction;
use pyo3::wrap_pyfunction;

use n3_machine::PORT;
use n3_machine_ffi::{MachineId, Program};
use n3_torch_ffi::pyo3;

use crate::code::BuildCode;

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute_delegate)(py)
}

#[pyfunction]
pub(self) fn n3_execute_delegate(py: Python, args: &PyAny, kwargs: &PyAny) -> PyResult<()> {
    let id_primary: MachineId = args.get_item(0)?.extract()?;
    let id_local: MachineId = args.get_item(1)?.extract()?;
    let id_world: MachineId = args.get_item(2)?.extract()?;
    let machine: &str = args.get_item(3)?.extract()?;
    let command: &str = args.get_item(4)?.extract()?;
    let program: &Program = args.get_item(5)?.extract()?;

    n3_execute(
        py, id_primary, id_local, id_world, machine, command, program, kwargs,
    )
}

#[allow(clippy::too_many_arguments)]
pub(self) fn n3_execute(
    py: Python,
    id_primary: MachineId,
    id_local: MachineId,
    id_world: MachineId,
    machine: &str,
    command: &str,
    program: &Program,
    kwargs: &PyAny,
) -> PyResult<()> {
    let is_root = id_primary == 0;

    let mut machine_token = machine.split(':');

    // Step 1. Load the program
    let mut program = n3_program::Program::load(program).unwrap();

    // Step 2. Attach variables
    if program.env.is_none() {
        program.env = Some(Default::default());
    }
    let env = program.env.as_mut().unwrap();
    env.insert("id".to_string(), Some(id_local.into()));
    env.insert(
        "machine".to_string(),
        Some(machine_token.next().unwrap().to_string().into()),
    );

    env.insert("is root".to_string(), Some(is_root.into()));

    let device_id = machine_token.next().unwrap_or("0").to_string();

    // Step 3. Ready for DDP
    {
        let env = py.import("os")?.get("environ")?;
        env.set_item("MASTER_ADDR", "localhost")?; // TODO: to be implemented
        env.set_item("MASTER_PORT", PORT.to_string())?;

        env.set_item("RANK", id_primary.to_string())?;
        env.set_item("LOCAL_RANK", id_local.to_string())?;
        env.set_item("WORLD_SIZE", id_world.to_string())?;

        env.set_item("CUDA_VISIBLE_DEVICES", device_id)?;
    }

    // Step 4. Define the node in REPL
    let program = program.build(py, ())?.into_py(py);

    // Step 5. Do its own work
    if let Err(e) = program.call_method1(py, command, (kwargs,)) {
        e.print_and_set_sys_last_vars(py);
        // manually stop & send the error message
        kwargs.set_item("is_running", false.into_py(py))?;
        kwargs.set_item("error_msg", e.to_string())?;
    }
    Ok(())
}
