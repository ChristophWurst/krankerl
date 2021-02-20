use color_eyre::Result;
use std::path::Path;

use crate::config;

pub fn init(app_path: &Path) -> Result<()> {
    config::app::init_config(app_path)
}
