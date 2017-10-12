use nextcloud_appinfo::error::Error as AppInfoError;
use nextcloud_appsignature::error::SignError;
use nextcloud_appstore::error::Error as AppStoreError;
use std::error;
use std::fmt;
use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum Error {
    AppInfo(AppInfoError),
    AppStore(AppStoreError),
    Io(io::Error),
    Other(String),
    Sign(SignError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AppInfo(ref err) => write!(f, "AppInfo error: {}", err),
            Error::AppStore(ref err) => write!(f, "AppStore error: {}", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Other(ref err) => write!(f, "Unknown error: {}", err),
            Error::Sign(ref err) => write!(f, "Sign error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AppInfo(ref err) => err.description(),
            Error::AppStore(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Other(_) => "Unknown error",
            Error::Sign(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::AppInfo(ref err) => Some(err),
            Error::AppStore(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Other(_) => None,
            Error::Sign(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<AppInfoError> for Error {
    fn from(err: AppInfoError) -> Error {
        Error::AppInfo(err)
    }
}

impl From<SignError> for Error {
    fn from(err: SignError) -> Error {
        Error::Sign(err)
    }
}
