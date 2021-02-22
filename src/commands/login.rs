use color_eyre::Result;

use crate::config;

pub fn log_in_to_appstore(token: &String) -> Result<()> {
    config::krankerl::set_appstore_token(token)
}

pub fn log_in_to_github(token: &String) -> Result<()> {
    config::krankerl::set_github_token(token)
}
