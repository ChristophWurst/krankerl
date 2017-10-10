extern crate docopt;
extern crate futures;
extern crate futures_cpupool;
extern crate krankerl;
extern crate nextcloud_appstore;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use docopt::Docopt;
use futures::Future;
use krankerl::config;
use krankerl::get_signature;
use krankerl::packaging::package_app;
use krankerl::sign::{sign_app, sign_package_async};
use nextcloud_appstore::*;
use std::path::PathBuf;
use tokio_core::reactor::Core;

const USAGE: &'static str = "
Krankerl. A CLI tool for the Nextcloud app store.

Usage:
  krankerl list apps <version>
  krankerl list categories
  krankerl login [--appstore | --github] <token>
  krankerl package <id>
  krankerl publish (--nightly) <id> <url>
  krankerl sign [--app <keypath> | --package] <packagepath>
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_id: Option<String>,
    arg_keypath: Option<String>,
    arg_packagepath: Option<String>,
    arg_token: Option<String>,
    arg_url: Option<String>,
    arg_version: Option<String>,
    cmd_apps: bool,
    cmd_categories: bool,
    cmd_list: bool,
    cmd_login: bool,
    cmd_package: bool,
    cmd_publish: bool,
    cmd_sign: bool,
    flag_app: bool,
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
    let mut pool_builder = futures_cpupool::Builder::new();
    pool_builder.pool_size(2);

    if args.cmd_list && args.cmd_apps {
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
    } else if args.cmd_login {
        if args.flag_appstore {
            let token = args.arg_token.unwrap();
            config::set_appstore_token(&token).expect("could not save appstore token");
            println!("App store token saved.");
        }
    } else if args.cmd_package {
        let app_id = args.arg_id.unwrap();

        package_app(&app_id).expect("could not package app");
        println!("Packaged app {}.", app_id);
    } else if args.cmd_publish {
        let app_id = args.arg_id.unwrap();
        let url = args.arg_url.unwrap();
        let is_nightly = args.flag_nightly;

        package_app(&app_id).expect("could not package app");
        let sig = get_signature(&app_id).expect("could not get signature");

        let config = config::get_config().expect("could not load config");
        assert!(config.appstore_token.is_some());
        let api_token = config.appstore_token.unwrap();

        let work = publish_app(&core.handle(), &url, is_nightly, &sig, &api_token);

        core.run(work).unwrap_or_else(|e| {
            println!("an error occured: {}", e);
        });
    } else if args.cmd_sign && args.flag_app {
        let path = args.arg_packagepath.unwrap();
        let path = PathBuf::from(&path);

        match sign_app(&path) {
            Ok(()) => println!("App signed successfully"),
            Err(err) => println!("Error signing app: {}", err),
        };
    } else if args.cmd_sign && args.flag_package {
        let path1 = args.arg_keypath.unwrap();
        let path2 = args.arg_packagepath.unwrap();
        let key_path = PathBuf::from(&path1);
        let package_path = PathBuf::from(&path2);

        let signing = sign_package_async(&mut pool_builder, key_path, package_path);
        let work = signing.and_then(|signature| {
            println!("Package signature: {}", signature);
            futures::future::ok(())
        });

        core.run(work).unwrap_or_else(|e| {
            println!("an error occured: {}", e);
        });
    } else if args.flag_version {
        println!("v0.1.1");
    }
}
