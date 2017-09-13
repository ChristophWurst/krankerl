extern crate docopt;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate krankerl;

use docopt::Docopt;
use futures::future::Future;
use krankerl::get_apps_and_releases;
use tokio_core::reactor::Core;

const USAGE: &'static str = "
Krankerl.

Usage:
  krankerl list apps <version>
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_version: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    let mut core = Core::new().unwrap();
    let work = get_apps_and_releases(&core.handle(), &args.arg_version).map(|apps| {
        println!("found {} apps:", apps.len());
        for app in apps {
            println!("{:?}", app)
        }
    });

    core.run(work).unwrap();
}
