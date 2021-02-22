#[cfg(test)]
extern crate fs_extra;
#[macro_use]
extern crate serde_derive;

pub mod commands;
pub mod config;
pub mod occ;
pub mod packaging;

use color_eyre::Result;

pub async fn publish_app(
    url: &String,
    is_nightly: bool,
    signature: &String,
    api_token: &String,
) -> Result<()> {
    Ok(nextcloud_appstore::publish_app(url, is_nightly, signature, api_token).await?)
}
