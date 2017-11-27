use std::fs::File;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::GzEncoder;
use nextcloud_appinfo::get_appinfo;
use walkdir::{DirEntry, WalkDir};

mod archive;
mod artifacts;
mod commands;
mod exclude;

use config;
use self::commands::PackageCommands;
use config::app::AppConfig;
use error;

fn build_file_list(build_path: &Path, excludes: &exclude::ExcludedFiles) -> Vec<DirEntry> {
    WalkDir::new(build_path)
        .into_iter()
        .filter_entry(|e| !excludes.is_excluded(e.path(), build_path))
        .map(|e| e.unwrap())
        .collect()
}

fn package(
    artifacts_path: &Path,
    app_id: &String,
    app_config: &AppConfig,
) -> Result<(), error::Error> {
    let excludes = exclude::ExcludedFiles::new(app_config.package().exclude())?;

    let mut compressed_archive_path = PathBuf::from(artifacts_path);
    let mut compressed_archive_filename = app_id.clone();
    compressed_archive_filename.push_str(".tar.gz");
    compressed_archive_path.push(compressed_archive_filename);
    println!(
        "Writing compressed app archive to {:?}",
        compressed_archive_path
    );

    let gz_archive_file = File::create(compressed_archive_path)?;
    let encoder = GzEncoder::new(gz_archive_file, Compression::Default);

    let mut app_path = PathBuf::from(artifacts_path);
    app_path.push(&app_id);
    let base = Path::new(app_id);

    let file_list = build_file_list(&app_path, &excludes);
    let encoder = archive::build_app_archive(&base, &app_path, file_list, encoder)?;
    encoder.finish()?;

    Ok(())
}

fn ensure_config_exists(app_path: &Path) -> Result<(), error::Error> {
    match config::app::get_config(app_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(error::Error::Other(
            format!("could not load krankerl.toml: {}", e),
        )),
    }
}

fn run_package_commands(app_path: &Path, cmds: commands::CommandList) -> Result<(), error::Error> {
    cmds.execute(app_path)
}

pub fn package_app() -> Result<(), error::Error> {
    let cwd = Path::new(".");
    let mut artifacts_path = PathBuf::from(cwd);
    artifacts_path.push("build");
    artifacts_path.push("artifacts");

    ensure_config_exists(&cwd)?;

    let app_info = get_appinfo(&cwd)?;
    let app_id = app_info.id();
    println!("packaging {}", app_id);

    let package_path = artifacts_path.clone();
    artifacts::clear(&artifacts_path)?;
    artifacts_path.push(&app_id);
    artifacts::clone_app(&cwd, &artifacts_path)?;
    let app_config = config::app::get_config(&artifacts_path).map(|config| {
        config.unwrap_or_else(|| {
            println!("Warning: No krankerl.toml found. A default configuration is used.");
            AppConfig::default()
        })
    })?;
    run_package_commands(&artifacts_path, app_config.package().into())?;
    package(package_path.as_path(), &app_id, &app_config)
}
