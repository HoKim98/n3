use std::ops::{Deref, DerefMut};

use n3_torch_ffi::pyo3_mp::Process;

use crate::python::{PyMachineBase, PyMachineImpl};
use crate::{Machine, Program, PyResult, Python, Query};

pub struct ProcessMachine {
    process: Process<'static>,
}

impl ProcessMachine {
    pub unsafe fn try_new<T>(query: &Query) -> Option<Box<dyn Machine>>
    where
        T: ProcessMachineImpl + 'static,
    {
        ProcessMachine::_new()
            .map(|x| T::try_new(x, query))
            .flatten()
            .map(PyMachineBase)
            .map(|x| x.into_box_trait())
    }

    unsafe fn _new() -> Option<Self> {
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

pub trait ProcessMachineImpl: PyMachineImpl {
    /// # Safety
    ///
    /// This function should not be called before the Python GIL is ready.
    unsafe fn try_new(process: ProcessMachine, query: &Query) -> Option<Self>
    where
        Self: Sized;
}

impl<T> PyMachineImpl for T
where
    T: ProcessMachineImpl + Deref<Target = ProcessMachine> + DerefMut,
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
