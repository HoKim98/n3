mod handler;

pub use pyo3;

use std::ops::{Deref, DerefMut};

use pyo3::PyResult;

use n3_machine_ffi::{MachineIdSet, Program, Query};

pub use self::handler::SignalHandler;

pub trait PyMachine {
    fn is_running(&self) -> bool;

    fn py_spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
        handler: SignalHandler,
    ) -> PyResult<()>;
    fn py_terminate(&mut self) -> PyResult<()>;
}

pub trait ProcessMachine<P>: PyMachine {
    /// # Safety
    ///
    /// This function should not be called before the Python GIL is ready.
    unsafe fn try_new(process: P) -> Self
    where
        Self: Sized;

    fn verify_query(query: &Query) -> Vec<Query>;
}

impl<T, P> PyMachine for T
where
    T: ProcessMachine<P> + Deref<Target = P> + DerefMut,
    P: PyMachine,
{
    fn is_running(&self) -> bool {
        self.deref().is_running()
    }

    fn py_spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
        handler: SignalHandler,
    ) -> PyResult<()> {
        self.deref_mut().py_spawn(id, program, command, handler)
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.deref_mut().py_terminate()
    }
}
