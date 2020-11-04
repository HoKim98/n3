use std::ops::{Deref, DerefMut};

use n3_machine::Query;
use n3_torch_ffi::ProcessMachine;

use crate::process::ProcessMachine as ProcessMachineImpl;

pub struct CudaMachine(ProcessMachineImpl);

impl ProcessMachine<ProcessMachineImpl> for CudaMachine {
    unsafe fn try_new(process: ProcessMachineImpl) -> Self {
        Self(process)
    }

    fn verify_query(query: &Query) -> Vec<Query> {
        vec![query.clone()]
    }
}

impl Deref for CudaMachine {
    type Target = ProcessMachineImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CudaMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
