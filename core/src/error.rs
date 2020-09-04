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
        names: Vec<String>,
    },
    NoSuchVariable {
        name: String,
        candidates: Vec<String>,
    },
}

impl Into<Error> for BuildError {
    fn into(self) -> Error {
        Error::BuildError(self)
    }
}
