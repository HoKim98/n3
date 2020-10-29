mod error;
mod host;

pub use n3_machine_ffi::{Machine, Program, Query, Result as MachineResult};

pub use self::error::{Error, Result};
pub use self::host::{Generator, HostMachine};
