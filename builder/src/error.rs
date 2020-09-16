use std::collections::BTreeSet;

use n3_parser::error::ParseError;

use crate::ast;

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
    NoSuchNode {
        name: String,
    },
    MismatchedNodeName {
        expected: String,
        given: String,
    },
    MismatchedGraphNodeId {
        expected: u64,
        given: u64,
    },
    MismatchedGraphNodeSize {
        expected: &'static [&'static str],
        given: usize,
    },
    MismatchedGraphNodeShapesExistence {
        expected: bool,
        given: bool,
    },
    MismatchedGraphCallName {
        expected: Vec<&'static str>,
        given: String,
    },
    MismatchedGraphCallSize {
        expected: Vec<&'static str>,
        given: usize,
    },
    MismatchedGraphCallInputs {
        expected: ast::GraphInputsType,
        given: ast::GraphInputsType,
    },
    MismatchedGraphCallRepeat {
        expected: bool,
        given: bool,
    },
    MismatchedGraphCallArgs {
        expected: &'static [&'static str],
        given: Vec<String>,
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

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
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
