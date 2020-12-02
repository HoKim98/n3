pub extern crate pyo3;

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

use pyo3::{PyObject, PyResult};

use n3_machine_ffi::{MachineIdSet, Program, Query};

pub trait PyMachine {
    fn is_running(&self) -> bool;

    fn py_spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
    ) -> PyResult<PyObject>;

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
    ) -> PyResult<PyObject> {
        self.deref_mut().py_spawn(id, program, command)
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.deref_mut().py_terminate()
    }
}

/// # Safety
///
/// This function should be called when the Python interpreter is idle.
pub unsafe fn finalize_python() {
    static FINALIZED: AtomicBool = AtomicBool::new(false);
    if FINALIZED.compare_and_swap(false, true, Ordering::SeqCst) {
        return;
    }

    // The GILGuard is acquired to finalize the Python interpreter.
    // Then, it should not be dropped normally, because the memory is already dropped.
    //
    // The GILGuard itself is under Stack, so it is unnecessary to manually drop the struct.
    let gil = ManuallyDrop::new(pyo3::Python::acquire_gil());
    pyo3::ffi::Py_FinalizeEx();
    drop(gil);
}
