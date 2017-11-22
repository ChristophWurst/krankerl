use std::fs::File;
use std::io::Read;
use std::path::Path;

use error;

pub trait ConfigReader {
    fn read(&self, path: &Path) -> Result<String, error::Error>;
}

pub struct ConfigFileReader {}

impl ConfigFileReader {
    pub fn new() -> Self {
        ConfigFileReader {}
    }
}

impl ConfigReader for ConfigFileReader {
    fn read(&self, path: &Path) -> Result<String, error::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}
