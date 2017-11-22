use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use error;

pub fn clone_app(src: &Path, dst: &Path) -> Result<(), error::Error> {
    // TODO: use libgit2 instead
    Command::new("git")
        .arg("clone")
        .arg(src)
        .arg(dst)
        .status()
        .map(|_| ())?;
    Ok(())
}

pub fn clear(path: &Path) -> Result<(), error::Error> {
    if let Err(e) = fs::remove_dir_all(path) {
        // We can savely ignoe NotFound errors here
        if e.kind() != io::ErrorKind::NotFound {
            return Err(error::Error::Other(
                "could not delete artifacts dir".to_string(),
            ));
        }
    }
    fs::create_dir_all(path)?;
    Ok(())
}
