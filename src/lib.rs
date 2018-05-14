extern crate base64;
#[macro_use]
extern crate failure;
extern crate flate2;
#[cfg(test)]
extern crate fs_extra;
extern crate futures;
extern crate globset;
extern crate hex;
extern crate indicatif;
extern crate composer;
extern crate nextcloud_appinfo;
extern crate nextcloud_appsignature;
extern crate nextcloud_appstore;
extern crate npm_scripts;
extern crate openssl;
extern crate pathdiff;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tar;
#[cfg(test)]
extern crate tempdir;
extern crate tokio_core;
extern crate toml;
extern crate walkdir;
extern crate xdg;

pub mod commands;
pub mod config;
pub mod error;
pub mod occ;
pub mod packaging;

use std::env;
use std::path::{Path, PathBuf};

use failure::Error;
use nextcloud_appinfo::get_appinfo;
pub use nextcloud_appstore::{get_apps_and_releases, get_categories};
use tokio_core::reactor::Handle;
use occ::Occ;

pub fn enable_app() -> Result<(), Error> {
    let app_path = Path::new(".").canonicalize()?;
    let info = get_appinfo(&app_path)?;
    let occ = Occ::new("../../occ");
    occ.enable_app(info.id())
}

pub fn disable_app() -> Result<(), Error> {
    let app_path = Path::new(".").canonicalize()?;
    let info = get_appinfo(&app_path)?;
    let occ = Occ::new("../../occ");
    occ.disable_app(info.id())
}

fn get_home_dir() -> Result<PathBuf, Error> {
    env::home_dir().ok_or(format_err!(
        "Could not resolve home dir",
    ))
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

pub fn publish_app(handle: &Handle,
                   url: &String,
                   is_nightly: bool,
                   signature: &String,
                   api_token: &String)
                   -> Box<futures::Future<Item = (), Error = Error>> {
    Box::new(nextcloud_appstore::publish_app(handle, url, is_nightly, signature, api_token))
}
