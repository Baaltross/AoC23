use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum GenericError {
    ParseIntError(ParseIntError),
    IOError(std::io::Error),
}

impl From<ParseIntError> for GenericError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<std::io::Error> for GenericError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::ParseIntError(e) => write!(f, "invalid integer: {}", e),
            Self::IOError(e) => write!(f, "io error: {}", e)
        }
    }
}

pub type GenericResult<T> = Result<T, GenericError>;