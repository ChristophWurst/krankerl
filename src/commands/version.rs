use std::fs::OpenOptions;
use std::io::{prelude::*, SeekFrom};
use std::path::Path;

use nextcloud_appinfo;

pub enum VersionChange {
    Major,
    Minor,
    Patch,
}

impl std::str::FromStr for VersionChange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(VersionChange::Major),
            "minor" => Ok(VersionChange::Minor),
            "patch" => Ok(VersionChange::Patch),
            _ => Err(()),
        }
    }
}

fn version_string(v: &nextcloud_appinfo::Version) -> String {
    format!("{}.{}.{}", v.major, v.minor, v.patch)
}

pub fn bump_version(bump: &str) {
    let cwd = Path::new(".");
    let app_info = nextcloud_appinfo::get_appinfo(&cwd).expect("could not get app info");
    println!("current version is {}", app_info.version());
    let mut version = app_info.version().clone();

    match (&bump).parse() {
        Ok(VersionChange::Major) => version.increment_major(),
        Ok(VersionChange::Minor) => version.increment_minor(),
        Ok(VersionChange::Patch) => version.increment_patch(),
        _ => {
            eprintln!("invalid argument supplied. Use major, minor or patch.");
            std::process::exit(2);
        }
    };

    let mut info_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./appinfo/info.xml")
        .expect("could not open info.xml");
    let mut contents = String::new();
    info_file
        .read_to_string(&mut contents)
        .expect("could not read info.xml");
    let new_contents = contents.replace(
        &version_string(app_info.version()),
        &version_string(&version),
    );
    info_file
        .seek(SeekFrom::Start(0))
        .expect("could not reset file write position");
    info_file
        .write_all(new_contents.as_bytes())
        .expect("could not write to info.xml");
    println!("next version is {}", version);
}
