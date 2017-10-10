extern crate base64;
extern crate futures;
extern crate futures_cpupool;
extern crate hex;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;
extern crate xdg;

pub mod config;
pub mod packaging;
pub mod sign;

use std::fs::File;
use std::io::prelude::*;

pub fn get_signature(app_id: &String) -> Option<String> {
    let mut file = File::open(format!("./build/artifacts/{}.sig", app_id))
        .expect("could not open signature file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read signature file");
    Some(contents)
}
