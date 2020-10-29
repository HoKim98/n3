use std::ops::{Deref, DerefMut};

use crate::process::ProcessMachine;
use crate::process::ProcessMachineImpl;
use crate::Query;

pub struct CudaMachine {
    process: ProcessMachine,
    query: Query,
}

impl ProcessMachineImpl for CudaMachine {
    unsafe fn try_new(process: ProcessMachine, query: &Query) -> Option<Self> {
        Some(Self {
            process,
            query: query.clone(),
        })
    }
}

impl Deref for CudaMachine {
    type Target = ProcessMachine;

    fn deref(&self) -> &Self::Target {
        &self.process
    }
}

impl DerefMut for CudaMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.process
    }
}
