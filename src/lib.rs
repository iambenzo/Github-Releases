pub mod github;
pub mod util;
pub mod download;

use failure::Error;

pub fn install(repo: &str) -> Result<(String, usize), Error> {
    let download_url = github::get_download_url(repo)?;
    let file_name = util::gen_filename(repo, &download_url);
    download::download_file(repo, &download_url, &file_name)
}