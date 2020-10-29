use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyCFunction};
use pyo3::{wrap_pyfunction, wrap_pymodule};

use n3_machine::Program;
pub use n3_torch_ffi::PyInit_n3_torch_ffi;
use n3_torch_ffi::{pyo3, Torch};

use crate::blocker::ImportBlocker;

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute)(py)
}

#[pyfunction]
fn n3_execute(
    py: Python,
    id: usize,
    machine: &str,
    command: &str,
    program: &Program,
) -> PyResult<()> {
    let program = n3_builder::Program::load(program).unwrap();
    dbg!(id, machine);

    let builtins = py.import("builtins")?.into_py(py);
    let sys = py.import("sys")?.into_py(py);

    let torch = Torch(py);

    let n3 = wrap_pymodule!(n3_torch_ffi)(py);

    // make n3 importable
    let blocker = ImportBlocker { module: n3.clone() }.into_py(py);
    py.run(
        "sys.meta_path += [blocker]",
        Some([("__builtins__", &builtins), ("sys", &sys)].into_py_dict(py)),
        Some([("blocker", blocker)].into_py_dict(py)),
    )?;

    let main = &program.scripts["__main__"];

    // execute the script
    py.run(
        &format!(
            r#"{}

# let's define the trainer
trainer = {}()

# let the trainer do its own command
trainer.{}()
"#,
            &main.source, &main.name, command,
        ),
        Some([("__builtins__", builtins), ("n3_torch_ffi", n3)].into_py_dict(py)),
        None,
    )
}
