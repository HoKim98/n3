pub use n3_machine_ffi::{Error as MachineError, ParseError};

use crate::Query;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoadError(LoadError),
    MachineError(MachineError),
}

#[derive(Debug)]
pub enum LoadError {
    NoSuchMachine { query: Query },
}

impl From<LoadError> for Error {
    fn from(error: LoadError) -> Self {
        Self::LoadError(error)
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
