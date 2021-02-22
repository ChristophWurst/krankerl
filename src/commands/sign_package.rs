use std::path::{Path, PathBuf};

use color_eyre::{eyre::WrapErr, Report, Result};
use dirs::home_dir;
use nextcloud_appinfo::get_appinfo;
use nextcloud_appsignature;

fn get_home_dir() -> Result<PathBuf> {
    home_dir().ok_or(Report::msg("Could not resolve home dir"))
}

fn get_private_key_path(app_id: &String) -> Result<PathBuf> {
    let mut key_path = get_home_dir()?;
    key_path.push(".nextcloud");
    key_path.push("certificates");
    key_path.push(app_id.to_string() + ".key");
    Ok(key_path)
}

fn get_package_path(app_id: &String) -> Result<PathBuf> {
    let mut path = PathBuf::from(".")
        .canonicalize()
        .wrap_err("Invalid app path")?;
    path.push("build");
    path.push("artifacts");
    path.push(app_id.to_string() + ".tar.gz");
    Ok(path)
}

pub fn sign_package() -> Result<String> {
    let app_path = Path::new(".").canonicalize().wrap_err("Invalid app path")?;
    let appinfo = get_appinfo(&app_path).wrap_err("Failed to parse appinfo")?;
    let app_id = appinfo.id();
    let key_path = get_private_key_path(app_id).wrap_err("Failed to get private key path")?;
    let package_path = get_package_path(app_id).wrap_err("Failed to get package path")?;

    if !package_path.exists() {
        return Err(Report::msg(
            "No built package found, build one using `krankerl package`",
        ));
    }

    let signature = nextcloud_appsignature::sign_package(&key_path, &package_path)
        .wrap_err("Failed to sign package")?;

    Ok(signature)
}
