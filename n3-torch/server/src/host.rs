use std::ops::{Deref, DerefMut};

use n3_machine::{HostMachine as NativeHostMachine, Result};

use crate::BUILTIN_MACHINES;

pub struct HostMachine {
    host: NativeHostMachine,
}

impl HostMachine {
    pub fn try_new() -> Result<Self> {
        // register built-in machine generators
        let mut host = NativeHostMachine::default();
        for (name, generator) in BUILTIN_MACHINES {
            host.add_generator(name, *generator)?;
        }

        Ok(Self { host })
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
