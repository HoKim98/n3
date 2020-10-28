mod generic;
mod host;

pub use self::generic::GenericMachine;
pub use self::host::HostMachine;

use std::ops::{Deref, DerefMut};

use pyo3::prelude::*;

use crate::torch::Torch;

pub trait MachineImpl<'a>
where
    Self: Deref<Target = MachineData<'a>> + DerefMut,
{
}

pub struct Machine<'a> {
    inner: Box<dyn MachineImpl<'a> + 'a>,
}

impl<'a, T> From<T> for Machine<'a>
where
    T: MachineImpl<'a> + 'a,
{
    fn from(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<'a> Deref for Machine<'a> {
    type Target = MachineData<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Machine<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Machine<'a> {
    pub fn terminate(&mut self) -> PyResult<()> {
        if self._is_running {
            self._is_running = false;
            self.torch.terminate()?;
        }
        Ok(())
    }
}

pub struct MachineData<'a> {
    pub py: Python<'a>,
    pub torch: Torch<'a>,
    _is_running: bool,
}

impl<'a> MachineData<'a> {
    pub(super) fn new(py: Python<'a>) -> Self {
        Self {
            py,
            torch: Torch(py),
            _is_running: true,
        }
    }
}

impl<'a> Drop for MachineData<'a> {
    fn drop(&mut self) {
        if self._is_running {
            warn!(r#"The machine should be dropped manually! Use "terminate()" instead."#);
        }
    }
}
