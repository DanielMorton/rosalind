use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse(String),
    ParseIntError(ParseIntError),
    EmptyInput,
    InvalidSequence(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::ParseIntError(err) => write!(f, "Parse int error: {}", err),
            Error::EmptyInput => write!(f, "Input is empty"),
            Error::InvalidSequence(msg) => write!(f, "Invalid sequence: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(format!("Failed to parse integer: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
