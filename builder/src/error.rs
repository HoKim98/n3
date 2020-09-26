use std::collections::BTreeSet;
use std::path::PathBuf;

use glob::{GlobError, PatternError};

use n3_parser::error::ParseError;

use crate::ast;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    BuildError(BuildError),
    ExecBuildError(ExecBuildError),
    ExecError(ExecError),
    ExternalError(ExternalError),
}

#[derive(Debug, PartialEq)]
pub enum BuildError {
    TensorNodeError(TensorNodeError),
    GraphError(GraphError),
    GraphNodeError(GraphNodeError),
    GraphCallError(GraphCallError),
    LinkError(LinkError),
}

#[derive(Debug, PartialEq)]
pub enum ExecBuildError {
    UnexpectedWiths,
    UnexpectedChildren,
    UnexpectedGraph,
    MismatchedNodeType {
        expected: ast::LetNodeType,
        given: ast::LetNodeType,
    },
}

#[derive(Debug, PartialEq)]
pub enum ExecError {
    NoSuchDirectory { path: PathBuf },
    NotDirectory { path: PathBuf },
}

#[derive(Debug, PartialEq)]
pub enum TensorNodeError {
    NoSuchNode {
        name: String,
    },
    MismatchedName {
        expected: String,
        given: String,
    },
    MismatchedType {
        expected: ast::FinalNodeType,
        given: ast::FinalNodeType,
    },
}

#[derive(Debug, PartialEq)]
pub enum GraphError {
    NoSuchVariable {
        name: String,
        candidates: BTreeSet<String>,
    },
    DuplicatedVariable {
        name: String,
    },
    CycledVariables {
        names: BTreeSet<String>,
    },
    EmptyValue {
        name: String,
        expected: ast::LetType,
    },
    MismatchedType {
        name: String,
        expected: ast::LetType,
        given: Option<ast::LetType>,
    },
}

#[derive(Debug, PartialEq)]
pub enum GraphNodeError {
    NoSuchInput {
        out: ast::Out,
    },
    MismatchedId {
        expected: u64,
        given: u64,
    },
    MismatchedSize {
        expected: &'static [&'static str],
        given: usize,
    },
    MismatchedShapesExistence {
        expected: bool,
        given: bool,
    },
}

#[derive(Debug, PartialEq)]
pub enum GraphCallError {
    EmptyInputs,
    GenericListInputShape {
        index: usize,
    },
    GenericShape {
        name: String,
    },
    GenericShapes,
    MismatchedName {
        expected: Vec<&'static str>,
        given: String,
    },
    MismatchedSize {
        expected: Vec<&'static str>,
        given: usize,
    },
    MismatchedInputsType {
        expected: ast::GraphInputsType,
        given: ast::GraphInputsType,
    },
    MismatchedRepeat {
        expected: bool,
        given: bool,
    },
    MismatchedAxis {
        val_min: i64,
        val_max: i64,
        given: i64,
    },
    MismatchedArgType {
        expected: ast::LetType,
        given: Option<ast::LetType>,
    },
    MismatchedArgs {
        expected: &'static [&'static str],
        given: Vec<String>,
    },
    MismatchedShapes {
        expected: usize,
        given: usize,
    },
    MismatchedShapeKeys {
        expected: Vec<String>,
        given: Vec<String>,
    },
}

#[derive(Debug)]
pub enum LinkError {
    MismatchedDim {
        expected: ast::Value,
        given: ast::Value,
    },
    MismatchedShape {
        expected: ast::Shape,
        given: ast::Shape,
    },
}

#[derive(Debug)]
pub enum ExternalError {
    IOError(std::io::Error),
    GlobError(GlobError),
    PatternError(PatternError),
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

impl PartialEq for LinkError {
    fn eq(&self, other: &Self) -> bool {
        // note: only test the types
        match (self, other) {
            (
                Self::MismatchedDim {
                    expected: _,
                    given: _,
                },
                Self::MismatchedDim {
                    expected: _,
                    given: _,
                },
            ) => true,
            (
                Self::MismatchedShape {
                    expected: _,
                    given: _,
                },
                Self::MismatchedShape {
                    expected: _,
                    given: _,
                },
            ) => true,
            _ => false,
        }
    }
}

impl PartialEq for ExternalError {
    fn eq(&self, other: &Self) -> bool {
        // note: only test the types
        match (self, other) {
            (Self::IOError(_), Self::IOError(_)) => true,
            (Self::GlobError(_), Self::GlobError(_)) => true,
            (Self::PatternError(_), Self::PatternError(_)) => true,
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

impl From<ExecBuildError> for Error {
    fn from(error: ExecBuildError) -> Self {
        Self::ExecBuildError(error)
    }
}

impl From<ExecError> for Error {
    fn from(error: ExecError) -> Self {
        Self::ExecError(error)
    }
}

impl<T> From<ExecBuildError> for Result<T> {
    fn from(error: ExecBuildError) -> Self {
        Err(Error::from(error))
    }
}

impl<T> From<ExecError> for Result<T> {
    fn from(error: ExecError) -> Self {
        Err(Error::from(error))
    }
}

macro_rules! impl_into_build_error(
    ($t:ident) => {
        impl From<$t> for BuildError {
            fn from(error: $t) -> Self {
                Self::$t(error)
            }
        }

        impl From<$t> for Error {
            fn from(error: $t) -> Self {
                Self::BuildError(error.into())
            }
        }

        impl<T> From<$t> for Result<T> {
            fn from(error: $t) -> Self {
                Err(Error::from(error))
            }
        }
    }
);

macro_rules! impl_into_external_error(
    ($t:ident) => {
        impl From<$t> for ExternalError {
            fn from(error: $t) -> Self {
                Self::$t(error)
            }
        }

        impl From<$t> for Error {
            fn from(error: $t) -> Self {
                Self::ExternalError(error.into())
            }
        }
    }
);

impl_into_build_error!(TensorNodeError);
impl_into_build_error!(GraphError);
impl_into_build_error!(GraphNodeError);
impl_into_build_error!(GraphCallError);
impl_into_build_error!(LinkError);

impl_into_external_error!(GlobError);
impl_into_external_error!(PatternError);

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::ExternalError(ExternalError::IOError(error))
    }
}
