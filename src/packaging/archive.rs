use std::fs::File;
use std::io;
use std::path::Path;
use std::vec::Vec;

use failure::Error;
use pathdiff::diff_paths;
use tar::Builder;
use ignore::DirEntry;

pub fn build_app_archive<W>(root: &Path,
                            app_path: &Path,
                            files: Vec<DirEntry>,
                            dest: W)
                            -> Result<W, Error>
    where W: io::Write
{
    let mut archive = Builder::new(dest);

    for entry in files {
        if !entry.metadata().unwrap().is_dir() {
            let entry_path = entry.path();
            if let Some(normalized) = diff_paths(&entry_path, &app_path) {
                let mut file_path = root.to_path_buf();
                file_path.push(&normalized);
                let mut file = File::open(&entry_path)?;
                archive.append_file(file_path.as_path(), &mut file)?;
            }
        }
    }

    let dest = archive.into_inner()?;

    Ok(dest)
}
