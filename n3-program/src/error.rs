use bincode::ErrorKind as BincodeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    BincodeError(BincodeError),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        // note: only test the types
        matches!(
            (self, other),
            (Self::IOError(_), Self::IOError(_)) | (Self::BincodeError(_), Self::BincodeError(_)),
        )
    }
}

impl<T> From<Box<T>> for Error
where
    T: Into<Self>,
{
    fn from(error: Box<T>) -> Self {
        (*error).into()
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<BincodeError> for Error {
    fn from(error: BincodeError) -> Self {
        Self::BincodeError(error)
    }
}
