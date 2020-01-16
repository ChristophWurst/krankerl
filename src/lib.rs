#[macro_use]
extern crate failure;
#[cfg(test)]
extern crate fs_extra;
#[macro_use]
extern crate serde_derive;

pub mod commands;
pub mod config;
pub mod error;
pub mod occ;
pub mod packaging;

use failure::Error;

pub async fn publish_app(
    url: &String,
    is_nightly: bool,
    signature: &String,
    api_token: &String,
) -> Result<(), Error> {
    nextcloud_appstore::publish_app(url, is_nightly, signature, api_token).await
}
