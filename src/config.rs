use serde_json;
use std::fs::OpenOptions;
use std::io::prelude::*;
use xdg;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub appstore_token: Option<String>,
}

pub fn set_appstore_token(token: &String) -> Result<(), ()> {
    let mut config = get_config()?;

    config.appstore_token = Some(token.to_owned());
    save_config(&config)?;

    Ok(())
}

pub fn get_config() -> Result<Config, ()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("krankerl").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.json")
        .expect("cannot create configuration directory");
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)
        .map_err(|e| {
            println!("could not open config file {}", e);
            ()
        })?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).map_err(|e| {
        println!("could not open config file {}", e);
        ()
    })?;

    if contents.is_empty() {
        return Ok(Config {
            appstore_token: None,
        });
    }

    serde_json::from_str(&contents).map_err(|e| {
        println!("could not read config file {}", e);
        ()
    })
}

fn save_config(config: &Config) -> Result<(), ()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("krankerl").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.json")
        .expect("cannot create configuration directory");
    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)
        .map_err(|e| {
            println!("could not open config file {}", e);
            ()
        })?;

    let serialized = serde_json::to_string_pretty(config).map_err(|_| {
        println!("could not serialize config");
        ()
    })?;

    config_file.write_all(serialized.as_bytes()).map_err(|_| {
        println!("could not write config");
        ()
    })?;

    Ok(())
}
