extern crate docopt;
extern crate futures;
extern crate krankerl;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use docopt::Docopt;
use futures::future::Future;
use krankerl::*;
use tokio_core::reactor::Core;

const USAGE: &'static str = "
Krankerl.

Usage:
  krankerl list apps <version>
  krankerl list categories
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_version: Option<String>,
    cmd_list: bool,
    cmd_apps: bool,
    cmd_categories: bool,
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
                println!("- {}", app.id)
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
    }
}
