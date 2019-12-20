use failure::Error;
use serde_json;
use std::fs::OpenOptions;
use std::io::prelude::*;
use xdg;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub appstore_token: Option<String>,
    pub github_token: Option<String>,
}

pub fn set_appstore_token(token: &String) -> Result<(), Error> {
    let mut config = get_config()?;

    config.appstore_token = Some(token.to_owned());
    save_config(&config)?;

    Ok(())
}

pub fn set_github_token(token: &String) -> Result<(), Error> {
    let mut config = get_config()?;

    config.github_token = Some(token.to_owned());
    save_config(&config)?;

    Ok(())
}

pub fn get_config() -> Result<Config, Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("krankerl")?;
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

    let config = serde_json::from_str(&contents)
        .map_err(|err| format_err!("could not parse config.json: {}", err));

    config
}

fn save_config(config: &Config) -> Result<(), Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("krankerl")?;
    let config_path = xdg_dirs.place_config_file("config.json")?;
    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)?;

    let serialized = serde_json::to_string_pretty(config)?;

    config_file.write_all(serialized.as_bytes())?;

    Ok(())
}
