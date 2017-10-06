use base64::encode;
use futures::future;
use futures_cpupool;
use openssl;
use openssl::sign::Signer;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use std::error;
use std::fmt;
use std::fs::File;
use std::convert::From;
use std::io::{self, copy, BufReader};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum SignError {
    Io(io::Error),
    Ssl(openssl::error::ErrorStack),
}

impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignError::Io(ref err) => write!(f, "IO error: {}", err),
            SignError::Ssl(ref err) => write!(f, "SSL error: {}", err),
        }
    }
}

impl error::Error for SignError {
    fn description(&self) -> &str {
        match *self {
            SignError::Io(ref err) => err.description(),
            SignError::Ssl(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SignError::Io(ref err) => Some(err),
            SignError::Ssl(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for SignError {
    fn from(err: io::Error) -> SignError {
        SignError::Io(err)
    }
}

impl From<openssl::error::ErrorStack> for SignError {
    fn from(err: openssl::error::ErrorStack) -> SignError {
        SignError::Ssl(err)
    }
}

fn get_private_key(key_path: &Path) -> Result<String, SignError> {
    let mut file = File::open(key_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn sign_package(key_path: &Path, file_path: &Path) -> Result<String, SignError> {
    let package_file = File::open(file_path)?;
    let key = get_private_key(key_path)?;
    let keypair = PKey::private_key_from_pem(key.as_bytes())?;
    let mut signer = Signer::new(MessageDigest::sha512(), &keypair)?;

    let mut buf_read = BufReader::new(package_file);
    copy(&mut buf_read, &mut signer)?;

    let signature = signer.finish().unwrap();

    Ok(encode(&signature))
}

pub fn sign_package_async(
    pool_builder: &mut futures_cpupool::Builder,
    key_path: PathBuf,
    file_path: PathBuf,
) -> futures_cpupool::CpuFuture<String, SignError> {
    let pool = pool_builder.create();

    pool.spawn_fn(move || match sign_package(&key_path, &file_path) {
        Ok(signature) => return future::ok(signature),
        Err(err) => return future::err(err),
    })
}
