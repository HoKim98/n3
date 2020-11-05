mod error;
mod host;

pub use n3_machine_ffi::{
    JobId, LocalQuery, Machine, MachineId, MachineIdSet, Program, Query, Result as MachineResult,
    SignalHandler,
};

pub use self::error::{Error, MachineError, Result};
pub use self::host::{Generator, HostMachine};

pub const PORT: u16 = 40961;
