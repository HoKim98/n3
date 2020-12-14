pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    QueryError(QueryError),
    NetError(NetError),
    SMPError(SMPError),
    DeviceError(String),
}

#[derive(Debug)]
pub enum QueryError {
    UnexpectedTokens { query: String },
    EmptyMachines,
}

#[derive(Debug)]
pub struct NetError(pub Box<dyn std::error::Error>);

#[derive(Debug)]
pub struct SMPError(pub Box<dyn std::error::Error>);

impl From<QueryError> for Error {
    fn from(error: QueryError) -> Self {
        Self::QueryError(error)
    }
}

impl<T> From<T> for NetError
where
    T: 'static + std::error::Error,
{
    fn from(error: T) -> Self {
        Self(Box::new(error))
    }
}

impl From<NetError> for Error {
    fn from(error: NetError) -> Self {
        Self::NetError(error)
    }
}

impl<T> From<T> for SMPError
where
    T: 'static + std::error::Error,
{
    fn from(error: T) -> Self {
        Self(Box::new(error))
    }
}

impl From<SMPError> for Error {
    fn from(error: SMPError) -> Self {
        Self::SMPError(error)
    }
}

impl From<&'static str> for Error {
    fn from(error: &'static str) -> Self {
        Self::NetError(NetError(error.into()))
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

impl<T> From<SMPError> for Result<T> {
    fn from(error: SMPError) -> Self {
        Err(Error::from(error))
    }
}
