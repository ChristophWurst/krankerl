use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

use failure::Error;
use nextcloud_appinfo;

pub enum VersionChange {
    Major,
    Minor,
    Patch,
}

impl std::str::FromStr for VersionChange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(VersionChange::Major),
            "minor" => Ok(VersionChange::Minor),
            "patch" => Ok(VersionChange::Patch),
            _ => Err(format_err!("Could not parse version bump type")),
        }
    }
}

fn version_string(v: &nextcloud_appinfo::Version) -> String {
    format!("{}.{}.{}", v.major, v.minor, v.patch)
}

pub fn bump_version(bump: &str) -> Result<(), Error> {
    let cwd = Path::new(".");
    let app_info = nextcloud_appinfo::get_appinfo(&cwd)?;
    println!("current version is {}", app_info.version());
    let mut version = app_info.version().clone();

    match (&bump).parse() {
        Ok(VersionChange::Major) => version.increment_major(),
        Ok(VersionChange::Minor) => version.increment_minor(),
        Ok(VersionChange::Patch) => version.increment_patch(),
        _ => bail!("invalid argument supplied. Use major, minor or patch."),
    };

    let mut info_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./appinfo/info.xml")?;
    let mut contents = String::new();
    info_file.read_to_string(&mut contents)?;
    let new_contents = contents.replace(
        &version_string(app_info.version()),
        &version_string(&version),
    );
    info_file.seek(SeekFrom::Start(0))?;
    info_file.write_all(new_contents.as_bytes())?;
    println!("next version is {}", version);
    Ok(())
}
