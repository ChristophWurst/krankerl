use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use toml;

use super::super::error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    package: PackageConfig,
}

impl AppConfig {
    pub fn package(&self) -> &PackageConfig {
        &self.package
    }
}

#[derive(Debug, Deserialize)]
pub struct PackageConfig {
    before_cmds: Option<Vec<String>>,
    exclude: Vec<String>,
}

impl PackageConfig {
    pub fn before_cmds(&self) -> &Option<Vec<String>> {
        &self.before_cmds
    }
    pub fn exclude(&self) -> &Vec<String> {
        &self.exclude
    }
}

pub fn init_config(app_path: &Path) -> Result<(), error::Error> {
    let mut path_buf = app_path.to_path_buf();
    path_buf.push("krankerl.toml");

    if let Ok(_) = File::open(&path_buf) {
        return Err(error::Error::Other(
            "krankerl.toml already exists.".to_string(),
        ));
    }

    let mut config_file = File::create(&path_buf)?;

    config_file.write_all(
        r#"[packaging]
exclude = [

]
"#.as_bytes(),
    )?;

    Ok(())
}

fn load_config(path: &Path) -> Result<String, error::Error> {
    let mut path_buf = path.to_path_buf();
    path_buf.push("krankerl.toml");

    let mut file = File::open(path_buf)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_config(config: String) -> Result<AppConfig, error::Error> {
    toml::from_str(&config).map_err(|e| {
        error::Error::Other(format!(
            "could not parse krankerl.toml: {}",
            e.description()
        ))
    })
}

pub fn get_config(path: &Path) -> Result<AppConfig, error::Error> {
    let config_str = load_config(path)?;
    parse_config(config_str)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;

    use fs_extra::dir::{copy, CopyOptions};
    use tempdir::TempDir;

    use super::*;

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

        let mut krankl_path = app_path.clone();
        krankl_path.push("krankerl.toml");
        File::open(&krankl_path).unwrap_err();

        init_config(&app_path).unwrap();

        File::open(&krankl_path).unwrap();
        tmp.close().unwrap();
    }

    #[test]
    fn test_init_stops_if_config_exists() {
        let (app_path, tmp) = prepare_fs_test("app2");

        let mut krankl_path = app_path.clone();
        krankl_path.push("krankerl.toml");
        File::open(&krankl_path).unwrap();

        init_config(&app_path).unwrap_err();

        File::open(&krankl_path).unwrap();
        tmp.close().unwrap();
    }

    #[test]
    fn test_load_config() {
        let (app_path, tmp) = prepare_fs_test("app3");

        load_config(&app_path).unwrap();

        tmp.close().unwrap();
    }

    #[test]
    fn test_parse_minimal_config() {
        let toml = r#"
            [package]
            exclude = []
        "#;

        let config = parse_config(toml.to_owned());

        assert!(config.is_ok());
    }

    #[test]
    fn test_parse_config_without_commands() {
        let toml = r#"
            [package]
            exclude = [
                ".git",
                "composer.json",
                "composer.lock",
            ]
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
        ]

        exclude = []"#;

        let config = parse_config(toml.to_owned());

        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.package.before_cmds.is_some());
    }
}
