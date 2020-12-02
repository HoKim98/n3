pub use n3_machine_ffi::{Error as MachineError, ParseError};
use n3_machine_ffi::{Query, WorkId};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoadError(LoadError),
    WorkError(WorkError),
    MachineError(MachineError),
}

#[derive(Debug)]
pub enum LoadError {
    NoSuchMachine { query: Query },
}

#[derive(Debug)]
pub enum WorkError {
    NoSuchWork { id: WorkId },
}

impl From<LoadError> for Error {
    fn from(error: LoadError) -> Self {
        Self::LoadError(error)
    }
}

impl From<WorkError> for Error {
    fn from(error: WorkError) -> Self {
        Self::WorkError(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::MachineError(error.into())
    }
}

impl From<MachineError> for Error {
    fn from(error: MachineError) -> Self {
        Self::MachineError(error)
    }
}

impl<T> From<LoadError> for Result<T> {
    fn from(error: LoadError) -> Self {
        Err(Error::from(error))
    }
}

impl<T> From<WorkError> for Result<T> {
    fn from(error: WorkError) -> Self {
        Err(Error::from(error))
    }
}
