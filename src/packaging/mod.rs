use std::path::PathBuf;

use color_eyre::Result;

mod archive;
mod artifacts;
mod commands;
mod pipeline;

use crate::packaging::pipeline::App;
use color_eyre::eyre::WrapErr;

fn build_archive(app_path: PathBuf) -> Result<()> {
    App::new(app_path)
        .clone()?
        .install_dependencies()
        .wrap_err("Failed to install dependencies")?
        .build()
        .wrap_err("Failed to build app")?
        .into_archive()
        .wrap_err("Failed to build app archive")?;
    Ok(())
}

fn build_shipped(app_path: PathBuf) -> Result<()> {
    App::new(app_path)
        .clone()?
        .install_dependencies()
        .wrap_err("Failed to install dependencies")?
        .build()
        .wrap_err("Failed to build app")?
        .into_shipped()
        .wrap_err("Failed to build app archive")?;
    Ok(())
}

pub fn package_app(app_path: &PathBuf, shipped: bool) -> Result<()> {
    let app_path = app_path.clone();

    if shipped {
        build_shipped(app_path)
    } else {
        build_archive(app_path)
    }
}
