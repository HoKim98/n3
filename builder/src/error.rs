use std::collections::BTreeSet;

use n3_parser::error::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    BuildError(BuildError),
    ExternalError(ExternalError),
}

#[derive(Debug, PartialEq)]
pub enum BuildError {
    DuplicatedVariable {
        name: String,
    },
    CycledVariables {
        names: BTreeSet<String>,
    },
    NoSuchVariable {
        name: String,
        candidates: BTreeSet<String>,
    },
}

#[derive(Debug)]
pub enum ExternalError {
    IOError(std::io::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ParseError(a), Self::ParseError(b)) => a.eq(b),
            (Self::BuildError(a), Self::BuildError(b)) => a.eq(b),
            (Self::ExternalError(a), Self::ExternalError(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl PartialEq for ExternalError {
    fn eq(&self, other: &Self) -> bool {
        // note: only test the types
        match (self, other) {
            (Self::IOError(_), Self::IOError(_)) => true,
            _ => false,
        }
    }
}

impl From<BuildError> for Error {
    fn from(error: BuildError) -> Self {
        Self::BuildError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::ExternalError(ExternalError::IOError(error))
    }
}
