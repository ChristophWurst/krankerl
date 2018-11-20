use std::path::PathBuf;
use std::thread;

use failure::Error;
use git2::{PushOptions, Repository};
use hubcaps::{Credentials, Github};
use indicatif::{MultiProgress, ProgressBar};
use regex::Regex;
use nextcloud_appinfo::get_appinfo;

use config;
use console::default_spinner;

/// TODO: sanity check: are we on master
/// TODO: sanity check: is master up-to-date?
fn git_tag(app_path: &PathBuf) -> Result<String, Error> {
    let repo = Repository::open(app_path)?;
    let app_info = get_appinfo(app_path)?;
    let version_str = app_info.version().to_string();
    let tag_name = format!("v{}", version_str);

    if let Ok(existing) = repo.revparse_single(&format!("refs/tags/{}", tag_name)) {
        println!("tag {} already exists", tag_name);
    } else {
        let master_rev = repo.revparse_single("refs/heads/master");
        if let Err(e) = master_rev {
            bail!("could not find master rev: {}", e)
        }
        let master_rev = master_rev.unwrap();

        let tag_signature = git2::Signature::now("Krankerl", "krankerl@nextcloud.com")?;
        repo.tag(&tag_name, &master_rev, &tag_signature, "", false)?;

        println!("tagged current master as {}", tag_name);
    }

    Ok(tag_name)
}

/// TODO: iterate remotes and filter github ones, then take first
fn git_push(app_path: &PathBuf, version_str: &String) -> Result<(), Error> {
    let repo = Repository::open(app_path)?;
    let repo_config = repo.config()?;
    let origin = repo.find_remote("origin");
    if let Err(e) = origin {
        bail!("could not find a remote named origin");
    }
    let mut origin = origin.unwrap();

    // TODO: Note that you'll likely want to use RemoteCallbacks and set push_update_reference
    //       to test whether all the references were pushed successfully.
    let mut callbacks = git2::RemoteCallbacks::new();
    let mut cred_helper = git2::CredentialHelper::new("foo");
    cred_helper.config(&repo_config);
    callbacks.credentials(move |url, username, allowed| {
        if allowed.contains(git2::CredentialType::SSH_KEY) {
            let user = username
                .map(|s| s.to_string())
                .or_else(|| cred_helper.username.clone())
                .unwrap_or("git".to_string());
            git2::Cred::ssh_key_from_agent(&user)
        } else if allowed.contains(git2::CredentialType::USERNAME) {
            let username = username.unwrap_or("git");
            git2::Cred::username(username)
        } else {
            Err(git2::Error::from_str(
                "unable to find an appropriate authentication method",
            ))
        }
    });
    let mut opts = PushOptions::new();
    opts.remote_callbacks(callbacks);

    origin.push(
        vec![format!("refs/tags/{}", version_str).as_str()].as_slice(),
        Some(&mut opts),
    )?;

    println!("pushed tag {} to origin", version_str);

    Ok(())
}

fn remote_to_user_repo(url: &str) -> Option<(String, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(github.com):?/?([a-zA-Z0-9]+)/([a-zA-Z0-9]+)").unwrap();
    }
    match (RE.captures(url)) {
        Some(matches) => match ((matches.get(2), matches.get(3))) {
            (Some(username), Some(repo)) => {
                Some((username.as_str().to_owned(), repo.as_str().to_owned()))
            }
            _ => None,
        },
        _ => None,
    }
}

fn find_github_remote(app_path: &PathBuf) -> Result<(String, String), Error> {
    let repo = Repository::open(app_path)?;
    let remotes = repo.remotes()?;

    let mut github_remotes = remotes.into_iter().filter_map(|r| {
        r.map(|remote_name| {
            repo.find_remote(remote_name)
                .map(|remote| match (remote.url()) {
                    Some(url) => remote_to_user_repo(url),
                    _ => None,
                })
                .unwrap_or(None)
        }).unwrap_or(None)
    });
    let first = github_remotes.next();
    match (first) {
        Some(s) => Ok(s),
        None => Err(format_err!("No GitHub remote found.")),
    }
}

fn github_create_release(
    app_path: &PathBuf,
    version_str: &String,
    github_token: String,
) -> Result<(), Error> {
    let remote = find_github_remote(app_path)?;
    println!("Using GitHub repository {}/{}", remote.0, remote.1);
    let github = Github::new("krankerl", Credentials::Token(github_token));

    Ok(())
}

fn github_upload_asset(app_path: &PathBuf, version_str: &String) -> Result<(), Error> {
    Ok(())
}

pub fn release(app_path: &PathBuf) -> Result<(), Error> {
    let app_path = app_path.to_owned();
    let config = config::krankerl::get_config()?;
    if config.github_token.is_none() {
        bail!("No GitHub access token set. Use `krankerl login --github` to set it.")
    }
    let github_token = config.github_token.unwrap();

    let version_str = git_tag(&app_path)?;
    git_push(&app_path, &version_str)?;
    github_create_release(&app_path, &version_str, github_token)?;
    github_upload_asset(&app_path, &version_str)?;

    Ok(())
}
