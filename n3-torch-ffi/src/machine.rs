use pyo3::{PyResult, Python};

pub use n3_machine::*;

use crate::torch::Torch;

pub struct PyMachine<'a> {
    pub py: Python<'a>,
    pub torch: Torch<'a>,
    is_running: bool,
}

impl<'a> PyMachine<'a> {
    pub fn new(py: Python<'a>) -> Self {
        Self {
            py,
            torch: Torch(py),
            is_running: true,
        }
    }

    pub fn py_terminate(&mut self) -> PyResult<()> {
        if self.is_running {
            self.is_running = false;
            self.torch.terminate()?;
        }
        Ok(())
    }
}

impl<'a> Machine for PyMachine<'a> {
    fn spawn(&mut self, program: &Program) -> MachineResult<()> {
        todo!()
    }

    fn join(&mut self) -> MachineResult<()> {
        self.terminate()
    }

    fn terminate(&mut self) -> MachineResult<()> {
        Ok(self.py_terminate()?)
    }
}
