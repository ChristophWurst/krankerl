use std::path::PathBuf;
use std::thread;

use failure::Error;

mod archive;
mod artifacts;
mod commands;
mod pipeline;

use crate::packaging::pipeline::App;

fn build_archive(app_path: PathBuf) -> Result<(), Error> {
    App::new(app_path)
        .clone()?
        .install_dependencies()?
        .build()?
        .into_archive()?;
    Ok(())
}

fn build_shipped(app_path: PathBuf) -> Result<(), Error> {
    App::new(app_path)
        .clone()?
        .install_dependencies()?
        .build()?
        .into_shipped()?;
    Ok(())
}

pub fn package_app(app_path: &PathBuf, shipped: bool) -> Result<(), Error> {
    let app_path = app_path.clone();

    let worker = thread::spawn(move || {
        if shipped {
            build_shipped(app_path)
        } else {
            build_archive(app_path)
        }
    });

    worker.join().unwrap()?;
    Ok(())
}
