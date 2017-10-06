extern crate base64;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate openssl;
extern crate xdg;

pub mod config;
pub mod sign;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn package_app(app_id: &String) -> Result<(), ()> {
    println!("packaging {}", app_id);

    Command::new("make")
        .arg("appstore")
        .status()
        .map(|_| ())
        .map_err(|e| {
            println!("could not build target 'appstore': {}", e);
            ()
        })
}

pub fn get_signature(app_id: &String) -> Option<String> {
    let mut file = File::open(format!("./build/artifacts/{}.sig", app_id)).expect("could not open signature file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read signature file");
    Some(contents)
}
