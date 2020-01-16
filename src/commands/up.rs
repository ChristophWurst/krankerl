use std::path::PathBuf;
use std::thread;

use composer::Composer;
use failure::Error;
use npm_scripts::NpmScripts;

fn find_npm_scripts(app_path: &PathBuf) -> Option<NpmScripts> {
    let in_root = NpmScripts::new(app_path);
    let in_js = NpmScripts::new(app_path.join("js"));

    if in_root.is_available() {
        Some(in_root)
    } else if in_js.is_available() {
        Some(in_js)
    } else {
        None
    }
}

fn npm_up(app_path: &PathBuf) -> Result<(), Error> {
    let npm_script = "build".to_owned();

    match find_npm_scripts(app_path) {
        Some(scripts) => {
            println!("Installing npm packages...");
            scripts.install()?;
            let has_npm_build_task = scripts.has_script(&npm_script)?;
            if has_npm_build_task {
                println!("Running npm build script...");
                scripts.run_script(&npm_script)?;
                println!("Installed npm packages and ran build script.");
            } else {
                println!("Installed npm packages.");
            }
            Ok(())
        }
        None => {
            println!("No npm config found.");
            Ok(())
        }
    }
}

fn composer_up(app_path: &PathBuf) -> Result<(), Error> {
    let composer = Composer::new(app_path);
    if composer.is_available() {
        println!("Installing composer packages...");
        composer.install()?;
        println!("Installed composer packages.");
    } else {
        println!("No composer config found.");
    }
    Ok(())
}

pub fn up(app_path: &PathBuf) -> Result<(), Error> {
    let p1 = app_path.to_owned();
    let t1 = thread::spawn(move || npm_up(&p1));
    let p2 = app_path.to_owned();
    let t2 = thread::spawn(move || composer_up(&p2));

    t1.join().unwrap()?;
    t2.join().unwrap()?;

    Ok(())
}
