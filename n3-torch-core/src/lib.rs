mod device;
mod exec;
mod graph;
mod host;
mod process;
mod python;

pub use n3_torch_ffi::pyo3;

pub use self::host::HostMachine;

use self::process::ProcessMachine;

/// Define built-in machine generators here.
pub(crate) const BUILTIN_MACHINES: &[(&str, n3_machine::Generator)] =
    &[("cuda", ProcessMachine::try_new::<self::device::CudaMachine>)];
