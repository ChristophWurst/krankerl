extern crate docopt;
extern crate futures;
extern crate nextcloud_appstore;
extern crate krankerl;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use docopt::Docopt;
use futures::Future;
use krankerl::config;
use krankerl::{get_signature, package_app};
use nextcloud_appstore::*;
use tokio_core::reactor::Core;

const USAGE: &'static str = "
Krankerl.

Usage:
  krankerl list apps <version>
  krankerl list categories
  krankerl login [--appstore | --github] <token>
  krankerl package <id>
  krankerl publish (--nightly) <id> <url>
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_id: Option<String>,
    arg_token: Option<String>,
    arg_url: Option<String>,
    arg_version: Option<String>,
    cmd_apps: bool,
    cmd_categories: bool,
    cmd_list: bool,
    cmd_login: bool,
    cmd_package: bool,
    cmd_publish: bool,
    flag_appstore: bool,
    flag_github: bool,
    flag_nightly: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut core = Core::new().unwrap();

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

        core.run(work).unwrap();
    }
}
