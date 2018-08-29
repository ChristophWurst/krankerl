use std::path::PathBuf;

use chrono::naive::NaiveDate;
use chrono::prelude::*;
use failure::Error;
use nextcloud_changelog::{changelog, generator};

pub fn create_changelog<P>(path: P, prev: String, curr: String) -> Result<(), Error>
where
    P: Into<PathBuf>,
{
    let prev_version = changelog::Version::from(&prev).expect("could not parse `prev`");
    let curr_version = changelog::Version::from(&curr).expect("could not parse `curr`");
    let now = Utc::now();
    let cl = generator::generate_from_git(
        path,
        prev_version,
        curr_version,
        NaiveDate::from_ymd(now.year(), now.month(), now.day()),
    )?;
    let release = cl
        .get_release(changelog::Version::from(&curr).expect("could not parse `curr`"))
        .expect("no changelog for release found");
    let cl_md = changelog::generate_markdown(release.as_markdown());
    let cl_fixed = changelog::fix_markdown_convention(cl_md);

    println!("{}", cl_fixed);

    Ok(())
}
