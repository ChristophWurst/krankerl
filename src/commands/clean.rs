use std::fs;
use std::path::PathBuf;

use failure::Error;

pub fn clean(app_path: &PathBuf) -> Result<(), Error> {
    let artifacts_path = app_path.join("build").join("artifacts");

    if artifacts_path.exists() {
        fs::remove_dir_all(artifacts_path)?;
        println!("Build directory cleaned.");
    } else {
        println!("Nothing to clean.");
    }
    Ok(())
}
