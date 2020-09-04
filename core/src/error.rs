use std::collections::BTreeSet;

use n3_parser::error::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    ParseError(ParseError),
    BuildError(BuildError),
}

#[derive(Clone, Debug, PartialEq)]
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

impl Into<Error> for BuildError {
    fn into(self) -> Error {
        Error::BuildError(self)
    }
}
