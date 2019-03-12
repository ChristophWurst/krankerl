use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

use error;
use git2;
use failure::Error;

pub fn clone_app(src: &Path, dst: &Path) -> Result<(), Error> {
    git2::Repository::clone(src.as_os_str()
                                .to_str()
                                .expect("could not convert clone destination to str"),
                            dst)?;

    Ok(())
}

pub fn clear(path: &Path) -> Result<(), Error> {
    if let Err(e) = fs::remove_dir_all(path) {
        // We can safely ignore NotFound errors here
        if e.kind() != io::ErrorKind::NotFound {
            bail!(error::KrankerlError::Other {
                      cause: "could not delete artifacts dir".to_string(),
                  });
        }
    }
    fs::create_dir_all(path)?;
    Ok(())
}
