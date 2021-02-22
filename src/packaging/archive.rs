use std::fs::File;
use std::io;
use std::path::Path;
use std::vec::Vec;

use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use ignore::DirEntry;
use pathdiff::diff_paths;
use tar::Builder;

pub fn build_app_archive<W>(
    root: &Path,
    app_path: &Path,
    files: Vec<DirEntry>,
    dest: W,
) -> Result<W>
where
    W: io::Write,
{
    let mut archive = Builder::new(dest);

    for entry in files {
        if !entry.metadata().unwrap().is_dir() {
            let entry_path = entry.path();
            if let Some(normalized) = diff_paths(&entry_path, &app_path) {
                let mut file_path = root.to_path_buf();
                file_path.push(&normalized);
                let mut file = File::open(&entry_path).wrap_err_with(|| {
                    format!(
                        "Failed to open {} for packaging",
                        file_path.to_string_lossy()
                    )
                })?;
                archive.append_file(file_path.as_path(), &mut file)?;
            }
        }
    }

    let dest = archive.into_inner()?;

    Ok(dest)
}
