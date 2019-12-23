pub mod download;
pub mod github;
pub mod sources;
pub mod util;

use failure::format_err;
use failure::Error;

pub fn install(repo: &str, pre_release: bool) -> Result<(), Error> {
    let mut sources = sources::Sources::new();

    if sources.contains(repo) {
        return Err(format_err!("Repository already exists.\nTry `ghr update`"));
    }

    let release_info  = {
        if pre_release {
            github::get_pre_release_info(repo)?
        } else {
            github::get_release_info(repo)?
        }
    };

    let download_url = &release_info.latest_release.zipball_url.clone();
    
    sources.add_source(repo, release_info)?;
    sources.save()?;

    let file_name = util::gen_filename(repo, &download_url);

    if let Err(e) = download::download_file(repo, &download_url, &file_name) {
        return Err(format_err!("{}", e));
    } else {
        return Ok(());
    };
}

pub fn list() -> Result<(), Error> {
    let mut sources = sources::Sources::new();
    let repos = sources.list()?;
    for repo in repos.iter() {
        println!("\t{}", repo);
    }
    Ok(())
}
