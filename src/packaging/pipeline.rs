use std::fs::{copy, create_dir_all, File};
use std::path::{Path, PathBuf};

use failure::Error;
use flate2::write::GzEncoder;
use flate2::Compression;
use ignore::{DirEntry, WalkBuilder};
use nextcloud_appinfo::{get_appinfo, AppInfo};
use pathdiff::diff_paths;
use tempdir::TempDir;

use crate::config;
use crate::packaging::commands::{self, PackageCommands};
use crate::packaging::{archive, artifacts, exclude};

fn tmp_app_path(base: &Path, app_id: &str) -> PathBuf {
    let mut buf = base.to_path_buf();
    buf.push(app_id);
    buf
}

pub struct App {
    source_path: PathBuf,
}

impl App {
    pub fn new(source_path: PathBuf) -> Self {
        App {
            source_path: source_path,
        }
    }

    pub fn clone(self) -> Result<ClonedApp, Error> {
        println!("Cloning app");

        let app_info = get_appinfo(&self.source_path)?;
        let tmp = TempDir::new("krankerl")?;
        artifacts::clone_app(&self.source_path, &tmp_app_path(tmp.path(), app_info.id()))?;

        println!("App cloned to {:?}", tmp.path());
        Ok(ClonedApp::new(self, app_info, tmp))
    }
}

pub struct ClonedApp {
    app: App,
    app_info: AppInfo,
    tmp_dir: TempDir,
}

impl ClonedApp {
    pub fn new(app: App, app_info: AppInfo, tmp_dir: TempDir) -> Self {
        ClonedApp {
            app: app,
            app_info: app_info,
            tmp_dir: tmp_dir,
        }
    }

    pub fn install_dependencies(self) -> Result<AppWithDependencies, Error> {
        // TODO: automatically install npm and composer dependencies
        // progress
        //    .as_ref()
        //    .map(|prog| prog.set_message("Installing dependencies"));

        println!("Dependency installation skipped");
        Ok(AppWithDependencies::new(self))
    }
}

pub struct AppWithDependencies {
    app: App,
    app_info: AppInfo,
    tmp_dir: TempDir,
}

impl AppWithDependencies {
    pub fn new(clone: ClonedApp) -> Self {
        AppWithDependencies {
            app: clone.app,
            app_info: clone.app_info,
            tmp_dir: clone.tmp_dir,
        }
    }

    pub fn build(self) -> Result<BuiltApp, Error> {
        println!("Building app");

        let opt_config = config::app::get_config(&self.app.source_path)?;
        let (config, default) = match opt_config {
            Some(config) => (config, false),
            None => (config::app::AppConfig::default(), true),
        };
        let cmds = commands::CommandList::from(config.package());
        cmds.execute(&tmp_app_path(&self.tmp_dir.path(), self.app_info.id()))?;

        if default {
            println!("App built with default config");
        } else {
            println!("App built");
        }
        Ok(BuiltApp::new(self, config))
    }
}

pub struct BuiltApp {
    app: App,
    app_info: AppInfo,
    config: config::app::AppConfig,
    tmp_dir: TempDir,
}

impl BuiltApp {
    pub fn new(with_deps: AppWithDependencies, config: config::app::AppConfig) -> Self {
        BuiltApp {
            app: with_deps.app,
            app_info: with_deps.app_info,
            config: config,
            tmp_dir: with_deps.tmp_dir,
        }
    }

    pub fn into_archive(self) -> Result<AppArchive, Error> {
        let excludes = exclude::ExcludedFiles::new(self.config.package().exclude())?;

        let mut compressed_archive_path = self.app.source_path.to_path_buf();
        compressed_archive_path.push("build");
        compressed_archive_path.push("artifacts");
        artifacts::clear(&compressed_archive_path)?;

        compressed_archive_path.push(format!("{}.tar.gz", self.app_info.id()));
        println!("Writing compressed app archive to {:?}...", compressed_archive_path);

        let gz_archive_file = File::create(&compressed_archive_path)?;
        let encoder = GzEncoder::new(gz_archive_file, Compression::default());

        let app_path = tmp_app_path(self.tmp_dir.path(), self.app_info.id());
        {
            let base = Path::new(self.app_info.id());

            let file_list = build_file_list(&app_path, &excludes);
            let encoder = archive::build_app_archive(&base, &app_path, file_list, encoder)?;
            encoder.finish()?;
        }

        println!("Packaged app as {:?}", compressed_archive_path);

        Ok(AppArchive::new(self))
    }

    pub fn into_shipped(self) -> Result<ShippedApp, Error> {
        let mut ship_path = self.app.source_path.to_path_buf();
        ship_path.push("build");
        ship_path.push("artifacts");
        artifacts::clear(&ship_path)?;
        ship_path.push(self.app_info.id());
        println!("Writing app files to {:?}...", ship_path);

        let app_path = tmp_app_path(self.tmp_dir.path(), self.app_info.id());
        {
            let excludes = exclude::ExcludedFiles::new(self.config.package().exclude())?;
            for entry in build_file_list(&app_path, &excludes) {
                if !entry.metadata().unwrap().is_dir() {
                    let entry_path = entry.path();
                    if let Some(normalized) = diff_paths(&entry_path, &app_path) {
                        let mut file_path = ship_path.clone();
                        file_path.push(&normalized);
                        if let Some(parent) = file_path.parent() {
                            create_dir_all(parent)?;
                        }
                        copy(&entry.path(), &file_path)?;
                    }
                }
            }
        }

        println!("App directory created at {:?}", ship_path);
        Ok(ShippedApp::new(self))
    }
}

fn build_file_list(build_path: &Path, excludes: &exclude::ExcludedFiles) -> Vec<DirEntry> {
    WalkBuilder::new(build_path)
        .standard_filters(false)
        .add_custom_ignore_filename(".nextcloudignore")
        .build()
        .into_iter()
        .filter(|e| match e {
            Ok(entry) => !excludes.is_excluded(entry.path(), build_path),
            Err(_) => false,
        })
        .map(|e| e.unwrap())
        .collect()
}

pub struct AppArchive {}

impl AppArchive {
    pub fn new(_app: BuiltApp) -> Self {
        AppArchive {}
    }
}

pub struct ShippedApp {}

impl ShippedApp {
    pub fn new(_app: BuiltApp) -> Self {
        ShippedApp {}
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use flate2::read::GzDecoder;
    use tar::Archive;
    use tempdir::TempDir;

    use super::*;

    const MINIMALIST_APP: &'static [u8] = include_bytes!("../../tests/assets/minimalist.tar.gz");
    const APP_ID: &'static str = "recommendations";

    fn create_test_app_dir(tarball: &'static [u8]) -> TempDir {
        let tmp = TempDir::new("krankerl-test").unwrap();

        let decoder = GzDecoder::new(tarball);
        let mut archive = Archive::new(decoder);
        archive.unpack(tmp.path()).unwrap();

        println!("Unpacked temporary app at {:?}", tmp.path());
        let mut app_path = tmp.path().to_path_buf();
        app_path.push(APP_ID);
        println!("  test app path: {:?}", app_path);

        tmp
    }

    fn get_test_app_path(temp_path: &Path) -> PathBuf {
        tmp_app_path(temp_path, APP_ID)
    }

    #[test]
    fn create_app() {
        let dir = create_test_app_dir(MINIMALIST_APP);
        let app = App::new(get_test_app_path(dir.path()));

        assert!(app.source_path.exists());
    }

    #[test]
    fn clones_app() {
        let dir = create_test_app_dir(MINIMALIST_APP);
        let app = App::new(get_test_app_path(dir.path()));
        assert!(get_test_app_path(dir.path()).exists());

        let clone = app.clone().unwrap();

        assert!(clone.tmp_dir.path().exists());
        let mut cloned_app_dir = clone.tmp_dir.path().to_path_buf();
        cloned_app_dir.push(APP_ID);
        assert!(cloned_app_dir.exists());
    }

    #[test]
    fn install_app_dependencies() {
        let dir = create_test_app_dir(MINIMALIST_APP);
        let app = App::new(get_test_app_path(dir.path()));
        let clone = app.clone().unwrap();

        clone.install_dependencies().unwrap();
    }

    #[test]
    fn build_app() {
        let dir = create_test_app_dir(MINIMALIST_APP);
        let app = App::new(get_test_app_path(dir.path()));
        let clone = app.clone().unwrap();
        let installed = clone.install_dependencies().unwrap();

        installed.build().unwrap();
    }

    #[test]
    fn create_app_archive() {
        let dir = create_test_app_dir(MINIMALIST_APP);
        let app = App::new(get_test_app_path(dir.path()));
        let clone = app.clone().unwrap();
        let installed = clone.install_dependencies().unwrap();
        let built = installed.build().unwrap();

        built.into_archive().unwrap();

        let mut final_path = get_test_app_path(dir.path());
        final_path.push("build");
        assert!(final_path.exists(), "build directory does not exist");
        final_path.push("artifacts");
        assert!(final_path.exists(), "artifacts directory does not exist");
        final_path.push(format!("{}.tar.gz", APP_ID));
        assert!(final_path.exists(), "app archive does not exist");
    }

    #[test]
    fn create_shipped_app_directory() {
        let dir = create_test_app_dir(MINIMALIST_APP);
        let app = App::new(get_test_app_path(dir.path()));
        let clone = app.clone().unwrap();
        let installed = clone.install_dependencies().unwrap();
        let built = installed.build().unwrap();

        built.into_shipped().unwrap();

        let mut app_info_path = dir.path().to_path_buf();
        app_info_path.push(APP_ID);
        app_info_path.push("appinfo");
        app_info_path.push("info.xml");
        assert!(app_info_path.exists(), "info.xml is missing");
        let mut package_json_path = dir.path().to_path_buf();
        package_json_path.push(APP_ID);
        package_json_path.push("package.json");
        assert!(
            !package_json_path.exists(),
            "package.json should not be copied"
        );
    }
}
