use std::fs;
use std::path::PathBuf;

use failure::Error;

use packaging::package_app as package;

pub fn package_app(app_path: &PathBuf) -> Result<(), Error> {
    package(app_path)
}
