mod cmd;
pub mod config;
pub mod download;
pub mod github;
pub mod sources;
pub mod util;

use cmd::run_cmd;
use failure::format_err;
use failure::Error;

pub fn install(
    repo: &str,
    pre_release: bool,
    install_command: &str,
    update_command: &str,
    remove_command: &str,
) -> Result<(), Error> {
    let mut sources = sources::Sources::new();

    if sources.contains(repo) {
        return Err(format_err!("Repository already exists.\nTry `ghr update`"));
    }

    let mut release_info = match pre_release {
        true => github::get_pre_release_info(repo)?,
        false => github::get_release_info(repo)?,
    };

    release_info.install_command = install_command.to_string();
    release_info.update_command = update_command.to_string();
    release_info.remove_command = remove_command.to_string();

    let download_url = &release_info.latest_release.zipball_url.clone();
    let file_name = util::gen_filename(repo, &download_url);
    download::download_file(repo, &download_url, &file_name)?;

    if &release_info.install_command != "" {
        run_cmd(&release_info.install_command)?;
    }

    sources.add_source(repo, release_info)?;
    sources.save()
}

pub fn update() -> Result<(), Error> {
    let mut repos = sources::Sources::new();
    let current_sources = repos.clone();
    for (repo, old_source) in current_sources.sources.iter() {
        let latest_source = match old_source.pre_release {
            true => github::get_pre_release_info(&repo)?,
            false => github::get_release_info(&repo)?,
        };

        if latest_source.is_newer(old_source)? {
            // println!("There's an update for {}", repo);

            // Delete old release from fs
            util::delete_old_release(&repo, &old_source.latest_release.zipball_url)?;

            // Download latest release
            let file_name = util::gen_filename(&repo, &latest_source.latest_release.zipball_url);
            download::download_file(&repo, &latest_source.latest_release.zipball_url, &file_name)?;

            // Run update script
            if &old_source.update_command != "" {
                run_cmd(&old_source.update_command)?;
            }

            // Update source's latest release
            repos.update_latest_release(&repo, latest_source.latest_release)?;
        }
    }
    repos.save()
}

pub fn remove(repo: &str) -> Result<(), Error> {
    let mut repos = sources::Sources::new();

    let source = match repos.sources.get(repo) {
        Some(x) => x,
        None => return Err(format_err!("Repository doesn't exists.\nTry `ghr list`")),
    };

    // Remove from fs
    util::delete_old_release(&repo, &source.latest_release.zipball_url)?;

    // Run remove script
    if &source.remove_command != "" {
        run_cmd(&source.remove_command)?;
    }

    repos.remove_source(&repo)?;
    repos.save()
}

pub fn list() -> Result<(), Error> {
    let mut sources = sources::Sources::new();
    let repos = sources.list()?;
    for repo in repos.iter() {
        println!("\t{}", repo);
    }
    Ok(())
}
