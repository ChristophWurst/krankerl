use failure::Error;
use std::path::Path;

use crate::config;

pub fn init(app_path: &Path) -> Result<(), Error> {
    config::app::init_config(app_path)
}
