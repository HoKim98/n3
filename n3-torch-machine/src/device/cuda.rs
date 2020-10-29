use std::ops::{Deref, DerefMut};

use crate::process::ProcessMachine;
use crate::python::PyMachineBase;
use crate::{Machine, Query};

pub struct CudaMachine(ProcessMachine);

impl CudaMachine {
    pub unsafe fn try_new(query: &Query) -> Option<Box<dyn Machine>> {
        ProcessMachine::try_new()
            .map(Self)
            .map(PyMachineBase)
            .map(|x| x.into_box_trait())
    }
}

impl Deref for CudaMachine {
    type Target = ProcessMachine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CudaMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
