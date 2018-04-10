use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use failure::Error;
use error;

pub fn clone_app(src: &Path, dst: &Path) -> Result<(), Error> {
    // TODO: use libgit2 instead
    Command::new("git")
        .arg("clone")
        .arg(src)
        .arg(dst)
        .status()
        .map(|_| ())?;
    Ok(())
}

pub fn clear(path: &Path) -> Result<(), Error> {
    if let Err(e) = fs::remove_dir_all(path) {
        // We can savely ignoe NotFound errors here
        if e.kind() != io::ErrorKind::NotFound {
            bail!(error::KrankerlError::Other {
                      cause: "could not delete artifacts dir".to_string(),
                  });
        }
    }
    fs::create_dir_all(path)?;
    Ok(())
}
