#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

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
async fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_enable {
        krankerl::commands::enable_app().unwrap_or_else(|e| {
            println!("an error occured: {}", e);
        });
    } else if args.cmd_disable {
        krankerl::commands::disable_app().unwrap_or_else(|e| {
            println!("an error occured: {}", e);
        });
    } else if args.cmd_init {
        let cwd = Path::new(".");
        match krankerl::commands::init(&cwd) {
            Ok(_) => println!("krankerl.toml created."),
            Err(e) => println!("could not create krankerl.toml: {}", e),
        };
    } else if args.cmd_clean {
        let cwd = PathBuf::from(".");
        krankerl::commands::clean(&cwd).unwrap_or_else(|e| {
            println!("an error occured: {}", e);
        });
    } else if args.cmd_login {
        if args.flag_appstore {
            let token = args.arg_token.unwrap();
            krankerl::commands::log_in_to_appstore(&token).expect("could not save appstore token");
        } else if args.flag_github {
            let token = args.arg_token.unwrap();
            krankerl::commands::log_in_to_github(&token).expect("could not save github token");
        }
    } else if args.cmd_package {
        krankerl::commands::package_app(&PathBuf::from("."), args.flag_shipped)
            .unwrap_or_else(|e| println!("could not package app: {}", e));
    } else if args.cmd_publish {
        let url = args.arg_url.unwrap();
        let is_nightly = args.flag_nightly;

        match krankerl::commands::sign_package() {
            Ok(signature) => {
                let config = config::krankerl::get_config().expect("could not load config");

                if !config.appstore_token.is_some() {
                    println!("No appstore token set, run: krankerl login --appstore <token>");
                    return;
                }
                let api_token = config.appstore_token.unwrap();

                match publish_app(&url, is_nightly, &signature, &api_token).await {
                    Ok(_) => {
                        println!("app released successfully");
                    }
                    Err(e) => {
                        eprintln!("an error occured: {:?}", e);
                    }
                };
            }
            Err(err) => {
                eprintln!("Could not sign package: {}", err);
            }
        }
    } else if args.cmd_sign && args.flag_package {
        let signature = krankerl::commands::sign_package();
        match signature {
            Ok(signature) => println!("Package signature: {}", signature),
            Err(err) => eprintln!("an error occured: {}", err),
        }
    } else if args.cmd_up {
        let cwd = PathBuf::from(".");
        krankerl::commands::up(&cwd).unwrap_or_else(|e| {
            eprintln!("an error occured: {}", e);
        });
    } else if args.cmd_version {
        let bump = if args.cmd_major {
            "major"
        } else if args.cmd_minor {
            "minor"
        } else {
            "patch"
        };

        krankerl::commands::bump_version(bump)
            .unwrap_or_else(|e| eprintln!("Could not bump version: {}", e))
    } else if args.flag_version {
        eprintln!(env!("CARGO_PKG_VERSION"));
    }
}
