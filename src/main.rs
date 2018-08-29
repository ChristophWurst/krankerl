extern crate docopt;
extern crate futures;
extern crate krankerl;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use std::path::{Path, PathBuf};

use docopt::Docopt;
use futures::{future, Future};
use krankerl::packaging::package_app;
use krankerl::*;
use tokio_core::reactor::Core;

const USAGE: &'static str = "
Krankerl. A CLI helper to manage Nextcloud apps.

Usage:
  krankerl clean
  krankerl enable
  krankerl disable
  krankerl init
  krankerl list apps <version>
  krankerl list categories
  krankerl login (--appstore | --github) <token>
  krankerl package
  krankerl changelog <prevversion> <currversion>
  krankerl publish [--nightly] <url>
  krankerl sign --package
  krankerl up
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
    arg_prevversion: Option<String>,
    arg_currversion: Option<String>,
    cmd_apps: bool,
    cmd_categories: bool,
    cmd_changelog: bool,
    cmd_clean: bool,
    cmd_enable: bool,
    cmd_disable: bool,
    cmd_init: bool,
    cmd_list: bool,
    cmd_login: bool,
    cmd_package: bool,
    cmd_publish: bool,
    cmd_sign: bool,
    cmd_up: bool,
    flag_appstore: bool,
    flag_github: bool,
    flag_nightly: bool,
    flag_package: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut core = Core::new().unwrap();

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
    } else if args.cmd_list && args.cmd_apps {
        let version = &args.arg_version.unwrap();

        let work = get_apps_and_releases(&core.handle(), &version.to_owned()).map(|apps| {
            println!("found {} apps for {}:", apps.len(), version);
            for app in apps {
                if app.isFeatured {
                    println!("- {} (featured)", app.id);
                } else {
                    println!("- {}", app.id);
                }
            }
        });

        core.run(work).unwrap();
    } else if args.cmd_list && args.cmd_categories {
        let work = get_categories(&core.handle()).map(|cats| {
            println!("found {} categories:", cats.len());
            for cat in cats {
                println!("- {}", cat.id)
            }
        });

        core.run(work).unwrap();
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
        package_app().unwrap_or_else(|e| println!("could not package app: {}", e));
    } else if args.cmd_changelog {
        krankerl::commands::create_changelog(
            ".",
            args.arg_prevversion.expect("no prevversion found"),
            args.arg_currversion.expect("no currversion found"),
        ).unwrap_or_else(|e| {
            println!("an error occured: {:?}", e);
        })
    } else if args.cmd_publish {
        let url = args.arg_url.unwrap();
        let is_nightly = args.flag_nightly;

        let signing = future::lazy(|| krankerl::commands::sign_package());
        let handle = core.handle();

        let work = signing
            .and_then(|signature| {
                let config = config::krankerl::get_config().expect("could not load config");
                assert!(config.appstore_token.is_some());
                let api_token = config.appstore_token.unwrap();

                publish_app(&handle, &url, is_nightly, &signature, &api_token)
            })
            .and_then(|_| {
                println!("app released successfully");
                Ok(())
            });

        core.run(work).unwrap_or_else(|e| {
            println!("an error occured: {:?}", e);
        });
    } else if args.cmd_sign && args.flag_package {
        let signature = krankerl::commands::sign_package();
        match signature {
            Ok(signature) => println!("Package signature: {}", signature),
            Err(err) => println!("an error occured: {}", err),
        }
    } else if args.cmd_up {
        let cwd = PathBuf::from(".");
        krankerl::commands::up(&cwd).unwrap_or_else(|e| {
            println!("an error occured: {}", e);
        });
    } else if args.flag_version {
        println!(env!("CARGO_PKG_VERSION"));
    }
}
