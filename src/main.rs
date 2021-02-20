#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use docopt::Docopt;
use krankerl::*;

const USAGE: &'static str = "
Krankerl. A CLI helper to manage Nextcloud apps.

Usage:
  krankerl clean
  krankerl enable
  krankerl disable
  krankerl init
  krankerl login (--appstore | --github) <token>
  krankerl package [--shipped]
  krankerl publish [--nightly] <url>
  krankerl sign --package
  krankerl up
  krankerl version (major|minor|patch)
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_token: Option<String>,
    arg_url: Option<String>,
    arg_version: Option<String>,
    cmd_clean: bool,
    cmd_enable: bool,
    cmd_disable: bool,
    cmd_init: bool,
    cmd_login: bool,
    cmd_package: bool,
    cmd_publish: bool,
    cmd_sign: bool,
    cmd_up: bool,
    cmd_version: bool,
    cmd_major: bool,
    cmd_minor: bool,
    flag_appstore: bool,
    flag_github: bool,
    flag_nightly: bool,
    flag_package: bool,
    flag_shipped: bool,
    flag_version: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_enable {
        krankerl::commands::enable_app()?;
    } else if args.cmd_disable {
        krankerl::commands::disable_app()?;
    } else if args.cmd_init {
        let cwd = Path::new(".");
        krankerl::commands::init(&cwd).wrap_err("could not create krankerl.toml")?;
        println!("krankerl.toml created.");
    } else if args.cmd_clean {
        let cwd = PathBuf::from(".");
        krankerl::commands::clean(&cwd)?;
    } else if args.cmd_login {
        if args.flag_appstore {
            let token = args.arg_token.unwrap();
            krankerl::commands::log_in_to_appstore(&token)
                .wrap_err("could not save appstore token")?;
        } else if args.flag_github {
            let token = args.arg_token.unwrap();
            krankerl::commands::log_in_to_github(&token).wrap_err("could not save github token")?;
        }
    } else if args.cmd_package {
        krankerl::commands::package_app(&PathBuf::from("."), args.flag_shipped)
            .wrap_err("could not package app")?;
    } else if args.cmd_publish {
        let url = args.arg_url.unwrap();
        let is_nightly = args.flag_nightly;

        let signature = krankerl::commands::sign_package().wrap_err("Could not sign package")?;
        let config = config::krankerl::get_config().wrap_err("could not load config")?;

        let api_token = match config.appstore_token {
            None => {
                println!("No appstore token set, run: krankerl login --appstore <token>");
                return Ok(());
            }
            Some(api_token) => api_token,
        };

        publish_app(&url, is_nightly, &signature, &api_token).await?;
        println!("app released successfully");
    } else if args.cmd_sign && args.flag_package {
        let signature = krankerl::commands::sign_package()?;
        println!("Package signature: {}", signature);
    } else if args.cmd_up {
        let cwd = PathBuf::from(".");
        krankerl::commands::up(&cwd)?;
    } else if args.cmd_version {
        let bump = if args.cmd_major {
            "major"
        } else if args.cmd_minor {
            "minor"
        } else {
            "patch"
        };

        krankerl::commands::bump_version(bump).wrap_err("Could not bump version")?;
    } else if args.flag_version {
        eprintln!(env!("CARGO_PKG_VERSION"));
    }

    Ok(())
}
