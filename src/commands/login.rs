use config;
use failure::Error;

pub fn log_in_to_appstore(token: &String) -> Result<(), Error> {
    config::krankerl::set_appstore_token(token)
}

pub fn log_in_to_github(token: &String) -> Result<(), Error> {
    config::krankerl::set_github_token(token)
}
