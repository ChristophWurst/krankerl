use failure::Error;
use nextcloud_appinfo::get_appinfo;

use crate::occ::Occ;
use std::path::Path;

pub fn enable_app() -> Result<(), Error> {
    let app_path = Path::new(".").canonicalize()?;
    let info = get_appinfo(&app_path)?;
    let occ = Occ::new("../../occ");
    occ.enable_app(info.id())
}
