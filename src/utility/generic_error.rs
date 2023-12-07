use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug)]
pub enum GenericError {
    BasicError(String),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    IOError(std::io::Error),
    SscanfError(sscanf::Error),
    StrumParseError(strum::ParseError),
}

impl From<ParseIntError> for GenericError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseFloatError> for GenericError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl From<std::io::Error> for GenericError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<sscanf::Error> for GenericError {
    fn from(e: sscanf::Error) -> Self {
        Self::SscanfError(e)
    }
}

impl From<strum::ParseError> for GenericError {
    fn from(e: strum::ParseError) -> Self {
        Self::StrumParseError(e)
    }
}

impl Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::BasicError(e) => write!(f, "basic error: {}", e),
            Self::ParseIntError(e) => write!(f, "invalid integer: {}", e),
            Self::ParseFloatError(e) => write!(f, "invalid float: {}", e),
            Self::IOError(e) => write!(f, "io error: {}", e),
            Self::SscanfError(e) => write!(f, "sscanf error: {}", e),
            Self::StrumParseError(e) => write!(f, "strum parse error: {}", e),
        }
    }
}

pub type GenericResult<T> = Result<T, GenericError>;