use std::fs::File;
use std::path::{Path, PathBuf};
use std::thread;

use failure::Error;
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{MultiProgress, ProgressBar};
use nextcloud_appinfo::get_appinfo;
use walkdir::{DirEntry, WalkDir};

mod archive;
mod artifacts;
mod commands;
mod exclude;

use self::commands::PackageCommands;
use config;
use config::app::AppConfig;
use console::default_spinner;

fn build_file_list(build_path: &Path, excludes: &exclude::ExcludedFiles) -> Vec<DirEntry> {
    WalkDir::new(build_path)
        .into_iter()
        .filter_entry(|e| !excludes.is_excluded(e.path(), build_path))
        .map(|e| e.unwrap())
        .collect()
}

fn package(artifacts_path: &Path,
           app_id: &String,
           app_config: &AppConfig,
           progress: ProgressBar)
           -> Result<(), Error> {
    progress.set_message("Packaging app files into an archive...");
    let excludes = exclude::ExcludedFiles::new(app_config.package().exclude())?;

    let mut compressed_archive_path = PathBuf::from(artifacts_path);
    let mut compressed_archive_filename = app_id.clone();
    compressed_archive_filename.push_str(".tar.gz");
    compressed_archive_path.push(compressed_archive_filename);
    progress.set_message(&format!("Writing compressed app archive to {:?}...",
                                  compressed_archive_path));

    let gz_archive_file = File::create(&compressed_archive_path)?;
    let encoder = GzEncoder::new(gz_archive_file, Compression::default());

    let mut app_path = PathBuf::from(artifacts_path);
    app_path.push(&app_id);
    let base = Path::new(app_id);

    let file_list = build_file_list(&app_path, &excludes);
    let encoder = archive::build_app_archive(&base, &app_path, file_list, encoder)?;
    encoder.finish()?;

    progress.finish_with_message(&format!("Packaged app as {:?}", compressed_archive_path));
    Ok(())
}

fn ensure_config_exists(app_path: &Path) -> Result<(), Error> {
    match config::app::get_config(app_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format_err!("could not load krankerl.toml: {}", e)),
    }
}

fn run_package_commands(app_path: &Path,
                        cmds: commands::CommandList,
                        prog: ProgressBar)
                        -> Result<(), Error> {
    cmds.execute(app_path, prog)
}

fn package_app_thread(app_path: PathBuf,
                      prog_clone: ProgressBar,
                      prog_config: ProgressBar,
                      prog_package_cmds: ProgressBar,
                      prog_package: ProgressBar)
                      -> Result<(), Error> {
    let mut artifacts_path = PathBuf::from(&app_path);
    artifacts_path.push("build");
    artifacts_path.push("artifacts");

    ensure_config_exists(&app_path)?;

    let app_info = get_appinfo(&app_path)?;
    let app_id = app_info.id();

    let package_path = artifacts_path.clone();
    artifacts::clear(&artifacts_path)?;
    artifacts_path.push(&app_id);
    prog_clone.set_message("Cloning app to build directory...");
    artifacts::clone_app(&app_path, &artifacts_path)?;
    prog_clone.finish_with_message("App cloned to build directory.");

    prog_config.set_message("Reading config...");
    let app_config = config::app::get_config(&artifacts_path)
        .map(|config| match config {
                 Some(cfg) => {
            prog_config.finish_with_message("Found krankerl.toml config file.");
            cfg
        }
                 None => {
            prog_config.finish_with_message(
                "Warning: No krankerl.toml found. A default configuration is used.",
            );
            AppConfig::default()
        }
             })?;

    run_package_commands(&artifacts_path,
                         app_config.package().into(),
                         prog_package_cmds)?;

    package(package_path.as_path(), &app_id, &app_config, prog_package)?;
    Ok(())
}

pub fn package_app(app_path: &PathBuf) -> Result<(), Error> {
    let app_path = app_path.clone();
    let mp = MultiProgress::new();
    let prog_clone = mp.add(default_spinner());
    prog_clone.enable_steady_tick(200);
    prog_clone.set_message("waiting...");
    let prog_config = mp.add(default_spinner());
    prog_config.enable_steady_tick(200);
    prog_config.set_message("waiting...");
    let prog_package_cmds = mp.add(default_spinner());
    prog_package_cmds.enable_steady_tick(200);
    prog_package_cmds.set_message("waiting...");
    let prog_package = mp.add(default_spinner());
    prog_package.enable_steady_tick(200);
    prog_package.set_message("waiting...");

    let worker = thread::spawn(move || {
                                   package_app_thread(app_path,
                                                      prog_clone,
                                                      prog_config,
                                                      prog_package_cmds,
                                                      prog_package)
                               });

    mp.join()?;
    worker.join().unwrap()?;
    Ok(())
}
