use serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use xdg;

use error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub appstore_token: Option<String>,
    pub github_token: Option<String>,
}

pub fn set_appstore_token(token: &String) -> Result<(), error::Error> {
    let mut config = get_config()?;

    config.appstore_token = Some(token.to_owned());
    save_config(&config)?;

    Ok(())
}

pub fn set_github_token(token: &String) -> Result<(), error::Error> {
    let mut config = get_config()?;

    config.github_token = Some(token.to_owned());
    save_config(&config)?;

    Ok(())
}

pub fn get_config() -> Result<Config, error::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("krankerl")
        .map_err(|e| error::Error::Other(e.description().to_string()))?;
    let config_path = xdg_dirs.place_config_file("config.json")?;
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        return Ok(Config {
                      appstore_token: None,
                      github_token: None,
                  });
    }

    serde_json::from_str(&contents).map_err(|e| error::Error::Other(e.description().to_string()))
}

fn save_config(config: &Config) -> Result<(), error::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("krankerl")
        .map_err(|e| error::Error::Other(e.description().to_string()))?;
    let config_path = xdg_dirs.place_config_file("config.json")?;
    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)?;

    let serialized = serde_json::to_string_pretty(config)
        .map_err(|e| error::Error::Other(e.description().to_string()))?;

    config_file.write_all(serialized.as_bytes())?;

    Ok(())
}
