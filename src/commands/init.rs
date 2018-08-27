use std::path::Path;
use failure::Error;
use config;

pub fn init(app_path: &Path) -> Result<(), Error> {
    config::app::init_config(app_path)
}
