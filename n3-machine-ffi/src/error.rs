pub type Result<T> = std::result::Result<T, Error>;

pub type NetError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum Error {
    QueryError(QueryError),
    NetError(NetError),
    DeviceError(String),
}

#[derive(Debug)]
pub enum QueryError {
    UnexpectedTokens { query: String },
    EmptyMachines,
}

impl From<QueryError> for Error {
    fn from(error: QueryError) -> Self {
        Self::QueryError(error)
    }
}

impl From<NetError> for Error {
    fn from(error: NetError) -> Self {
        Self::NetError(error)
    }
}

impl From<&'static str> for Error {
    fn from(error: &'static str) -> Self {
        Self::NetError(error.into())
    }
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(error)
    }
}

impl<T> From<QueryError> for Result<T> {
    fn from(error: QueryError) -> Self {
        Err(error.into())
    }
}
