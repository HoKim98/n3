mod device;
mod host;
mod process;
mod python;

pub use n3_machine::{Error, Machine, MachineResult, Program, Query, Result};
pub use n3_torch_ffi::pyo3::{PyResult, Python};
pub use n3_torch_ffi::Torch;

pub use self::host::HostMachine;
pub use self::python::PyMachineImpl;

/// Define built-in machine generators here.
pub(crate) const BUILTIN_MACHINES: &[(&str, n3_machine::Generator)] =
    &[("cuda", crate::device::CudaMachine::try_new)];
