use std::ops::{Deref, DerefMut};

use n3_torch_ffi::pyo3_mp::Process;

use crate::python::PyMachineImpl;
use crate::{Program, PyResult, Python};

pub struct ProcessMachine {
    process: Process<'static>,
}

impl ProcessMachine {
    pub unsafe fn try_new() -> Option<Self> {
        let py = Python::assume_gil_acquired();

        Some(Self {
            process: Process::new(py).ok()?,
        })
    }
}

impl PyMachineImpl for ProcessMachine {
    fn is_running(&self) -> bool {
        self.process.is_running()
    }

    fn py_spawn(&mut self, id: usize, program: &Program) -> PyResult<()> {
        dbg!(&id, program.len());
        todo!();
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.process.join()
    }
}

impl<T> PyMachineImpl for T
where
    T: Deref<Target = ProcessMachine> + DerefMut,
{
    fn is_running(&self) -> bool {
        self.deref().is_running()
    }

    fn py_spawn(&mut self, id: usize, program: &Program) -> PyResult<()> {
        self.deref_mut().py_spawn(id, program)
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.deref_mut().py_terminate()
    }
}
