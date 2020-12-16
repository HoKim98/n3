use n3_machine_ffi::WorkId;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BuildError(n3_builder::Error),
    MachineError(n3_machine_ffi::Error),
    NoSuchWork { id: WorkId },
    RequireAKey,
    // Exposing database error can cause fatal security problem.
    DatabaseError,
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

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Self::DatabaseError
    }
}
