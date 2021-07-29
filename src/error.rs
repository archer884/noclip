use std::fmt::{self, Display};
use std::io;
use std::{error, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Clipboard(Box<dyn error::Error + 'static>),
    IO(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Clipboard(e) => write!(f, "{}", e),
            Error::IO(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Clipboard(e) => Some(e.as_ref()),
            Error::IO(ref e) => Some(e),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ParseModeError;

impl Display for ParseModeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("modes include c, copy, v, p, paste")
    }
}

impl error::Error for ParseModeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<Box<dyn error::Error + 'static>> for Error {
    fn from(e: Box<dyn error::Error + 'static>) -> Self {
        Error::Clipboard(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}
