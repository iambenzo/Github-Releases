mod gh_prerelease;
mod gh_release;

use crate::sources::{Source, Sourceable};

use failure::format_err;
use failure::Error;
use reqwest;

pub fn get_release_info(repo: &str) -> Result<Source, Error> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    // println!("Request URL: {}", url);

    let mut response = reqwest::get(&url)?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(format_err!(
            "No release information available.\nTry using the `--pre-release` flag."
        ));
    }

    let latest_release: gh_release::Release = response.json()?;
    Ok(latest_release.to_source())
}

pub fn get_pre_release_info(repo: &str) -> Result<Source, Error> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    // println!("Request URL: {}", url);

    let mut response = reqwest::get(&url)?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(format_err!("No release information available."));
    }

    let json: Vec<gh_prerelease::PreRelease> = response.json()?;

    let latest_release: gh_prerelease::PreRelease = match json.first() {
        Some(x) => x.clone(),
        None => return Err(format_err!("No release information available.")),
    };

    Ok(latest_release.to_source())
}
