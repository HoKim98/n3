mod error;
mod host;

pub use self::error::{Error, MachineError, Result};
pub use self::host::{Generator, HostMachine};

pub const PORT: u16 = 40961;
