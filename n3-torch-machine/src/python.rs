use n3_machine::{Machine, MachineResult, Program};
use n3_torch_ffi::pyo3::PyResult;

pub trait PyMachineImpl {
    fn is_running(&self) -> bool;

    fn py_spawn(&mut self, id: usize, program: &Program) -> PyResult<()>;
    fn py_terminate(&mut self) -> PyResult<()>;
}

pub struct PyMachineBase<T>(pub T)
where
    T: PyMachineImpl + 'static;

impl<T> PyMachineBase<T>
where
    T: PyMachineImpl + 'static,
{
    pub fn into_box_trait(self) -> Box<dyn Machine> {
        Box::new(self)
    }
}

impl<T> Machine for PyMachineBase<T>
where
    T: PyMachineImpl,
{
    fn spawn(&mut self, id: usize, program: &Program) -> MachineResult<()> {
        Ok(self.0.py_spawn(id, program)?)
    }

    fn join(&mut self) -> MachineResult<()> {
        self.terminate()
    }

    fn terminate(&mut self) -> MachineResult<()> {
        Ok(self.0.py_terminate()?)
    }
}
