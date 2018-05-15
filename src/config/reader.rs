use std::convert::Into;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Error;

pub trait ConfigReader {
    fn has_config(&self) -> bool;
    fn read(&self) -> Result<String, Error>;
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

    fn read(&self) -> Result<String, Error> {
        let mut file = File::open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}
