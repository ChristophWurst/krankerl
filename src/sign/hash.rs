use hex::ToHex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;
use openssl::hash::{MessageDigest, hash2};

use super::error;

#[derive(Debug)]
pub struct FileHash {
    file: String,
    hash: String,
}

impl FileHash {
    pub fn file(&self) -> &String {
        &self.file
    }
    pub fn hash(&self) -> &String {
        &self.hash
    }
}

fn hash_file_contents(contents: &Vec<u8>) -> Result<String, error::SignError> {
    let hash = hash2(MessageDigest::sha512(), contents.as_slice())?;
    Ok(hash.to_hex())
}

pub fn hash_file(app_path: &Path, path: &Path) -> Result<FileHash, error::SignError> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = vec![];
    buf_reader.read_to_end(&mut contents)?;

    let hash = hash_file_contents(&contents)?;

    let abs_app_path = app_path.canonicalize()?;
    let app_path = abs_app_path.to_str().unwrap();
    let abs_path = path.canonicalize()?;
    let abs_path = abs_path.to_str().unwrap().to_string();
    let (_, rel_path) = abs_path.split_at(app_path.len());

    Ok(FileHash {
        file: rel_path.to_string(),
        hash: hash,
    })
}
