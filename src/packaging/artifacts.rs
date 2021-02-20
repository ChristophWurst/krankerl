use std::fs;
use std::io;
use std::path::Path;

use color_eyre::{Report, Result};
use git2;

use color_eyre::eyre::WrapErr;

pub fn clone_app(src: &Path, dst: &Path) -> Result<()> {
    git2::Repository::clone(
        src.as_os_str()
            .to_str()
            .ok_or(Report::msg("Clone destination path is not valid utf8"))?,
        dst,
    )?;

    Ok(())
}

pub fn clear(path: &Path) -> Result<()> {
    if let Err(e) = fs::remove_dir_all(path) {
        // We can safely ignore NotFound errors here
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e).wrap_err("Failed to delete artifacts directory");
        }
    }
    fs::create_dir_all(path)?;
    Ok(())
}
