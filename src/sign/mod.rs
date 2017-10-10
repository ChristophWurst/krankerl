use base64::encode;
use futures::future;
use futures_cpupool;
use openssl::sign::Signer;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use std::fs::File;
use std::io::{copy, BufReader};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::{self, WalkDirIterator};

pub mod error;
mod hash;

fn get_private_key(key_path: &Path) -> Result<String, error::SignError> {
    let mut file = File::open(key_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn sign_package(key_path: &Path, file_path: &Path) -> Result<String, error::SignError> {
    let package_file = File::open(file_path)?;
    let key = get_private_key(key_path)?;
    let keypair = PKey::private_key_from_pem(key.as_bytes())?;
    let mut signer = Signer::new(MessageDigest::sha512(), &keypair)?;

    let mut buf_read = BufReader::new(package_file);
    copy(&mut buf_read, &mut signer)?;

    let signature = signer.finish()?;

    Ok(encode(&signature))
}

/// Ignore hidden files and signature.json
fn is_ignored(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
        || entry
            .file_name()
            .to_str()
            .map(|s| s == "signature.json")
            .unwrap_or(false)
}

pub fn sign_app(app_path: &Path) -> Result<(), error::SignError> {
    let walker = walkdir::WalkDir::new(app_path).into_iter();
    for entry in walker.filter_entry(|e| !is_ignored(e)) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let hash = hash::hash_file(entry.path())?;
            println!("{:?}: {}", entry, hash.hash());
        }
    }
    Ok(())
}

pub fn sign_package_async(
    pool_builder: &mut futures_cpupool::Builder,
    key_path: PathBuf,
    file_path: PathBuf,
) -> futures_cpupool::CpuFuture<String, error::SignError> {
    let pool = pool_builder.create();

    pool.spawn_fn(move || match sign_package(&key_path, &file_path) {
        Ok(signature) => return future::ok(signature),
        Err(err) => return future::err(err),
    })
}
