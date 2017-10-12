use std::fs;
use std::io;
use std::process::Command;
use std::path::{Path, PathBuf};
use super::error;

fn clear_artifacts_dir(path: &Path) -> Result<(), error::Error> {
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

fn clone_repo(path: &Path) -> Result<(), error::Error> {
    // TODO: use libgit2 instead
    Command::new("git")
        .arg("clone")
        .arg(".")
        .arg(path)
        .status()
        .map(|_| ())?;
    Ok(())
}

fn package(build_path: &Path) -> Result<(), error::Error> {
    Command::new("make")
        .arg("appstore")
        .current_dir(build_path)
        .status()
        .map(|_| ())?;
    Ok(())
}

pub fn package_app(app_id: &String) -> Result<(), error::Error> {
    println!("packaging {}", app_id);

    let mut path = PathBuf::from("./build/artifacts");
    clear_artifacts_dir(path.as_path())?;
    path.push(app_id);
    clone_repo(path.as_path())?;
    package(path.as_path())
}
