extern crate base64;
extern crate composer;
#[macro_use]
extern crate failure;
extern crate flate2;
#[cfg(test)]
extern crate fs_extra;
extern crate futures;
extern crate globset;
extern crate hex;
extern crate indicatif;
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
mod console;
pub mod error;
pub mod occ;
pub mod packaging;

use failure::Error;
pub use nextcloud_appstore::{get_apps_and_releases, get_categories};

pub fn publish_app(
    url: &String,
    is_nightly: bool,
    signature: &String,
    api_token: &String,
) -> Box<futures::Future<Item = (), Error = Error>> {
    Box::new(nextcloud_appstore::publish_app(
        url,
        is_nightly,
        signature,
        api_token,
    ))
}
