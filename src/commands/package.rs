use std::path::PathBuf;

use color_eyre::Result;

use crate::packaging::package_app as package;

pub fn package_app(app_path: &PathBuf, shipped: bool) -> Result<()> {
    package(app_path, shipped)
}
