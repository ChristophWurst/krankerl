use std::fs;
use std::path::PathBuf;

use color_eyre::{eyre::WrapErr, Result};

pub fn clean(app_path: &PathBuf) -> Result<()> {
    let artifacts_path = app_path.join("build").join("artifacts");

    if artifacts_path.exists() {
        fs::remove_dir_all(artifacts_path).wrap_err("Failed to remove artifact directory")?;
        println!("Build directory cleaned.");
    } else {
        println!("Nothing to clean.");
    }
    Ok(())
}
