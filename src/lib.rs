#[macro_use]
extern crate failure;
#[cfg(test)]
extern crate fs_extra;
#[macro_use]
extern crate serde_derive;

pub mod commands;
pub mod config;
mod console;
pub mod error;
pub mod occ;
pub mod packaging;

use failure::Error;
pub use nextcloud_appstore::{get_apps_and_releases, get_categories};

pub fn publish_app(url: &String,
                   is_nightly: bool,
                   signature: &String,
                   api_token: &String)
                   -> Box<futures::Future<Item = (), Error = Error> + Send> {
    Box::new(nextcloud_appstore::publish_app(url, is_nightly, signature, api_token))
}
