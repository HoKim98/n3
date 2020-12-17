use std::mem::ManuallyDrop;

use pyo3::prelude::*;

use n3_machine::PORT;
use n3_machine_ffi::Program;

use crate::args::parse_python_path;
use crate::code::BuildCode;
use crate::handler::PyHandler;

pub fn n3_execute(py: Python, program: &Program, handler: PyHandler) -> PyResult<()> {
    let is_root = program.id.primary == 0;
    let is_distributed = program.id.world > 1;

    let mut machine_token = program.machine.split(':');

    // Step 1. Load the program text
    let mut text = n3_program::Program::load(&*program.text).unwrap();

    // Step 2. Attach variables
    if text.env.is_none() {
        text.env = Some(Default::default());
    }
    let env = text.env.as_mut().unwrap();
    env.insert("id".to_string(), Some(program.id.local.into()));
    env.insert(
        "machine".to_string(),
        Some(machine_token.next().unwrap().to_string().into()),
    );

    env.insert("is root".to_string(), Some(is_root.into()));
    env.insert("is distributed".to_string(), Some(is_distributed.into()));

    let device_id = machine_token.next().unwrap_or("0").to_string();

    // Step 3. Ready for DDP
    {
        let env = py.import("os")?.get("environ")?;
        env.set_item("MASTER_ADDR", program.id.master_addr.to_string())?;
        env.set_item("MASTER_PORT", PORT.to_string())?;

        env.set_item("RANK", program.id.primary.to_string())?;
        env.set_item("LOCAL_RANK", program.id.local.to_string())?;
        env.set_item("WORLD_SIZE", program.id.world.to_string())?;

        env.set_item("CUDA_VISIBLE_DEVICES", device_id)?;

        // set python path to spawn the processes (workers)
        py.import("multiprocessing")?
            .call1("set_executable", (parse_python_path(),))?;
    }

    // Step 4. Define the node in REPL
    let text = text.build(py, ())?.into_py(py);

    // Step 5. Do its own work
    text.call_method1(py, &program.command, (handler,))?;
    Ok(())
}

/// # Safety
///
/// This function should be called when the Python interpreter is idle.
pub unsafe fn finalize_python() {
    // The GILGuard is acquired to finalize the Python interpreter.
    // Then, it should not be dropped normally, because the memory is already dropped.
    //
    // The GILGuard itself is under Stack, so it is unnecessary to manually drop the struct.
    let _gil = ManuallyDrop::new(pyo3::Python::acquire_gil());
    pyo3::ffi::Py_FinalizeEx();
}
