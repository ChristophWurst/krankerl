use std::path::PathBuf;
use std::thread;

use failure::Error;
use indicatif::{MultiProgress, ProgressBar};

mod archive;
mod artifacts;
mod commands;
mod exclude;
mod pipeline;

use crate::console::default_spinner;
use crate::packaging::pipeline::App;

fn build_archive(
    app_path: PathBuf,
    prog_clone: ProgressBar,
    prog_dependencies: ProgressBar,
    prog_build: ProgressBar,
    prog_package: ProgressBar,
) -> Result<(), Error> {
    App::new(app_path)
        .clone(Some(prog_clone))?
        .install_dependencies(Some(prog_dependencies))?
        .build(Some(prog_build))?
        .into_archive(Some(prog_package))?;
    Ok(())
}

fn build_shipped(
    app_path: PathBuf,
    prog_clone: ProgressBar,
    prog_dependencies: ProgressBar,
    prog_build: ProgressBar,
    prog_package: ProgressBar,
) -> Result<(), Error> {
    App::new(app_path)
        .clone(Some(prog_clone))?
        .install_dependencies(Some(prog_dependencies))?
        .build(Some(prog_build))?
        .into_shipped(Some(prog_package))?;
    Ok(())
}

pub fn package_app(app_path: &PathBuf, shipped: bool) -> Result<(), Error> {
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
        if shipped {
            build_shipped(
                app_path,
                prog_clone,
                prog_dependencies,
                prog_package_cmds,
                prog_package,
            )
        } else {
            build_archive(
                app_path,
                prog_clone,
                prog_dependencies,
                prog_package_cmds,
                prog_package,
            )
        }
    });

    mp.join()?;
    worker.join().unwrap()?;
    Ok(())
}
