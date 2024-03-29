use std::convert::Into;
use std::default::Default;
use std::fs;
use std::path::Path;

use color_eyre::{eyre::WrapErr, Report, Result};
use toml;

use super::{ConfigFileReader, ConfigReader};

#[derive(Debug, Deserialize)]
struct ParsedAppConfig {
    package: Option<ParsedPackageConfig>,
}

#[derive(Debug, Deserialize)]
struct ParsedPackageConfig {
    before_cmds: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct AppConfig {
    package: PackageConfig,
}

impl AppConfig {
    pub fn package(&self) -> &PackageConfig {
        &self.package
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            package: PackageConfig::default(),
        }
    }
}

impl Into<AppConfig> for ParsedAppConfig {
    fn into(self) -> AppConfig {
        AppConfig {
            package: self
                .package
                .map(|pc| pc.into())
                .unwrap_or(PackageConfig::default()),
        }
    }
}

#[derive(Debug)]
pub struct PackageConfig {
    before_cmds: Vec<String>,
}

impl PackageConfig {
    pub fn before_cmds(&self) -> &Vec<String> {
        &self.before_cmds
    }
}

impl Into<PackageConfig> for ParsedPackageConfig {
    fn into(self) -> PackageConfig {
        PackageConfig {
            before_cmds: self.before_cmds.unwrap_or(vec![]),
        }
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        PackageConfig {
            before_cmds: vec![],
        }
    }
}

pub fn init_config(app_path: &Path) -> Result<()> {
    let config_path = app_path.join("krankerl.toml");

    if config_path.exists() {
        return Err(Report::msg("krankerl.toml already exists"));
    }

    fs::write(
        &config_path,
        r#"[package]
before_cmds = [

]
"#,
    )
    .wrap_err("Failed to write krankerl.toml")?;

    let ignore_path = app_path.join(".nextcloudignore");
    if !ignore_path.exists() {
        fs::write(
            &ignore_path,
            r#".drone
.git
.github
.gitignore
.scrutinizer.yml
.travis.yml
.tx
krankerl.toml
screenshots
.nextcloudignore
"#,
        )
        .wrap_err("Failed to write .nextcloudignore")?;
    }

    Ok(())
}

fn load_config<R>(reader: &R) -> Result<String>
where
    R: ConfigReader,
{
    reader.read()
}

fn parse_config(config: String) -> Result<ParsedAppConfig> {
    toml::from_str(&config).wrap_err("Failed to parse config as toml")
}

pub fn get_config(path: &Path) -> Result<Option<AppConfig>> {
    let mut path_buf = path.to_path_buf();
    path_buf.push("krankerl.toml");
    let reader = ConfigFileReader::new(path_buf);

    if !reader.has_config() {
        Ok(None)
    } else {
        let config_str = load_config(&reader)?;
        parse_config(config_str).map(|config| Some(config.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use fs_extra::dir::{copy, CopyOptions};
    use tempdir::TempDir;

    use super::*;

    struct StaticReader {}

    impl ConfigReader for StaticReader {
        fn has_config(&self) -> bool {
            true
        }

        fn read(&self) -> Result<String> {
            Ok("some config".to_owned())
        }
    }

    #[test]
    fn test_load_config() {
        let reader = StaticReader {};

        let conf = load_config(&reader).unwrap();

        assert_eq!("some config".to_owned(), conf);
    }

    fn prepare_fs_test(id: &'static str) -> (PathBuf, TempDir) {
        let mut src = PathBuf::from("./tests/apps");
        src.push(id);

        let tmp = TempDir::new("krankerl").unwrap();
        let options = CopyOptions::new();
        copy(&src, tmp.path(), &options).expect("copy app files");

        let mut app_path = tmp.path().to_path_buf();
        app_path.push(id);
        (app_path, tmp)
    }

    #[test]
    fn test_init_creates_config() {
        let (app_path, tmp) = prepare_fs_test("app1");

        let krankerl_path = app_path.join("krankerl.toml");
        assert!(!krankerl_path.exists());

        init_config(&app_path).unwrap();

        assert!(krankerl_path.exists());
        tmp.close().unwrap();
    }

    #[test]
    fn test_init_creates_ignore() {
        let (app_path, tmp) = prepare_fs_test("app1");

        let ignore_path = app_path.join(".nextcloudignore");
        assert!(!ignore_path.exists());

        init_config(&app_path).unwrap();

        assert!(ignore_path.exists());
        tmp.close().unwrap();
    }

    #[test]
    fn test_init_stops_if_config_exists() {
        let (app_path, tmp) = prepare_fs_test("app2");

        let krankerl_path = app_path.join("krankerl.toml");
        assert!(krankerl_path.exists());

        assert!(init_config(&app_path).is_err());

        assert!(krankerl_path.exists());
        tmp.close().unwrap();
    }

    #[test]
    fn test_init_doesnt_change_ignore() {
        let (app_path, tmp) = prepare_fs_test("app1");

        let ignore_path = app_path.join(".nextcloudignore");
        fs::write(&ignore_path, "dummy").unwrap();

        init_config(&app_path).unwrap();

        assert_eq!("dummy", fs::read_to_string(&ignore_path).unwrap());
        tmp.close().unwrap();
    }

    #[test]
    fn test_load_real_config() {
        let (app_path, tmp) = prepare_fs_test("app3");
        let config_path = app_path.join("krankerl.toml");
        let reader = ConfigFileReader::new(config_path);

        load_config(&reader).unwrap();

        tmp.close().unwrap();
    }

    #[test]
    fn test_parse_empty_config() {
        let toml = r#""#;

        let config = parse_config(toml.to_owned());

        assert!(config.is_ok());
    }

    #[test]
    fn test_parse_simple_config() {
        let toml = r#"
            [package]
        "#;

        let config = parse_config(toml.to_owned());

        assert!(config.is_ok());
    }

    #[test]
    fn test_parse_config_with_commands() {
        let toml = r#"
        [package]
        before_cmds = [
            "composer install",
            "npm install",
            "npm run build",
        ]"#;

        let config = parse_config(toml.to_owned());

        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.package.is_some());
        let package_config = config.package.unwrap();
        assert!(package_config.before_cmds.is_some());
        let cmds = package_config.before_cmds.unwrap();
        assert_eq!(3, cmds.len());
    }
}
