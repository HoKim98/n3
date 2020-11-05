pub type Result<T> = std::result::Result<T, Error>;

pub type ExternalError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    ExternalError(ExternalError),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedTokens { query: String },
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl From<ExternalError> for Error {
    fn from(error: ExternalError) -> Self {
        Self::ExternalError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::ExternalError(error.into())
    }
}

impl From<&'static str> for Error {
    fn from(error: &'static str) -> Self {
        Self::ExternalError(error.into())
    }
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(error)
    }
}

impl<T> From<ParseError> for Result<T> {
    fn from(error: ParseError) -> Self {
        Err(error.into())
    }
}
