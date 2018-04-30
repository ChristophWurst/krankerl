use std::path::PathBuf;

use failure::Error;
use composer::Composer;
use npm_scripts::NpmScripts;

fn npm_up(app_path: &PathBuf) -> Result<(), Error> {
    let npm_script = "build".to_owned();
    let scripts = NpmScripts::new(app_path);

    if !scripts.is_available() {
        println!("No npm config found, skipping npm installation.");
        return Ok(());
    } else {
        println!("Found npm config, installing packages …");
        scripts.install()?;
        println!("Installed npm packages.");
    }

    let has_npm_build_task = scripts.has_script(&npm_script)?;
    if has_npm_build_task {
        println!("Found npm build script, running it …");
        scripts.run_script(&npm_script)?;
        println!("Ran npm build script.");
    } else {
        println!("No npm build task found, skipping build step.");
    }
    Ok(())
}

fn composer_up(app_path: &PathBuf) -> Result<(), Error> {
    let composer = Composer::new(app_path);
    if composer.is_available() {
        println!("Found composer config, installing packages …");
        composer.install()?;
        println!("Installed composer packages.");
    } else {
        println!("No composer config found, skipping composer installation.");
    }
    Ok(())
}

pub fn up(app_path: &PathBuf) -> Result<(), Error> {
    npm_up(app_path)?;
    composer_up(app_path)?;
    Ok(())
}
