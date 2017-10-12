use std::fs;
use std::io;
use std::process::Command;
use std::path::{Path, PathBuf};
use super::error;
use nextcloud_appinfo::get_appinfo;

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

fn get_app_id() -> Result<String, error::Error> {
    let app_path = Path::new(".").canonicalize()?;
    let app_info = get_appinfo(&app_path)?;
    Ok(app_info.id().to_owned())
}

pub fn package_app() -> Result<(), error::Error> {
    let app_id = get_app_id()?;
    println!("packaging {}", app_id);

    let mut path = PathBuf::from("./build/artifacts");
    clear_artifacts_dir(path.as_path())?;
    path.push(app_id);
    clone_repo(path.as_path())?;
    package(path.as_path())
}
