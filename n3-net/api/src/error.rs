pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    BuildError(n3_builder::Error),
    MachineError(n3_machine_ffi::Error),
}

impl From<n3_builder::Error> for Error {
    fn from(error: n3_builder::Error) -> Self {
        Self::BuildError(error)
    }
}

impl From<n3_machine_ffi::Error> for Error {
    fn from(error: n3_machine_ffi::Error) -> Self {
        Self::MachineError(error)
    }
}
