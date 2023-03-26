
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error  {
    Error(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            Error::Error(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            Error::Error(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        return Some(self);
    }
}