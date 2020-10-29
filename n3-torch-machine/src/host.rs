use std::ops::{Deref, DerefMut};

use n3_machine::HostMachine as NativeHostMachine;
use n3_torch_ffi::pyo3::GILGuard;

use crate::process::{ProcessMachine, ProcessMachineImpl};
use crate::{Error, PyResult, Python, Result, Torch, BUILTIN_MACHINES};

pub struct HostMachine {
    host: NativeHostMachine,
    is_running: bool,
    // GILGuard is required to make Python GIL alive.
    _py: GILGuard,
}

impl HostMachine {
    pub fn try_new() -> Result<Self> {
        // acquire Python GIL first
        let _py = Python::acquire_gil();

        // register built-in machine generators
        let mut host = NativeHostMachine::default();
        for (name, generator) in BUILTIN_MACHINES {
            host.add_generator(name, *generator)?;
        }

        Ok(Self {
            host,
            is_running: true,
            _py,
        })
    }

    pub fn add_process_generator<T>(&mut self, query: &str) -> Result<()>
    where
        T: ProcessMachineImpl + 'static,
    {
        self.add_generator(query, ProcessMachine::try_new::<T>)
    }

    pub fn py_terminate(&mut self) -> PyResult<()> {
        if !self.is_running {
            self.is_running = false;
            Torch(self._py.python()).terminate()?;
        }
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
        self.py_terminate()
            .map_err(|e| e.into())
            .map_err(Error::MachineError)
    }
}

impl Deref for HostMachine {
    type Target = NativeHostMachine;

    fn deref(&self) -> &Self::Target {
        &self.host
    }
}

impl DerefMut for HostMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.host
    }
}

impl Drop for HostMachine {
    fn drop(&mut self) {
        self.terminate().unwrap()
    }
}
