mod gh_release;

use failure::format_err;
use failure::Error;
use reqwest;

pub fn get_release_info(repo: &str) -> Result<gh_release::Release, Error> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    // println!("Request URL: {}", url);

    let mut response = reqwest::get(&url)?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(format_err!(
            "No release information available.\nTry using the `--pre-release` flag."
        ));
    }

    let latest_release: gh_release::Release = response.json()?;
    Ok(latest_release)
}
