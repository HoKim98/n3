mod blocker;
mod device;
mod exec;
mod host;
mod process;
mod python;

pub use self::exec::PyInit_n3_torch_ffi;
pub use self::host::HostMachine;

use self::process::ProcessMachine;

/// Define built-in machine generators here.
pub(crate) const BUILTIN_MACHINES: &[(&str, n3_machine::Generator)] =
    &[("cuda", ProcessMachine::try_new::<self::device::CudaMachine>)];
