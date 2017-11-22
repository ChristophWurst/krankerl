use std::convert::Into;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use toml;

use super::super::error;

#[derive(Debug, Deserialize)]
struct ParsedAppConfig {
    package: Option<ParsedPackageConfig>,
}

#[derive(Debug, Deserialize)]
struct ParsedPackageConfig {
    before_cmds: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
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
            package: self.package
                .map(|pc| pc.into())
                .unwrap_or(PackageConfig::default()),
        }
    }
}

#[derive(Debug)]
pub struct PackageConfig {
    before_cmds: Vec<String>,
    exclude: Vec<String>,
}

impl PackageConfig {
    pub fn before_cmds(&self) -> &Vec<String> {
        &self.before_cmds
    }
    pub fn exclude(&self) -> &Vec<String> {
        &self.exclude
    }
}

impl Into<PackageConfig> for ParsedPackageConfig {
    fn into(self) -> PackageConfig {
        PackageConfig {
            before_cmds: self.before_cmds.unwrap_or(vec![]),
            exclude: self.exclude.unwrap_or(vec![]),
        }
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        PackageConfig {
            before_cmds: vec![],
            exclude: vec![],
        }
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

    let mut config_file = File::create(path_buf)?;

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

fn parse_config(config: String) -> Result<ParsedAppConfig, error::Error> {
    toml::from_str(&config).map_err(|e| {
        error::Error::Other(format!(
            "could not parse krankerl.toml: {}",
            e.description()
        ))
    })
}

pub fn get_config(path: &Path) -> Result<AppConfig, error::Error> {
    let config_str = load_config(path)?;
    parse_config(config_str).map(|config| config.into())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(config.package.is_some());
        let package_config = config.package.unwrap();
        assert!(package_config.before_cmds.is_some());
        let cmds = package_config.before_cmds.unwrap();
        assert_eq!(3, cmds.len());
    }
}
