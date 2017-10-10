use openssl;
use std::error;
use std::fmt;
use std::convert::From;
use std::io;
use walkdir;

#[derive(Debug)]
pub enum SignError {
    Io(io::Error),
    Ssl(openssl::error::ErrorStack),
    Other(String),
}

impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignError::Io(ref err) => write!(f, "IO error: {}", err),
            SignError::Ssl(ref err) => write!(f, "SSL error: {}", err),
            SignError::Other(ref err) => write!(f, "Unknown error: {}", err),
        }
    }
}

impl error::Error for SignError {
    fn description(&self) -> &str {
        match *self {
            SignError::Io(ref err) => err.description(),
            SignError::Ssl(ref err) => err.description(),
            SignError::Other(_) => "Unknown error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SignError::Io(ref err) => Some(err),
            SignError::Ssl(ref err) => Some(err),
            SignError::Other(_) => None,
        }
    }
}

impl From<io::Error> for SignError {
    fn from(err: io::Error) -> SignError {
        SignError::Io(err)
    }
}

impl From<walkdir::Error> for SignError {
    fn from(err: walkdir::Error) -> SignError {
        SignError::Io(io::Error::new(io::ErrorKind::Other, err))
    }
}

impl From<openssl::error::ErrorStack> for SignError {
    fn from(err: openssl::error::ErrorStack) -> SignError {
        SignError::Ssl(err)
    }
}
