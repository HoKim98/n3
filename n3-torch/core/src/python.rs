use pyo3::Python;

use n3_machine::{Machine, MachineError, MachineIdSet, MachineResult, Program};
use n3_torch_ffi::{pyo3, PyMachine};

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
    fn spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
        handler: n3_machine::SignalHandler,
    ) -> MachineResult<()> {
        Ok(self
            .0
            .py_spawn(id, program, command, handler.into())
            .map_err(|e| {
                Python::with_gil(|py| {
                    e.print_and_set_sys_last_vars(py);
                    e
                })
            })
            .map_err(|x| x.into())
            .map_err(MachineError::ExternalError)?)
    }

    fn join(&mut self) -> MachineResult<()> {
        self.terminate()
    }

    fn terminate(&mut self) -> MachineResult<()> {
        Ok(self
            .0
            .py_terminate()
            .map_err(|x| x.into())
            .map_err(MachineError::ExternalError)?)
    }
}
