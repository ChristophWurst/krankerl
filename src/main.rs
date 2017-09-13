extern crate docopt;
extern crate dotenv;
extern crate futures;
extern crate krankerl;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use docopt::Docopt;
use dotenv::dotenv;
use futures::future::Future;
use krankerl::*;
use std::env;
use tokio_core::reactor::Core;

const USAGE: &'static str = "
Krankerl.

Usage:
  krankerl list apps <version>
  krankerl list categories
  krankerl release (--nightly) <id> <url>
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_id: Option<String>,
    arg_url: Option<String>,
    arg_version: Option<String>,
    cmd_list: bool,
    cmd_apps: bool,
    cmd_categories: bool,
    cmd_release: bool,
    flag_nightly: bool,
}

fn main() {
    dotenv().ok();
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
    } else if args.cmd_release {
        let app_id = args.arg_id.unwrap();
        let url = args.arg_url.unwrap();
        let is_nightly = args.flag_nightly;
        let api_token = env::var("TOKEN").unwrap();

        let work = release_app(&core.handle(), &app_id, &url, is_nightly, &api_token);

        core.run(work).unwrap();
    }
}
