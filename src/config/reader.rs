use std::convert::Into;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use color_eyre::{eyre::WrapErr, Result};

pub trait ConfigReader {
    fn has_config(&self) -> bool;
    fn read(&self) -> Result<String>;
}

pub struct ConfigFileReader {
    path: PathBuf,
}

impl ConfigFileReader {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        ConfigFileReader { path: path.into() }
    }
}

impl ConfigReader for ConfigFileReader {
    fn has_config(&self) -> bool {
        File::open(&self.path).is_ok()
    }

    fn read(&self) -> Result<String> {
        let mut file = File::open(&self.path).wrap_err(format!(
            "Failed to open config file '{}'",
            self.path.to_string_lossy()
        ))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .wrap_err("Failed to read config file")?;

        Ok(contents)
    }
}
