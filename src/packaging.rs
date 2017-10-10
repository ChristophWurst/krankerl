use std::fs;
use std::io;
use std::process::Command;
use std::path::{Path, PathBuf};

fn clear_artifacts_dir(path: &Path) -> Result<(), ()> {
    if let Err(e) = fs::remove_dir_all(path) {
        // We can savely ignoe NotFound errors here
        if e.kind() != io::ErrorKind::NotFound {
            return Err(());
        }
    }
    fs::create_dir_all(path).map_err(|_| ())
}

fn clone_repo(path: &Path) -> Result<(), ()> {
    // TODO: use libgit2 instead
    Command::new("git")
        .arg("clone")
        .arg(".")
        .arg(path)
        .status()
        .map(|_| ())
        .map_err(|e| {
            println!("could not clone app repository: {}", e);
            ()
        })
}

fn package(build_path: &Path) -> Result<(), ()> {
    Command::new("make")
        .arg("appstore")
        .current_dir(build_path)
        .status()
        .map(|_| ())
        .map_err(|e| {
            println!("could not build target 'appstore': {}", e);
            ()
        })
}

pub fn package_app(app_id: &String) -> Result<(), ()> {
    println!("packaging {}", app_id);

    let mut path = PathBuf::from("./build/artifacts");
    clear_artifacts_dir(path.as_path())?;
    path.push(app_id);
    clone_repo(path.as_path())?;
    package(path.as_path())
}
