use std::ops::{Deref, DerefMut};

use pyo3::Python;

use super::{MachineData, MachineImpl};

pub struct GenericMachine<'a>(MachineData<'a>);

impl<'a> GenericMachine<'a> {
    pub fn new(py: Python<'a>) -> Self {
        Self(MachineData::new(py))
    }
}

impl<'a> Deref for GenericMachine<'a> {
    type Target = MachineData<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for GenericMachine<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> MachineImpl<'a> for GenericMachine<'a> {}
