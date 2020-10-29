use std::ops::{Deref, DerefMut};

use n3_torch_ffi::pyo3::{PyResult, Python};
use n3_torch_ffi::pyo3_mp::Process;

use n3_machine::{Machine, Program, Query};

use crate::exec::n3_execute_wrapper;
use crate::python::{PyMachineBase, PyMachineImpl};

pub struct ProcessMachine {
    process: Process<'static>,
    query: Query,
}

impl ProcessMachine {
    pub unsafe fn try_new<T>(query: &Query) -> Option<Box<dyn Machine>>
    where
        T: ProcessMachineImpl + 'static,
    {
        T::verify_query(query)
            .map(|x| ProcessMachine::_new(x))
            .flatten()
            .map(|x| T::try_new(x))
            .map(PyMachineBase)
            .map(|x| x.into_box_trait())
    }

    unsafe fn _new(query: Query) -> Option<Self> {
        let py = Python::assume_gil_acquired();

        Some(Self {
            process: Process::new(py).ok()?,
            query,
        })
    }
}

impl PyMachineImpl for ProcessMachine {
    fn is_running(&self) -> bool {
        self.process.is_running()
    }

    fn py_spawn(&mut self, id: usize, program: &Program) -> PyResult<()> {
        // the GIL is acquired by HostMachine
        let py = unsafe { Python::assume_gil_acquired() };

        // the machine's name
        let machine = format!("{}", self.query);

        // the function to execute the program
        let n3_execute = n3_execute_wrapper(py)?;

        // spawn to new process
        self.process
            .spawn(n3_execute, (id, &machine, program), None)?;
        Ok(())
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.process.join()
    }
}

pub trait ProcessMachineImpl: PyMachineImpl {
    /// # Safety
    ///
    /// This function should not be called before the Python GIL is ready.
    unsafe fn try_new(process: ProcessMachine) -> Self
    where
        Self: Sized;

    fn verify_query(query: &Query) -> Option<Query>;
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
