pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MonitorError(MonitorError),
    MachineError(MachineError),
    ParseError(ParseError),
}

#[derive(Debug)]
pub enum MonitorError {}

#[derive(Debug)]
pub enum MachineError {}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedTokens { query: String },
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

impl_into_error!(MonitorError);
impl_into_error!(MachineError);
impl_into_error!(ParseError);
