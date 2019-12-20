pub mod github;
pub mod util;
pub mod download;
pub mod sources;

use failure::Error;
use failure::format_err;

pub fn install(repo: &str) -> Result<(), Error> {
    let release_info = github::get_release_info(repo)?;
    let file_name = util::gen_filename(repo, &release_info.zipball_url);

    if let Err(e) = download::download_file(repo, &release_info.zipball_url, &file_name) {
        return Err(format_err!("{}", e))
    };

    // let sources = sources::load_sources()?;

    // sources.add_source(repo, source: Source)
    // sources.save()
    Ok(())
}