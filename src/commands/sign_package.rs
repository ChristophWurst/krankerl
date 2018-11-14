use failure::Error;

use nextcloud_appinfo::get_appinfo;
use nextcloud_appsignature;
use std::env;
use std::path::{Path, PathBuf};

fn get_home_dir() -> Result<PathBuf, Error> {
    env::home_dir().ok_or(format_err!("Could not resolve home dir",))
}

fn get_private_key_path(app_id: &String) -> Result<PathBuf, Error> {
    let mut key_path = get_home_dir()?;
    key_path.push(".nextcloud");
    key_path.push("certificates");
    key_path.push(app_id.to_string() + ".key");
    Ok(key_path)
}

fn get_package_path(app_id: &String) -> Result<PathBuf, Error> {
    let mut path = PathBuf::from(".").canonicalize()?;
    path.push("build");
    path.push("artifacts");
    path.push(app_id.to_string() + ".tar.gz");
    Ok(path)
}

pub fn sign_package() -> Result<String, Error> {
    let app_path = Path::new(".").canonicalize()?;
    let appinfo = get_appinfo(&app_path)?;
    let app_id = appinfo.id();
    let key_path = get_private_key_path(app_id)?;
    let package_path = get_package_path(app_id)?;

    if !package_path.exists() {
        bail!("No package found");
    }

    let signature = nextcloud_appsignature::sign_package(&key_path, &package_path)?;

    Ok(signature)
}
