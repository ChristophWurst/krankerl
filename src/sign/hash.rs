use hex::ToHex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use openssl::hash::{MessageDigest, hash2};

use super::error;

#[derive(Debug)]
pub struct FileHash {
    file: String,
    hash: String,
}

impl FileHash {
    pub fn hash(&self) -> &String {
        &self.hash
    }
}

fn hash_file_contents(contents: &String) -> Result<String, error::SignError> {
    let hash = hash2(MessageDigest::sha512(), contents.as_bytes())?;
    Ok(hash.to_hex())
}

pub fn hash_file(path: &Path) -> Result<FileHash, error::SignError> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let hash = hash_file_contents(&contents)?;

    Ok(FileHash {
        file: "file".to_string(),
        hash: hash,
    })
}
