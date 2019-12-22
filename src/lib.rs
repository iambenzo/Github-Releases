pub mod download;
pub mod github;
pub mod sources;
pub mod util;

use failure::format_err;
use failure::Error;

pub fn install(repo: &str) -> Result<(), Error> {
    let mut sources = sources::Sources::new();

    if sources.contains(repo) {
        return Err(format_err!("Repository already exists.\nTry `ghr update`"));
    }

    let release_info = github::get_release_info(repo)?;

    let info = sources::ReleaseInfo::new(
        &release_info.tag_name,
        &release_info.name,
        &release_info.created_at,
        &release_info.published_at,
        &release_info.tarball_url,
        &release_info.zipball_url,
    );
    let source = sources::Source::new("".to_string(), "".to_string(), info);
    sources.add_source(repo, source)?;
    sources.save()?;

    let file_name = util::gen_filename(repo, &release_info.zipball_url);

    if let Err(e) = download::download_file(repo, &release_info.zipball_url, &file_name) {
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
