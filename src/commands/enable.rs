use nextcloud_appinfo::get_appinfo;

use crate::occ::Occ;
use color_eyre::{eyre::WrapErr, Result};
use std::path::Path;

pub fn enable_app() -> Result<()> {
    let app_path = Path::new(".").canonicalize().wrap_err("Invalid app path")?;
    let info = get_appinfo(&app_path).wrap_err("Failed to parse appinfo")?;
    let occ = Occ::new("../../occ");
    occ.enable_app(info.id()).wrap_err("Failed to enable app")
}
