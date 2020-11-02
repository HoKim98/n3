use n3_machine::{Machine, MachineResult, Program};
use n3_torch_ffi::PyMachine;

pub struct PyMachineBase<T>(pub T)
where
    T: PyMachine + 'static;

impl<T> PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    pub fn into_box_trait(self) -> Box<dyn Machine> {
        Box::new(self)
    }
}

impl<T> Machine for PyMachineBase<T>
where
    T: PyMachine,
{
    fn spawn(&mut self, id: usize, program: &Program, command: &str) -> MachineResult<()> {
        Ok(self.0.py_spawn(id, program, command)?)
    }

    fn join(&mut self) -> MachineResult<()> {
        self.terminate()
    }

    fn terminate(&mut self) -> MachineResult<()> {
        Ok(self.0.py_terminate()?)
    }
}
