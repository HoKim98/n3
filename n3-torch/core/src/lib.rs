mod code;
mod device;
mod exec;
mod host;
mod process;
mod python;

pub use n3_machine::Result;
pub use n3_torch_ffi::{self as ffi, pyo3, SignalHandler};

pub use self::host::HostMachine;

use self::process::ProcessMachine;

/// Define built-in machine generators here.
pub(crate) const BUILTIN_MACHINES: &[(&str, n3_machine::Generator)] =
    &[("cuda", ProcessMachine::try_new::<self::device::CudaMachine>)];
