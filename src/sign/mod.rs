use base64::encode;
use futures::future;
use futures_cpupool;
use openssl::sign::Signer;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use serde_json;
use std::error::Error;
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
        .map(|s| s != "." && s.starts_with("."))
        .unwrap_or(false)
        || entry
            .file_name()
            .to_str()
            .map(|s| s == "signature.json")
            .unwrap_or(false)
}

#[derive(Serialize)]
struct AppSignature {
    hashes: serde_json::map::Map<String, serde_json::Value>,
    signature: String,
    certificate: String,
}

fn sign_string(string: &String, key: &String) -> Result<String, error::SignError> {
    let keypair = PKey::private_key_from_pem(key.as_bytes())?;
    let mut signer = Signer::new(MessageDigest::sha512(), &keypair)?;
    signer.update(string.as_bytes())?;
    let signature = signer.finish()?;

    Ok(encode(&signature))
}

fn get_certificate(private_key: &String) -> Result<String, error::SignError> {
    Ok("".to_string())
}

fn create_app_signature(app_path: &Path) -> Result<AppSignature, error::SignError> {
    let walker = walkdir::WalkDir::new(app_path).into_iter();
    let mut hashes = serde_json::Map::new();
    for entry in walker.filter_entry(|e| !is_ignored(e)) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let hash = hash::hash_file(app_path, entry.path())?;
            println!("{:?}: {}", entry, hash.hash());
            hashes.insert(
                hash.file().to_owned(),
                serde_json::Value::String(hash.hash().to_owned()),
            );
        }
    }

    let signature = AppSignature {
        hashes: hashes,
        signature: "".to_string(),
        certificate: "".to_string(),
    };

    Ok(signature)
}

pub fn sign_app(key_path: &Path, app_path: &Path) -> Result<(), error::SignError> {
    let unsigned = create_app_signature(app_path)?;
    let hashes_json = serde_json::to_string(&unsigned.hashes)
        .map_err(|e| error::SignError::Other(e.description().to_string()))?;
    let private_key = get_private_key(key_path)?;
    let signature = sign_string(&hashes_json, &private_key)?;
    let cert = get_certificate(&private_key)?;

    let signature = AppSignature {
        hashes: unsigned.hashes,
        signature: signature,
        certificate: cert,
    };

    println!("{}", serde_json::to_string(&signature).unwrap());
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
