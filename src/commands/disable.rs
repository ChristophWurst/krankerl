use color_eyre::{eyre::WrapErr, Result};
use nextcloud_appinfo::get_appinfo;

use crate::occ::Occ;
use std::path::Path;

pub fn disable_app() -> Result<()> {
    let app_path = Path::new(".").canonicalize().wrap_err("Invalid app path")?;
    let info = get_appinfo(&app_path).wrap_err("Failed to parse appinfo")?;
    let occ = Occ::new("../../occ");
    occ.disable_app(info.id()).wrap_err("Failed to enable app")
}
