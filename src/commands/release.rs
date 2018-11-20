use std::path::PathBuf;
use std::thread;

use failure::Error;
use git2::{PushOptions, Repository};
use indicatif::{MultiProgress, ProgressBar};
use nextcloud_appinfo::get_appinfo;

use console::default_spinner;

/// TODO: sanity check: are we on master
/// TODO: sanity check: is master up-to-date?
fn git_tag(app_path: &PathBuf, pb: ProgressBar) -> Result<String, Error> {
    let repo = Repository::open(app_path)?;
    let app_info = get_appinfo(app_path)?;
    let version_str = app_info.version().to_string();
    let tag_name = format!("v{}", version_str);

    if let Ok(existing) = repo.revparse_single(&format!("refs/tags/{}", tag_name)) {
        bail!("tag {} already exists", tag_name);
    }

    let master_rev = repo.revparse_single("refs/heads/master");
    if let Err(e) = master_rev {
        bail!("could not find master rev: {}", e)
    }
    let master_rev = master_rev.unwrap();

    let tag_signature = git2::Signature::now("Krankerl", "krankerl@nextcloud.com")?;
    repo.tag(&tag_name, &master_rev, &tag_signature, "", false)?;

    pb.finish_with_message(&format!("tagged current master as {}", tag_name));

    Ok(version_str)
}

/// TODO: iterate remotes and filter github ones, then take first
fn git_push(app_path: &PathBuf, version_str: &String, pb: ProgressBar) -> Result<(), Error> {
    let repo = Repository::open(app_path)?;
    let origin = repo.find_remote("origin");
    if let Err(e) = origin {
        bail!("could not find a remote named origin");
    }
    let mut origin = origin.unwrap();

    // TODO: Note that you'll likely want to use RemoteCallbacks and set push_update_reference
    //       to test whether all the references were pushed successfully.
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|url, username, allowed| {
        println!("{}, {:?}", url, username);

        let username = username.unwrap_or("git");

        if allowed.contains(git2::CredentialType::USERNAME) {
            git2::Cred::username(username)
        } else {
            Err(git2::Error::from_str("unable to find an appropriate authentication method"))
        }
    });
    let mut opts = PushOptions::new();
    opts.remote_callbacks(callbacks);
    

    origin.push(vec![version_str.as_str()].as_slice(), Some(&mut opts))?;

    Ok(())
}

fn github_create_release(
    app_path: &PathBuf,
    version_str: &String,
    pb: ProgressBar,
) -> Result<(), Error> {
    std::thread::sleep_ms(400);
    pb.finish_with_message("tbh, I did nothing.");

    Ok(())
}


fn github_upload_asset(
    app_path: &PathBuf,
    version_str: &String,
    pb: ProgressBar,
) -> Result<(), Error> {
    std::thread::sleep_ms(400);
    pb.finish_with_message("tbh, I did nothing.");

    Ok(())
}

pub fn release(app_path: &PathBuf) -> Result<(), Error> {
    let m = MultiProgress::new();

    let pb_tag = m.add(default_spinner());
    pb_tag.enable_steady_tick(200);
    pb_tag.set_message("waiting...");
    let pb_push = m.add(default_spinner());
    pb_push.enable_steady_tick(200);
    pb_push.set_message("waiting...");
    let pb_create_release = m.add(default_spinner());
    pb_create_release.enable_steady_tick(200);
    pb_create_release.set_message("waiting...");
    let pb_upload = m.add(default_spinner());
    pb_upload.enable_steady_tick(200);
    pb_upload.set_message("waiting...");

    let app_path = app_path.to_owned();
    let t: std::thread::JoinHandle<Result<(), Error>> = thread::spawn(move || {
        let version_str = git_tag(&app_path, pb_tag)?;
        git_push(&app_path, &version_str, pb_push)?;
        github_create_release(&app_path, &version_str, pb_create_release)?;
        github_upload_asset(&app_path, &version_str, pb_upload)?;
        Ok(())
    });

    m.join()?;
    t.join().unwrap()?;

    Ok(())
}
