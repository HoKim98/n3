use crate::Query;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    LoadError(LoadError),
    MachineError(MachineError),
}

pub type MachineError = n3_machine_ffi::Error;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedTokens { query: String },
}

#[derive(Debug)]
pub enum LoadError {
    NoSuchMachine { query: Query },
}

macro_rules! impl_into_error(
    ($t:ident) => {
        impl From<$t> for Error {
            fn from(error: $t) -> Self {
                Self::$t(error.into())
            }
        }

        impl<T> From<$t> for Result<T> {
            fn from(error: $t) -> Self {
                Err(Error::from(error))
            }
        }
    }
);

impl_into_error!(ParseError);
impl_into_error!(LoadError);

impl From<MachineError> for Error {
    fn from(error: MachineError) -> Self {
        Self::MachineError(error)
    }
}
