use color_eyre::{eyre::WrapErr, Result};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use xdg;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub appstore_token: Option<String>,
    pub github_token: Option<String>,
}

pub fn set_appstore_token(token: &String) -> Result<()> {
    let mut config = get_config()?;

    config.appstore_token = Some(token.to_owned());
    save_config(&config)?;

    Ok(())
}

pub fn set_github_token(token: &String) -> Result<()> {
    let mut config = get_config().wrap_err("Failed to load config")?;

    config.github_token = Some(token.to_owned());
    save_config(&config).wrap_err("Failed to save config")?;

    Ok(())
}

fn open_config() -> Result<File> {
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix("krankerl").wrap_err("Failed to get config path")?;
    let config_path = xdg_dirs
        .place_config_file("config.json")
        .wrap_err("Failed to get config path")?;
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config_path)
        .wrap_err(format!(
            "Failed to open config file '{}'",
            config_path.to_string_lossy()
        ))
}

pub fn get_config() -> Result<Config> {
    let mut config_file = open_config()?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        return Ok(Config {
            appstore_token: None,
            github_token: None,
        });
    }

    serde_json::from_str(&contents).wrap_err("Failed to parse config.json")
}

fn save_config(config: &Config) -> Result<()> {
    let mut config_file = open_config()?;
    serde_json::to_writer_pretty(&mut config_file, config).wrap_err("Failed to write config file")
}
