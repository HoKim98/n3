use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

use pyo3::{GILGuard, Python};

use n3_machine::{HostMachine as NativeHostMachine, Result};
use n3_torch_ffi::{finalize_python, pyo3, ProcessMachine as ProcessMachineTrait};

use crate::process::ProcessMachine;
use crate::BUILTIN_MACHINES;

pub struct HostMachine {
    host: ManuallyDrop<NativeHostMachine>,
    // GILGuard is required to make Python GIL alive.
    _py: ManuallyDrop<GILGuard>,
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
            host: ManuallyDrop::new(host),
            _py: ManuallyDrop::new(_py),
        })
    }

    pub fn add_process_generator<T>(&mut self, query: &str) -> Result<()>
    where
        T: ProcessMachineTrait<ProcessMachine> + 'static,
    {
        self.add_generator(query, ProcessMachine::try_new::<T>)
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
        self.host.join_all();
        // drop order: GIL -> Python -> host
        unsafe {
            ManuallyDrop::drop(&mut self._py);
            finalize_python();
            ManuallyDrop::drop(&mut self.host);
        }
    }
}
