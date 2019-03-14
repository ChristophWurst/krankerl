use std::path::PathBuf;
use std::thread;

use failure::Error;
use indicatif::MultiProgress;

mod archive;
mod artifacts;
mod commands;
mod exclude;
mod pipeline;

use console::default_spinner;

pub fn package_app(app_path: &PathBuf) -> Result<(), Error> {
    let app_path = app_path.clone();
    let mp = MultiProgress::new();
    let prog_clone = mp.add(default_spinner());
    prog_clone.enable_steady_tick(200);
    prog_clone.set_message("waiting...");
    let prog_dependencies = mp.add(default_spinner());
    prog_dependencies.enable_steady_tick(200);
    prog_dependencies.set_message("waiting...");
    let prog_package_cmds = mp.add(default_spinner());
    prog_package_cmds.enable_steady_tick(200);
    prog_package_cmds.set_message("waiting...");
    let prog_package = mp.add(default_spinner());
    prog_package.enable_steady_tick(200);
    prog_package.set_message("waiting...");

    let worker = thread::spawn(move || {
        use self::pipeline::App;

        App::new(app_path)
            .clone(Some(prog_clone))?
            .install_dependencies(Some(prog_dependencies))?
            .build(Some(prog_package_cmds))?
            .into_archive(Some(prog_package))
    });

    mp.join()?;
    worker.join().unwrap()?;
    Ok(())
}
