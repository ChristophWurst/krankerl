use std::path::PathBuf;

use failure::Error;

use crate::packaging::package_app as package;

pub fn package_app(app_path: &PathBuf, shipped: bool) -> Result<(), Error> {
    package(app_path, shipped)
}
