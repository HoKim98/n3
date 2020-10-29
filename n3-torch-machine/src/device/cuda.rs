use std::ops::{Deref, DerefMut};

use n3_machine::Query;

use crate::process::ProcessMachine;
use crate::process::ProcessMachineImpl;

pub struct CudaMachine(ProcessMachine);

impl ProcessMachineImpl for CudaMachine {
    unsafe fn try_new(process: ProcessMachine) -> Self {
        Self(process)
    }

    fn verify_query(query: &Query) -> Option<Query> {
        Some(query.clone())
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
